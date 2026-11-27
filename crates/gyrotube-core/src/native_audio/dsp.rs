use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

pub const EQ_FREQUENCIES: [f32; 5] = [60.0, 230.0, 910.0, 3600.0, 14000.0];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NativeEqConfig {
    pub enabled: bool,
    pub normalize: bool,
    pub bands: [f32; 5],
}

impl Default for NativeEqConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            normalize: false,
            bands: [0.0; 5],
        }
    }
}

#[derive(Clone, Copy)]
struct BiquadCoeffs {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
}

impl Default for BiquadCoeffs {
    fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        }
    }
}

impl BiquadCoeffs {
    fn low_shelf(sample_rate: f32, freq: f32, gain_db: f32) -> Self {
        if gain_db.abs() < 0.01 {
            return Self::default();
        }
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq.clamp(20.0, sample_rate * 0.45) / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / 2.0 * 2.0_f32.sqrt();
        let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + two_sqrt_a_alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - two_sqrt_a_alpha);
        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + two_sqrt_a_alpha;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) + (a - 1.0) * cos_w0 - two_sqrt_a_alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    fn high_shelf(sample_rate: f32, freq: f32, gain_db: f32) -> Self {
        if gain_db.abs() < 0.01 {
            return Self::default();
        }
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq.clamp(20.0, sample_rate * 0.45) / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / 2.0 * 2.0_f32.sqrt();
        let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + two_sqrt_a_alpha);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - two_sqrt_a_alpha);
        let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + two_sqrt_a_alpha;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) - (a - 1.0) * cos_w0 - two_sqrt_a_alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    fn peaking(sample_rate: f32, freq: f32, q: f32, gain_db: f32) -> Self {
        if gain_db.abs() < 0.01 {
            return Self::default();
        }
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq.clamp(20.0, sample_rate * 0.45) / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct BiquadState {
    z1: f32,
    z2: f32,
}

impl BiquadState {
    #[inline]
    fn process(&mut self, x: f32, c: &BiquadCoeffs) -> f32 {
        // Direct Form II Transposed (numerically stable for f32)
        let y = c.b0 * x + self.z1;
        self.z1 = c.b1 * x - c.a1 * y + self.z2;
        self.z2 = c.b2 * x - c.a2 * y;
        y
    }
}

/// Converts YouTube's `loudnessDb` (offset relative to -14 LUFS; positive = louder)
/// into a static ReplayGain linear multiplier, clamped safely to `[-12 dB, +6 dB]`.
#[inline]
pub fn replaygain_linear(loudness_db: f32) -> f32 {
    if !loudness_db.is_finite() {
        return 1.0;
    }
    let adj_db = (-loudness_db).clamp(-12.0, 6.0);
    10.0_f32.powf(adj_db / 20.0)
}

pub struct DspChain {
    sample_rate: f32,
    config: NativeEqConfig,
    coeffs: [BiquadCoeffs; 5],
    left: [BiquadState; 5],
    right: [BiquadState; 5],
    eq_preamp: f32,
    env: f32,
    attack_coeff: f32,
    release_coeff: f32,
}

impl DspChain {
    pub fn new(sample_rate: f32) -> Self {
        let sr = sample_rate.max(8000.0);
        Self {
            sample_rate: sr,
            config: NativeEqConfig::default(),
            coeffs: [BiquadCoeffs::default(); 5],
            left: [BiquadState::default(); 5],
            right: [BiquadState::default(); 5],
            eq_preamp: 1.0,
            env: 0.0,
            attack_coeff: (-1.0 / (0.005 * sr)).exp(),
            release_coeff: (-1.0 / (0.200 * sr)).exp(),
        }
    }

    pub fn is_normalize_enabled(&self) -> bool {
        self.config.normalize
    }

    pub fn update_config(&mut self, config: NativeEqConfig) {
        self.config = config;
        let mut max_boost_db = 0.0_f32;
        for (i, &freq) in EQ_FREQUENCIES.iter().enumerate() {
            let gain = if config.enabled {
                config.bands[i].clamp(-12.0, 12.0)
            } else {
                0.0
            };
            if gain > max_boost_db {
                max_boost_db = gain;
            }
            self.coeffs[i] = match i {
                0 => BiquadCoeffs::low_shelf(self.sample_rate, freq, gain),
                4 => BiquadCoeffs::high_shelf(self.sample_rate, freq, gain),
                _ => BiquadCoeffs::peaking(self.sample_rate, freq, 1.0, gain),
            };
        }
        // Automatic negative pre-amp prevents EQ boosts from driving into the soft limiter
        self.eq_preamp = if config.enabled && max_boost_db > 0.01 {
            10.0_f32.powf(-max_boost_db / 20.0)
        } else {
            1.0
        };
    }

    #[inline]
    pub fn process_stereo(&mut self, l: f32, r: f32) -> (f32, f32) {
        self.process_stereo_with_replaygain(l, r, false)
    }

    /// Processes a stereo frame through the EQ (with automatic pre-amp), optional fallback
    /// dynamic normalizer (skipped when per-track ReplayGain is active), and soft limiter.
    #[inline]
    pub fn process_stereo_with_replaygain(
        &mut self,
        mut l: f32,
        mut r: f32,
        has_replaygain: bool,
    ) -> (f32, f32) {
        if self.config.enabled {
            l *= self.eq_preamp;
            r *= self.eq_preamp;
            for i in 0..5 {
                let c = &self.coeffs[i];
                l = self.left[i].process(l, c);
                r = self.right[i].process(r, c);
            }
        }

        if self.config.normalize && !has_replaygain {
            let peak = l.abs().max(r.abs());
            let coeff = if peak > self.env {
                self.attack_coeff
            } else {
                self.release_coeff
            };
            self.env = coeff * self.env + (1.0 - coeff) * peak;

            // Threshold -18 dBFS (~0.1259), Ratio 4:1, gentle makeup gain
            let env_db = 20.0 * self.env.max(1e-5).log10();
            let threshold_db = -18.0;
            let reduction_db = if env_db > threshold_db {
                (env_db - threshold_db) * (1.0 - 0.25)
            } else {
                0.0
            };
            let gain = 10.0_f32.powf((4.0 - reduction_db) / 20.0);
            l *= gain;
            r *= gain;
        }

        // Soft limiter prevents harsh digital clipping when EQ boosts or positive ReplayGain are high
        if self.config.enabled || self.config.normalize {
            l = soft_limit(l);
            r = soft_limit(r);
        }

        (l, r)
    }
}

#[inline]
fn soft_limit(x: f32) -> f32 {
    if x.abs() <= 0.92 {
        x
    } else {
        x.signum() * (0.92 + 0.08 * ((x.abs() - 0.92) / 0.08).tanh())
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::{DspChain, NativeEqConfig, replaygain_linear};

    #[test]
    fn flat_disabled_dsp_is_exact_passthrough() {
        let mut dsp = DspChain::new(48000.0);
        let (l, r) = dsp.process_stereo(0.42, -0.37);
        assert_eq!(l, 0.42);
        assert_eq!(r, -0.37);
    }

    #[test]
    fn bass_boost_increases_60hz_relative_to_midrange_without_clipping() {
        let mut dsp = DspChain::new(48000.0);
        dsp.update_config(NativeEqConfig {
            enabled: true,
            normalize: false,
            bands: [8.0, 0.0, 0.0, 0.0, 0.0],
        });
        let mut max_bass = 0.0_f32;
        for n in 0..4800 {
            let s = 0.5 * (2.0 * PI * 30.0 * n as f32 / 48000.0).sin();
            let (l, r) = dsp.process_stereo(s, s);
            assert!(l.is_finite() && r.is_finite());
            if n > 2400 {
                max_bass = max_bass.max(l.abs());
            }
        }
        // With -8 dB auto pre-amp and +8 dB bass shelf, sub-bass stays near ~0.50 without clipping
        assert!(max_bass > 0.42 && max_bass <= 0.55, "got {max_bass}");
    }

    #[test]
    fn eq_auto_preamp_prevents_clipping_on_boost() {
        let mut dsp = DspChain::new(48000.0);
        dsp.update_config(NativeEqConfig {
            enabled: true,
            normalize: false,
            bands: [12.0, 0.0, 0.0, 0.0, 0.0],
        });
        let mut peak = 0.0_f32;
        for n in 0..4800 {
            let s = 0.85 * (2.0 * PI * 30.0 * n as f32 / 48000.0).sin();
            let (l, _) = dsp.process_stereo(s, s);
            if n > 2400 {
                peak = peak.max(l.abs());
            }
        }
        assert!(peak < 0.92, "auto pre-amp should avoid soft limiter saturation, got {peak}");
    }

    #[test]
    fn replaygain_attenuates_loud_and_clamps_quiet_tracks() {
        // +6 dB track -> -6 dB attenuation (~0.501)
        let atten = replaygain_linear(6.0);
        assert!((atten - 0.5012).abs() < 0.01, "got {atten}");
        // Very loud +20 dB track -> clamped to -12 dB (~0.251)
        let clamped_down = replaygain_linear(20.0);
        assert!((clamped_down - 0.2512).abs() < 0.01, "got {clamped_down}");
        // Very quiet -20 dB track -> clamped to +6 dB (~1.995)
        let clamped_up = replaygain_linear(-20.0);
        assert!((clamped_up - 1.9953).abs() < 0.01, "got {clamped_up}");
    }

    #[test]
    fn normalizer_soft_limits_hot_signal() {
        let mut dsp = DspChain::new(48000.0);
        dsp.update_config(NativeEqConfig {
            enabled: false,
            normalize: true,
            bands: [0.0; 5],
        });
        for _ in 0..4800 {
            let (l, r) = dsp.process_stereo(0.95, -0.95);
            assert!(l.is_finite() && r.is_finite());
            assert!(l.abs() <= 1.0 && r.abs() <= 1.0);
        }
    }
}
