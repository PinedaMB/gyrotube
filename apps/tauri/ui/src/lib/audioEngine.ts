import type { Track } from "./types";

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
  private crossfadeFired = false;
  private fadeTimer: number | null = null;
  private fadeStep: (() => void) | null = null;
  private loading = false;
  private opGen = 0;
  private disposingTimers = new Map<HTMLAudioElement, number>();
  private callbacks: AudioCallbacks;

  constructor(callbacks: AudioCallbacks) {
    this.callbacks = callbacks;
    this.decks = [this.createDeck(0), this.createDeck(1)];
  }

  private createDeck(index: 0 | 1): HTMLAudioElement {
    const audio = document.createElement("audio");
    audio.preload = "auto";

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

      if (
        this.crossfadeSecs > 0 &&
        !this.crossfadeFired &&
        dur > this.crossfadeSecs + 2 &&
        dur - pos <= this.crossfadeSecs
      ) {
        this.crossfadeFired = true;
        this.callbacks.onCrossfadeTrigger();
      }
    });

    audio.addEventListener("play", () => {
      if (this.decks[index] === audio && index === this.activeIndex) {
        this.callbacks.onPlayStateChange(true);
      }
    });
    audio.addEventListener("pause", () => {
      if (this.decks[index] === audio && index === this.activeIndex) {
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

  private cancelFade(): void {
    if (this.fadeTimer !== null) {
      window.clearInterval(this.fadeTimer);
      this.fadeTimer = null;
    }
    this.fadeStep = null;
  }

  public get activeDeck(): HTMLAudioElement {
    return this.decks[this.activeIndex];
  }

  public setVolume(v: number): void {
    this.volume = Math.max(0, Math.min(1, v));
    this.activeDeck.volume = this.volume;
  }

  public setCrossfadeSecs(secs: number): void {
    this.crossfadeSecs = Math.max(0, Math.min(10, Math.round(secs)));
  }

  public async loadAndPlay(
    url: string,
    startPositionSecs = 0,
    useCrossfade = false,
    startPaused = false,
  ): Promise<void> {
    this.cancelFade();
    this.crossfadeFired = false;
    this.loading = true;
    const gen = ++this.opGen;

    const outgoingIndex = this.activeIndex;
    const outgoing = this.decks[outgoingIndex];
    const wantDualDeck = useCrossfade && !startPaused && this.crossfadeSecs > 0;
    const canCrossfade =
      wantDualDeck &&
      !outgoing.paused &&
      !outgoing.ended &&
      Boolean(outgoing.src);

    // Always allocate a brand-new HTMLAudioElement for the incoming track so the
    // WebView / GStreamer audio pipeline never flushes residual PCM frames from
    // the previous track when starting playback on a cold load.
    const incomingIndex: 0 | 1 = canCrossfade ? (outgoingIndex === 0 ? 1 : 0) : outgoingIndex;
    if (!canCrossfade) {
      // Ensure any previous fading-out deck is fully torn down before starting
      // the new track (avoids Android AudioFocus conflict on fast warm switches)
      this.flushDisposing();
      this.resetDeck(0, true);
      this.resetDeck(1, true);
    } else {
      this.resetDeck(incomingIndex, true);
    }
    const incoming = this.decks[incomingIndex];

    this.activeIndex = incomingIndex;
    incoming.src = url;
    incoming.load();

    if (startPositionSecs > 0) {
      const targetPos = startPositionSecs;
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

    try {
      if (canCrossfade) {
        const startVolOut = outgoing.volume;
        incoming.volume = 0.01;
        try {
          await incoming.play();
        } catch {
          if (this.opGen !== gen) return;
          // On mobile WebViews (Android), two simultaneous <audio> elements can
          // fail AudioFocus; fall back to stopping the outgoing deck first.
          this.resetDeck(outgoingIndex);
          incoming.volume = this.volume;
          await incoming.play();
          return;
        }
        if (this.opGen !== gen) return;

        const durationMs = this.crossfadeSecs * 1000;
        const startedAt = Date.now();
        const step = () => {
          const t = Math.min(1, (Date.now() - startedAt) / durationMs);
          outgoing.volume = Math.max(0, startVolOut * (1 - t));
          incoming.volume = Math.min(1, this.volume * t);
          if (t >= 1 || this.activeIndex !== incomingIndex || this.opGen !== gen) {
            this.cancelFade();
            if (this.activeIndex === incomingIndex && this.opGen === gen) {
              incoming.volume = this.volume;
            }
            if (this.activeIndex !== outgoingIndex) {
              this.resetDeck(outgoingIndex);
            }
          }
        };
        this.fadeStep = step;
        this.fadeTimer = window.setInterval(step, 50);
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

  public async togglePlay(): Promise<void> {
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
    deck.currentTime = Math.max(0, secs);
  }

  public stop(): void {
    ++this.opGen;
    this.loading = false;
    this.cancelFade();
    this.resetDeck(0);
    this.resetDeck(1);
    this.callbacks.onPlayStateChange(false);
    this.callbacks.onTimeUpdate(0, 0);
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
    if (!("mediaSession" in navigator)) return;
    if (!track) {
      navigator.mediaSession.metadata = null;
      return;
    }
    const art = track.cover_large_url || track.cover_url;
    navigator.mediaSession.metadata = new MediaMetadata({
      title: track.title,
      artist: track.artist,
      album: track.album ?? "",
      artwork: art ? [{ src: art, sizes: "512x512", type: "image/jpeg" }] : [],
    });
    navigator.mediaSession.setActionHandler("play", handlers.onPlay);
    navigator.mediaSession.setActionHandler("pause", handlers.onPause);
    navigator.mediaSession.setActionHandler("previoustrack", handlers.onPrev);
    navigator.mediaSession.setActionHandler("nexttrack", handlers.onNext);
    navigator.mediaSession.setActionHandler("seekto", (details) => {
      if (details.seekTime != null) handlers.onSeek(details.seekTime);
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
