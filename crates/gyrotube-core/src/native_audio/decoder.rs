use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rubato::{FftFixedIn, Resampler};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;
use tokio::sync::watch;

use crate::audio_streamer::ChunkProgress;
use crate::native_audio::protocol::DeckId;

/// Ring buffer capacity: 2.5 seconds of interleaved stereo f32 at the target device rate.
const RING_BUFFER_SECS: f32 = 2.5;
const RESAMPLE_CHUNK_FRAMES: usize = 1024;

/// Audio input source for a native deck decoder.
pub enum DecoderSource {
    /// Complete audio file on disk (cache hit or offline download).
    LocalFile { path: PathBuf, ext_hint: Option<String> },
    /// In-flight progressive file being written by `AudioStreamer`.
    ProgressiveFile {
        path: PathBuf,
        ext_hint: Option<String>,
        progress_rx: watch::Receiver<ChunkProgress>,
    },
}

/// Handle to a running deck decoder thread; dropping or calling `cancel()` stops it cleanly.
pub struct DecoderHandle {
    cancel: Arc<AtomicBool>,
}

impl DecoderHandle {
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

impl Drop for DecoderHandle {
    fn drop(&mut self) {
        self.cancel();
    }
}

/// Custom Symphonia `MediaSource` that reads from a file being progressively written by
/// `AudioStreamer`, waiting transparently when the read cursor catches up to `written`.
pub struct ProgressiveFileSource {
    file: File,
    pos: u64,
    progress_rx: Option<watch::Receiver<ChunkProgress>>,
    cancel: Arc<AtomicBool>,
    known_len: Option<u64>,
}

impl ProgressiveFileSource {
    pub fn open_local(path: &PathBuf, cancel: Arc<AtomicBool>) -> io::Result<Self> {
        let file = File::open(path)?;
        let len = file.metadata().ok().map(|m| m.len());
        Ok(Self {
            file,
            pos: 0,
            progress_rx: None,
            cancel,
            known_len: len,
        })
    }

    pub fn open_progressive(
        path: &PathBuf,
        progress_rx: watch::Receiver<ChunkProgress>,
        cancel: Arc<AtomicBool>,
    ) -> io::Result<Self> {
        let file = File::open(path)?;
        let known_len = progress_rx.borrow().total_bytes;
        Ok(Self {
            file,
            pos: 0,
            progress_rx: Some(progress_rx),
            cancel,
            known_len,
        })
    }

    fn wait_for_bytes(&self, min_end: u64) -> io::Result<bool> {
        let Some(ref rx) = self.progress_rx else {
            return Ok(true);
        };
        let mut waited_ms = 0u32;
        loop {
            if self.cancel.load(Ordering::Relaxed) {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "decoder cancelled"));
            }
            let snap = rx.borrow().clone();
            if snap.written >= min_end {
                return Ok(true);
            }
            if snap.done {
                if let Some(err) = snap.error {
                    return Err(io::Error::other(err));
                }
                return Ok(snap.written > self.pos);
            }
            if waited_ms >= 15_000 {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "Timed out waiting for progressive audio bytes",
                ));
            }
            thread::sleep(Duration::from_millis(12));
            waited_ms += 12;
        }
    }
}

impl Read for ProgressiveFileSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if self.progress_rx.is_some() {
            let has_more = self.wait_for_bytes(self.pos + 1)?;
            if !has_more {
                return Ok(0);
            }
        }
        let n = self.file.read(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl Seek for ProgressiveFileSource {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let target = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(delta) => self.pos.saturating_add_signed(delta),
            SeekFrom::End(delta) => {
                let len = self
                    .progress_rx
                    .as_ref()
                    .and_then(|rx| rx.borrow().total_bytes)
                    .or(self.known_len)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::Unsupported, "Unknown progressive length")
                    })?;
                len.saturating_add_signed(delta)
            }
        };
        if self.progress_rx.is_some() {
            let _ = self.wait_for_bytes(target.saturating_add(1))?;
        }
        let actual = self.file.seek(SeekFrom::Start(target))?;
        self.pos = actual;
        Ok(actual)
    }
}

impl MediaSource for ProgressiveFileSource {
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        self.progress_rx
            .as_ref()
            .and_then(|rx| rx.borrow().total_bytes)
            .or(self.known_len)
    }
}

pub struct DecodedDeckStream {
    pub consumer: rtrb::Consumer<f32>,
    pub handle: DecoderHandle,
    pub total_frames: u64,
    pub start_frame: u64,
    pub duration_secs: f64,
}

