import { isTauriRuntime } from "./api";
import { DEFAULT_EQ_SETTINGS, EQ_FREQUENCIES, type EqSettings } from "./equalizer";
import type { AudioBufferSlice } from "./mixAnalyzer";
import type { TransitionPlan } from "./mixPlanner";
import { MixTransitionBridge } from "./mixTransitionBridge";
import type { Track } from "./types";

interface DeckAudioChannel {
  source: MediaElementAudioSourceNode;
  lowShelf: BiquadFilterNode;
  gain: GainNode;
}

function usesNativeLinuxMpris(): boolean {
  if (!isTauriRuntime() || typeof navigator === "undefined") return false;
  const ua = navigator.userAgent || "";
  return /Linux/i.test(ua) && !/Android/i.test(ua);
}

function streamTrackKey(url: string | null): string | null {
  if (!url) return null;
  const m = /\/stream\/([A-Za-z0-9_-]{11})-\d+\.[a-z0-9]+$/i.exec(url);
  return m ? m[1] : url;
}

function enablePitchPreservation(audio: HTMLAudioElement): void {
  const pitchEl = audio as HTMLAudioElement & {
    preservesPitch?: boolean;
    mozPreservesPitch?: boolean;
    webkitPreservesPitch?: boolean;
  };
  pitchEl.preservesPitch = true;
  pitchEl.mozPreservesPitch = true;
  pitchEl.webkitPreservesPitch = true;
}

export interface AudioCallbacks {
  onTimeUpdate: (position: number, duration: number) => void;
  onPlayStateChange: (playing: boolean) => void;
  onEnded: () => void;
  onCrossfadeTrigger: () => void;
  onError: (message: string) => void;
}

/**
 * Dual-deck HTML5 Audio engine supporting smooth crossfading via requestAnimationFrame
 * volume ramping directly from gyrotube-core's local HTTP Range streamer (`127.0.0.1`).
 *
 * Avoids Web Audio API `createMediaElementSource` because WebKitGTK / WKWebView have
 * CORS and background-suspension issues with media element audio graphs.
 */
export class DualDeckAudioEngine {
  private decks: [HTMLAudioElement, HTMLAudioElement];
  private activeIndex: 0 | 1 = 0;
  private volume = 1.0;
  private crossfadeSecs = 0;
  private mixMode = false;
  private plannedTransition: TransitionPlan | null = null;
  private pendingPreCueUrl: string | null = null;
  private preCuedUrl: string | null = null;
  private preCuedIndex: 0 | 1 | null = null;
  private crossfadeFired = false;
  private precisionTriggerTimer: number | null = null;
  private fadeTimer: number | null = null;
  private handoffRampTimer: number | null = null;
  private fadeStep: (() => void) | null = null;
  private dspBridge = new MixTransitionBridge();
  private loading = false;
  private opGen = 0;
  private lastPositionStateSync = 0;
  private disposingTimers = new Map<HTMLAudioElement, number>();
  private callbacks: AudioCallbacks;

  // Optional Web Audio API graph (lazy: only allocated when EQ or Normalization is enabled;
  // never forced by Mix Mode alone because WebKitGTK's createMediaElementSource can silence <audio>)
  private eqSettings: EqSettings = { ...DEFAULT_EQ_SETTINGS, bands: [...DEFAULT_EQ_SETTINGS.bands] };
  private audioCtx: AudioContext | null = null;
  private eqFilters: BiquadFilterNode[] = [];
  private compressor: DynamicsCompressorNode | null = null;
  private graphInput: AudioNode | null = null;
  private deckChannels = new Map<HTMLAudioElement, DeckAudioChannel>();

  constructor(callbacks: AudioCallbacks) {
    this.callbacks = callbacks;
    this.decks = [this.createDeck(0), this.createDeck(1)];
  }

  private needsAudioGraph(): boolean {
    return this.eqSettings.enabled || this.eqSettings.normalize;
  }

  private ensureAudioGraph(): AudioNode | null {
    if (!this.needsAudioGraph()) return null;
    if (typeof window === "undefined") return null;
    const Ctx = window.AudioContext || (window as unknown as { webkitAudioContext?: typeof AudioContext }).webkitAudioContext;
    if (!Ctx) return null;

    if (!this.audioCtx) {
      this.audioCtx = new Ctx();
      this.eqFilters = EQ_FREQUENCIES.map((freq, idx) => {
        const filter = this.audioCtx!.createBiquadFilter();
        filter.type = idx === 0 ? "lowshelf" : idx === EQ_FREQUENCIES.length - 1 ? "highshelf" : "peaking";
        filter.frequency.value = freq;
        filter.Q.value = 1.0;
        filter.gain.value = this.eqSettings.enabled ? this.eqSettings.bands[idx] : 0;
        return filter;
      });
      for (let i = 0; i < this.eqFilters.length - 1; i++) {
        this.eqFilters[i].connect(this.eqFilters[i + 1]);
      }
      this.compressor = this.audioCtx.createDynamicsCompressor();
      this.applyCompressorParams();
      this.eqFilters[this.eqFilters.length - 1].connect(this.compressor);
      this.compressor.connect(this.audioCtx.destination);
      this.graphInput = this.eqFilters[0];
    }
    return this.graphInput;
  }

