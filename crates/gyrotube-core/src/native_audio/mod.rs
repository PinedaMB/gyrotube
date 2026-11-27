mod deck;
pub mod dsp;
mod progressive_reader;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::Serialize;

use crate::audio_streamer::NativeStreamSource;
use deck::Deck;
pub use dsp::NativeEqConfig;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NativeAudioEvent {
    Tick {
        position_secs: f64,
        duration_secs: f64,
    },
    PlayState {
        playing: bool,
    },
    Ended,
    CrossfadeTrigger,
    Error {
        message: String,
    },
}

struct CrossfadeState {
    started_at: Instant,
    duration_secs: f64,
}

struct MixerState {
    active_deck: Option<Arc<Deck>>,
    outgoing_deck: Option<Arc<Deck>>,
    crossfade: Option<CrossfadeState>,
    crossfade_secs: f64,
    crossfade_fired: bool,
    target_volume: f32,
    current_volume: f32,
    dsp: dsp::DspChain,
    output_rate: u32,
    last_playing_emitted: bool,
    ended_emitted: bool,
}

/// Pure-Rust Dual-Deck audio engine backed by `symphonia` + `cpal`.
/// Handles progressive disk decoding, crossfading, 5-band biquad EQ, and volume normalization
/// without depending on WebKitGTK / GStreamer.
pub struct NativeAudioEngine {
    mixer: Arc<Mutex<MixerState>>,
    // Keep cpal::Stream alive in a dedicated holder thread (cpal::Stream is !Send on some OSes)
    _output_tx: std::sync::mpsc::Sender<OutputCmd>,
    event_tx: async_channel::Sender<NativeAudioEvent>,
}

enum OutputCmd {
    EnsureStarted,
}

