use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use symphonia::core::codecs::audio::{
    AudioDecoder, AudioDecoderOptions, CODEC_ID_NULL_AUDIO, well_known::CODEC_ID_OPUS,
};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::units::Time;

use super::progressive_reader::ProgressiveFileReader;
use crate::audio_streamer::NativeStreamSource;

/// Maximum Opus packet length is 120 ms at 48 kHz = 5760 samples per channel.
const OPUS_MAX_FRAME_SAMPLES: usize = 5760;

/// Parses RFC 7845 `OpusHead` extradata (if present) into `(channels, pre_skip, output_gain_linear)`.
pub(crate) fn parse_opus_head(extra: Option<&[u8]>, fallback_channels: usize) -> (usize, usize, f32) {
    let default_ch = fallback_channels.clamp(1, 2);
    let Some(data) = extra else {
        return (default_ch, 0, 1.0);
    };
    if data.len() < 19 || &data[0..8] != b"OpusHead" {
        return (default_ch, 0, 1.0);
    }
    let ch = (data[9] as usize).clamp(1, 2);
    let pre_skip = u16::from_le_bytes([data[10], data[11]]) as usize;
    let gain_q78 = i16::from_le_bytes([data[16], data[17]]);
    let gain_db = (gain_q78 as f32 / 256.0).clamp(-24.0, 24.0);
    let output_gain = if gain_q78 == 0 {
        1.0
    } else {
        10.0_f32.powf(gain_db / 20.0)
    };
    (ch, pre_skip, output_gain)
}

enum TrackDecoder {
    Symphonia(Box<dyn AudioDecoder>),
    Opus {
        decoder: Box<rusty_opus::OpusDecoder>,
        channels: usize,
        pre_skip_remaining: usize,
        output_gain: f32,
        pcm_f32: Vec<f32>,
    },
}

impl TrackDecoder {
    fn reset(&mut self) {
        match self {
            Self::Symphonia(dec) => dec.reset(),
            Self::Opus {
                decoder,
                channels,
                pre_skip_remaining,
                ..
            } => {
                if let Ok(fresh) = rusty_opus::OpusDecoder::new(48_000, *channels) {
                    **decoder = fresh;
                }
                *pre_skip_remaining = 0;
            }
        }
    }
}

/// 4-point Catmull-Rom / Cubic Hermite resampler (zero external dependencies).
/// Preserves high-frequency detail and suppresses aliasing compared to linear interpolation.
pub(crate) struct CubicResampler {
    frac: f64,
    /// Last 3 input stereo frames `[y_{-3}, y_{-2}, y_{-1}]` for seamless block boundaries.
    hist: [(f32, f32); 3],
    primed: bool,
}

impl CubicResampler {
    pub(crate) fn new() -> Self {
        Self {
            frac: 0.0,
            hist: [(0.0, 0.0); 3],
            primed: false,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.frac = 0.0;
        self.hist = [(0.0, 0.0); 3];
        self.primed = false;
    }

    #[inline]
    fn hermite(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
        let c0 = y1;
        let c1 = 0.5 * (y2 - y0);
        let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
        let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
        ((c3 * t + c2) * t + c1) * t + c0
    }

    /// Resamples interleaved input `samples` (`channels` >= 1, `src_rate`) into stereo interleaved `out` at `dst_rate`.
    pub(crate) fn process(
        &mut self,
        samples: &[f32],
        channels: usize,
        src_rate: u32,
        dst_rate: u32,
        out: &mut Vec<f32>,
    ) {
        let src_frames = samples.len() / channels.max(1);
        if src_frames == 0 {
            return;
        }
        if !self.primed {
            let l0 = samples[0];
            let r0 = if channels > 1 { samples[1] } else { l0 };
            self.hist = [(l0, r0); 3];
            self.primed = true;
        }

        let hist = self.hist;
        let get_frame = |idx: isize| -> (f32, f32) {
            if idx < 0 {
                let h = (idx + 3).clamp(0, 2) as usize;
                hist[h]
            } else {
                let f = (idx as usize).min(src_frames - 1);
                let base = f * channels;
                let l = samples[base];
                let r = if channels > 1 { samples[base + 1] } else { l };
                (l, r)
            }
        };

        let ratio = src_rate as f64 / dst_rate.max(1) as f64;
        while self.frac < src_frames as f64 {
            let idx = self.frac.floor() as isize;
            let t = (self.frac - idx as f64) as f32;
            // Center interpolation between `idx - 1` (y1) and `idx` (y2)
            let (l0, r0) = get_frame(idx - 2);
            let (l1, r1) = get_frame(idx - 1);
            let (l2, r2) = get_frame(idx);
            let (l3, r3) = get_frame(idx + 1);
            out.push(Self::hermite(l0, l1, l2, l3, t));
            out.push(Self::hermite(r0, r1, r2, r3, t));
            self.frac += ratio;
        }
        self.frac -= src_frames as f64;

        // Shift last 3 frames into history for the next block
        let n = src_frames as isize;
        self.hist = [get_frame(n - 3), get_frame(n - 2), get_frame(n - 1)];
    }
}

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
    loudness_db: Option<f32>,
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
        let loudness_db = source.loudness_db;

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
            loudness_db,
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