  private applyCompressorParams(): void {
    if (!this.compressor || !this.audioCtx) return;
    const now = this.audioCtx.currentTime;
    if (this.eqSettings.normalize) {
      this.compressor.threshold.setValueAtTime(-18, now);
      this.compressor.knee.setValueAtTime(24, now);
      this.compressor.ratio.setValueAtTime(4, now);
      this.compressor.attack.setValueAtTime(0.005, now);
      this.compressor.release.setValueAtTime(0.2, now);
    } else {
      // Bypass-like neutral settings when only EQ is active
      this.compressor.threshold.setValueAtTime(0, now);
      this.compressor.knee.setValueAtTime(0, now);
      this.compressor.ratio.setValueAtTime(1, now);
    }
  }

  private attachAudioGraphIfNeeded(audio: HTMLAudioElement): void {
    if (!this.needsAudioGraph() || this.deckChannels.has(audio)) return;
    const input = this.ensureAudioGraph();
    if (!input || !this.audioCtx) return;
    try {
      const source = this.audioCtx.createMediaElementSource(audio);
      const lowShelf = this.audioCtx.createBiquadFilter();
      lowShelf.type = "lowshelf";
      lowShelf.frequency.value = 200;
      lowShelf.gain.value = 0;
      const gain = this.audioCtx.createGain();
      gain.gain.value = 1;
      source.connect(lowShelf);
      lowShelf.connect(gain);
      gain.connect(input);
      this.deckChannels.set(audio, { source, lowShelf, gain });
    } catch {
      // Ignore if media element cannot be attached (falls back to native HTML5 audio.volume)
    }
  }

  private setDeckBassDb(audio: HTMLAudioElement, db: number, immediate = false): void {
    const ch = this.deckChannels.get(audio);
    if (!ch || !this.audioCtx) return;
    const now = this.audioCtx.currentTime;
    try {
      if (immediate) {
        ch.lowShelf.gain.setValueAtTime(db, now);
      } else {
        ch.lowShelf.gain.setTargetAtTime(db, now, 0.025);
      }
    } catch {
      // Ignore Web Audio parameter scheduling errors on suspended contexts
    }
  }

  private resetDeckMixState(audio: HTMLAudioElement): void {
    this.setDeckBassDb(audio, 0, true);
    if (audio.playbackRate !== 1) {
      audio.playbackRate = 1;
    }
  }

  public applyEqSettings(settings: EqSettings): void {
    this.eqSettings = { ...settings, bands: [...settings.bands] };
    if (this.needsAudioGraph()) {
      this.ensureAudioGraph();
      this.attachAudioGraphIfNeeded(this.activeDeck);
      if (this.audioCtx?.state === "suspended") {
        void this.audioCtx.resume().catch(() => {});
      }
    }
    if (this.audioCtx && this.eqFilters.length === EQ_FREQUENCIES.length) {
      const now = this.audioCtx.currentTime;
      for (let i = 0; i < this.eqFilters.length; i++) {
        const gain = this.eqSettings.enabled ? this.eqSettings.bands[i] : 0;
        this.eqFilters[i].gain.setValueAtTime(gain, now);
      }
      this.applyCompressorParams();
    }
  }

  private createDeck(index: 0 | 1): HTMLAudioElement {
    const audio = document.createElement("audio");
    audio.preload = "auto";
    audio.crossOrigin = "anonymous";

    audio.addEventListener("timeupdate", () => {
      if (this.decks[index] !== audio || index !== this.activeIndex) return;
      // Keep crossfade progressing even if background timers are throttled by the OS
      if (this.fadeStep) {
        this.fadeStep();
      } else if (Math.abs(audio.volume - this.volume) > 0.01) {
        audio.volume = this.volume;
      }
      const pos = Number.isFinite(audio.currentTime) ? audio.currentTime : 0;
      const dur = Number.isFinite(audio.duration) ? audio.duration : 0;
      this.callbacks.onTimeUpdate(pos, dur);
      const now = performance.now();
      if (now - this.lastPositionStateSync >= 1000) {
        this.lastPositionStateSync = now;
        this.syncMediaSessionPosition(pos, dur, audio.paused ? 0 : audio.playbackRate || 1);
      }

      const triggerAt = this.resolveTriggerAtSecs(dur);
      if (triggerAt !== null && !this.crossfadeFired) {
        const remaining = triggerAt - pos;
        // Just-in-time pre-cue: only spin up the standby GStreamer/WebView <audio> element
        // ~6 seconds before the transition (when the current song has finished downloading)
        if (remaining <= 6.5 && remaining > 0.25 && this.pendingPreCueUrl && !this.preCuedUrl) {
          this.executePreCue(this.pendingPreCueUrl, this.plannedTransition);
        }
        if (remaining <= 0.04) {
          this.fireTransitionTrigger();
        } else if (remaining <= 1.6 && this.precisionTriggerTimer === null && !audio.paused) {
          const rate = Math.max(0.5, audio.playbackRate || 1);
          const delayMs = Math.max(0, Math.round((remaining * 1000) / rate));
          const expectedGen = this.opGen;
          this.precisionTriggerTimer = window.setTimeout(() => {
            this.precisionTriggerTimer = null;
            if (
              this.opGen === expectedGen &&
              this.decks[index] === audio &&
              index === this.activeIndex &&
              !audio.paused &&
              !this.crossfadeFired
            ) {
              this.fireTransitionTrigger();
            }
          }, delayMs);
        }
      }
    });

    audio.addEventListener("loadedmetadata", () => {
      if (this.decks[index] === audio && index === this.activeIndex) {
        const pos = Number.isFinite(audio.currentTime) ? audio.currentTime : 0;
        const dur = Number.isFinite(audio.duration) ? audio.duration : 0;
        this.syncMediaSessionPosition(pos, dur, audio.paused ? 0 : audio.playbackRate || 1);
      }
    });

    audio.addEventListener("play", () => {
      if (this.decks[index] === audio && index === this.activeIndex) {
        this.setMediaSessionPlaybackState("playing");
        const pos = Number.isFinite(audio.currentTime) ? audio.currentTime : 0;
        const dur = Number.isFinite(audio.duration) ? audio.duration : 0;
        this.syncMediaSessionPosition(pos, dur, audio.playbackRate || 1);
        this.callbacks.onPlayStateChange(true);
      }
    });
    audio.addEventListener("pause", () => {
      if (this.decks[index] === audio && index === this.activeIndex) {
        this.setMediaSessionPlaybackState("paused");
        const pos = Number.isFinite(audio.currentTime) ? audio.currentTime : 0;
        const dur = Number.isFinite(audio.duration) ? audio.duration : 0;
        this.syncMediaSessionPosition(pos, dur, 0);
        this.callbacks.onPlayStateChange(false);
      }
    });
    audio.addEventListener("ended", () => {
      if (this.decks[index] === audio && index === this.activeIndex) {
        this.callbacks.onEnded();
      }
    });
    audio.addEventListener("error", () => {
      if (
        this.decks[index] === audio &&
        index === this.activeIndex &&
        audio.src &&
        !this.loading
      ) {
        const code = audio.error?.code;
        const detail =
          audio.error?.message ||
          (code ? `Audio error (code ${code})` : "Audio playback error");
        console.error("[GyroTube audio error]", {
          src: audio.src,
          code,
          message: audio.error?.message,
          networkState: audio.networkState,
          readyState: audio.readyState,
        });
        this.callbacks.onError(detail);
      }
    });
    return audio;
  }

