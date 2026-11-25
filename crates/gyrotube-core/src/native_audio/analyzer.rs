use std::fs::File;
use std::io;
use std::path::Path;

use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::native_audio::protocol::TrackWaveformAnalysis;

const FRAME_RATE_HZ: usize = 100; // 10 ms envelope frames
const MIN_BPM: f64 = 70.0;
const MAX_BPM: f64 = 180.0;
const BPM_CONFIDENCE_THRESHOLD: f64 = 0.25;
pub const DEFAULT_WAVEFORM_PEAKS: usize = 240;

fn rms_to_db(v: f32) -> f32 {
    20.0 * v.max(1e-7).log10()
}

fn smooth3(arr: &[f32]) -> Vec<f32> {
    if arr.len() < 3 {
        return arr.to_vec();
    }
    let mut out = vec![0.0_f32; arr.len()];
    out[0] = arr[0];
    out[arr.len() - 1] = arr[arr.len() - 1];
    for i in 1..(arr.len() - 1) {
        out[i] = arr[i - 1] * 0.25 + arr[i] * 0.5 + arr[i + 1] * 0.25;
    }
    out
}

/// Decodes a local audio file in a single streaming pass (low RAM) and computes:
/// - Reduced waveform peak array (`num_peaks` normalized `[0.0..1.0]` bars) for `<canvas>`
/// - Intro beat-drop cue & natural outro cue points
/// - BPM, confidence (`0..1`), and beat-grid phase offset via `rustfft` autocorrelation.
pub fn analyze_audio_file(
    track_id: &str,
    path: &Path,
    num_peaks: usize,
) -> Result<TrackWaveformAnalysis, String> {
    let file = File::open(path)
        .map_err(|e| format!("Failed to open audio file for analysis {}: {e}", path.display()))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
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
        .map_err(|e| format!("Symphonia probe failed during analysis: {e}"))?;

    let mut format = probed.format;
    let codecs = symphonia::default::get_codecs();
    let (track_id_num, sample_rate, mut decoder) = {
        let mut selected = None;
        let mut last_err = String::from("No decodable audio track found");
        for t in format.tracks() {
            if t.codec_params.codec == CODEC_TYPE_NULL {
                continue;
            }
            // Skip video tracks in muxed MP4 containers (e.g., itag 18 has H.264 track 0 + AAC track 1)
            match codecs.make(&t.codec_params, &DecoderOptions::default()) {
                Ok(dec) => {
                    let sr = t.codec_params.sample_rate.unwrap_or(44_100).max(8_000);
                    selected = Some((t.id, sr, dec));
                    break;
                }
                Err(e) => {
                    last_err = format!(
                        "Symphonia codec init failed during analysis (codec={:?}): {e}",
                        t.codec_params.codec
                    );
                }
            }
        }
        selected.ok_or(last_err)?
    };

    let hop = (sample_rate as usize / FRAME_RATE_HZ).max(1);
    let alpha_low = 1.0 - (-2.0 * std::f32::consts::PI * 150.0 / sample_rate as f32).exp();
    let alpha_mid = 1.0 - (-2.0 * std::f32::consts::PI * 2500.0 / sample_rate as f32).exp();

    let mut lp_low = 0.0_f32;
    let mut lp_mid = 0.0_f32;
    let mut acc_count = 0usize;
    let mut sum_sq = 0.0_f32;
    let mut low_sq = 0.0_f32;
    let mut mid_sq = 0.0_f32;
    let mut high_sq = 0.0_f32;
    let mut total_samples: u64 = 0;

    let mut rms = Vec::with_capacity(24_000);
    let mut low_rms = Vec::with_capacity(24_000);
    let mut mid_rms = Vec::with_capacity(24_000);
    let mut high_rms = Vec::with_capacity(24_000);
    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    loop {
        let packet = match format.next_packet() {
            Ok(pkt) => pkt,
            Err(SymphoniaError::IoError(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(SymphoniaError::ResetRequired) => {
                decoder.reset();
                continue;
            }
            Err(_) => break,
        };
        if packet.track_id() != track_id_num {
            continue;
        }
        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(_) => break,
        };
        let spec = *decoded.spec();
        let ch_count = spec.channels.count().max(1);
        let frames = decoded.frames();
        if frames == 0 {
            continue;
        }

        let buf = sample_buf
            .get_or_insert_with(|| SampleBuffer::<f32>::new(decoded.capacity() as u64, spec));
        if buf.capacity() < decoded.capacity() * ch_count {
            *buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
        }
        buf.copy_interleaved_ref(decoded);
        let samples = buf.samples();

        for frame in samples.chunks_exact(ch_count) {
            let s = if ch_count == 1 {
                frame[0]
            } else {
                (frame[0] + frame[1]) * 0.5
            };
            total_samples += 1;
            sum_sq += s * s;
            lp_low += alpha_low * (s - lp_low);
            lp_mid += alpha_mid * (s - lp_mid);
            let low = lp_low;
            let mid = lp_mid - lp_low;
            let high = s - lp_mid;
            low_sq += low * low;
            mid_sq += mid * mid;
            high_sq += high * high;
            acc_count += 1;

            if acc_count >= hop {
                let inv = 1.0 / acc_count as f32;
                rms.push((sum_sq * inv).sqrt());
                low_rms.push((low_sq * inv).sqrt());
                mid_rms.push((mid_sq * inv).sqrt());
                high_rms.push((high_sq * inv).sqrt());
                acc_count = 0;
                sum_sq = 0.0;
                low_sq = 0.0;
                mid_sq = 0.0;
                high_sq = 0.0;
            }
        }
    }

    let duration_secs = total_samples as f64 / sample_rate as f64;
    if rms.is_empty() || duration_secs <= 0.1 {
        return Err("Decoded audio stream is empty".into());
    }

    // 1. Reduced normalized waveform peaks for Svelte <canvas>
    let target_peaks = num_peaks.clamp(32, 1024);
    let peaks = compute_waveform_peaks(&rms, target_peaks);

    // 2. 3-band smoothed onset flux
    let s_low = smooth3(&low_rms);
    let s_mid = smooth3(&mid_rms);
    let s_high = smooth3(&high_rms);
    let mut onset = vec![0.0_f32; rms.len()];
    let mut prev_low = (12.0 * s_low[0]).ln_1p();
    let mut prev_mid = (12.0 * s_mid[0]).ln_1p();
    let mut prev_high = (12.0 * s_high[0]).ln_1p();
    for f in 1..rms.len() {
        let cur_low = (12.0 * s_low[f]).ln_1p();
        let cur_mid = (12.0 * s_mid[f]).ln_1p();
        let cur_high = (12.0 * s_high[f]).ln_1p();
        onset[f] = (cur_low - prev_low).max(0.0) * 0.45
            + (cur_mid - prev_mid).max(0.0) * 0.35
            + (cur_high - prev_high).max(0.0) * 0.20;
        prev_low = cur_low;
        prev_mid = cur_mid;
        prev_high = cur_high;
    }

    // 3. Intro / outro cue points + FFT-accelerated BPM & beat grid
    let (intro_cue_secs, outro_cue_secs) = detect_intro_outro(&rms, &onset, duration_secs);
    let (bpm, bpm_confidence, beat_offset_secs) = detect_bpm_fft(&onset);

    Ok(TrackWaveformAnalysis {
        track_id: track_id.to_string(),
        duration_secs: (duration_secs * 1000.0).round() / 1000.0,
        peaks,
        bpm,
        bpm_confidence,
        beat_offset_secs,
        intro_cue_secs,
        outro_cue_secs,
    })
}

fn compute_waveform_peaks(rms: &[f32], num_peaks: usize) -> Vec<f32> {
    let mut raw = Vec::with_capacity(num_peaks);
    let n = rms.len();
    let mut max_val = 1e-6_f32;
    for i in 0..num_peaks {
        let start = (i * n) / num_peaks;
        let end = (((i + 1) * n) / num_peaks).max(start + 1).min(n);
        let mut bucket_peak = 0.0_f32;
        for &v in &rms[start..end] {
            if v > bucket_peak {
                bucket_peak = v;
            }
        }
        if bucket_peak > max_val {
            max_val = bucket_peak;
        }
        raw.push(bucket_peak);
    }
    raw.into_iter()
        .map(|v| ((v / max_val).clamp(0.0, 1.0) * 1000.0).round() / 1000.0)
        .collect()
}

fn detect_intro_outro(rms: &[f32], onset: &[f32], duration_secs: f64) -> (f64, f64) {
    let fr = FRAME_RATE_HZ as f64;
    let step = (rms.len() / 2000).max(1);
    let mut samples: Vec<f32> = rms
        .iter()
        .step_by(step)
        .copied()
        .filter(|&v| v > 1e-5)
        .collect();
    if samples.is_empty() {
        return (0.0, (duration_secs - 10.0).max(0.0));
    }
    samples.sort_by(f32::total_cmp);
    let peak_ref = samples[(samples.len() * 9 / 10).min(samples.len() - 1)];
    let peak_ref_db = rms_to_db(peak_ref);

    let win_frames = (2.0 * fr) as usize;
    let mut prefix = vec![0.0_f64; rms.len() + 1];
    for (i, &v) in rms.iter().enumerate() {
        prefix[i + 1] = prefix[i] + v as f64;
    }

    let mut win_samples = Vec::new();
    let win_step = (rms.len() / 400).max(1);
    let mut f = win_frames;
    while f < rms.len() {
        let avg = ((prefix[f] - prefix[f - win_frames]) / win_frames as f64) as f32;
        if avg > 1e-5 {
            win_samples.push(avg);
        }
        f += win_step;
    }
    win_samples.sort_by(f32::total_cmp);
    let sustain_ref = if !win_samples.is_empty() {
        win_samples[(win_samples.len() * 9 / 10).min(win_samples.len() - 1)]
    } else {
        peak_ref
    };
    let sustain_ref_db = rms_to_db(sustain_ref);

    // Intro beat-drop cue
    let max_intro_secs = (duration_secs * 0.16).min(18.0);
    let max_intro_frame = ((max_intro_secs * fr) as usize).min(rms.len() - 1);
    let short_win = (0.8 * fr) as usize;
    let first_sound_frame = (0..=max_intro_frame)
        .find(|&i| rms_to_db(rms[i]) >= peak_ref_db - 24.0)
        .unwrap_or(0);
    let mut groove_frame = (first_sound_frame..=max_intro_frame)
        .find(|&i| {
            let hi = (i + short_win).min(rms.len());
            let avg = ((prefix[hi] - prefix[i]) / (hi - i).max(1) as f64) as f32;
            rms_to_db(avg) >= sustain_ref_db - 10.5
        })
        .unwrap_or(first_sound_frame);

    let lo = groove_frame.saturating_sub((0.2 * fr) as usize).max(first_sound_frame);
    let hi = (groove_frame + (0.45 * fr) as usize).min(max_intro_frame);
    if let Some((best_i, &best_v)) = onset[lo..=hi]
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        && best_v > 0.02
    {
        groove_frame = lo + best_i;
    }
    let intro_cue_secs = ((groove_frame as f64 / fr) - 0.04).clamp(0.0, max_intro_secs);

    // Natural outro start
    let mut last_audible = rms.len() - 1;
    while last_audible > 0 && rms_to_db(rms[last_audible]) < peak_ref_db - 22.0 {
        last_audible -= 1;
    }
    let audible_end_secs = ((last_audible + 1) as f64 / fr).min(duration_secs);
    let max_outro_cut = (duration_secs * 0.24).min(26.0);
    let min_outro_secs = (duration_secs - max_outro_cut).max(intro_cue_secs + 6.0);
    let scan_start = ((min_outro_secs.max(duration_secs - 28.0) * fr) as usize).max(win_frames);

    let mut outro_frame = last_audible;
    let mut idx = last_audible;
    while idx >= scan_start {
        let win_start = idx.saturating_sub(win_frames);
        let avg = ((prefix[idx] - prefix[win_start]) / (idx - win_start).max(1) as f64) as f32;
        if rms_to_db(avg) >= sustain_ref_db - 4.5 {
            outro_frame = (idx + (0.2 * fr) as usize).min(last_audible);
            break;
        }
        outro_frame = idx;
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    let outro_cue_secs = (outro_frame as f64 / fr).clamp(min_outro_secs, audible_end_secs);

    (
        (intro_cue_secs * 1000.0).round() / 1000.0,
        (outro_cue_secs * 1000.0).round() / 1000.0,
    )
}

/// FFT-accelerated autocorrelation (`rustfft`) + continuous 0.5 BPM harmonic comb scoring.
fn detect_bpm_fft(onset: &[f32]) -> (Option<f64>, f64, f64) {
    let n = onset.len();
    if n < FRAME_RATE_HZ * 8 {
        return (None, 0.0, 0.0);
    }

    // Adaptive local-mean subtraction (0.6 s window)
    let half_win = FRAME_RATE_HZ * 3 / 10;
    let mut prefix = vec![0.0_f64; n + 1];
    for (i, &v) in onset.iter().enumerate() {
        prefix[i + 1] = prefix[i] + v as f64;
    }
    let mut norm = vec![0.0_f32; n];
    let mut energy_sum = 0.0_f64;
    for i in 0..n {
        let lo = i.saturating_sub(half_win);
        let hi = (i + half_win + 1).min(n);
        let local_mean = (prefix[hi] - prefix[lo]) / (hi - lo) as f64;
        let v = (onset[i] as f64 - local_mean).max(0.0) as f32;
        norm[i] = v;
        energy_sum += (v as f64) * (v as f64);
    }
    let mean_energy = energy_sum / (n as f64);
    if mean_energy < 5e-4 {
        return (None, 0.0, 0.0);
    }

    // Analyze up to 60 s from the core middle section
    let max_body = FRAME_RATE_HZ * 60;
    let (body_start, body_end) = if n > max_body {
        let margin = (n * 15 / 100).min((n - max_body) / 2);
        (margin, (margin + max_body).min(n))
    } else {
        (0, n)
    };
    let signal = &norm[body_start..body_end];
    let len = signal.len();

    // O(N log N) autocorrelation via rustfft (zero-padded power-of-2 FFT)
    let fft_len = (len * 2).next_power_of_two();
    let mut planner = FftPlanner::<f32>::new();
    let fft_fwd = planner.plan_fft_forward(fft_len);
    let fft_inv = planner.plan_fft_inverse(fft_len);

    let mut spectrum = vec![Complex::new(0.0_f32, 0.0); fft_len];
    for (i, &v) in signal.iter().enumerate() {
        spectrum[i].re = v;
    }
    fft_fwd.process(&mut spectrum);
    for c in &mut spectrum {
        let mag_sq = c.re * c.re + c.im * c.im;
        c.re = mag_sq;
        c.im = 0.0;
    }
    fft_inv.process(&mut spectrum);

    let max_harmonic_lag = (len / 2).min((FRAME_RATE_HZ as f64 * 60.0 / MIN_BPM) as usize * 4 + 2);
    let inv_fft = 1.0 / fft_len as f64;
    let acf_at = |lag: f64| -> f64 {
        if lag < 1.0 || lag >= max_harmonic_lag as f64 {
            return 0.0;
        }
        let lo = lag.floor() as usize;
        let frac = lag - lo as f64;
        let v0 = (spectrum[lo].re as f64 * inv_fft) / (len - lo).max(1) as f64;
        let v1 = (spectrum[lo + 1].re as f64 * inv_fft) / (len - lo - 1).max(1) as f64;
        v0 * (1.0 - frac) + v1 * frac
    };

    let bpm_step = 0.5;
    let cand_count = ((MAX_BPM - MIN_BPM) / bpm_step) as usize + 1;
    let mut comb_scores = vec![0.0_f64; cand_count];
    let mut weighted_scores = vec![0.0_f64; cand_count];
    let mut sum_score = 0.0_f64;

    for i in 0..cand_count {
        let bpm_cand = MIN_BPM + i as f64 * bpm_step;
        let lag = FRAME_RATE_HZ as f64 * 60.0 / bpm_cand;
        let comb = acf_at(lag)
            + 0.6 * acf_at(lag * 2.0)
            + 0.35 * acf_at(lag * 3.0)
            + 0.2 * acf_at(lag * 4.0);
        comb_scores[i] = comb;
        sum_score += comb;

        let oct_dist = (bpm_cand / 120.0).log2();
        let prior = (-0.5 * (oct_dist / 0.55).powi(2)).exp();
        weighted_scores[i] = comb * (0.72 + 0.28 * prior);
    }

    let mut best_idx = 0usize;
    for i in 1..cand_count {
        if weighted_scores[i] > weighted_scores[best_idx] {
            best_idx = i;
        }
    }

    let mut bpm = MIN_BPM + best_idx as f64 * bpm_step;
    if best_idx > 0 && best_idx + 1 < cand_count {
        let y0 = weighted_scores[best_idx - 1];
        let y1 = weighted_scores[best_idx];
        let y2 = weighted_scores[best_idx + 1];
        let denom = y0 - 2.0 * y1 + y2;
        if denom.abs() > 1e-12 {
            let delta = (0.5 * (y0 - y2) / denom).clamp(-0.5, 0.5);
            bpm += delta * bpm_step;
        }
    }

    let mean_score = sum_score / cand_count as f64;
    let var_sum: f64 = comb_scores
        .iter()
        .map(|&s| (s - mean_score).powi(2))
        .sum();
    let std_score = (var_sum / cand_count as f64).sqrt();
    let z_score = if std_score > 1e-12 {
        (comb_scores[best_idx] - mean_score) / std_score
    } else {
        0.0
    };
    let confidence = ((z_score - 1.2) / 2.8).clamp(0.0, 1.0);
    if confidence < BPM_CONFIDENCE_THRESHOLD {
        return (None, (confidence * 1000.0).round() / 1000.0, 0.0);
    }

    // Beat phase histogram
    let period_frames = FRAME_RATE_HZ as f64 * 60.0 / bpm;
    let bins = (period_frames.round() as usize).max(8);
    let mut phase_hist = vec![0.0_f64; bins];
    for (i, &v) in norm.iter().enumerate() {
        let phase = ((i as f64 % period_frames) / period_frames) * bins as f64;
        let bin = (phase.floor() as usize) % bins;
        phase_hist[bin] += v as f64;
    }
    let mut best_bin = 0usize;
    let mut best_val = -1.0_f64;
    for b in 0..bins {
        let prev = phase_hist[(b + bins - 1) % bins];
        let cur = phase_hist[b];
        let next = phase_hist[(b + 1) % bins];
        let sm = prev * 0.25 + cur * 0.5 + next * 0.25;
        if sm > best_val {
            best_val = sm;
            best_bin = b;
        }
    }
    let beat_offset_secs = (best_bin as f64 / bins as f64) * (60.0 / bpm);

    (
        Some((bpm * 100.0).round() / 100.0),
        (confidence * 1000.0).round() / 1000.0,
        (beat_offset_secs * 10_000.0).round() / 10_000.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_audio::decoder::{spawn_deck_decoder, DecoderSource};
    use crate::native_audio::protocol::DeckId;
    use std::io::Write;

    fn write_synthetic_wav(path: &Path, sample_rate: u32, duration_secs: f32, bpm: f32) {
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let beat_period = 60.0 / bpm;
        let mut pcm = vec![0i16; num_samples * 2];

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            if t < 1.8 {
                continue; // 1.8 s silent intro
            }
            let beat_pos = t % beat_period;
            let mut s = 0.0_f32;
            if beat_pos < 0.085 {
                let env = (-beat_pos * 34.0).exp();
                let freq = 55.0 + 85.0 * (-beat_pos * 42.0).exp();
                s += 0.8 * env * (2.0 * std::f32::consts::PI * freq * beat_pos).sin();
            }
            if t > duration_secs - 7.0 {
                let rem = (duration_secs - t).max(0.0) / 7.0;
                s *= rem * rem;
            }
            let sample_i16 = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            pcm[i * 2] = sample_i16;
            pcm[i * 2 + 1] = sample_i16;
        }

        let data_bytes = (pcm.len() * 2) as u32;
        let mut f = File::create(path).unwrap();
        f.write_all(b"RIFF").unwrap();
        f.write_all(&(36 + data_bytes).to_le_bytes()).unwrap();
        f.write_all(b"WAVEfmt ").unwrap();
        f.write_all(&16u32.to_le_bytes()).unwrap();
        f.write_all(&1u16.to_le_bytes()).unwrap(); // PCM
        f.write_all(&2u16.to_le_bytes()).unwrap(); // Stereo
        f.write_all(&sample_rate.to_le_bytes()).unwrap();
        f.write_all(&(sample_rate * 4).to_le_bytes()).unwrap();
        f.write_all(&4u16.to_le_bytes()).unwrap();
        f.write_all(&16u16.to_le_bytes()).unwrap();
        f.write_all(b"data").unwrap();
        f.write_all(&data_bytes.to_le_bytes()).unwrap();
        for sample in pcm {
            f.write_all(&sample.to_le_bytes()).unwrap();
        }
    }

    #[test]
    fn analyzes_synthetic_wav_peaks_bpm_and_decodes_with_resampling() {
        let dir = tempfile::tempdir().unwrap();
        let wav_path = dir.path().join("beat124.wav");
        write_synthetic_wav(&wav_path, 44_100, 32.0, 124.0);

        let analysis = analyze_audio_file("abcdefghijk", &wav_path, 120).unwrap();
        assert_eq!(analysis.peaks.len(), 120);
        let bpm = analysis.bpm.expect("Expected BPM detection");
        assert!((bpm - 124.0).abs() <= 1.5, "Expected ~124 BPM, got {bpm}");
        assert!(
            analysis.intro_cue_secs >= 1.4 && analysis.intro_cue_secs <= 2.3,
            "intro_cue_secs={}",
            analysis.intro_cue_secs
        );

        // Verify Symphonia + Rubato (44.1 kHz -> 48 kHz) streaming into rtrb ring buffer
        let mut stream = spawn_deck_decoder(
            DeckId::A,
            DecoderSource::LocalFile {
                path: wav_path,
                ext_hint: Some("wav".into()),
            },
            48_000,
            2.0,
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(80));
        let mut popped = 0usize;
        while stream.consumer.pop().is_ok() {
            popped += 1;
            if popped >= 4096 {
                break;
            }
        }
        assert!(popped >= 1024, "Expected resampled frames in ring buffer, got {popped}");
        stream.handle.cancel();
    }
}
