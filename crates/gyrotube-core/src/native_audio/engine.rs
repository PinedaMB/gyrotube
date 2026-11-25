use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{bounded, Receiver, Sender};

use crate::native_audio::deck::VirtualDeck;
use crate::native_audio::dsp::{equal_power_gains, soft_clip, BeatEchoBus, MasterEqChain};
use crate::native_audio::protocol::{
    AudioEngineError, DeckId, EngineCommand, EqParams, PlaybackTick, SharedTelemetry,
    MAX_BEATMATCH_RATE, MIN_BEATMATCH_RATE,
};

/// Control-block interval for biquad coefficient updates (~1.3 ms at 48 kHz).
const CONTROL_BLOCK_FRAMES: usize = 64;

struct ActiveTransition {
    from: DeckId,
    to: DeckId,
    elapsed_frames: u64,
    total_frames: u64,
    from_start_rate: f64,
    from_target_rate: f64,
    to_initial_rate: f64,
    enable_bass_swap: bool,
    enable_echo: bool,
    bridge_bpm: f32,
}

/// Real-time render state owned exclusively by the cpal audio callback (0 allocations, 0 locks).
pub(crate) struct AudioRenderState {
    sample_rate: f32,
    active_deck: DeckId,
    deck_a: VirtualDeck,
    deck_b: VirtualDeck,
    master_volume: f32,
    master_eq: MasterEqChain,
    echo_bus: BeatEchoBus,
    transition: Option<ActiveTransition>,
    cmd_rx: Receiver<EngineCommand>,
    telemetry: Arc<SharedTelemetry>,
    control_counter: usize,
}

impl AudioRenderState {
    pub(crate) fn new(
        sample_rate: f32,
        cmd_rx: Receiver<EngineCommand>,
        telemetry: Arc<SharedTelemetry>,
    ) -> Self {
        Self {
            sample_rate,
            active_deck: DeckId::A,
            deck_a: VirtualDeck::new(sample_rate),
            deck_b: VirtualDeck::new(sample_rate),
            master_volume: 1.0,
            master_eq: MasterEqChain::new(sample_rate),
            echo_bus: BeatEchoBus::new(sample_rate),
            transition: None,
            cmd_rx,
            telemetry,
            control_counter: 0,
        }
    }

    #[inline]
    fn deck_mut(&mut self, id: DeckId) -> &mut VirtualDeck {
        match id {
            DeckId::A => &mut self.deck_a,
            DeckId::B => &mut self.deck_b,
        }
    }