    pub fn loudness_db(&self) -> Option<f32> {
        self.loudness_db
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

    let mut format = symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
        .map_err(|e| format!("No se pudo abrir el formato de audio ({extension}): {e}"))?;

    let (track_id, codec_params, track_dur, track_time_base, track_num_frames) = {
        let track = format
            .tracks()
            .iter()
            .find(|t| {
                t.codec_params
                    .as_ref()
                    .and_then(|p| p.audio())
                    .is_some_and(|a| a.codec != CODEC_ID_NULL_AUDIO)
            })
            .ok_or_else(|| "No se encontró pista de audio decodificable".to_string())?;
        let audio = track
            .codec_params
            .as_ref()
            .and_then(|p| p.audio())
            .cloned()
            .ok_or_else(|| "No se encontró pista de audio decodificable".to_string())?;
        (
            track.id,
            audio,
            track.duration,
            track.time_base,
            track.num_frames,
        )
    };

    let dur_secs = if let (Some(dur), Some(tb)) = (track_dur, track_time_base)
        && let Some(t) = tb.calc_duration(dur)
    {
        t.as_secs_f64()
    } else if let (Some(dur), Some(tb)) = (format.media_info().duration, format.media_info().time_base)
        && let Some(t) = tb.calc_duration(dur)
    {
        t.as_secs_f64()
    } else if let (Some(n_frames), Some(sr)) = (track_num_frames, codec_params.sample_rate)
        && sr > 0
    {
        n_frames as f64 / sr as f64
    } else {
        0.0
    };
    if dur_secs > 0.0 && dur_secs.is_finite() {
        duration_ms.store((dur_secs * 1000.0) as u64, Ordering::Relaxed);
    }

    let mut decoder = if codec_params.codec == CODEC_ID_OPUS {
        let fallback_ch = codec_params
            .channels
            .as_ref()
            .map(|c| c.count())
            .unwrap_or(2)
            .clamp(1, 2);
        let (channels, pre_skip, output_gain) =
            parse_opus_head(codec_params.extra_data.as_deref(), fallback_ch);
        let opus = rusty_opus::OpusDecoder::new(48_000, channels)
            .map_err(|e| format!("No se pudo inicializar el decodificador Opus: {e:?}"))?;
        TrackDecoder::Opus {
            decoder: Box::new(opus),
            channels,
            pre_skip_remaining: pre_skip,
            output_gain,
            pcm_f32: vec![0.0_f32; OPUS_MAX_FRAME_SAMPLES * channels],
        }
    } else {
        let sym = symphonia::default::get_codecs()
            .make_audio_decoder(&codec_params, &AudioDecoderOptions::default())
            .map_err(|e| format!("No se pudo inicializar el decodificador de audio: {e}"))?;
        TrackDecoder::Symphonia(sym)
    };

    if start_secs > 0.2 {
        let time = Time::try_from_secs_f64(start_secs).unwrap_or_default();
        let _ = format.seek(
            SeekMode::Coarse,
            SeekTo::Time {
                time,
                track_id: Some(track_id),
            },
        );
        decoder.reset();
    }

    let mut sym_pcm_f32: Vec<f32> = Vec::with_capacity(8192);
    let mut out_chunk: Vec<f32> = Vec::with_capacity(8192);
    let mut resampler = CubicResampler::new();

    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Ok(());
        }