  private hardDisposeAudio(audio: HTMLAudioElement): void {
    const timer = this.disposingTimers.get(audio);
    if (timer !== undefined) {
      window.clearInterval(timer);
      this.disposingTimers.delete(audio);
    }
    const ch = this.deckChannels.get(audio);
    if (ch) {
      try {
        ch.source.disconnect();
        ch.lowShelf.disconnect();
        ch.gain.disconnect();
      } catch {
        // ignore
      }
      this.deckChannels.delete(audio);
    }
    audio.muted = true;
    audio.volume = 0;
    audio.pause();
    audio.removeAttribute("src");
    audio.load();
  }

  private flushDisposing(): void {
    for (const audio of [...this.disposingTimers.keys()]) {
      this.hardDisposeAudio(audio);
    }
  }

  private disposeAudio(audio: HTMLAudioElement, immediate = false): void {
    if (immediate || audio.paused || audio.ended || !audio.src || audio.muted || audio.volume <= 0.01) {
      this.hardDisposeAudio(audio);
      return;
    }

    // 1) Smooth 30ms ease-out fade to 0.0 (avoids abrupt click when skipping).
    // 2) Keep the detached <audio> running at volume=0 (with muted=false) for
    //    ~180ms before calling pause(). On Android WebView (Chromium AudioRendererMixer
    //    + AudioTrack/AudioFlinger), pausing immediately freezes ~80-120ms of the
    //    previous track's PCM inside the hardware ring buffer, which then drains
    //    audibly the moment the next cold track wakes the sink. Pumping ~180ms of
    //    zero-amplitude PCM flushes the ring buffer completely with silence first.
    const startVol = audio.volume;
    const fadeMs = 30;
    const silenceFlushMs = 180;
    const totalMs = fadeMs + silenceFlushMs;
    const startedAt = performance.now();
    const timer = window.setInterval(() => {
      const elapsed = performance.now() - startedAt;
      if (elapsed < fadeMs) {
        const t = elapsed / fadeMs;
        audio.volume = Math.max(0, startVol * (1 - t) * (1 - t));
      } else {
        audio.volume = 0;
      }
      if (elapsed >= totalMs) {
        this.hardDisposeAudio(audio);
      }
    }, 10);
    this.disposingTimers.set(audio, timer);
  }

  private resetDeck(index: 0 | 1, immediate = false): HTMLAudioElement {
    const old = this.decks[index];
    const fresh = this.createDeck(index);
    this.decks[index] = fresh;
    this.disposeAudio(old, immediate);
    return fresh;
  }

  private clearPrecisionTrigger(): void {
    if (this.precisionTriggerTimer !== null) {
      window.clearTimeout(this.precisionTriggerTimer);
      this.precisionTriggerTimer = null;
    }
  }

  private resolveTriggerAtSecs(dur: number): number | null {
    if (dur <= 4) return null;
    if (this.plannedTransition) {
      return Math.min(dur - 0.5, Math.max(1, this.plannedTransition.triggerAtSecs));
    }
    if (this.mixMode) {
      const mixDur = Math.min(Math.max(4, this.crossfadeSecs || 8), Math.max(3, dur * 0.25));
      return dur > mixDur + 2 ? dur - mixDur : null;
    }
    if (this.crossfadeSecs > 0 && dur > this.crossfadeSecs + 2) {
      return dur - this.crossfadeSecs;
    }
    return null;
  }

  private fireTransitionTrigger(): void {
    if (this.crossfadeFired) return;
    this.clearPrecisionTrigger();
    this.crossfadeFired = true;
    this.callbacks.onCrossfadeTrigger();
  }

