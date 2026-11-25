use crate::native_audio::dsp::StereoBassFilter;
use crate::native_audio::protocol::{MAX_BEATMATCH_RATE, MIN_BEATMATCH_RATE};

/// RT-safe virtual playback deck.
/// Consumes interleaved stereo f32 samples from an SPSC `rtrb::Consumer<f32>` and applies
/// pitch-safe fractional resampling (clamped to ±3.5% to avoid noticeable pitch shift),
/// per-deck biquad bass-swap filtering, and smooth gain control with zero heap allocations.
pub struct VirtualDeck {
    consumer: Option<rtrb::Consumer<f32>>,
    pub playing: bool,
    pub ended: bool,
    pub playback_rate: f64,
    pub gain: f32,
    pub bpm: f32,
    pub total_frames: u64,
    pub position_frames: f64,
    pub filter: StereoBassFilter,

    cur_frame: (f32, f32),
    next_frame: (f32, f32),
    frac: f64,
    primed: bool,
    consecutive_empty: u32,
}

impl VirtualDeck {
    pub fn new(sample_rate_hz: f32) -> Self {
        Self {
            consumer: None,
            playing: false,
            ended: false,
            playback_rate: 1.0,
            gain: 1.0,
            bpm: 120.0,
            total_frames: 0,
            position_frames: 0.0,
            filter: StereoBassFilter::new(sample_rate_hz),
            cur_frame: (0.0, 0.0),
            next_frame: (0.0, 0.0),
            frac: 0.0,
            primed: false,
            consecutive_empty: 0,
        }
    }

    pub fn attach_consumer(
        &mut self,
        consumer: rtrb::Consumer<f32>,
        total_frames: u64,
        start_frame: u64,
        bpm: f32,
        autoplay: bool,
    ) {
        self.consumer = Some(consumer);
        self.total_frames = total_frames;
        self.position_frames = start_frame as f64;
        self.bpm = bpm;
        self.playback_rate = 1.0;
        self.gain = 1.0;
        self.playing = autoplay;
        self.ended = false;
        self.cur_frame = (0.0, 0.0);
        self.next_frame = (0.0, 0.0);
        self.frac = 0.0;
        self.primed = false;
        self.consecutive_empty = 0;
        self.filter.bypass();
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.ended = false;
        self.consumer = None;
        self.position_frames = 0.0;
        self.playback_rate = 1.0;
        self.primed = false;
        self.consecutive_empty = 0;
        self.filter.bypass();
    }

    #[inline]
    pub fn set_rate_clamped(&mut self, rate: f64) {
        self.playback_rate = rate.clamp(MIN_BEATMATCH_RATE, MAX_BEATMATCH_RATE);
    }

    #[inline]
    fn pop_stereo_frame(&mut self) -> Option<(f32, f32)> {
        let cons = self.consumer.as_mut()?;
        if cons.slots() < 2 {
            return None;
        }
        let l = cons.pop().ok()?;
        let r = cons.pop().ok()?;
        Some((l, r))
    }

    /// Generates the next stereo output frame with zero allocations.
    #[inline]
    pub fn next_stereo_sample(&mut self) -> (f32, f32) {
        if !self.playing {
            return (0.0, 0.0);
        }

        if !self.primed {
            if let (Some(f0), Some(f1)) = (self.pop_stereo_frame(), self.pop_stereo_frame()) {
                self.cur_frame = f0;
                self.next_frame = f1;
                self.frac = 0.0;
                self.primed = true;
                self.consecutive_empty = 0;
            } else {
                return (0.0, 0.0);
            }
        }

        let t = self.frac as f32;
        let raw_l = self.cur_frame.0 + (self.next_frame.0 - self.cur_frame.0) * t;
        let raw_r = self.cur_frame.1 + (self.next_frame.1 - self.cur_frame.1) * t;

        let rate = self.playback_rate.clamp(MIN_BEATMATCH_RATE, MAX_BEATMATCH_RATE);
        self.frac += rate;
        self.position_frames += rate;

        while self.frac >= 1.0 {
            self.frac -= 1.0;
            self.cur_frame = self.next_frame;
            match self.pop_stereo_frame() {
                Some(frame) => {
                    self.next_frame = frame;
                    self.consecutive_empty = 0;
                }
                None => {
                    self.next_frame = (0.0, 0.0);
                    self.consecutive_empty = self.consecutive_empty.saturating_add(1);
                    let producer_abandoned = self
                        .consumer
                        .as_ref()
                        .is_some_and(rtrb::Consumer::is_abandoned);
                    let reached_end = self.total_frames > 0
                        && self.position_frames as u64 >= self.total_frames.saturating_sub(256);
                    if producer_abandoned || (reached_end && self.consecutive_empty > 64) {
                        self.playing = false;
                        self.ended = true;
                    }
                    break;
                }
            }
        }

        let (fl, fr) = self.filter.process_stereo(raw_l, raw_r);
        (fl * self.gain, fr * self.gain)
    }
}
