import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { api, isTauriRuntime } from "./api";
import type { EqSettings } from "./equalizer";
import type { TransitionPlan } from "./mixPlanner";
import type { DeckId, NativeLoadResult, PlaybackTick, Track } from "./types";

export interface NativeAudioCallbacks {
  onTimeUpdate: (positionSecs: number, durationSecs: number) => void;
  onPlayStateChange: (playing: boolean) => void;
  onTransitionProgress: (progress: number | null) => void;
  onEnded: () => void;
  onCrossfadeTrigger: () => void;
}

/**
 * Lightweight frontend bridge to GyroTube's lock-free native Rust cpal/Symphonia engine.
 * Subscribes to `playback-tick` (30 ms while active, 0% CPU when paused) and interpolates
 * `getExactPositionSecs()` with `performance.now()` for silky 60 FPS canvas & video sync.
 */
export class NativeAudioClient {
  private callbacks: NativeAudioCallbacks;
  private unlisten: UnlistenFn | null = null;
  private available = isTauriRuntime();
  private activeDeck: DeckId = "A";
  private deckDuration: Record<DeckId, number> = { A: 0, B: 0 };
  private isPlaying = false;
  private lastTickPosSecs = 0;
  private lastTickPerfMs = 0;
  private lastEndedFlag = false;
  private lastTransitionProgress: number | null = null;
  private userVolume = 1.0;
  private duckFactor = 1.0;
  private crossfadeSecs = 0;
  private mixMode = false;
  private plannedTransition: TransitionPlan | null = null;
  private transitionTriggered = false;
  private opGen = 0;

  constructor(callbacks: NativeAudioCallbacks) {
    this.callbacks = callbacks;
  }

  public get isAvailable(): boolean {
    return this.available;
  }

  public get currentDeck(): DeckId {
    return this.activeDeck;
  }

  public async bind(): Promise<void> {
    if (!isTauriRuntime()) {
      this.available = false;
      return;
    }
    this.dispose();
    try {
      this.unlisten = await listen<PlaybackTick>("playback-tick", (event) => {
        this.handleTick(event.payload);
      });
    } catch {
      this.available = false;
    }
  }

  public dispose(): void {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
  }

  private handleTick(tick: PlaybackTick): void {
    const active = tick.active_deck;
    this.activeDeck = active;
    const pos = active === "A" ? tick.deck_a_secs : tick.deck_b_secs;
    const playing = active === "A" ? tick.deck_a_playing : tick.deck_b_playing;
    const ended = active === "A" ? tick.deck_a_ended : tick.deck_b_ended;
    const dur = this.deckDuration[active] || 0;

    this.lastTickPosSecs = Math.max(0, pos);
    this.lastTickPerfMs = performance.now();

    if (playing !== this.isPlaying) {
      this.isPlaying = playing;
      this.callbacks.onPlayStateChange(playing);
    }

    if (tick.transition_progress !== this.lastTransitionProgress) {
      this.lastTransitionProgress = tick.transition_progress;
      this.callbacks.onTransitionProgress(tick.transition_progress);
    }

    this.callbacks.onTimeUpdate(this.lastTickPosSecs, dur);

    // Trigger next mix/crossfade when active deck reaches planned cue point
    if (playing && !this.transitionTriggered && tick.transition_progress === null && dur > 4) {
      const triggerAt = this.resolveTriggerAtSecs(dur);
      if (triggerAt !== null && pos >= triggerAt) {
        this.transitionTriggered = true;
        this.callbacks.onCrossfadeTrigger();
      }
    }

    // Fire ended once when active deck finishes naturally outside an active transition
    if (ended && !this.lastEndedFlag && tick.transition_progress === null) {
      this.lastEndedFlag = true;
      this.callbacks.onEnded();
    } else if (!ended) {
      this.lastEndedFlag = false;
    }
  }

  private resolveTriggerAtSecs(dur: number): number | null {
    if (dur <= 4) return null;
    if (this.plannedTransition) {
      return Math.min(dur - 0.5, Math.max(1, this.plannedTransition.triggerAtSecs));
    }
    if (this.mixMode) {
      const mixDur = Math.min(Math.max(4, this.crossfadeSecs || 10), Math.max(3, dur * 0.24));
      return dur > mixDur + 2 ? dur - mixDur : null;
    }
    if (this.crossfadeSecs > 0 && dur > this.crossfadeSecs + 2) {
      return dur - this.crossfadeSecs;
    }
    return null;
  }

  /**
   * Returns sub-millisecond interpolated playback position for 60 FPS canvas & video sync.
   */
  public getExactPositionSecs(): number {
    if (!this.isPlaying) {
      return this.lastTickPosSecs;
    }
    const elapsed = (performance.now() - this.lastTickPerfMs) / 1000;
    const raw = this.lastTickPosSecs + Math.max(0, elapsed);
    const dur = this.deckDuration[this.activeDeck] || 0;
    return dur > 0 ? Math.min(dur, raw) : raw;
  }

  public setUserVolume(volume: number): void {
    this.userVolume = Math.max(0, Math.min(1, volume));
    void this.syncEffectiveVolume();
  }

  public setDucking(ducked: boolean): void {
    const next = ducked ? 0.25 : 1.0;
    if (this.duckFactor === next) return;
    this.duckFactor = next;
    void this.syncEffectiveVolume();
  }

  private async syncEffectiveVolume(): Promise<void> {
    if (!this.available) return;
    const eff = Math.max(0, Math.min(1, this.userVolume * this.duckFactor));
    try {
      await api.nativeSetVolume(eff);
    } catch {
      // ignore
    }
  }