  private cancelFade(): void {
    this.clearPrecisionTrigger();
    this.dspBridge.stop();
    if (this.handoffRampTimer !== null) {
      window.clearInterval(this.handoffRampTimer);
      this.handoffRampTimer = null;
    }
    if (this.fadeTimer !== null) {
      window.clearInterval(this.fadeTimer);
      this.fadeTimer = null;
    }
    this.fadeStep = null;
  }

  public get activeDeck(): HTMLAudioElement {
    return this.decks[this.activeIndex];
  }

  public get isBridgeActive(): boolean {
    return this.dspBridge.isActive;
  }

  public getBridgePositionSecs(): number | null {
    return this.dspBridge.getCurrentIncomingPosSecs();
  }

  public setVolume(v: number): void {
    this.volume = Math.max(0, Math.min(1, v));
    this.activeDeck.volume = this.volume;
    this.dspBridge.setVolume(this.volume);
  }

  public setCrossfadeSecs(secs: number): void {
    this.crossfadeSecs = Math.max(0, Math.min(10, Math.round(secs)));
  }

  public setMixMode(enabled: boolean): void {
    this.mixMode = enabled;
    if (!enabled) {
      this.plannedTransition = null;
      this.clearPrecisionTrigger();
      this.resetDeckMixState(this.activeDeck);
    }
  }

  public setPlannedTransition(plan: TransitionPlan | null): void {
    this.plannedTransition = plan;
    this.clearPrecisionTrigger();
  }

  /**
   * Registers the next track's URL & plan for just-in-time pre-cuing (~6 s before triggerAtSecs)
   * so we don't hold a second active GStreamer/WebView decoder pipeline during normal playback.
   */
  public preCueNext(url: string, plan: TransitionPlan | null): void {
    if (!this.mixMode || !url) return;
    this.pendingPreCueUrl = url;
    this.plannedTransition = plan;
    const deck = this.activeDeck;
    const dur = Number.isFinite(deck.duration) ? deck.duration : 0;
    const pos = Number.isFinite(deck.currentTime) ? deck.currentTime : 0;
    const triggerAt = this.resolveTriggerAtSecs(dur);
    if (triggerAt !== null && triggerAt - pos <= 6.5) {
      this.executePreCue(url, plan);
    }
  }

  private executePreCue(url: string, plan: TransitionPlan | null): void {
    if (!this.mixMode || !url || !this.activeDeck.src || this.fadeTimer !== null) return;
    const standbyIndex: 0 | 1 = this.activeIndex === 0 ? 1 : 0;
    // On Linux WebKitGTK (GStreamer), dynamic playbackRate changes trigger flushing seeks;
    // keep playbackRate at 1.0 on WebKitGTK so transitions never stutter.
    const allowRateShift = !usesNativeLinuxMpris();
    const initialRate =
      allowRateShift && plan?.mode === "beatmatch"
        ? Math.max(0.88, Math.min(1.12, plan.initialPlaybackRate))
        : 1.0;
    const startPos = plan ? Math.max(0, plan.incomingStartSecs) : 0;
    const targetKey = streamTrackKey(url);

    if (targetKey && streamTrackKey(this.preCuedUrl) === targetKey && this.preCuedIndex === standbyIndex) {
      const existing = this.decks[standbyIndex];
      if (existing.src && existing.paused) {
        existing.playbackRate = initialRate;
        if (existing.readyState >= 1 && Math.abs(existing.currentTime - startPos) > 0.08) {
          try {
            existing.currentTime = startPos;
          } catch {
            // ignore
          }
        }
        return;
      }
    }

    this.resetDeck(standbyIndex, true);
    const standby = this.decks[standbyIndex];
    enablePitchPreservation(standby);
    standby.playbackRate = initialRate;
    standby.volume = 0;
    standby.src = url;
    standby.load();
    this.preCuedUrl = url;
    this.preCuedIndex = standbyIndex;

    if (startPos > 0) {
      standby.addEventListener(
        "loadedmetadata",
        () => {
          if (
            streamTrackKey(this.preCuedUrl) === targetKey &&
            this.decks[standbyIndex] === standby &&
            standby.paused
          ) {
            try {
              standby.currentTime = startPos;
            } catch {
              // ignore
            }
          }
        },
        { once: true },
      );
    }
  }

