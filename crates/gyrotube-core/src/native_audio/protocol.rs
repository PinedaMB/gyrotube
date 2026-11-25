use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Maximum pitch-coupled resampling stretch (±3.5% ≈ ±0.6 semitones) so beatmatching
/// stays completely natural on vocals without chipmunk/slow-mo artifacts.
pub const MAX_BEATMATCH_STRETCH: f64 = 0.035;
pub const MIN_BEATMATCH_RATE: f64 = 1.0 - MAX_BEATMATCH_STRETCH;
pub const MAX_BEATMATCH_RATE: f64 = 1.0 + MAX_BEATMATCH_STRETCH;

#[derive(Debug, Error)]
pub enum AudioEngineError {
    #[error("No default audio output device found")]
    NoOutputDevice,
    #[error("Failed to query audio output configuration: {0}")]
    DeviceConfig(String),
    #[error("Failed to build cpal output stream: {0}")]
    StreamBuild(String),
    #[error("Failed to start cpal output stream: {0}")]
    StreamPlay(String),
    #[error("Audio engine command channel is closed or full")]
    CommandChannel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeckId {
    A,
    B,
}

impl DeckId {
    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            DeckId::A => DeckId::B,
            DeckId::B => DeckId::A,
        }
    }
}

/// 5-band master EQ parameters matching GyroTube's UI bands (60, 230, 910, 3600, 14000 Hz).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EqParams {
    pub enabled: bool,
    pub normalize: bool,
    pub bands_db: [f32; 5],
}

impl Default for EqParams {
    fn default() -> Self {
        Self {
            enabled: false,
            normalize: false,
            bands_db: [0.0; 5],
        }
    }
}

/// Lock-free commands sent from Tauri commands to the real-time cpal audio callback.
/// No heap-allocated Strings or Vecs are carried inside variants to avoid RT drops.
pub enum EngineCommand {
    /// Attaches a freshly created SPSC ring-buffer consumer from the decoder thread to a deck.
    AttachDeckConsumer {
        deck: DeckId,
        consumer: rtrb::Consumer<f32>,
        total_frames: u64,
        start_frame: u64,
        bpm: f32,
        autoplay: bool,
    },
    Play(DeckId),
    Pause(DeckId),
    Stop(DeckId),
    SetPlaybackRate {
        deck: DeckId,
        rate: f64,
    },
    /// Triggers a DJ transition with equal-power crossfade, biquad bass swap,
    /// optional beat-synced dub echo, and pitch-safe tempo morphing (±3.5% max).
    StartTransition {
        from: DeckId,
        to: DeckId,
        duration_secs: f32,
        from_target_rate: f64,
        to_initial_rate: f64,
        enable_bass_swap: bool,
        enable_echo: bool,
        bridge_bpm: f32,
    },
    CancelTransition,
    SetMasterVolume(f32),
    SetEq(EqParams),
}

/// Compact waveform & mix metadata sent once per track over IPC (never raw PCM).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackWaveformAnalysis {
    pub track_id: String,
    pub duration_secs: f64,
    pub peaks: Vec<f32>,
    pub bpm: Option<f64>,
    pub bpm_confidence: f64,
    pub beat_offset_secs: f64,
    pub intro_cue_secs: f64,
    pub outro_cue_secs: f64,
}

/// Lightweight telemetry emitted every ~30 ms (`playback-tick`) to Svelte.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlaybackTick {
    pub active_deck: DeckId,
    pub deck_a_secs: f64,
    pub deck_b_secs: f64,
    pub deck_a_playing: bool,
    pub deck_b_playing: bool,
    pub deck_a_ended: bool,
    pub deck_b_ended: bool,
    pub transition_progress: Option<f32>,
}

/// Lock-free atomic telemetry written by the RT audio callback and read by the 30 ms emitter.
pub struct SharedTelemetry {
    pub active_deck_is_b: AtomicBool,
    pub deck_a_frame: AtomicU64,
    pub deck_b_frame: AtomicU64,
    pub deck_a_playing: AtomicBool,
    pub deck_b_playing: AtomicBool,
    pub deck_a_ended: AtomicBool,
    pub deck_b_ended: AtomicBool,
    pub transition_progress_bits: AtomicU32,
    pub device_sample_rate: AtomicU32,
}

impl Default for SharedTelemetry {
    fn default() -> Self {
        Self {
            active_deck_is_b: AtomicBool::new(false),
            deck_a_frame: AtomicU64::new(0),
            deck_b_frame: AtomicU64::new(0),
            deck_a_playing: AtomicBool::new(false),
            deck_b_playing: AtomicBool::new(false),
            deck_a_ended: AtomicBool::new(false),
            deck_b_ended: AtomicBool::new(false),
            transition_progress_bits: AtomicU32::new(f32::NAN.to_bits()),
            device_sample_rate: AtomicU32::new(48_000),
        }
    }
}

impl SharedTelemetry {
    #[inline]
    pub fn set_transition_progress(&self, progress: Option<f32>) {
        let bits = progress.unwrap_or(f32::NAN).to_bits();
        self.transition_progress_bits.store(bits, Ordering::Relaxed);
    }

    #[inline]
    pub fn snapshot(&self) -> PlaybackTick {
        let sr = self.device_sample_rate.load(Ordering::Relaxed).max(8_000) as f64;
        let active_deck = if self.active_deck_is_b.load(Ordering::Relaxed) {
            DeckId::B
        } else {
            DeckId::A
        };
        let prog_raw = f32::from_bits(self.transition_progress_bits.load(Ordering::Relaxed));
        PlaybackTick {
            active_deck,
            deck_a_secs: self.deck_a_frame.load(Ordering::Relaxed) as f64 / sr,
            deck_b_secs: self.deck_b_frame.load(Ordering::Relaxed) as f64 / sr,
            deck_a_playing: self.deck_a_playing.load(Ordering::Relaxed),
            deck_b_playing: self.deck_b_playing.load(Ordering::Relaxed),
            deck_a_ended: self.deck_a_ended.load(Ordering::Relaxed),
            deck_b_ended: self.deck_b_ended.load(Ordering::Relaxed),
            transition_progress: prog_raw.is_finite().then_some(prog_raw),
        }
    }
}