  public applyEqSettings(settings: EqSettings): void {
    if (!this.available) return;
    const b = settings.bands;
    void api
      .nativeSetEq({
        enabled: settings.enabled,
        normalize: settings.normalize,
        bands_db: [b[0] ?? 0, b[1] ?? 0, b[2] ?? 0, b[3] ?? 0, b[4] ?? 0],
      })
      .catch(() => {});
  }

  public setCrossfadeSecs(secs: number): void {
    this.crossfadeSecs = Math.max(0, Math.min(10, Math.round(secs)));
  }

  public setMixMode(enabled: boolean): void {
    this.mixMode = enabled;
    if (!enabled) {
      this.plannedTransition = null;
    }
  }

  public setPlannedTransition(plan: TransitionPlan | null): void {
    this.plannedTransition = plan;
  }

  public async loadAndPlay(
    track: Track,
    startPositionSecs = 0,
    useCrossfade = false,
    startPaused = false,
    transitionPlan: TransitionPlan | null = null,
  ): Promise<NativeLoadResult | null> {
    if (!this.available) return null;
    const gen = ++this.opGen;
    this.transitionTriggered = false;
    this.lastEndedFlag = false;
    this.plannedTransition = null;

    const outgoingDeck = this.activeDeck;
    const effectivePlan = useCrossfade ? transitionPlan : null;
    const effectiveFadeSecs = effectivePlan
      ? effectivePlan.durationSecs
      : this.mixMode
        ? Math.max(6, this.crossfadeSecs || 10)
        : this.crossfadeSecs;
    const canMix = useCrossfade && !startPaused && this.isPlaying && effectiveFadeSecs > 0;
    const incomingDeck: DeckId = canMix ? (outgoingDeck === "A" ? "B" : "A") : outgoingDeck;
    const effectiveStart =
      startPositionSecs > 0
        ? startPositionSecs
        : canMix && effectivePlan
          ? effectivePlan.incomingStartSecs
          : 0;

    try {
      const res = await api.nativeLoadDeck(
        incomingDeck,
        track,
        effectiveStart,
        !canMix && !startPaused,
      );
      if (this.opGen !== gen) return null;

      this.deckDuration[incomingDeck] =
        res.duration_secs > 0 ? res.duration_secs : (track.duration_secs ?? 0);

      if (canMix) {
        const isBeatmatch = effectivePlan?.mode === "beatmatch";
        const bridgeBpm =
          effectivePlan?.outBpm ?? effectivePlan?.inBpm ?? 120;
        await api.nativeTriggerMix({
          from: outgoingDeck,
          to: incomingDeck,
          durationSecs: effectiveFadeSecs,
          fromTargetRate: isBeatmatch ? effectivePlan.outgoingTargetPlaybackRate : 1.0,
          toInitialRate: isBeatmatch ? effectivePlan.initialPlaybackRate : 1.0,
          enableBassSwap: Boolean(effectivePlan?.useBassSwap ?? this.mixMode),
          enableEcho: this.mixMode,
          bridgeBpm,
        });
        this.activeDeck = incomingDeck;
        this.lastTickPosSecs = effectiveStart;
        this.lastTickPerfMs = performance.now();
        this.isPlaying = true;
        this.callbacks.onPlayStateChange(true);
      } else {
        const other: DeckId = incomingDeck === "A" ? "B" : "A";
        void api.nativeStop(other).catch(() => {});
        this.activeDeck = incomingDeck;
        this.lastTickPosSecs = effectiveStart;
        this.lastTickPerfMs = performance.now();
        this.isPlaying = !startPaused;
        this.callbacks.onPlayStateChange(!startPaused);
      }

      return res;
    } catch (err) {
      const msg = String(err);
      console.warn("[GyroTube] native_load_deck fallback to HTML5:", msg);
      if (msg.includes("Native audio output device unavailable")) {
        this.available = false;
      }
      return null;
    }
  }

  public async play(): Promise<void> {
    if (!this.available) return;
    await api.nativePlay(this.activeDeck);
    this.lastTickPerfMs = performance.now();
    this.isPlaying = true;
    this.callbacks.onPlayStateChange(true);
  }

  public async pause(): Promise<void> {
    if (!this.available) return;
    this.lastTickPosSecs = this.getExactPositionSecs();
    await api.nativePause(this.activeDeck);
    this.isPlaying = false;
    this.callbacks.onPlayStateChange(false);
  }

  public async seek(secs: number): Promise<void> {
    if (!this.available) return;
    const dur = this.deckDuration[this.activeDeck] || 0;
    const clamped = dur > 0 ? Math.max(0, Math.min(secs, dur)) : Math.max(0, secs);
    const triggerAt = this.resolveTriggerAtSecs(dur);
    if (triggerAt !== null && clamped < triggerAt - 0.5) {
      this.transitionTriggered = false;
    }
    this.lastTickPosSecs = clamped;
    this.lastTickPerfMs = performance.now();
    await api.nativeCancelMix().catch(() => {});
    await api.nativeSeek(this.activeDeck, clamped);
    this.callbacks.onTimeUpdate(clamped, dur);
  }

  public stop(): void {
    ++this.opGen;
    this.isPlaying = false;
    this.lastTickPosSecs = 0;
    this.lastTransitionProgress = null;
    this.transitionTriggered = false;
    if (this.available) {
      void api.nativeStop(null).catch(() => {});
    }
  }
}