  public async loadAndPlay(
    url: string,
    startPositionSecs = 0,
    useCrossfade = false,
    startPaused = false,
    transitionPlan: TransitionPlan | null = null,
    bridgeSlices: { outSlice: AudioBufferSlice; inSlice: AudioBufferSlice } | null = null,
  ): Promise<void> {
    const outgoingIndex = this.activeIndex;
    const outgoing = this.decks[outgoingIndex];
    const outgoingPosBeforeCancel = Number.isFinite(outgoing.currentTime) ? outgoing.currentTime : 0;

    this.cancelFade();
    this.crossfadeFired = false;
    this.plannedTransition = null;
    this.loading = true;
    const gen = ++this.opGen;

    const effectivePlan = useCrossfade ? transitionPlan : null;
    const effectiveFadeSecs = effectivePlan
      ? effectivePlan.durationSecs
      : this.mixMode
        ? Math.max(4, this.crossfadeSecs || 8)
        : this.crossfadeSecs;
    const wantDualDeck = useCrossfade && !startPaused && effectiveFadeSecs > 0;
    const canCrossfade =
      wantDualDeck &&
      !outgoing.paused &&
      !outgoing.ended &&
      Boolean(outgoing.src);

    // Always allocate a brand-new HTMLAudioElement for the incoming track unless it was
    // already pre-cued specifically for this exact beat-aligned Mix transition.
    const incomingIndex: 0 | 1 = canCrossfade ? (outgoingIndex === 0 ? 1 : 0) : outgoingIndex;
    const incomingKey = streamTrackKey(url);
    const usePreCued =
      canCrossfade &&
      this.preCuedIndex === incomingIndex &&
      incomingKey !== null &&
      streamTrackKey(this.preCuedUrl) === incomingKey &&
      Boolean(this.decks[incomingIndex].src) &&
      this.decks[incomingIndex].paused &&
      this.decks[incomingIndex].readyState >= 1;
    this.preCuedUrl = null;
    this.preCuedIndex = null;
    this.pendingPreCueUrl = null;

    if (!canCrossfade) {
      // Ensure any previous fading-out deck is fully torn down before starting
      // the new track (avoids Android AudioFocus conflict on fast warm switches)
      this.flushDisposing();
      this.resetDeck(0, true);
      this.resetDeck(1, true);
    } else if (!usePreCued) {
      this.resetDeck(incomingIndex, true);
    }
    const incoming = this.decks[incomingIndex];

    this.activeIndex = incomingIndex;
    this.attachAudioGraphIfNeeded(incoming);
    if (canCrossfade) {
      this.attachAudioGraphIfNeeded(outgoing);
    }
    if (this.audioCtx?.state === "suspended") {
      void this.audioCtx.resume().catch(() => {});
    }

    const effectiveStartPos =
      startPositionSecs > 0
        ? startPositionSecs
        : canCrossfade && effectivePlan
          ? effectivePlan.incomingStartSecs
          : 0;
    // On Linux WebKitGTK, GStreamer's setRate issues a flushing seek (GST_SEEK_FLAG_FLUSH)
    // on every playbackRate mutation, which causes audible stutters/freezes.
    const allowRateShift = !usesNativeLinuxMpris();
    const initialRate =
      allowRateShift && canCrossfade && effectivePlan?.mode === "beatmatch"
        ? Math.max(0.88, Math.min(1.12, effectivePlan.initialPlaybackRate))
        : 1.0;
    const outTargetRate =
      allowRateShift && canCrossfade && effectivePlan?.mode === "beatmatch"
        ? Math.max(0.88, Math.min(1.12, effectivePlan.outgoingTargetPlaybackRate))
        : 1.0;

    // Preserve vocal/harmonic pitch when tempo-stretching both decks for beatmatch
    enablePitchPreservation(incoming);
    incoming.playbackRate = initialRate;
    if (canCrossfade && outTargetRate !== 1.0) {
      enablePitchPreservation(outgoing);
    }

    if (!usePreCued) {
      incoming.src = url;
      incoming.load();
    } else if (effectiveStartPos > 0 && Math.abs(incoming.currentTime - effectiveStartPos) > 0.1) {
      try {
        incoming.currentTime = effectiveStartPos;
      } catch {
        // ignore
      }
    }

    if (effectiveStartPos > 0 && !usePreCued && !(canCrossfade && effectivePlan && bridgeSlices)) {
      if (canCrossfade) {
        // Pre-cue incoming deck before starting play() so beat/intro alignment lands cleanly
        await new Promise<void>((resolve) => {
          if (incoming.readyState >= 1) {
            resolve();
            return;
          }
          const done = () => {
            window.clearTimeout(timer);
            incoming.removeEventListener("loadedmetadata", done);
            incoming.removeEventListener("error", done);
            resolve();
          };
          const timer = window.setTimeout(done, 600);
          incoming.addEventListener("loadedmetadata", done, { once: true });
          incoming.addEventListener("error", done, { once: true });
        });
        if (this.opGen !== gen) return;
        if (incoming.src === url && Number.isFinite(effectiveStartPos)) {
          try {
            incoming.currentTime = effectiveStartPos;
          } catch {
            // Fallback to post-metadata seek if browser wasn't seekable yet
          }
        }
      } else {
        const targetPos = effectiveStartPos;
        incoming.addEventListener(
          "loadedmetadata",
          () => {
            if (this.opGen === gen && incoming.src === url && Number.isFinite(targetPos)) {
              incoming.currentTime = targetPos;
            }
          },
          { once: true },
        );
      }
    }

    try {
      if (canCrossfade && effectivePlan && bridgeSlices) {
        const liveOutgoingPos =
          Number.isFinite(outgoing.currentTime) && outgoing.currentTime > 0
            ? outgoing.currentTime
            : outgoingPosBeforeCancel > 0
              ? outgoingPosBeforeCancel
              : effectivePlan.triggerAtSecs;
        const bridgeRes = await this.dspBridge.startTransition({
          outSlice: bridgeSlices.outSlice,
          inSlice: bridgeSlices.inSlice,
          outgoingPosSecs: liveOutgoingPos,
          plan: effectivePlan,
          masterVolume: this.volume,
          onProgress: (pos, dur) => {
            if (this.opGen !== gen || this.activeIndex !== incomingIndex) return;
            this.callbacks.onTimeUpdate(pos, dur);
            const now = performance.now();
            if (now - this.lastPositionStateSync >= 1000) {
              this.lastPositionStateSync = now;
              this.syncMediaSessionPosition(pos, dur, 1);
            }
          },
          onHandoffReady: (handoffPos) => {
            if (this.opGen !== gen || this.activeIndex !== incomingIndex) return;
            const startIncomingHandoff = () => {
              if (this.opGen !== gen || this.activeIndex !== incomingIndex) return;
              incoming.playbackRate = 1.0;
              if (incoming.readyState >= 1 && Math.abs(incoming.currentTime - handoffPos) > 0.15) {
                try {
                  incoming.currentTime = handoffPos;
                } catch {
                  // ignore
                }
              }
              incoming.volume = Math.min(this.volume, 0.08);
              void incoming
                .play()
                .then(() => {
                  if (this.opGen !== gen || this.activeIndex !== incomingIndex) return;
                  // Begin bridge dry fade-out the exact instant <audio> Deck B starts outputting PCM
                  this.dspBridge.beginHandoffFadeOut();
                  if (this.handoffRampTimer !== null) {
                    window.clearInterval(this.handoffRampTimer);
                  }
                  const rampStart = performance.now();
                  this.handoffRampTimer = window.setInterval(() => {
                    const p = Math.min(1, (performance.now() - rampStart) / 85);
                    if (this.opGen !== gen || this.activeIndex !== incomingIndex) {
                      if (this.handoffRampTimer !== null) {
                        window.clearInterval(this.handoffRampTimer);
                        this.handoffRampTimer = null;
                      }
                      return;
                    }
                    incoming.volume = this.volume * (0.08 + 0.92 * p);
                    if (p >= 1 && this.handoffRampTimer !== null) {
                      window.clearInterval(this.handoffRampTimer);
                      this.handoffRampTimer = null;
                    }
                  }, 16);
                })
                .catch(() => {
                  this.dspBridge.beginHandoffFadeOut();
                });
            };

            if (incoming.readyState >= 2) {
              startIncomingHandoff();
            } else {
              // Wait briefly for canplay while inSource keeps playing seamlessly in the bridge
              const onReady = () => {
                window.clearTimeout(waitTimer);
                incoming.removeEventListener("canplay", onReady);
                startIncomingHandoff();
              };
              const waitTimer = window.setTimeout(onReady, 350);
              incoming.addEventListener("canplay", onReady, { once: true });
            }
          },
        });

        if (this.opGen !== gen) return;
        if (bridgeRes.started) {
          // Pre-position paused <audio> Deck B to the exact analytical handoff timestamp
          // so zero seek happens when onHandoffReady triggers play() 10+ seconds later.
          incoming.playbackRate = 1.0;
          incoming.volume = 0;
          const seekIncomingToHandoff = () => {
            if (this.opGen === gen && incoming.paused && Number.isFinite(bridgeRes.handoffPosSecs)) {
              try {
                incoming.currentTime = bridgeRes.handoffPosSecs;
              } catch {
                // ignore
              }
            }
          };
          if (incoming.readyState >= 1) {
            seekIncomingToHandoff();
          } else {
            incoming.addEventListener("loadedmetadata", seekIncomingToHandoff, { once: true });
          }

          // DSP Bridge now owns outgoing outro + echo + reverb + incoming intro;
          // smoothly dispose outgoing <audio> (30ms micro-fade in disposeAudio)
          this.resetDeck(outgoingIndex);
          this.setMediaSessionPlaybackState("playing");
          this.callbacks.onPlayStateChange(true);
          return;
        }
      }

      if (canCrossfade) {
        const startVolOut = outgoing.volume;
        const outStartRate = outgoing.playbackRate || 1.0;
        const useEqualPower = Boolean(effectivePlan && effectivePlan.mode !== "crossfade") || this.mixMode;
        const useBassSwap =
          Boolean(effectivePlan?.useBassSwap ?? this.mixMode) &&
          this.audioCtx?.state === "running" &&
          this.deckChannels.has(outgoing) &&
          this.deckChannels.has(incoming);

        if (useBassSwap) {
          this.setDeckBassDb(outgoing, 0, true);
          this.setDeckBassDb(incoming, -18, true);
        }

        incoming.volume = 0.01;
        try {
          await incoming.play();
        } catch {
          if (this.opGen !== gen) return;
          // On mobile WebViews (Android), two simultaneous <audio> elements can
          // fail AudioFocus; fall back to stopping the outgoing deck first.
          this.resetDeck(outgoingIndex);
          this.resetDeckMixState(incoming);
          incoming.volume = this.volume;
          await incoming.play();
          return;
        }
        if (this.opGen !== gen) return;

        const durationMs = Math.max(500, effectiveFadeSecs * 1000);
        const startedAt = Date.now();
        const smoothstep = (x: number) => x * x * (3 - 2 * x);
        const step = () => {
          const t = Math.min(1, (Date.now() - startedAt) / durationMs);
          if (useEqualPower) {
            // DJ phrase curve: decisive beat-locked entry -> full-energy groove overlap -> clean outro exit
            let outFactor: number;
            let inFactor: number;
            if (t < 0.3) {
              const p = t / 0.3;
              outFactor = 1.0 - 0.12 * p;
              inFactor = 0.14 + 0.78 * smoothstep(p);
            } else if (t <= 0.68) {
              const p = (t - 0.3) / 0.38;
              outFactor = 0.88 - 0.16 * p;
              inFactor = 0.92 + 0.08 * p;
            } else {
              const p = (t - 0.68) / 0.32;
              outFactor = 0.72 * Math.cos(p * Math.PI * 0.5);
              inFactor = 1.0;
            }
            outgoing.volume = Math.max(0, startVolOut * outFactor);
            incoming.volume = Math.min(1, this.volume * inFactor);
          } else {
            outgoing.volume = Math.max(0, startVolOut * (1 - t));
            incoming.volume = Math.min(1, this.volume * t);
          }

          if (useBassSwap) {
            if (t < 0.45) {
              this.setDeckBassDb(outgoing, 0);
              this.setDeckBassDb(incoming, -18);
            } else if (t <= 0.65) {
              const p = (t - 0.45) / 0.2;
              this.setDeckBassDb(outgoing, -18 * p);
              this.setDeckBassDb(incoming, -18 * (1 - p));
            } else {
              this.setDeckBassDb(outgoing, -18);
              this.setDeckBassDb(incoming, 0);
            }
          }

          // Dual-deck tempo morphing (quantized to >=1.5% steps to avoid GStreamer/Chromium
          // time-stretcher ring-buffer thrashing on 30ms ticks)
          if (outTargetRate !== 1.0) {
            const pOut = Math.min(1, t / 0.35);
            const targetOut = outStartRate + (outTargetRate - outStartRate) * pOut;
            if (Math.abs(outgoing.playbackRate - targetOut) >= 0.015) {
              outgoing.playbackRate = Number(targetOut.toFixed(3));
            }
          }
          if (initialRate !== 1.0 && t > 0.6) {
            const pIn = (t - 0.6) / 0.4;
            const targetIn = initialRate + (1.0 - initialRate) * pIn;
            if (Math.abs(incoming.playbackRate - targetIn) >= 0.015 || t >= 0.98) {
              incoming.playbackRate = t >= 0.98 ? 1.0 : Number(targetIn.toFixed(3));
            }
          }

          if (t >= 1 || this.activeIndex !== incomingIndex || this.opGen !== gen) {
            this.cancelFade();
            if (this.activeIndex === incomingIndex && this.opGen === gen) {
              incoming.volume = this.volume;
              this.resetDeckMixState(incoming);
            }
            if (this.activeIndex !== outgoingIndex) {
              this.resetDeck(outgoingIndex);
            }
          }
        };
        this.fadeStep = step;
        this.fadeTimer = window.setInterval(step, 30);
        step();
      } else {
        incoming.volume = this.volume;
        if (!startPaused) {
          try {
            await incoming.play();
          } catch (firstErr) {
            if (this.opGen !== gen) return;
            // If WebView rejected play() before initial metadata/buffer arrived,
            // wait briefly for canplay on this exact URL and retry once.
            if (incoming.src === url && incoming.readyState < 2) {
              await new Promise<void>((resolve) => {
                const done = () => {
                  window.clearTimeout(timer);
                  incoming.removeEventListener("canplay", done);
                  incoming.removeEventListener("error", done);
                  resolve();
                };
                const timer = window.setTimeout(done, 2500);
                incoming.addEventListener("canplay", done, { once: true });
                incoming.addEventListener("error", done, { once: true });
              });
              if (this.opGen === gen && incoming.src === url && this.activeIndex === incomingIndex) {
                await incoming.play();
              }
            } else {
              throw firstErr;
            }
          }
        }
      }
    } catch (err) {
      if (this.opGen !== gen) return;
      console.error("[GyroTube play() failed]", {
        url,
        err,
        canPlayWebm: incoming.canPlayType("audio/webm"),
        canPlayMp4: incoming.canPlayType("audio/mp4"),
        errorCode: incoming.error?.code,
        errorMessage: incoming.error?.message,
      });
      throw err;
    } finally {
      if (this.opGen === gen) {
        this.loading = false;
      }
    }
  }