    fn process_commands(&mut self) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
                EngineCommand::AttachDeckConsumer {
                    deck,
                    consumer,
                    total_frames,
                    start_frame,
                    bpm,
                    autoplay,
                } => {
                    self.deck_mut(deck)
                        .attach_consumer(consumer, total_frames, start_frame, bpm, autoplay);
                    if autoplay && self.transition.is_none() {
                        self.active_deck = deck;
                    }
                }
                EngineCommand::Play(deck) => {
                    let d = self.deck_mut(deck);
                    d.gain = 1.0;
                    d.ended = false;
                    d.playing = true;
                    if self.transition.is_none() {
                        self.active_deck = deck;
                    }
                }
                EngineCommand::Pause(deck) => {
                    self.deck_mut(deck).playing = false;
                }
                EngineCommand::Stop(deck) => {
                    self.deck_mut(deck).stop();
                    if self
                        .transition
                        .as_ref()
                        .is_some_and(|tr| tr.from == deck || tr.to == deck)
                    {
                        self.transition = None;
                        self.echo_bus.clear();
                        self.telemetry.set_transition_progress(None);
                    }
                }
                EngineCommand::SetPlaybackRate { deck, rate } => {
                    self.deck_mut(deck).set_rate_clamped(rate);
                }
                EngineCommand::SetMasterVolume(vol) => {
                    self.master_volume = vol.clamp(0.0, 1.0);
                }
                EngineCommand::SetEq(params) => {
                    self.master_eq.update(params);
                }
                EngineCommand::CancelTransition => {
                    if let Some(tr) = self.transition.take() {
                        self.deck_mut(tr.from).stop();
                        let to_deck = self.deck_mut(tr.to);
                        to_deck.gain = 1.0;
                        to_deck.playback_rate = 1.0;
                        to_deck.filter.bypass();
                        self.active_deck = tr.to;
                        self.echo_bus.clear();
                        self.telemetry.set_transition_progress(None);
                    }
                }
                EngineCommand::StartTransition {
                    from,
                    to,
                    duration_secs,
                    from_target_rate,
                    to_initial_rate,
                    enable_bass_swap,
                    enable_echo,
                    bridge_bpm,
                } => {
                    let total_frames = ((duration_secs.max(0.5)) * self.sample_rate) as u64;
                    let from_start_rate = self.deck_mut(from).playback_rate;
                    let safe_from_target =
                        from_target_rate.clamp(MIN_BEATMATCH_RATE, MAX_BEATMATCH_RATE);
                    let safe_to_initial =
                        to_initial_rate.clamp(MIN_BEATMATCH_RATE, MAX_BEATMATCH_RATE);

                    let to_deck = self.deck_mut(to);
                    to_deck.set_rate_clamped(safe_to_initial);
                    to_deck.gain = 0.0;
                    to_deck.ended = false;
                    if enable_bass_swap {
                        to_deck.filter.set_high_pass(20.0);
                        to_deck.filter.set_low_shelf_db(-18.0);
                    } else {
                        to_deck.filter.bypass();
                    }
                    to_deck.playing = true;
                    self.active_deck = to;

                    self.transition = Some(ActiveTransition {
                        from,
                        to,
                        elapsed_frames: 0,
                        total_frames,
                        from_start_rate,
                        from_target_rate: safe_from_target,
                        to_initial_rate: safe_to_initial,
                        enable_bass_swap,
                        enable_echo,
                        bridge_bpm: bridge_bpm.clamp(70.0, 180.0),
                    });
                }
            }
        }
    }

    fn update_transition_automation(&mut self) {
        let Some(tr) = self.transition.as_ref() else {
            self.telemetry.set_transition_progress(None);
            return;
        };

        let t = (tr.elapsed_frames as f32 / tr.total_frames.max(1) as f32).clamp(0.0, 1.0);
        self.telemetry.set_transition_progress(Some(t));

        let (out_gain, in_gain) = equal_power_gains(t);
        let from_id = tr.from;
        let to_id = tr.to;
        let enable_bass_swap = tr.enable_bass_swap;
        let enable_echo = tr.enable_echo;
        let bridge_bpm = tr.bridge_bpm;

        // Smooth pitch-safe tempo morphing (±3.5% max) across both decks
        let out_rate = {
            let p = (t / 0.40).min(1.0) as f64;
            tr.from_start_rate + (tr.from_target_rate - tr.from_start_rate) * p
        };
        let in_rate = if t <= 0.55 {
            tr.to_initial_rate
        } else {
            let p = ((t - 0.55) / 0.45).clamp(0.0, 1.0) as f64;
            tr.to_initial_rate + (1.0 - tr.to_initial_rate) * p
        };

        {
            let from_deck = self.deck_mut(from_id);
            from_deck.gain = out_gain;
            from_deck.set_rate_clamped(out_rate);
            if enable_bass_swap {
                let hp_t = ((t - 0.25) / 0.50).clamp(0.0, 1.0);
                let cutoff_hz = 20.0 * (380.0_f32 / 20.0).powf(hp_t);
                from_deck.filter.set_high_pass(cutoff_hz);
            }
        }

        {
            let to_deck = self.deck_mut(to_id);
            to_deck.gain = in_gain;
            to_deck.set_rate_clamped(in_rate);
            if enable_bass_swap {
                let bass_t = ((t - 0.35) / 0.25).clamp(0.0, 1.0);
                let shelf_db = -18.0 * (1.0 - bass_t);
                to_deck.filter.set_low_shelf_db(shelf_db);
            }
        }

        if enable_echo {
            let echo_send = if t < 0.35 {
                0.0
            } else if t < 0.85 {
                ((t - 0.35) / 0.50) * 0.36
            } else {
                ((1.0 - t) / 0.15) * 0.36
            };
            self.echo_bus.configure(bridge_bpm, echo_send, 0.42);
        }

        if t >= 1.0 {
            self.deck_mut(from_id).stop();
            let to_deck = self.deck_mut(to_id);
            to_deck.gain = 1.0;
            to_deck.playback_rate = 1.0;
            to_deck.filter.bypass();
            self.active_deck = to_id;
            self.echo_bus.configure(bridge_bpm, 0.0, 0.42);
            self.transition = None;
            self.telemetry.set_transition_progress(None);
        }
    }

    pub(crate) fn render(&mut self, output: &mut [f32], channels: usize) {
        self.process_commands();

        for frame in output.chunks_mut(channels) {
            if self.control_counter == 0 {
                self.update_transition_automation();
            }
            self.control_counter = (self.control_counter + 1) % CONTROL_BLOCK_FRAMES;

            if let Some(tr) = self.transition.as_mut() {
                tr.elapsed_frames = tr.elapsed_frames.saturating_add(1);
            }

            let (al, ar) = self.deck_a.next_stereo_sample();
            let (bl, br) = self.deck_b.next_stereo_sample();

            // Feed outgoing deck into beat-synced dub echo bus during transitions
            let (echo_in_l, echo_in_r) = match self.transition.as_ref().map(|tr| tr.from) {
                Some(DeckId::A) => (al, ar),
                Some(DeckId::B) => (bl, br),
                None => (0.0, 0.0),
            };
            let (el, er) = self.echo_bus.process_stereo(echo_in_l, echo_in_r);

            let (eq_l, eq_r) = self.master_eq.process_stereo(al + bl + el, ar + br + er);
            let mix_l = soft_clip(eq_l * self.master_volume);
            let mix_r = soft_clip(eq_r * self.master_volume);

            if channels == 1 {
                frame[0] = (mix_l + mix_r) * 0.5;
            } else {
                frame[0] = mix_l;
                frame[1] = mix_r;
                for extra in frame.iter_mut().skip(2) {
                    *extra = 0.0;
                }
            }
        }

        self.telemetry
            .active_deck_is_b
            .store(self.active_deck == DeckId::B, Ordering::Relaxed);
        self.telemetry
            .deck_a_frame
            .store(self.deck_a.position_frames as u64, Ordering::Relaxed);
        self.telemetry
            .deck_b_frame
            .store(self.deck_b.position_frames as u64, Ordering::Relaxed);
        self.telemetry
            .deck_a_playing
            .store(self.deck_a.playing, Ordering::Relaxed);
        self.telemetry
            .deck_b_playing
            .store(self.deck_b.playing, Ordering::Relaxed);
        self.telemetry
            .deck_a_ended
            .store(self.deck_a.ended, Ordering::Relaxed);
        self.telemetry
            .deck_b_ended
            .store(self.deck_b.ended, Ordering::Relaxed);
    }
}

