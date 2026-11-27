use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
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

/// Lock-free SPSC ring capacity in interleaved stereo f32 samples.
/// 262_144 samples = 131_072 stereo frames ≈ 2.73 s at 48 kHz (power of 2 for fast masking).
const RING_CAPACITY: usize = 262_144;
const RING_MASK: usize = RING_CAPACITY - 1;
/// Decoder stays awake while buffered audio is below ~2.0 s, preventing underruns on jittery I/O.
const RING_LOW_WATERMARK: usize = 192_000;

/// Wait-free single-producer / single-consumer audio ring buffer (100% safe Rust).
/// Producer: decoder thread (`push_slice`, `request_clear`).
/// Consumer: CPAL realtime callback (`pop_add`, `drain_clear`).
struct SpscRing {
    buf: Box<[AtomicU32]>,
    read_pos: AtomicUsize,
    write_pos: AtomicUsize,
    drop_target: AtomicUsize,
}

impl SpscRing {
    fn new() -> Self {
        let buf = (0..RING_CAPACITY)
            .map(|_| AtomicU32::new(0))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            buf,
            read_pos: AtomicUsize::new(0),
            write_pos: AtomicUsize::new(0),
            drop_target: AtomicUsize::new(0),
        }
    }

    #[inline]
    fn buffered_len(&self) -> usize {
        let r = self
            .read_pos
            .load(Ordering::Acquire)
            .max(self.drop_target.load(Ordering::Acquire));
        let w = self.write_pos.load(Ordering::Acquire);
        w.saturating_sub(r)
    }

    /// Producer-only: marks all currently buffered samples as discarded so the consumer skips them.
    #[inline]
    fn request_clear(&self) {
        let w = self.write_pos.load(Ordering::Relaxed);
        self.drop_target.store(w, Ordering::Release);
    }

    /// Producer-only: pushes as many samples as fit without blocking. Returns count pushed.
    fn push_slice(&self, src: &[f32]) -> usize {
        let w = self.write_pos.load(Ordering::Relaxed);
        let r = self.read_pos.load(Ordering::Acquire);
        let used = w.saturating_sub(r);
        let free = RING_CAPACITY.saturating_sub(used);
        let count = src.len().min(free);
        if count == 0 {
            return 0;
        }
        for (i, &sample) in src.iter().take(count).enumerate() {
            self.buf[(w.wrapping_add(i)) & RING_MASK].store(sample.to_bits(), Ordering::Relaxed);
        }
        self.write_pos.store(w.wrapping_add(count), Ordering::Release);
        count
    }

    /// Consumer-only: advances `read_pos` past any cleared samples.
    #[inline]
    fn apply_pending_clear(&self) -> usize {
        let mut r = self.read_pos.load(Ordering::Relaxed);
        let target = self.drop_target.load(Ordering::Acquire);
        if r < target {
            r = target;
            self.read_pos.store(r, Ordering::Release);
        }
        r
    }

    /// Consumer-only: mixes up to `out.len()` interleaved stereo samples into `out`.
    fn pop_add(&self, out: &mut [f32], gain: f32) -> usize {
        let r = self.apply_pending_clear();
        let w = self.write_pos.load(Ordering::Acquire);
        let avail = w.saturating_sub(r).min(out.len()) & !1;
        if avail == 0 {
            return 0;
        }
        for (i, slot) in out.iter_mut().take(avail).enumerate() {
            let bits = self.buf[(r.wrapping_add(i)) & RING_MASK].load(Ordering::Relaxed);
            *slot += f32::from_bits(bits) * gain;
        }
        self.read_pos.store(r.wrapping_add(avail), Ordering::Release);
        avail
    }
}

pub struct Deck {
    ring: Arc<SpscRing>,
    playing: Arc<AtomicBool>,
    cancelled: Arc<AtomicBool>,
    seeking: Arc<AtomicBool>,
    eof: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
    seek_req_ms: Arc<AtomicU64>,
    has_seek_req: Arc<AtomicBool>,
    base_position_ms: Arc<AtomicU64>,
    played_frames: Arc<AtomicU64>,
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
        let ring = Arc::new(SpscRing::new());
        let playing = Arc::new(AtomicBool::new(!start_paused));
        let cancelled = Arc::new(AtomicBool::new(false));
        let seeking = Arc::new(AtomicBool::new(false));
        let eof = Arc::new(AtomicBool::new(false));
        let error = Arc::new(Mutex::new(None));
        let seek_req_ms = Arc::new(AtomicU64::new(0));
        let has_seek_req = Arc::new(AtomicBool::new(false));
        let start_ms = (start_secs.max(0.0) * 1000.0) as u64;
        let base_position_ms = Arc::new(AtomicU64::new(start_ms));
        let played_frames = Arc::new(AtomicU64::new(0));
        let duration_ms = Arc::new(AtomicU64::new(0));