  public async play(): Promise<void> {
    if (this.dspBridge.isActive) return;
    const deck = this.activeDeck;
    if (!deck.src) return;
    if (deck.paused) {
      await deck.play();
    }
  }

  public pause(): void {
    if (this.dspBridge.isActive) {
      const resumePos = this.dspBridge.getCurrentIncomingPosSecs();
      this.dspBridge.stop();
      const deck = this.activeDeck;
      if (deck.src) {
        this.resetDeckMixState(deck);
        deck.volume = this.volume;
        if (resumePos !== null && Number.isFinite(resumePos)) {
          try {
            deck.currentTime = resumePos;
          } catch {
            // ignore
          }
        }
        deck.pause();
      }
      this.setMediaSessionPlaybackState("paused");
      this.callbacks.onPlayStateChange(false);
      return;
    }
    const deck = this.activeDeck;
    if (!deck.src) return;
    if (!deck.paused) {
      deck.pause();
    }
  }

  public async togglePlay(): Promise<void> {
    if (this.dspBridge.isActive) {
      this.pause();
      return;
    }
    const deck = this.activeDeck;
    if (!deck.src) return;
    if (deck.paused) {
      await deck.play();
    } else {
      deck.pause();
    }
  }

  public seek(secs: number): void {
    const deck = this.activeDeck;
    if (!deck.src || !Number.isFinite(secs)) return;
    const wasInBridge = this.dspBridge.isActive;
    if (wasInBridge) {
      this.dspBridge.stop();
    }
    const dur = Number.isFinite(deck.duration) ? deck.duration : 0;
    const clamped = dur > 0 ? Math.max(0, Math.min(secs, dur)) : Math.max(0, secs);
    this.clearPrecisionTrigger();
    const triggerAt = this.resolveTriggerAtSecs(dur);
    if (triggerAt !== null && clamped < triggerAt - 0.5) {
      this.crossfadeFired = false;
    }
    this.resetDeckMixState(deck);
    deck.volume = this.volume;
    deck.currentTime = clamped;
    if (wasInBridge && deck.paused) {
      void deck.play().catch(() => {});
    }
    this.lastPositionStateSync = performance.now();
    this.syncMediaSessionPosition(clamped, dur, deck.paused ? 0 : deck.playbackRate || 1);
  }