impl NativeAudioEngine {
    pub fn start() -> (Self, async_channel::Receiver<NativeAudioEvent>) {
        let (event_tx, event_rx) = async_channel::unbounded();
        let (output_tx, output_rx) = std::sync::mpsc::channel::<OutputCmd>();
        let (rate_tx, rate_rx) = std::sync::mpsc::channel::<u32>();

        let mixer = Arc::new(Mutex::new(MixerState {
            active_deck: None,
            outgoing_deck: None,
            crossfade: None,
            crossfade_secs: 0.0,
            crossfade_fired: false,
            target_volume: 1.0,
            current_volume: 1.0,
            dsp: dsp::DspChain::new(48000.0),
            output_rate: 48000,
            last_playing_emitted: false,
            ended_emitted: false,
        }));

        // Dedicated audio output owner thread (keeps cpal::Stream alive on platforms where it's !Send)
        let mixer_for_out = mixer.clone();
        let event_for_out = event_tx.clone();
        let _ = std::thread::Builder::new()
            .name("gyro-cpal-owner".into())
            .spawn(move || {
                let mut active_stream: Option<cpal::Stream> = None;
                let default_rate = probe_default_sample_rate().unwrap_or(48000);
                if let Ok(mut m) = mixer_for_out.lock() {
                    m.output_rate = default_rate;
                    m.dsp = dsp::DspChain::new(default_rate as f32);
                }
                let _ = rate_tx.send(default_rate);

                while let Ok(cmd) = output_rx.recv() {
                    match cmd {
                        OutputCmd::EnsureStarted => {
                            if active_stream.is_none() {
                                match open_cpal_stream(mixer_for_out.clone()) {
                                    Ok((stream, actual_rate)) => {
                                        if let Ok(mut m) = mixer_for_out.lock()
                                            && m.output_rate != actual_rate
                                        {
                                            m.output_rate = actual_rate;
                                            let cfg = NativeEqConfig::default();
                                            m.dsp = dsp::DspChain::new(actual_rate as f32);
                                            m.dsp.update_config(cfg);
                                        }
                                        let _ = stream.play();
                                        active_stream = Some(stream);
                                    }
                                    Err(err) => {
                                        let _ = event_for_out.try_send(NativeAudioEvent::Error {
                                            message: err,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            });

        let _ = rate_rx.recv_timeout(Duration::from_secs(2));

        // 100ms status/tick emitter thread (never holds mixer lock across channel sends or deck mutexes)
        let mixer_for_tick = mixer.clone();
        let event_for_tick = event_tx.clone();
        let _ = std::thread::Builder::new()
            .name("gyro-audio-ticker".into())
            .spawn(move || {
                loop {
                    std::thread::sleep(Duration::from_millis(100));

                    // Snapshot deck pointer and flags under a minimal lock
                    let (deck, crossfade_secs, crossfade_fired, last_playing_emitted, ended_emitted) = {
                        let Ok(m) = mixer_for_tick.lock() else {
                            continue;
                        };
                        let Some(deck) = m.active_deck.clone() else {
                            continue;
                        };
                        (
                            deck,
                            m.crossfade_secs,
                            m.crossfade_fired,
                            m.last_playing_emitted,
                            m.ended_emitted,
                        )
                    };

                    if let Some(err) = deck.take_error() {
                        let _ = event_for_tick.try_send(NativeAudioEvent::Error { message: err });
                    }

                    let playing = deck.is_playing();
                    let pos = deck.position_secs();
                    let dur = deck.duration_secs();
                    let finished = deck.is_finished();

                    let mut emit_play_state: Option<bool> = None;
                    let mut emit_crossfade = false;
                    let mut emit_ended = false;

                    if playing != last_playing_emitted {
                        emit_play_state = Some(playing);
                    }
                    if crossfade_secs > 0.0
                        && !crossfade_fired
                        && playing
                        && dur > crossfade_secs + 2.0
                        && dur - pos <= crossfade_secs
                    {
                        emit_crossfade = true;
                    }
                    if finished && !ended_emitted {
                        emit_ended = true;
                        emit_play_state = Some(false);
                    }

                    if (emit_play_state.is_some() || emit_crossfade || emit_ended)
                        && let Ok(mut m) = mixer_for_tick.lock()
                    {
                        // Verify deck hasn't changed before mutating flags
                        if m.active_deck
                            .as_ref()
                            .is_some_and(|d| Arc::ptr_eq(d, &deck))
                        {
                            if let Some(p) = emit_play_state {
                                m.last_playing_emitted = p;
                            }
                            if emit_crossfade {
                                m.crossfade_fired = true;
                            }
                            if emit_ended {
                                m.ended_emitted = true;
                                m.last_playing_emitted = false;
                            }
                        } else {
                            continue;
                        }
                    }

                    if emit_ended {
                        deck.set_playing(false);
                    }
                    if let Some(p) = emit_play_state {
                        let _ = event_for_tick.try_send(NativeAudioEvent::PlayState { playing: p });
                    }
                    if playing && !emit_ended {
                        let _ = event_for_tick.try_send(NativeAudioEvent::Tick {
                            position_secs: pos,
                            duration_secs: dur,
                        });
                    }
                    if emit_crossfade {
                        let _ = event_for_tick.try_send(NativeAudioEvent::CrossfadeTrigger);
                    }
                    if emit_ended {
                        let _ = event_for_tick.try_send(NativeAudioEvent::Ended);
                    }
                }
            });

        (
            Self {
                mixer,
                _output_tx: output_tx,
                event_tx,
            },
            event_rx,
        )
    }

    pub fn load(
        &self,
        source: NativeStreamSource,
        start_secs: f64,
        use_crossfade: bool,
        start_paused: bool,
    ) -> Result<(), String> {
        let _ = self._output_tx.send(OutputCmd::EnsureStarted);

        let output_rate = self
            .mixer
            .lock()
            .map(|m| m.output_rate)
            .unwrap_or(48000);
        let new_deck = Arc::new(Deck::spawn(source, output_rate, start_secs, start_paused)?);

        let mut m = self
            .mixer
            .lock()
            .map_err(|_| "Audio mixer lock poisoned".to_string())?;

        if let Some(old_out) = m.outgoing_deck.take() {
            old_out.stop();
        }

        if use_crossfade && m.crossfade_secs > 0.2 && !start_paused && m.active_deck.is_some() {
            m.outgoing_deck = m.active_deck.take();
            m.crossfade = Some(CrossfadeState {
                started_at: Instant::now(),
                duration_secs: m.crossfade_secs,
            });
        } else {
            if let Some(old_active) = m.active_deck.take() {
                old_active.stop();
            }
            m.crossfade = None;
        }

        m.active_deck = Some(new_deck);
        m.crossfade_fired = false;
        m.ended_emitted = false;
        m.last_playing_emitted = !start_paused;
        let _ = self.event_tx.try_send(NativeAudioEvent::PlayState {
            playing: !start_paused,
        });
        Ok(())
    }

    pub fn play(&self) {
        let _ = self._output_tx.send(OutputCmd::EnsureStarted);
        if let Ok(mut m) = self.mixer.lock()
            && let Some(ref deck) = m.active_deck
        {
            deck.set_playing(true);
            m.ended_emitted = false;
            m.last_playing_emitted = true;
            let _ = self
                .event_tx
                .try_send(NativeAudioEvent::PlayState { playing: true });
        }
    }

    pub fn pause(&self) {
        if let Ok(mut m) = self.mixer.lock()
            && let Some(ref deck) = m.active_deck
        {
            deck.set_playing(false);
            m.last_playing_emitted = false;
            let _ = self
                .event_tx
                .try_send(NativeAudioEvent::PlayState { playing: false });
        }
    }

    pub fn stop(&self) {
        if let Ok(mut m) = self.mixer.lock() {
            if let Some(d) = m.active_deck.take() {
                d.stop();
            }
            if let Some(d) = m.outgoing_deck.take() {
                d.stop();
            }
            m.crossfade = None;
            m.last_playing_emitted = false;
            m.ended_emitted = false;
        }
    }

    pub fn seek(&self, secs: f64) {
        if let Ok(mut m) = self.mixer.lock()
            && let Some(deck) = m.active_deck.clone()
        {
            deck.seek(secs);
            m.ended_emitted = false;
            m.crossfade_fired = false;
            let dur = deck.duration_secs();
            let _ = self.event_tx.try_send(NativeAudioEvent::Tick {
                position_secs: secs.max(0.0),
                duration_secs: dur,
            });
        }
    }

    pub fn set_volume(&self, volume: f32) {
        if let Ok(mut m) = self.mixer.lock() {
            m.target_volume = volume.clamp(0.0, 1.0);
        }
    }

    pub fn set_crossfade_secs(&self, secs: f64) {
        if let Ok(mut m) = self.mixer.lock() {
            m.crossfade_secs = secs.clamp(0.0, 10.0);
        }
    }

    pub fn set_eq(&self, config: NativeEqConfig) {
        if let Ok(mut m) = self.mixer.lock() {
            m.dsp.update_config(config);
        }
    }
}

fn probe_default_sample_rate() -> Option<u32> {
    let host = cpal::default_host();
    let device = host.default_output_device()?;
    let cfg = device.default_output_config().ok()?;
    Some(cfg.sample_rate())
}

fn open_cpal_stream(mixer: Arc<Mutex<MixerState>>) -> Result<(cpal::Stream, u32), String> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| "No se encontró dispositivo de salida de audio".to_string())?;
    let supported = device
        .default_output_config()
        .map_err(|e| format!("No se pudo leer la configuración de audio: {e}"))?;
    let channels = supported.channels() as usize;
    let sample_rate = supported.sample_rate();
    let config: cpal::StreamConfig = supported.into();

    let err_fn = |err| eprintln!("[gyro-native-audio] cpal stream error: {err}");

    // Pre-allocated scratch buffer reused across every realtime callback (zero steady-state allocs)
    let mut stereo_buf: Vec<f32> = vec![0.0_f32; 4096 * 2];

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [f32], _| {
                data.fill(0.0);

                // Briefly spin on try_lock so a nanosecond ticker/control lock never drops an audio buffer
                let mut m = None;
                for _ in 0..64 {
                    if let Ok(guard) = mixer.try_lock() {
                        m = Some(guard);
                        break;
                    }
                    std::hint::spin_loop();
                }
                let Some(mut m) = m else {
                    return;
                };

                let frames = data.len() / channels.max(1);
                let needed = frames * 2;
                if stereo_buf.len() < needed {
                    stereo_buf.resize(needed, 0.0);
                }
                let stereo = &mut stereo_buf[..needed];
                stereo.fill(0.0);

                let (active_gain, outgoing_gain, clear_outgoing) = if let Some(ref cf) = m.crossfade
                {
                    let elapsed = cf.started_at.elapsed().as_secs_f64();
                    let p = (elapsed / cf.duration_secs.max(0.1)).clamp(0.0, 1.0) as f32;
                    // Equal-power crossfade curve
                    let in_gain = (p * std::f32::consts::FRAC_PI_2).sin();
                    let out_gain = (p * std::f32::consts::FRAC_PI_2).cos();
                    (in_gain, out_gain, p >= 1.0)
                } else {
                    (1.0, 0.0, false)
                };

                let normalize_on = m.dsp.is_normalize_enabled();
                let mut active_has_rg = false;

                if let Some(ref deck) = m.active_deck {
                    let rg = if normalize_on {
                        deck.loudness_db().map(|db| {
                            active_has_rg = true;
                            dsp::replaygain_linear(db)
                        })
                    } else {
                        None
                    }
                    .unwrap_or(1.0);
                    deck.pull_stereo(stereo, active_gain * rg);
                }
                if outgoing_gain > 0.001
                    && let Some(ref out_deck) = m.outgoing_deck
                {
                    let rg = if normalize_on {
                        out_deck.loudness_db().map(dsp::replaygain_linear)
                    } else {
                        None
                    }
                    .unwrap_or(1.0);
                    out_deck.pull_stereo(stereo, outgoing_gain * rg);
                }
                if clear_outgoing {
                    if let Some(old) = m.outgoing_deck.take() {
                        old.stop();
                    }
                    m.crossfade = None;
                }

                // Smooth master volume ramping (~30ms anti-click) + DSP chain
                let target_vol = m.target_volume;
                let ramp_step = 1.0 / (sample_rate as f32 * 0.03).max(1.0);

                for f in 0..frames {
                    let diff = target_vol - m.current_volume;
                    if diff.abs() <= ramp_step {
                        m.current_volume = target_vol;
                    } else {
                        m.current_volume += diff.signum() * ramp_step;
                    }
                    let vol = m.current_volume;
                    let (l, r) = m.dsp.process_stereo_with_replaygain(
                        stereo[f * 2] * vol,
                        stereo[f * 2 + 1] * vol,
                        active_has_rg,
                    );

                    if channels == 1 {
                        data[f] = (l + r) * 0.5;
                    } else {
                        let base = f * channels;
                        data[base] = l;
                        data[base + 1] = r;
                    }
                }
            },
            err_fn,
            None,
        )
        .map_err(|e| format!("No se pudo abrir el stream de salida de audio: {e}"))?;

    Ok((stream, sample_rate))
}
