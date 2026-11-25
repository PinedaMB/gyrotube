use std::f32::consts::FRAC_PI_2;

use biquad::{Biquad, Coefficients, DirectForm2Transposed, Q_BUTTERWORTH_F32, ToHertz, Type};

use crate::native_audio::protocol::EqParams;

pub const EQ_BAND_FREQUENCIES_HZ: [f32; 5] = [60.0, 230.0, 910.0, 3600.0, 14000.0];

/// Equal-power crossfade gains: cos²(t·π/2) + sin²(t·π/2) = 1.0.
#[inline]
pub fn equal_power_gains(t: f32) -> (f32, f32) {
    let clamped = t.clamp(0.0, 1.0);
    let angle = clamped * FRAC_PI_2;
    (angle.cos(), angle.sin())
}

/// Smooth analog-style soft-clipper limiter to protect against clipping when summing two decks.
#[inline]
pub fn soft_clip(sample: f32) -> f32 {
    if sample.abs() <= 0.85 {
        sample
    } else {
        (sample * 1.15).tanh() / 1.15_f32.tanh() * 0.98
    }
}

/// Pre-allocated stereo biquad filter pair for RT-safe DJ bass swap & filter sweeps.
/// Coefficients are updated in-place per control block (zero heap allocations).
pub struct StereoBassFilter {
    sample_rate_hz: f32,
    hp_l: DirectForm2Transposed<f32>,
    hp_r: DirectForm2Transposed<f32>,
    shelf_l: DirectForm2Transposed<f32>,
    shelf_r: DirectForm2Transposed<f32>,
    hp_active: bool,
    shelf_active: bool,
}

impl StereoBassFilter {
    pub fn new(sample_rate_hz: f32) -> Self {
        let sr = sample_rate_hz.max(8_000.0);
        let hp = Self::make_hp(sr, 20.0);
        let shelf = Self::make_low_shelf(sr, 180.0, 0.0);
        Self {
            sample_rate_hz: sr,
            hp_l: DirectForm2Transposed::<f32>::new(hp),
            hp_r: DirectForm2Transposed::<f32>::new(hp),
            shelf_l: DirectForm2Transposed::<f32>::new(shelf),
            shelf_r: DirectForm2Transposed::<f32>::new(shelf),
            hp_active: false,
            shelf_active: false,
        }
    }

    #[inline]
    fn make_hp(sr: f32, cutoff_hz: f32) -> Coefficients<f32> {
        let safe_hz = cutoff_hz.clamp(20.0, (sr * 0.45).min(12_000.0));
        Coefficients::<f32>::from_params(
            Type::HighPass,
            sr.hz(),
            safe_hz.hz(),
            Q_BUTTERWORTH_F32,
        )
        .expect("Valid HighPass biquad parameters")
    }

    #[inline]
    fn make_low_shelf(sr: f32, freq_hz: f32, gain_db: f32) -> Coefficients<f32> {
        let safe_hz = freq_hz.clamp(40.0, (sr * 0.4).min(1_000.0));
        Coefficients::<f32>::from_params(
            Type::LowShelf(gain_db.clamp(-24.0, 12.0)),
            sr.hz(),
            safe_hz.hz(),
            Q_BUTTERWORTH_F32,
        )
        .expect("Valid LowShelf biquad parameters")
    }

    pub fn set_high_pass(&mut self, cutoff_hz: f32) {
        if cutoff_hz <= 22.0 {
            self.hp_active = false;
            return;
        }
        let coeffs = Self::make_hp(self.sample_rate_hz, cutoff_hz);
        self.hp_l.update_coefficients(coeffs);
        self.hp_r.update_coefficients(coeffs);
        self.hp_active = true;
    }

    pub fn set_low_shelf_db(&mut self, gain_db: f32) {
        if gain_db.abs() < 0.05 {
            self.shelf_active = false;
            return;
        }
        let coeffs = Self::make_low_shelf(self.sample_rate_hz, 180.0, gain_db);
        self.shelf_l.update_coefficients(coeffs);
        self.shelf_r.update_coefficients(coeffs);
        self.shelf_active = true;
    }

    pub fn bypass(&mut self) {
        self.hp_active = false;
        self.shelf_active = false;
    }

    #[inline]
    pub fn process_stereo(&mut self, mut l: f32, mut r: f32) -> (f32, f32) {
        if self.hp_active {
            l = self.hp_l.run(l);
            r = self.hp_r.run(r);
        }
        if self.shelf_active {
            l = self.shelf_l.run(l);
            r = self.shelf_r.run(r);
        }
        (l, r)
    }
}

/// Pre-allocated 5-band stereo master EQ + soft bus compressor (RT-safe).
pub struct MasterEqChain {
    sample_rate_hz: f32,
    enabled: bool,
    normalize: bool,
    filters_l: [DirectForm2Transposed<f32>; 5],
    filters_r: [DirectForm2Transposed<f32>; 5],
    env: f32,
}

impl MasterEqChain {
    pub fn new(sample_rate_hz: f32) -> Self {
        let sr = sample_rate_hz.max(8_000.0);
        let c = [
            Self::band_coeffs(sr, 0, 0.0),
            Self::band_coeffs(sr, 1, 0.0),
            Self::band_coeffs(sr, 2, 0.0),
            Self::band_coeffs(sr, 3, 0.0),
            Self::band_coeffs(sr, 4, 0.0),
        ];
        Self {
            sample_rate_hz: sr,
            enabled: false,
            normalize: false,
            filters_l: c.map(DirectForm2Transposed::<f32>::new),
            filters_r: c.map(DirectForm2Transposed::<f32>::new),
            env: 0.0,
        }
    }