  public stop(): void {
    ++this.opGen;
    this.loading = false;
    this.preCuedUrl = null;
    this.preCuedIndex = null;
    this.pendingPreCueUrl = null;
    this.cancelFade();
    this.resetDeck(0);
    this.resetDeck(1);
    this.setMediaSessionPlaybackState("none");
    this.callbacks.onPlayStateChange(false);
    this.callbacks.onTimeUpdate(0, 0);
  }

  private setMediaSessionPlaybackState(state: MediaSessionPlaybackState): void {
    if (usesNativeLinuxMpris()) return;
    if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
    try {
      navigator.mediaSession.playbackState = state;
    } catch {
      // Ignore on runtimes without playbackState support
    }
  }

  private syncMediaSessionPosition(position: number, duration: number, playbackRate: number): void {
    if (usesNativeLinuxMpris()) return;
    if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
    if (typeof navigator.mediaSession.setPositionState !== "function") return;
    if (!Number.isFinite(duration) || duration <= 0) return;
    const safePos = Number.isFinite(position) ? Math.max(0, Math.min(position, duration)) : 0;
    const safeRate = Number.isFinite(playbackRate) && playbackRate > 0 ? playbackRate : 1;
    try {
      navigator.mediaSession.setPositionState({
        duration,
        playbackRate: safeRate,
        position: safePos,
      });
    } catch {
      // Ignore invalid position state on stricter webviews
    }
  }