        let reader =
            ProgressiveFileReader::open(&source.file_path, source.progress_rx, cancelled.clone())
                .map_err(|e| format!("No se pudo abrir el audio nativo: {e}"))?;

        let deck = Self {
            ring: ring.clone(),
            playing: playing.clone(),
            cancelled: cancelled.clone(),
            seeking: seeking.clone(),
            eof: eof.clone(),
            error: error.clone(),
            seek_req_ms: seek_req_ms.clone(),
            has_seek_req: has_seek_req.clone(),
            base_position_ms: base_position_ms.clone(),
            played_frames: played_frames.clone(),
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
                    seeking.clone(),
                    eof.clone(),
                    seek_req_ms,
                    has_seek_req,
                    base_position_ms,
                    played_frames,
                    duration_ms,
                ) {
                    if !cancelled.load(Ordering::Relaxed)
                        && let Ok(mut guard) = error.lock()
                    {
                        *guard = Some(err);
                    }
                    seeking.store(false, Ordering::Release);
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
        self.base_position_ms.store(ms, Ordering::Relaxed);
        self.played_frames.store(0, Ordering::Relaxed);
        self.eof.store(false, Ordering::Release);
        self.seeking.store(true, Ordering::Release);
        self.has_seek_req.store(true, Ordering::Release);
    }

    pub fn stop(&self) {
        self.playing.store(false, Ordering::Relaxed);
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn position_secs(&self) -> f64 {
        let base_ms = self.base_position_ms.load(Ordering::Relaxed) as f64;
        let frames = self.played_frames.load(Ordering::Relaxed) as f64;
        let rate = self.output_rate.max(1) as f64;
        base_ms / 1000.0 + frames / rate
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
        self.ring.buffered_len() == 0
    }

    /// Reads stereo samples into `out` (interleaved L/R). Returns number of stereo frames read.
    /// Completely lock-free and wait-free so the realtime audio callback never stalls or drops out.
    pub fn pull_stereo(&self, out: &mut [f32], gain: f32) -> usize {
        self.ring.apply_pending_clear();
        if !self.is_playing()
            || self.cancelled.load(Ordering::Relaxed)
            || self.seeking.load(Ordering::Acquire)
        {
            return 0;
        }
        let avail_samples = self.ring.pop_add(out, gain);
        let frames = avail_samples / 2;
        if frames > 0 {
            self.played_frames.fetch_add(frames as u64, Ordering::Relaxed);
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
    ring: Arc<SpscRing>,
    cancelled: Arc<AtomicBool>,
    seeking: Arc<AtomicBool>,
    eof: Arc<AtomicBool>,
    seek_req_ms: Arc<AtomicU64>,
    has_seek_req: Arc<AtomicBool>,
    base_position_ms: Arc<AtomicU64>,
    played_frames: Arc<AtomicU64>,
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
    let mut out_chunk: Vec<f32> = Vec::with_capacity(8192);
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
                base_position_ms.store(target_ms, Ordering::Relaxed);
                played_frames.store(0, Ordering::Relaxed);
                resample_frac = 0.0;
                prev_l = 0.0;
                prev_r = 0.0;
                ring.request_clear();
            }
            seeking.store(false, Ordering::Release);
        }

        // Throttle decoding while ring buffer is comfortably full (~2.0 s buffered)
        while !cancelled.load(Ordering::Relaxed) && !has_seek_req.load(Ordering::Acquire) {
            if ring.buffered_len() < RING_LOW_WATERMARK {
                break;
            }
            std::thread::sleep(Duration::from_millis(8));
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
        out_chunk.clear();
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

        // Push decoded samples into the lock-free ring, waiting briefly only if completely full
        let mut offset = 0;
        while offset < out_chunk.len() {
            if cancelled.load(Ordering::Relaxed) || has_seek_req.load(Ordering::Acquire) {
                break;
            }
            let pushed = ring.push_slice(&out_chunk[offset..]);
            if pushed > 0 {
                offset += pushed;
            } else {
                std::thread::sleep(Duration::from_millis(4));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpscRing;

    #[test]
    fn spsc_ring_pushes_pops_and_clears_without_locks() {
        let ring = SpscRing::new();
        assert_eq!(ring.buffered_len(), 0);

        let input = [0.25_f32, -0.25, 0.5, -0.5, 0.75, -0.75];
        assert_eq!(ring.push_slice(&input), 6);
        assert_eq!(ring.buffered_len(), 6);

        let mut out = [0.0_f32; 4];
        assert_eq!(ring.pop_add(&mut out, 1.0), 4);
        assert_eq!(out, [0.25, -0.25, 0.5, -0.5]);
        assert_eq!(ring.buffered_len(), 2);

        ring.request_clear();
        assert_eq!(ring.buffered_len(), 0);
        let mut out2 = [0.0_f32; 4];
        assert_eq!(ring.pop_add(&mut out2, 1.0), 0);
        assert_eq!(out2, [0.0; 4]);
    }
}