/// Thread-safe (`Send + Sync`) handle managed by Tauri (`AppCoreState`).
/// Owns a dedicated background thread that holds the non-Send `cpal::Stream` and automatically
/// rebuilds it if the output device disconnects (e.g. Bluetooth headphones on Android/Linux).
#[derive(Clone)]
pub struct AudioController {
    cmd_tx: Sender<EngineCommand>,
    telemetry: Arc<SharedTelemetry>,
    shutdown: Arc<AtomicBool>,
}

impl AudioController {
    pub fn start() -> Result<Self, AudioEngineError> {
        let (cmd_tx, cmd_rx) = bounded::<EngineCommand>(64);
        let telemetry = Arc::new(SharedTelemetry::default());
        let shutdown = Arc::new(AtomicBool::new(false));

        let (ready_tx, ready_rx) = bounded::<Result<u32, AudioEngineError>>(1);
        let tel_bg = Arc::clone(&telemetry);
        let stop_bg = Arc::clone(&shutdown);

        thread::Builder::new()
            .name("gyrotube-cpal-owner".into())
            .spawn(move || {
                run_cpal_owner_loop(cmd_rx, tel_bg, stop_bg, ready_tx);
            })
            .map_err(|e| AudioEngineError::StreamBuild(e.to_string()))?;

        match ready_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(sr)) => {
                telemetry.device_sample_rate.store(sr, Ordering::Relaxed);
                Ok(Self {
                    cmd_tx,
                    telemetry,
                    shutdown,
                })
            }
            Ok(Err(err)) => Err(err),
            Err(_) => Err(AudioEngineError::StreamBuild(
                "Timed out initializing cpal output stream".into(),
            )),
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.telemetry
            .device_sample_rate
            .load(Ordering::Relaxed)
            .max(8_000)
    }

    pub fn send(&self, cmd: EngineCommand) -> Result<(), AudioEngineError> {
        self.cmd_tx
            .try_send(cmd)
            .map_err(|_| AudioEngineError::CommandChannel)
    }

    pub fn set_eq(&self, params: EqParams) -> Result<(), AudioEngineError> {
        self.send(EngineCommand::SetEq(params))
    }

    pub fn set_volume(&self, volume: f32) -> Result<(), AudioEngineError> {
        self.send(EngineCommand::SetMasterVolume(volume))
    }

    pub fn telemetry_snapshot(&self) -> PlaybackTick {
        self.telemetry.snapshot()
    }

    pub fn telemetry(&self) -> Arc<SharedTelemetry> {
        Arc::clone(&self.telemetry)
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn build_cpal_stream(
    cmd_rx: Receiver<EngineCommand>,
    telemetry: Arc<SharedTelemetry>,
    disconnect_flag: Arc<AtomicBool>,
) -> Result<(cpal::Stream, u32), AudioEngineError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(AudioEngineError::NoOutputDevice)?;
    let config = device
        .default_output_config()
        .map_err(|e| AudioEngineError::DeviceConfig(e.to_string()))?;

    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;
    telemetry
        .device_sample_rate
        .store(sample_rate, Ordering::Relaxed);

    let mut render_state = AudioRenderState::new(sample_rate as f32, cmd_rx, telemetry);
    let err_flag = Arc::clone(&disconnect_flag);
    let err_fn = move |err: cpal::StreamError| {
        eprintln!("[gyrotube-native-audio] cpal stream error: {err}");
        err_flag.store(true, Ordering::Relaxed);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| render_state.render(data, channels),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config.into(),
            move |data: &mut [i16], _| {
                let mut tmp = [0.0_f32; 128];
                for chunk in data.chunks_mut(128) {
                    let len = chunk.len();
                    render_state.render(&mut tmp[..len], channels);
                    for (dst, &src) in chunk.iter_mut().zip(&tmp[..len]) {
                        *dst = (src.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    }
                }
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config.into(),
            move |data: &mut [u16], _| {
                let mut tmp = [0.0_f32; 128];
                for chunk in data.chunks_mut(128) {
                    let len = chunk.len();
                    render_state.render(&mut tmp[..len], channels);
                    for (dst, &src) in chunk.iter_mut().zip(&tmp[..len]) {
                        let norm = (src.clamp(-1.0, 1.0) * 0.5 + 0.5) * u16::MAX as f32;
                        *dst = norm as u16;
                    }
                }
            },
            err_fn,
            None,
        ),
        other => {
            return Err(AudioEngineError::DeviceConfig(format!(
                "Unsupported cpal sample format: {other:?}"
            )));
        }
    }
    .map_err(|e| AudioEngineError::StreamBuild(e.to_string()))?;

    stream
        .play()
        .map_err(|e| AudioEngineError::StreamPlay(e.to_string()))?;

    Ok((stream, sample_rate))
}

fn run_cpal_owner_loop(
    cmd_rx: Receiver<EngineCommand>,
    telemetry: Arc<SharedTelemetry>,
    shutdown: Arc<AtomicBool>,
    ready_tx: Sender<Result<u32, AudioEngineError>>,
) {
    let disconnect_flag = Arc::new(AtomicBool::new(false));
    let mut active_stream = match build_cpal_stream(
        cmd_rx.clone(),
        Arc::clone(&telemetry),
        Arc::clone(&disconnect_flag),
    ) {
        Ok((stream, sr)) => {
            let _ = ready_tx.send(Ok(sr));
            Some(stream)
        }
        Err(err) => {
            let _ = ready_tx.send(Err(err));
            return;
        }
    };

    while !shutdown.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(150));
        if disconnect_flag.swap(false, Ordering::Relaxed) {
            // Output device changed (e.g. Bluetooth / wired headset plug/unplug on Android/Linux):
            // drop the stale stream and rebuild automatically on the new default output device.
            drop(active_stream.take());
            thread::sleep(Duration::from_millis(200));
            match build_cpal_stream(
                cmd_rx.clone(),
                Arc::clone(&telemetry),
                Arc::clone(&disconnect_flag),
            ) {
                Ok((stream, _)) => active_stream = Some(stream),
                Err(err) => {
                    eprintln!("[gyrotube-native-audio] Failed to rebuild cpal stream: {err}");
                    disconnect_flag.store(true, Ordering::Relaxed);
                }
            }
        }
    }
    drop(active_stream);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_power_crossfade_preserves_constant_power() {
        for i in 0..=20 {
            let t = i as f32 / 20.0;
            let (out_g, in_g) = equal_power_gains(t);
            let power = out_g * out_g + in_g * in_g;
            assert!((power - 1.0).abs() < 1e-5, "power={power} at t={t}");
        }
    }

    #[test]
    fn deck_clamps_beatmatch_rate_within_pitch_safe_bounds() {
        let mut deck = VirtualDeck::new(48_000.0);
        deck.set_rate_clamped(1.25);
        assert!((deck.playback_rate - MAX_BEATMATCH_RATE).abs() < 1e-9);
        deck.set_rate_clamped(0.75);
        assert!((deck.playback_rate - MIN_BEATMATCH_RATE).abs() < 1e-9);
    }

    #[test]
    fn lock_free_dual_deck_transition_renders_clean_audio_and_completes() {
        let (cmd_tx, cmd_rx) = bounded::<EngineCommand>(16);
        let telemetry = Arc::new(SharedTelemetry::default());
        let mut state = AudioRenderState::new(48_000.0, cmd_rx, Arc::clone(&telemetry));

        let (mut prod_a, cons_a) = rtrb::RingBuffer::<f32>::new(48_000 * 4);
        let (mut prod_b, cons_b) = rtrb::RingBuffer::<f32>::new(48_000 * 4);
        for i in 0..(48_000 * 2) {
            let s_a = (2.0 * std::f32::consts::PI * 220.0 * i as f32 / 48_000.0).sin() * 0.4;
            let s_b = (2.0 * std::f32::consts::PI * 440.0 * i as f32 / 48_000.0).sin() * 0.4;
            let _ = prod_a.push(s_a);
            let _ = prod_a.push(s_a);
            let _ = prod_b.push(s_b);
            let _ = prod_b.push(s_b);
        }

        cmd_tx
            .send(EngineCommand::AttachDeckConsumer {
                deck: DeckId::A,
                consumer: cons_a,
                total_frames: 96_000,
                start_frame: 0,
                bpm: 120.0,
                autoplay: true,
            })
            .unwrap();
        cmd_tx
            .send(EngineCommand::AttachDeckConsumer {
                deck: DeckId::B,
                consumer: cons_b,
                total_frames: 96_000,
                start_frame: 0,
                bpm: 123.0,
                autoplay: false,
            })
            .unwrap();
        cmd_tx
            .send(EngineCommand::StartTransition {
                from: DeckId::A,
                to: DeckId::B,
                duration_secs: 0.5,
                from_target_rate: 1.015,
                to_initial_rate: 0.985,
                enable_bass_swap: true,
                enable_echo: true,
                bridge_bpm: 121.5,
            })
            .unwrap();

        // Render 0.6 s of stereo audio (28,800 frames) so the 0.5 s transition finishes cleanly
        let mut out = vec![0.0_f32; 256 * 2];
        let mut max_abs = 0.0_f32;
        for _ in 0..120 {
            state.render(&mut out, 2);
            for &s in &out {
                assert!(s.is_finite());
                max_abs = max_abs.max(s.abs());
            }
        }

        assert!(max_abs > 0.05 && max_abs <= 1.0, "max_abs={max_abs}");
        let snap = telemetry.snapshot();
        assert_eq!(snap.active_deck, DeckId::B);
        assert!(!snap.deck_a_playing);
        assert!(snap.deck_b_playing);
        assert!(snap.transition_progress.is_none());
    }
}
