use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;

use super::progressive_reader::ProgressiveFileReader;
use crate::audio_streamer::NativeStreamSource;

/// Buffered stereo f32 ring (~0.5 s at 48 kHz = 48_000 interleaved samples).
const MAX_RING_SAMPLES: usize = 48_000;

pub struct Deck {
    ring: Arc<Mutex<VecDeque<f32>>>,
    playing: Arc<AtomicBool>,
    cancelled: Arc<AtomicBool>,
    eof: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
    seek_req_ms: Arc<AtomicU64>,
    has_seek_req: Arc<AtomicBool>,
    position_ms: Arc<AtomicU64>,
    duration_ms: Arc<AtomicU64>,
    output_rate: u32,
}

impl Deck {
    pub fn spawn(
        source: NativeStreamSource,
        output_rate: u32,
        start_secs: f64,
        start_paused: bool,
    ) -> Result<Self, String> {
        let ring = Arc::new(Mutex::new(VecDeque::with_capacity(MAX_RING_SAMPLES)));
        let playing = Arc::new(AtomicBool::new(!start_paused));
        let cancelled = Arc::new(AtomicBool::new(false));
        let eof = Arc::new(AtomicBool::new(false));
        let error = Arc::new(Mutex::new(None));
        let seek_req_ms = Arc::new(AtomicU64::new(0));
        let has_seek_req = Arc::new(AtomicBool::new(false));
        let start_ms = (start_secs.max(0.0) * 1000.0) as u64;
        let position_ms = Arc::new(AtomicU64::new(start_ms));
        let duration_ms = Arc::new(AtomicU64::new(0));

        let reader =
            ProgressiveFileReader::open(&source.file_path, source.progress_rx, cancelled.clone())
                .map_err(|e| format!("No se pudo abrir el audio nativo: {e}"))?;

        let deck = Self {
            ring: ring.clone(),
            playing: playing.clone(),
            cancelled: cancelled.clone(),
            eof: eof.clone(),
            error: error.clone(),
            seek_req_ms: seek_req_ms.clone(),
            has_seek_req: has_seek_req.clone(),
            position_ms: position_ms.clone(),
            duration_ms: duration_ms.clone(),
            output_rate,
        };

        let ext = source.extension.clone();
        std::thread::Builder::new()
            .name("gyro-deck-decoder".into())
            .spawn(move || {
                if let Err(err) = run_decoder_loop(
                    reader,
                    &ext,
                    output_rate,
                    start_secs,
                    ring,
                    cancelled.clone(),
                    eof.clone(),
                    seek_req_ms,
                    has_seek_req,
                    position_ms,
                    duration_ms,
                ) {
                    if !cancelled.load(Ordering::Relaxed)
                        && let Ok(mut guard) = error.lock()
                    {
                        *guard = Some(err);
                    }
                    eof.store(true, Ordering::Release);
                }
            })
            .map_err(|e| format!("Failed to spawn audio decoder thread: {e}"))?;

        Ok(deck)
    }

    pub fn set_playing(&self, play: bool) {
        self.playing.store(play, Ordering::Relaxed);
    }

    pub fn is_playing(&self) -> bool {
        self.playing.load(Ordering::Relaxed)
    }

    pub fn seek(&self, secs: f64) {
        let ms = (secs.max(0.0) * 1000.0) as u64;
        self.seek_req_ms.store(ms, Ordering::Relaxed);
        self.position_ms.store(ms, Ordering::Relaxed);
        self.eof.store(false, Ordering::Release);
        self.has_seek_req.store(true, Ordering::Release);
        if let Ok(mut ring) = self.ring.lock() {
            ring.clear();
        }
    }

    pub fn stop(&self) {
        self.playing.store(false, Ordering::Relaxed);
        self.cancelled.store(true, Ordering::Release);
        if let Ok(mut ring) = self.ring.lock() {
            ring.clear();
        }
    }

    pub fn position_secs(&self) -> f64 {
        self.position_ms.load(Ordering::Relaxed) as f64 / 1000.0
    }

    pub fn duration_secs(&self) -> f64 {
        self.duration_ms.load(Ordering::Relaxed) as f64 / 1000.0
    }

    pub fn take_error(&self) -> Option<String> {
        self.error.lock().ok().and_then(|mut g| g.take())
    }

    pub fn is_finished(&self) -> bool {
        if !self.eof.load(Ordering::Acquire) {
            return false;
        }
        self.ring.try_lock().map(|r| r.is_empty()).unwrap_or(false)
    }

    /// Reads stereo samples into `out` (interleaved L/R). Returns number of stereo frames read.
    /// Uses `try_lock` so the realtime audio callback never blocks.
    pub fn pull_stereo(&self, out: &mut [f32], gain: f32) -> usize {
        if !self.is_playing() || self.cancelled.load(Ordering::Relaxed) {
            return 0;
        }
        let Ok(mut ring) = self.ring.try_lock() else {
            return 0;
        };
        let avail_samples = ring.len().min(out.len()) & !1;
        for sample in out.iter_mut().take(avail_samples) {
            *sample += ring.pop_front().unwrap_or(0.0) * gain;
        }
        let frames = avail_samples / 2;
        if frames > 0 && self.output_rate > 0 {
            let delta_ms = (frames as f64 * 1000.0 / self.output_rate as f64).round() as u64;
            if delta_ms > 0 {
                self.position_ms.fetch_add(delta_ms, Ordering::Relaxed);
            }
        }
        frames
    }
}