  private safeSetActionHandler(
    action: MediaSessionAction,
    handler: MediaSessionActionHandler | null,
  ): void {
    if (usesNativeLinuxMpris()) return;
    if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
    try {
      navigator.mediaSession.setActionHandler(action, handler);
    } catch {
      // Ignore unsupported actions in WebKitGTK / Safari
    }
  }

  public updateMediaSession(
    track: Track | null,
    handlers: {
      onPlay: () => void;
      onPause: () => void;
      onPrev: () => void;
      onNext: () => void;
      onSeek: (secs: number) => void;
    },
  ): void {
    if (usesNativeLinuxMpris()) return;
    if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
    if (!track) {
      navigator.mediaSession.metadata = null;
      this.setMediaSessionPlaybackState("none");
      return;
    }
    const art = track.cover_large_url || track.cover_url;
    navigator.mediaSession.metadata = new MediaMetadata({
      title: track.title,
      artist: track.artist,
      album: track.album ?? "",
      artwork: art ? [{ src: art, sizes: "512x512", type: "image/jpeg" }] : [],
    });
    this.safeSetActionHandler("play", handlers.onPlay);
    this.safeSetActionHandler("pause", handlers.onPause);
    this.safeSetActionHandler("stop", handlers.onPause);
    this.safeSetActionHandler("previoustrack", handlers.onPrev);
    this.safeSetActionHandler("nexttrack", handlers.onNext);
    this.safeSetActionHandler("seekto", (details) => {
      if (details.seekTime != null && Number.isFinite(details.seekTime)) {
        handlers.onSeek(details.seekTime);
      }
    });
    this.safeSetActionHandler("seekbackward", (details) => {
      const offset = details.seekOffset && Number.isFinite(details.seekOffset) ? details.seekOffset : 5;
      const cur = Number.isFinite(this.activeDeck.currentTime) ? this.activeDeck.currentTime : 0;
      handlers.onSeek(Math.max(0, cur - offset));
    });
    this.safeSetActionHandler("seekforward", (details) => {
      const offset = details.seekOffset && Number.isFinite(details.seekOffset) ? details.seekOffset : 5;
      const cur = Number.isFinite(this.activeDeck.currentTime) ? this.activeDeck.currentTime : 0;
      const dur = Number.isFinite(this.activeDeck.duration) ? this.activeDeck.duration : 0;
      handlers.onSeek(dur > 0 ? Math.min(dur, cur + offset) : cur + offset);
    });
  }
}

/**
 * Extracts a vibrant dominant RGB tuple from an image URL for ambient backgrounds.
 */
export async function extractDominantRgb(url: string): Promise<[number, number, number] | null> {
  return new Promise((resolve) => {
    const img = new Image();
    img.crossOrigin = "anonymous";
    img.referrerPolicy = "no-referrer";
    img.onload = () => {
      try {
        const canvas = document.createElement("canvas");
        canvas.width = 24;
        canvas.height = 24;
        const ctx = canvas.getContext("2d");
        if (!ctx) return resolve(null);
        ctx.drawImage(img, 0, 0, 24, 24);
        const data = ctx.getImageData(0, 0, 24, 24).data;
        let rSum = 0;
        let gSum = 0;
        let bSum = 0;
        let wSum = 0;
        for (let i = 0; i < data.length; i += 4) {
          const r = data[i];
          const g = data[i + 1];
          const b = data[i + 2];
          const max = Math.max(r, g, b);
          const min = Math.min(r, g, b);
          const sat = max === 0 ? 0 : (max - min) / max;
          const lum = (max + min) / 510;
          if (lum < 0.08 || lum > 0.92) continue;
          const w = 0.25 + sat * 2.5;
          rSum += r * w;
          gSum += g * w;
          bSum += b * w;
          wSum += w;
        }
        if (wSum === 0) return resolve(null);
        resolve([Math.round(rSum / wSum), Math.round(gSum / wSum), Math.round(bSum / wSum)]);
      } catch {
        resolve(null);
      }
    };
    img.onerror = () => resolve(null);
    img.src = url;
  });
}