        if has_seek_req.swap(false, Ordering::AcqRel) {
            let target_ms = seek_req_ms.load(Ordering::Relaxed);
            let time = Time::try_from_secs_f64(target_ms as f64 / 1000.0).unwrap_or_default();
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
                resampler.reset();
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
            Ok(Some(p)) => p,
            Ok(None) => {
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

        if packet.track_id != track_id {
            continue;
        }

        let (samples, channels, src_rate): (&[f32], usize, u32) = match &mut decoder {
            TrackDecoder::Opus {
                decoder: opus,
                channels,
                pre_skip_remaining,
                output_gain,
                pcm_f32,
            } => {
                let decoded_frames =
                    match opus.decode(&packet.data, OPUS_MAX_FRAME_SAMPLES, pcm_f32) {
                        Ok(n) => n,
                        Err(_) => continue,
                    };
                if decoded_frames == 0 {
                    continue;
                }
                let ch = *channels;
                let total_samples = decoded_frames * ch;
                if (*output_gain - 1.0).abs() > 1e-4 {
                    for s in &mut pcm_f32[..total_samples] {
                        *s *= *output_gain;
                    }
                }
                let skip_frames = (*pre_skip_remaining).min(decoded_frames);
                *pre_skip_remaining -= skip_frames;
                let start_idx = skip_frames * ch;
                if start_idx >= total_samples {
                    continue;
                }
                (&pcm_f32[start_idx..total_samples], ch, 48_000)
            }
            TrackDecoder::Symphonia(sym) => {
                let decoded = match sym.decode(&packet) {
                    Ok(d) => d,
                    Err(SymphoniaError::DecodeError(_)) => continue,
                    Err(err) => return Err(format!("Error decodificando audio: {err}")),
                };
                let spec = decoded.spec();
                let ch = spec.channels().count().max(1);
                let sr = spec.rate().max(8000);
                sym_pcm_f32.clear();
                decoded.copy_to_vec_interleaved(&mut sym_pcm_f32);
                (&sym_pcm_f32[..], ch, sr)
            }
        };

        let src_frames = samples.len() / channels;
        if src_frames == 0 {
            continue;
        }

        // Convert to stereo interleaved f32 (exact passthrough at matching rate, Cubic Hermite otherwise)
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
            resampler.process(samples, channels, src_rate, output_rate, &mut out_chunk);
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
    use std::f32::consts::PI;

    use super::{CubicResampler, SpscRing, parse_opus_head};

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

    #[test]
    fn parses_opus_head_pre_skip_and_gain() {
        let mut head = [0u8; 19];
        head[0..8].copy_from_slice(b"OpusHead");
        head[8] = 1; // version
        head[9] = 2; // stereo
        head[10..12].copy_from_slice(&312u16.to_le_bytes()); // standard 6.5 ms pre-skip
        head[16..18].copy_from_slice(&(-512i16).to_le_bytes()); // -2.0 dB in Q7.8
        let (ch, pre_skip, gain) = parse_opus_head(Some(&head), 1);
        assert_eq!(ch, 2);
        assert_eq!(pre_skip, 312);
        assert!((gain - 10.0_f32.powf(-2.0 / 20.0)).abs() < 1e-4);

        let (ch_fb, pre_fb, gain_fb) = parse_opus_head(None, 2);
        assert_eq!((ch_fb, pre_fb, gain_fb), (2, 0, 1.0));
    }

    #[test]
    fn hermite_resampler_preserves_dc_and_smooth_sine() {
        // 1. Constant DC signal must remain exact unity across multiple blocks
        let mut dc_resampler = CubicResampler::new();
        let dc_block = vec![0.42_f32; 256 * 2];
        let mut dc_out = Vec::new();
        dc_resampler.process(&dc_block, 2, 44_100, 48_000, &mut dc_out);
        dc_resampler.process(&dc_block, 2, 44_100, 48_000, &mut dc_out);
        assert!(!dc_out.is_empty());
        for s in &dc_out {
            assert!((s - 0.42).abs() < 1e-4, "DC drift detected: {s}");
        }

        // 2. 1 kHz sine resampled 44.1 kHz -> 48 kHz across blocks stays smooth & finite
        let mut sine_resampler = CubicResampler::new();
        let mut out = Vec::new();
        for blk in 0..4 {
            let mut chunk = Vec::with_capacity(512 * 2);
            for i in 0..512 {
                let n = (blk * 512 + i) as f32;
                let s = 0.5 * (2.0 * PI * 1000.0 * n / 44_100.0).sin();
                chunk.push(s);
                chunk.push(-s);
            }
            sine_resampler.process(&chunk, 2, 44_100, 48_000, &mut out);
        }
        assert!(out.len() > 2000 * 2);
        for s in &out {
            assert!(s.is_finite() && s.abs() <= 0.55);
        }
    }

    #[test]
    fn rusty_opus_roundtrip_decodes_clean_stereo_audio() {
        let mut enc =
            rusty_opus::OpusEncoder::new(48_000, 2, rusty_opus::Application::Audio).unwrap();
        let mut dec = rusty_opus::OpusDecoder::new(48_000, 2).unwrap();
        let frame = 960usize; // 20 ms at 48 kHz
        let mut pcm_in = vec![0.0_f32; frame * 2];
        let mut pkt = [0u8; 1500];
        let mut pcm_out = vec![0.0_f32; frame * 2];
        let mut max_abs = 0.0_f32;

        for blk in 0..10 {
            for i in 0..frame {
                let n = (blk * frame + i) as f32;
                let s = 0.5 * (2.0 * PI * 440.0 * n / 48_000.0).sin();
                pcm_in[i * 2] = s;
                pcm_in[i * 2 + 1] = -s;
            }
            let len = enc.encode(&pcm_in, frame, &mut pkt).unwrap();
            let decoded = dec.decode(&pkt[..len], frame, &mut pcm_out).unwrap();
            assert_eq!(decoded, frame);
            for &s in &pcm_out {
                assert!(s.is_finite());
                max_abs = max_abs.max(s.abs());
            }
        }
        assert!(
            max_abs > 0.35 && max_abs < 0.85,
            "expected clean decoded sine peak ~0.5, got {max_abs}"
        );
    }

    #[test]
    #[ignore = "Requires a local cached YouTube .webm Opus file in ~/.cache/gyrotube/streams"]
    fn decodes_real_cached_webm_opus_file_end_to_end() {
        use std::path::PathBuf;
        use std::time::{Duration, Instant};

        use super::Deck;
        use crate::audio_streamer::NativeStreamSource;

        let dir = PathBuf::from(std::env::var("HOME").unwrap()).join(".cache/gyrotube/streams");
        let path = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .find(|p| p.extension().and_then(|s| s.to_str()) == Some("webm"))
            .expect("no cached .webm file found");

        let deck = Deck::spawn(
            NativeStreamSource {
                file_path: path.clone(),
                extension: "webm".into(),
                progress_rx: None,
                loudness_db: Some(3.5),
            },
            48_000,
            0.0,
            false,
        )
        .unwrap();

        let t0 = Instant::now();
        let mut total_frames = 0usize;
        let mut peak = 0.0_f32;
        let mut sum_sq = 0.0_f64;
        let mut buf = vec![0.0_f32; 1024 * 2];

        // Pull 2 seconds of stereo 48 kHz audio (96_000 frames)
        while total_frames < 96_000 && t0.elapsed() < Duration::from_secs(5) {
            buf.fill(0.0);
            let n = deck.pull_stereo(&mut buf, 1.0);
            if n == 0 {
                std::thread::sleep(Duration::from_millis(2));
                continue;
            }
            for &s in &buf[..n * 2] {
                assert!(s.is_finite(), "non-finite sample decoded");
                peak = peak.max(s.abs());
                sum_sq += (s as f64) * (s as f64);
            }
            total_frames += n;
        }

        let decode_elapsed = t0.elapsed();
        let dur = deck.duration_secs();
        let rms = (sum_sq / (total_frames * 2).max(1) as f64).sqrt();
        println!(
            "[e2e webm] file={} dur={dur:.1}s pulled_frames={total_frames} elapsed={decode_elapsed:?} peak={peak:.3} rms={rms:.4}",
            path.display()
        );
        assert!(deck.take_error().is_none(), "decoder reported error");
        assert!(dur > 30.0, "expected real duration > 30s, got {dur:.2}s");
        assert!(total_frames >= 96_000, "buffer underrun: only got {total_frames} frames");
        assert!(peak > 0.02 && peak <= 1.5, "unexpected peak: {peak}");
        assert!(rms > 0.005, "audio looks silent: rms={rms}");

        // Verify seek to 30s works cleanly
        deck.seek(30.0);
        let seek_t0 = Instant::now();
        let mut seek_frames = 0usize;
        while seek_frames < 24_000 && seek_t0.elapsed() < Duration::from_secs(3) {
            buf.fill(0.0);
            let n = deck.pull_stereo(&mut buf, 1.0);
            if n == 0 {
                std::thread::sleep(Duration::from_millis(2));
                continue;
            }
            seek_frames += n;
        }
        let pos_after_seek = deck.position_secs();
        println!(
            "[e2e webm] after seek(30.0): pos={pos_after_seek:.2}s pulled={seek_frames} in {:?}",
            seek_t0.elapsed()
        );
        assert!(deck.take_error().is_none(), "decoder error after seek");
        assert!(seek_frames >= 24_000, "seek stalled: only {seek_frames} frames");
        assert!(
            (pos_after_seek - 30.5).abs() < 0.25,
            "unexpected position after seek: {pos_after_seek}"
        );
        deck.stop();
    }
}