impl Drop for Deck {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

#[allow(clippy::too_many_arguments)]
fn run_decoder_loop(
    reader: ProgressiveFileReader,
    extension: &str,
    output_rate: u32,
    start_secs: f64,
    ring: Arc<Mutex<VecDeque<f32>>>,
    cancelled: Arc<AtomicBool>,
    eof: Arc<AtomicBool>,
    seek_req_ms: Arc<AtomicU64>,
    has_seek_req: Arc<AtomicBool>,
    position_ms: Arc<AtomicU64>,
    duration_ms: Arc<AtomicU64>,
) -> Result<(), String> {
    let mss = MediaSourceStream::new(Box::new(reader), Default::default());
    let mut hint = Hint::new();
    if !extension.is_empty() {
        hint.with_extension(extension);
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
        .map_err(|e| format!("No se pudo abrir el formato de audio ({extension}): {e}"))?;

    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| "No se encontró pista de audio decodificable".to_string())?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();

    if let (Some(n_frames), Some(sr)) = (codec_params.n_frames, codec_params.sample_rate)
        && sr > 0
    {
        let dur_ms = ((n_frames as f64 / sr as f64) * 1000.0) as u64;
        duration_ms.store(dur_ms, Ordering::Relaxed);
    }

    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .map_err(|e| format!("No se pudo inicializar el decodificador de audio: {e}"))?;

    if start_secs > 0.2 {
        let time = Time::from(start_secs);
        let _ = format.seek(
            SeekMode::Coarse,
            SeekTo::Time {
                time,
                track_id: Some(track_id),
            },
        );
        decoder.reset();
    }

    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    let mut resample_frac = 0.0_f64;
    let mut prev_l = 0.0_f32;
    let mut prev_r = 0.0_f32;

    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Ok(());
        }

        if has_seek_req.swap(false, Ordering::AcqRel) {
            let target_ms = seek_req_ms.load(Ordering::Relaxed);
            let time = Time::from(target_ms as f64 / 1000.0);
            if format
                .seek(
                    SeekMode::Coarse,
                    SeekTo::Time {
                        time,
                        track_id: Some(track_id),
                    },
                )
                .is_ok()
            {
                decoder.reset();
                position_ms.store(target_ms, Ordering::Relaxed);
                resample_frac = 0.0;
                if let Ok(mut q) = ring.lock() {
                    q.clear();
                }
            }
        }

        // Throttle decoding while ring buffer is comfortably full
        while !cancelled.load(Ordering::Relaxed) && !has_seek_req.load(Ordering::Acquire) {
            let len = ring.lock().map(|q| q.len()).unwrap_or(0);
            if len < MAX_RING_SAMPLES / 2 {
                break;
            }
            std::thread::sleep(Duration::from_millis(12));
        }

        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::IoError(ref io_err))
                if io_err.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                eof.store(true, Ordering::Release);
                // Keep thread alive briefly in case user seeks back before track ends
                while !cancelled.load(Ordering::Relaxed) {
                    if has_seek_req.load(Ordering::Acquire) {
                        eof.store(false, Ordering::Release);
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(25));
                }
                if cancelled.load(Ordering::Relaxed) {
                    return Ok(());
                }
                continue;
            }
            Err(SymphoniaError::ResetRequired) => {
                decoder.reset();
                continue;
            }
            Err(err) => {
                if cancelled.load(Ordering::Relaxed) {
                    return Ok(());
                }
                return Err(format!("Error leyendo flujo de audio: {err}"));
            }
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(err) => return Err(format!("Error decodificando audio: {err}")),
        };

        let spec = *decoded.spec();
        let channels = spec.channels.count().max(1);
        let src_rate = spec.rate.max(8000);
        let buf = sample_buf.get_or_insert_with(|| {
            SampleBuffer::<f32>::new(decoded.capacity() as u64, spec)
        });
        buf.copy_interleaved_ref(decoded);
        let samples = buf.samples();
        let src_frames = samples.len() / channels;
        if src_frames == 0 {
            continue;
        }

        // Convert to stereo interleaved f32 and resample to `output_rate` if needed
        let mut out_chunk = Vec::with_capacity(src_frames * 2 + 16);
        if src_rate == output_rate {
            for f in 0..src_frames {
                let l = samples[f * channels];
                let r = if channels > 1 {
                    samples[f * channels + 1]
                } else {
                    l
                };
                out_chunk.push(l);
                out_chunk.push(r);
            }
        } else {
            let ratio = src_rate as f64 / output_rate as f64;
            while resample_frac < src_frames as f64 {
                let idx = resample_frac.floor() as usize;
                let frac = (resample_frac - idx as f64) as f32;
                let (l0, r0) = if idx == 0 {
                    (prev_l, prev_r)
                } else {
                    let b = (idx - 1) * channels;
                    let l = samples[b];
                    let r = if channels > 1 { samples[b + 1] } else { l };
                    (l, r)
                };
                let b1 = idx.min(src_frames - 1) * channels;
                let l1 = samples[b1];
                let r1 = if channels > 1 { samples[b1 + 1] } else { l1 };
                out_chunk.push(l0 + (l1 - l0) * frac);
                out_chunk.push(r0 + (r1 - r0) * frac);
                resample_frac += ratio;
            }
            resample_frac -= src_frames as f64;
            let last_b = (src_frames - 1) * channels;
            prev_l = samples[last_b];
            prev_r = if channels > 1 {
                samples[last_b + 1]
            } else {
                prev_l
            };
        }

        if let Ok(mut q) = ring.lock() {
            q.extend(out_chunk);
        }
    }
}
