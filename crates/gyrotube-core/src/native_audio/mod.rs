pub mod analyzer;
pub mod deck;
pub mod decoder;
pub mod dsp;
pub mod engine;
pub mod protocol;

pub use analyzer::analyze_audio_file;
pub use decoder::{spawn_deck_decoder, DecoderHandle, DecoderSource};
pub use engine::AudioController;
pub use protocol::{
    AudioEngineError, DeckId, EngineCommand, EqParams, PlaybackTick, SharedTelemetry,
    TrackWaveformAnalysis,
};
