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

pub struct DspChain {
    sample_rate: f32,
    config: NativeEqConfig,
    coeffs: [BiquadCoeffs; 5],
    left: [BiquadState; 5],
    right: [BiquadState; 5],
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
            env: 0.0,
            attack_coeff: (-1.0 / (0.005 * sr)).exp(),
            release_coeff: (-1.0 / (0.200 * sr)).exp(),
        }
    }

    pub fn update_config(&mut self, config: NativeEqConfig) {
        self.config = config;
        for (i, &freq) in EQ_FREQUENCIES.iter().enumerate() {
            let gain = if config.enabled {
                config.bands[i].clamp(-12.0, 12.0)
            } else {
                0.0
            };
            self.coeffs[i] = match i {
                0 => BiquadCoeffs::low_shelf(self.sample_rate, freq, gain),
                4 => BiquadCoeffs::high_shelf(self.sample_rate, freq, gain),
                _ => BiquadCoeffs::peaking(self.sample_rate, freq, 1.0, gain),
            };
        }
    }

    #[inline]
    pub fn process_stereo(&mut self, mut l: f32, mut r: f32) -> (f32, f32) {
        if self.config.enabled {
            for i in 0..5 {
                let c = &self.coeffs[i];
                l = self.left[i].process(l, c);
                r = self.right[i].process(r, c);
            }
        }

        if self.config.normalize {
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

        // Soft limiter prevents harsh digital clipping when EQ boosts are high
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

    use super::{DspChain, NativeEqConfig};

    #[test]
    fn flat_disabled_dsp_is_exact_passthrough() {
        let mut dsp = DspChain::new(48000.0);
        let (l, r) = dsp.process_stereo(0.42, -0.37);
        assert_eq!(l, 0.42);
        assert_eq!(r, -0.37);
    }

    #[test]
    fn bass_boost_increases_60hz_sine_amplitude_without_nan() {
        let mut dsp = DspChain::new(48000.0);
        dsp.update_config(NativeEqConfig {
            enabled: true,
            normalize: false,
            bands: [8.0, 0.0, 0.0, 0.0, 0.0],
        });
        let mut max_out = 0.0_f32;
        for n in 0..4800 {
            let s = 0.2 * (2.0 * PI * 60.0 * n as f32 / 48000.0).sin();
            let (l, r) = dsp.process_stereo(s, s);
            assert!(l.is_finite() && r.is_finite());
            if n > 2400 {
                max_out = max_out.max(l.abs());
            }
        }
        // At the 60 Hz shelf corner frequency, +8 dB low-shelf yields ~+4 dB (~1.585x -> ~0.317)
        assert!(max_out > 0.30, "expected bass boost > 0.30, got {max_out}");
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