    fn band_coeffs(sr: f32, idx: usize, gain_db: f32) -> Coefficients<f32> {
        let freq = EQ_BAND_FREQUENCIES_HZ[idx].min(sr * 0.45);
        let gain = gain_db.clamp(-12.0, 12.0);
        let filter_type = match idx {
            0 => Type::LowShelf(gain),
            4 => Type::HighShelf(gain),
            _ => Type::PeakingEQ(gain),
        };
        Coefficients::<f32>::from_params(filter_type, sr.hz(), freq.hz(), 1.0)
            .expect("Valid EQ biquad parameters")
    }

    pub fn update(&mut self, params: EqParams) {
        self.enabled = params.enabled;
        self.normalize = params.normalize;
        for i in 0..5 {
            let db = if params.enabled { params.bands_db[i] } else { 0.0 };
            let coeffs = Self::band_coeffs(self.sample_rate_hz, i, db);
            self.filters_l[i].update_coefficients(coeffs);
            self.filters_r[i].update_coefficients(coeffs);
        }
    }

    #[inline]
    pub fn process_stereo(&mut self, mut l: f32, mut r: f32) -> (f32, f32) {
        if self.enabled {
            for i in 0..5 {
                l = self.filters_l[i].run(l);
                r = self.filters_r[i].run(r);
            }
        }
        if self.normalize {
            // Simple RT-safe RMS/peak bus compressor (threshold -16 dBFS, ratio 4:1)
            let peak = l.abs().max(r.abs());
            let coeff = if peak > self.env { 0.01 } else { 0.0005 };
            self.env += coeff * (peak - self.env);
            let thresh = 0.16_f32;
            if self.env > thresh {
                let over = self.env / thresh;
                let gain = over.powf(-0.75);
                l *= gain * 1.15;
                r *= gain * 1.15;
            }
        }
        (l, r)
    }
}

/// Pre-allocated stereo beat-synced dub echo (band-passed delay line, zero allocations in RT).
pub struct BeatEchoBus {
    sample_rate_hz: f32,
    buf_l: Vec<f32>,
    buf_r: Vec<f32>,
    write_idx: usize,
    delay_samples: usize,
    send_gain: f32,
    feedback: f32,
    bp_l: DirectForm2Transposed<f32>,
    bp_r: DirectForm2Transposed<f32>,
    active_tail_frames: usize,
}

impl BeatEchoBus {
    pub fn new(sample_rate_hz: f32) -> Self {
        let sr = sample_rate_hz.max(8_000.0);
        // Pre-allocate 1.0 second stereo delay buffer once at engine startup
        let cap = (sr as usize).max(8_000);
        let bp = Coefficients::<f32>::from_params(
            Type::BandPass,
            sr.hz(),
            1050.0_f32.min(sr * 0.4).hz(),
            0.85,
        )
        .expect("Valid BandPass parameters");
        Self {
            sample_rate_hz: sr,
            buf_l: vec![0.0; cap],
            buf_r: vec![0.0; cap],
            write_idx: 0,
            delay_samples: ((sr * 0.25) as usize).min(cap - 1),
            send_gain: 0.0,
            feedback: 0.42,
            bp_l: DirectForm2Transposed::<f32>::new(bp),
            bp_r: DirectForm2Transposed::<f32>::new(bp),
            active_tail_frames: 0,
        }
    }

    pub fn configure(&mut self, bpm: f32, send_gain: f32, feedback: f32) {
        let safe_bpm = bpm.clamp(70.0, 180.0);
        let delay_secs = ((60.0 / safe_bpm) * 0.5).clamp(0.15, 0.45);
        let max_delay = self.buf_l.len().saturating_sub(1).max(1);
        self.delay_samples = ((delay_secs * self.sample_rate_hz) as usize).clamp(1, max_delay);
        self.send_gain = send_gain.clamp(0.0, 0.6);
        self.feedback = feedback.clamp(0.0, 0.65);
        if self.send_gain > 0.001 {
            self.active_tail_frames = (self.sample_rate_hz * 2.2) as usize;
        }
    }

    pub fn clear(&mut self) {
        self.buf_l.fill(0.0);
        self.buf_r.fill(0.0);
        self.send_gain = 0.0;
        self.active_tail_frames = 0;
    }

    #[inline]
    pub fn process_stereo(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        if self.send_gain <= 0.0005 && self.active_tail_frames == 0 {
            return (0.0, 0.0);
        }
        if self.send_gain <= 0.0005 {
            self.active_tail_frames = self.active_tail_frames.saturating_sub(1);
        }

        let len = self.buf_l.len();
        let read_idx = (self.write_idx + len - self.delay_samples) % len;
        let delayed_l = self.bp_l.run(self.buf_l[read_idx]);
        let delayed_r = self.bp_r.run(self.buf_r[read_idx]);

        self.buf_l[self.write_idx] = in_l * self.send_gain + delayed_l * self.feedback;
        self.buf_r[self.write_idx] = in_r * self.send_gain + delayed_r * self.feedback;
        self.write_idx = (self.write_idx + 1) % len;

        (delayed_l * 0.7, delayed_r * 0.7)
    }
}