/// Opens a file with Symphonia, creates an SPSC `rtrb` ring buffer at `device_sample_rate`,
/// and spawns a dedicated decoder thread (with optional `rubato` resampling) that streams
/// interleaved stereo `f32` frames lock-free into the real-time audio engine.
pub fn spawn_deck_decoder(
    deck: DeckId,
    source: DecoderSource,
    device_sample_rate: u32,
    start_secs: f64,
) -> Result<DecodedDeckStream, String> {
    let cancel = Arc::new(AtomicBool::new(false));
    let (media_source, ext_hint): (Box<dyn MediaSource>, Option<String>) = match source {
        DecoderSource::LocalFile { path, ext_hint } => {
            let ext = ext_hint.or_else(|| {
                path.extension()
                    .and_then(|s| s.to_str())
                    .map(str::to_ascii_lowercase)
            });
            let src = ProgressiveFileSource::open_local(&path, Arc::clone(&cancel))
                .map_err(|e| format!("Failed to open audio file {}: {e}", path.display()))?;
            (Box::new(src), ext)
        }
        DecoderSource::ProgressiveFile {
            path,
            ext_hint,
            progress_rx,
        } => {
            let ext = ext_hint.or_else(|| {
                path.extension()
                    .and_then(|s| s.to_str())
                    .map(str::to_ascii_lowercase)
            });
            let src =
                ProgressiveFileSource::open_progressive(&path, progress_rx, Arc::clone(&cancel))
                    .map_err(|e| {
                        format!("Failed to open progressive audio {}: {e}", path.display())
                    })?;
            (Box::new(src), ext)
        }
    };

    let mss = MediaSourceStream::new(media_source, Default::default());
    let mut hint = Hint::new();
    if let Some(ref ext) = ext_hint {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions {
                enable_gapless: true,
                ..Default::default()
            },
            &MetadataOptions::default(),
        )
        .map_err(|e| format!("Symphonia probe failed: {e}"))?;

    let mut format = probed.format;
    let codecs = symphonia::default::get_codecs();
    let (track_id, src_sample_rate, src_channels, duration_secs, mut decoder) = {
        let mut selected = None;
        let mut last_err = String::from("No decodable audio track found");
        for t in format.tracks() {
            if t.codec_params.codec == CODEC_TYPE_NULL {
                continue;
            }
            // Skip video tracks in muxed MP4 containers (e.g., itag 18 has H.264 track 0 + AAC track 1)
            match codecs.make(&t.codec_params, &DecoderOptions::default()) {
                Ok(dec) => {
                    let sr = t.codec_params.sample_rate.unwrap_or(44_100);
                    let ch = t.codec_params.channels.map(|c| c.count()).unwrap_or(2);
                    let dur = t
                        .codec_params
                        .n_frames
                        .map(|n| n as f64 / sr.max(1) as f64)
                        .unwrap_or(0.0);
                    selected = Some((t.id, sr, ch, dur, dec));
                    break;
                }
                Err(e) => {
                    last_err = format!(
                        "Symphonia codec init failed (codec={:?}): {e}",
                        t.codec_params.codec
                    );
                }
            }
        }
        selected.ok_or(last_err)?
    };

    let clamped_start = if duration_secs > 0.0 {
        start_secs.clamp(0.0, (duration_secs - 0.1).max(0.0))
    } else {
        start_secs.max(0.0)
    };
    if clamped_start > 0.05 {
        let seek_time = Time::new(clamped_start.trunc() as u64, clamped_start.fract());
        let _ = format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time: seek_time,
                track_id: Some(track_id),
            },
        );
    }

    let total_frames = (duration_secs * device_sample_rate as f64).round() as u64;
    let start_frame = (clamped_start * device_sample_rate as f64).round() as u64;
    let cap_samples = ((device_sample_rate as f32 * RING_BUFFER_SECS) as usize * 2).max(16_384);
    let (mut producer, consumer) = rtrb::RingBuffer::<f32>::new(cap_samples);

    let cancel_bg = Arc::clone(&cancel);
    let thread_name = match deck {
        DeckId::A => "gyrotube-decode-a",
        DeckId::B => "gyrotube-decode-b",
    };

    thread::Builder::new()
        .name(thread_name.into())
        .spawn(move || {
            let mut resampler = if src_sample_rate != device_sample_rate {
                FftFixedIn::<f32>::new(
                    src_sample_rate as usize,
                    device_sample_rate as usize,
                    RESAMPLE_CHUNK_FRAMES,
                    2,
                    2,
                )
                .ok()
            } else {
                None
            };

            let mut resample_in_l = Vec::with_capacity(RESAMPLE_CHUNK_FRAMES * 2);
            let mut resample_in_r = Vec::with_capacity(RESAMPLE_CHUNK_FRAMES * 2);
            let mut sample_buf: Option<SampleBuffer<f32>> = None;

            let push_interleaved = |prod: &mut rtrb::Producer<f32>,
                                    l_slice: &[f32],
                                    r_slice: &[f32],
                                    cancel_flag: &AtomicBool|
             -> bool {
                let mut idx = 0;
                while idx < l_slice.len() {
                    if cancel_flag.load(Ordering::Relaxed) || prod.is_abandoned() {
                        return false;
                    }
                    let free_frames = prod.slots() / 2;
                    if free_frames == 0 {
                        thread::sleep(Duration::from_millis(4));
                        continue;
                    }
                    let batch = free_frames.min(l_slice.len() - idx);
                    for k in 0..batch {
                        let _ = prod.push(l_slice[idx + k]);
                        let _ = prod.push(r_slice[idx + k]);
                    }
                    idx += batch;
                }
                true
            };

            loop {
                if cancel_bg.load(Ordering::Relaxed) || producer.is_abandoned() {
                    break;
                }
                let packet = match format.next_packet() {
                    Ok(pkt) => pkt,
                    Err(SymphoniaError::IoError(ref e))
                        if e.kind() == io::ErrorKind::UnexpectedEof =>
                    {
                        break;
                    }
                    Err(SymphoniaError::ResetRequired) => {
                        decoder.reset();
                        continue;
                    }
                    Err(_) => break,
                };
                if packet.track_id() != track_id {
                    continue;
                }

                let decoded = match decoder.decode(&packet) {
                    Ok(d) => d,
                    Err(SymphoniaError::DecodeError(_)) => continue,
                    Err(_) => break,
                };

                let spec = *decoded.spec();
                let ch_count = spec.channels.count().max(src_channels).max(1);
                let frames = decoded.frames();
                if frames == 0 {
                    continue;
                }

                let buf = sample_buf.get_or_insert_with(|| {
                    SampleBuffer::<f32>::new(decoded.capacity() as u64, spec)
                });
                if buf.capacity() < decoded.capacity() * ch_count {
                    *buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
                }
                buf.copy_interleaved_ref(decoded);
                let samples = buf.samples();

                if let Some(ref mut rs) = resampler {
                    for frame in samples.chunks_exact(ch_count) {
                        let l = frame[0];
                        let r = if ch_count > 1 { frame[1] } else { l };
                        resample_in_l.push(l);
                        resample_in_r.push(r);
                    }
                    while resample_in_l.len() >= RESAMPLE_CHUNK_FRAMES {
                        let chunk_l: Vec<f32> = resample_in_l.drain(..RESAMPLE_CHUNK_FRAMES).collect();
                        let chunk_r: Vec<f32> = resample_in_r.drain(..RESAMPLE_CHUNK_FRAMES).collect();
                        if let Ok(out) = rs.process(&[chunk_l, chunk_r], None)
                            && out.len() == 2
                            && !push_interleaved(&mut producer, &out[0], &out[1], &cancel_bg)
                        {
                            return;
                        }
                    }
                } else {
                    let mut out_l = Vec::with_capacity(frames);
                    let mut out_r = Vec::with_capacity(frames);
                    for frame in samples.chunks_exact(ch_count) {
                        let l = frame[0];
                        let r = if ch_count > 1 { frame[1] } else { l };
                        out_l.push(l);
                        out_r.push(r);
                    }
                    if !push_interleaved(&mut producer, &out_l, &out_r, &cancel_bg) {
                        return;
                    }
                }
            }

            // Flush any remaining partial resample buffer at EOF
            if let Some(ref mut rs) = resampler
                && !resample_in_l.is_empty()
                && !cancel_bg.load(Ordering::Relaxed)
            {
                resample_in_l.resize(RESAMPLE_CHUNK_FRAMES, 0.0);
                resample_in_r.resize(RESAMPLE_CHUNK_FRAMES, 0.0);
                if let Ok(out) = rs.process(&[resample_in_l, resample_in_r], None)
                    && out.len() == 2
                {
                    let _ = push_interleaved(&mut producer, &out[0], &out[1], &cancel_bg);
                }
            }
        })
        .map_err(|e| format!("Failed to spawn decoder thread: {e}"))?;

    Ok(DecodedDeckStream {
        consumer,
        handle: DecoderHandle { cancel },
        total_frames,
        start_frame,
        duration_secs,
    })
}
