import { listen } from "@tauri-apps/api/event";
import { api } from "./api";
import type { AudioCallbacks } from "./audioEngine";
import type { EqSettings } from "./equalizer";
import type { Track } from "./types";

type NativeAudioEventPayload =
  | { kind: "tick"; position_secs: number; duration_secs: number }
  | { kind: "play_state"; playing: boolean }
  | { kind: "ended" }
  | { kind: "crossfade_trigger" }
  | { kind: "error"; message: string };

export interface AudioDeckFacade {
  readonly src: string;
  readonly paused: boolean;
  readonly currentTime: number;
  readonly readyState: number;
  readonly seeking: boolean;
  play(): Promise<void>;
  pause(): void;
}

export interface AudioEngineBackend {
  readonly activeDeck: AudioDeckFacade;
  applyEqSettings(settings: EqSettings): void;
  setVolume(volume: number): void;
  setCrossfadeSecs(secs: number): void;
  loadAndPlay(
    url: string,
    startPositionSecs?: number,
    useCrossfade?: boolean,
    startPaused?: boolean,
  ): Promise<void>;
  play(): Promise<void>;
  pause(): void;
  togglePlay(): Promise<void>;
  stop(): void;
  seek(secs: number): void;
  updateMediaSession(
    track: Track,
    handlers: {
      onPlay: () => void;
      onPause: () => void;
      onPrev: () => void;
      onNext: () => void;
      onSeek: (secs: number) => void;
    },
  ): void;
}

/**
 * Pure-Rust desktop audio engine bridge (`symphonia` + `cpal` in `gyrotube-core`).
 * Interpolates `currentTime` between 100ms native ticks so 60 FPS karaoke lyrics
 * and `SyncedVideoPlayer` stay sample-accurate without flooding Tauri IPC.
 */
export class NativeRustAudioEngine implements AudioEngineBackend {
  private callbacks: AudioCallbacks;
  private currentSrc = "";
  private playing = false;
  private lastPosSecs = 0;
  private lastDurSecs = 0;
  private lastPosAt = 0;
  private readonly deckFacade: AudioDeckFacade;

  constructor(callbacks: AudioCallbacks) {
    this.callbacks = callbacks;
    const self = this;
    this.deckFacade = {
      get src() {
        return self.currentSrc;
      },
      get paused() {
        return !self.playing;
      },
      get currentTime() {
        return self.interpolatedPositionSecs();
      },
      get readyState() {
        return self.currentSrc ? 4 : 0;
      },
      get seeking() {
        return false;
      },
      play() {
        return self.play();
      },
      pause() {
        self.pause();
      },
    };

    void listen<NativeAudioEventPayload>("native-audio:event", (event) => {
      const ev = event.payload;
      switch (ev.kind) {
        case "tick":
          this.lastPosSecs = ev.position_secs;
          if (ev.duration_secs > 0) this.lastDurSecs = ev.duration_secs;
          this.lastPosAt = performance.now();
          this.callbacks.onTimeUpdate(ev.position_secs, ev.duration_secs);
          break;
        case "play_state":
          this.lastPosSecs = this.interpolatedPositionSecs();
          this.lastPosAt = performance.now();
          this.playing = ev.playing;
          this.callbacks.onPlayStateChange(ev.playing);
          break;
        case "ended":
          this.playing = false;
          this.callbacks.onEnded();
          break;
        case "crossfade_trigger":
          this.callbacks.onCrossfadeTrigger();
          break;
        case "error":
          this.callbacks.onError(ev.message);
          break;
      }
    }).catch(() => {});
  }

  public get activeDeck(): AudioDeckFacade {
    return this.deckFacade;
  }

  private interpolatedPositionSecs(): number {
    if (!this.playing || this.lastPosAt === 0) {
      return this.lastPosSecs;
    }
    const elapsed = (performance.now() - this.lastPosAt) / 1000;
    const est = this.lastPosSecs + Math.max(0, elapsed);
    return this.lastDurSecs > 0 ? Math.min(this.lastDurSecs, est) : est;
  }

  public applyEqSettings(settings: EqSettings): void {
    void api
      .nativeAudioSetEq(settings.enabled, settings.normalize, [...settings.bands])
      .catch(() => {});
  }

  public setVolume(volume: number): void {
    const clamped = Math.max(0, Math.min(1, volume));
    void api.nativeAudioSetVolume(clamped).catch(() => {});
  }

  public setCrossfadeSecs(secs: number): void {
    const clamped = Math.max(0, Math.min(10, secs));
    void api.nativeAudioSetCrossfade(clamped).catch(() => {});
  }

  public async loadAndPlay(
    url: string,
    startPositionSecs = 0,
    useCrossfade = false,
    startPaused = false,
  ): Promise<void> {
    this.currentSrc = url;
    this.lastPosSecs = Math.max(0, startPositionSecs);
    this.lastDurSecs = 0;
    this.lastPosAt = performance.now();
    this.playing = !startPaused;
    await api.nativeAudioLoad(url, startPositionSecs, useCrossfade, startPaused);
    this.callbacks.onPlayStateChange(!startPaused);
  }

  public async play(): Promise<void> {
    if (!this.currentSrc) return;
    this.playing = true;
    this.lastPosAt = performance.now();
    await api.nativeAudioPlay();
    this.callbacks.onPlayStateChange(true);
  }

  public pause(): void {
    this.lastPosSecs = this.interpolatedPositionSecs();
    this.lastPosAt = performance.now();
    this.playing = false;
    void api.nativeAudioPause().catch(() => {});
    this.callbacks.onPlayStateChange(false);
  }

  public async togglePlay(): Promise<void> {
    if (this.playing) {
      this.pause();
    } else {
      await this.play();
    }
  }

  public stop(): void {
    this.currentSrc = "";
    this.playing = false;
    this.lastPosSecs = 0;
    this.lastDurSecs = 0;
    this.lastPosAt = 0;
    void api.nativeAudioStop().catch(() => {});
  }

  public seek(secs: number): void {
    const target = Math.max(0, secs);
    this.lastPosSecs = target;
    this.lastPosAt = performance.now();
    void api.nativeAudioSeek(target).catch(() => {});
  }

  public updateMediaSession(
    _track: Track,
    _handlers: {
      onPlay: () => void;
      onPause: () => void;
      onPrev: () => void;
      onNext: () => void;
      onSeek: (secs: number) => void;
    },
  ): void {
    // Desktop Linux uses native MPRIS in src-tauri; Windows/macOS use native OS controls.
  }
}
