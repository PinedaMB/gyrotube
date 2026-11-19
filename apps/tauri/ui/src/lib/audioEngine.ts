import type { Track } from "./types";

export interface AudioCallbacks {
  onTimeUpdate: (position: number, duration: number) => void;
  onPlayStateChange: (playing: boolean) => void;
  onEnded: () => void;
  onCrossfadeTrigger: () => void;
  onError: (message: string) => void;
}

/**
 * Dual-deck HTML5 Audio + Web Audio API engine supporting smooth crossfading
 * directly from gyrotube-core's local HTTP Range streamer (`127.0.0.1`).
 */
export class DualDeckAudioEngine {
  private decks: [HTMLAudioElement, HTMLAudioElement];
  private activeIndex: 0 | 1 = 0;
  private ctx: AudioContext | null = null;
  private gains: [GainNode, GainNode] | null = null;
  private volume = 1.0;
  private crossfadeSecs = 0;
  private crossfadeFired = false;
  private callbacks: AudioCallbacks;

  constructor(callbacks: AudioCallbacks) {
    this.callbacks = callbacks;
    this.decks = [this.createDeck(0), this.createDeck(1)];
  }

  private createDeck(index: 0 | 1): HTMLAudioElement {
    const audio = new Audio();
    audio.preload = "auto";
    audio.crossOrigin = "anonymous";

    audio.addEventListener("timeupdate", () => {
      if (index !== this.activeIndex) return;
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
      if (index === this.activeIndex) this.callbacks.onPlayStateChange(true);
    });
    audio.addEventListener("pause", () => {
      if (index === this.activeIndex) this.callbacks.onPlayStateChange(false);
    });
    audio.addEventListener("ended", () => {
      if (index === this.activeIndex) this.callbacks.onEnded();
    });
    audio.addEventListener("error", () => {
      if (index === this.activeIndex && audio.src) {
        this.callbacks.onError("Audio playback error");
      }
    });
    return audio;
  }

  private ensureWebAudio(): void {
    if (this.ctx) {
      if (this.ctx.state === "suspended") void this.ctx.resume();
      return;
    }
    try {
      const ctx = new AudioContext();
      const g0 = ctx.createGain();
      const g1 = ctx.createGain();
      g0.gain.value = this.activeIndex === 0 ? this.volume : 0;
      g1.gain.value = this.activeIndex === 1 ? this.volume : 0;

      const s0 = ctx.createMediaElementSource(this.decks[0]);
      const s1 = ctx.createMediaElementSource(this.decks[1]);
      s0.connect(g0).connect(ctx.destination);
      s1.connect(g1).connect(ctx.destination);

      this.ctx = ctx;
      this.gains = [g0, g1];
    } catch {
      // Fallback gracefully to direct HTMLAudioElement.volume if Web Audio is unavailable.
      this.decks[0].volume = this.volume;
      this.decks[1].volume = this.volume;
    }
  }

  public get activeDeck(): HTMLAudioElement {
    return this.decks[this.activeIndex];
  }

  public setVolume(v: number): void {
    this.volume = Math.max(0, Math.min(1, v));
    if (this.gains && this.ctx) {
      this.gains[this.activeIndex].gain.setValueAtTime(this.volume, this.ctx.currentTime);
    } else {
      this.activeDeck.volume = this.volume;
    }
  }

  public setCrossfadeSecs(secs: number): void {
    this.crossfadeSecs = Math.max(0, Math.min(10, Math.round(secs)));
  }

  public async loadAndPlay(url: string, startPositionSecs = 0, useCrossfade = false): Promise<void> {
    this.ensureWebAudio();
    this.crossfadeFired = false;

    const outgoingIndex = this.activeIndex;
    const incomingIndex: 0 | 1 = useCrossfade ? (outgoingIndex === 0 ? 1 : 0) : outgoingIndex;
    const outgoing = this.decks[outgoingIndex];
    const incoming = this.decks[incomingIndex];

    this.activeIndex = incomingIndex;
    incoming.src = url;
    incoming.currentTime = Math.max(0, startPositionSecs);

    if (useCrossfade && this.gains && this.ctx && this.crossfadeSecs > 0) {
      const now = this.ctx.currentTime;
      const dur = this.crossfadeSecs;
      const gOut = this.gains[outgoingIndex].gain;
      const gIn = this.gains[incomingIndex].gain;

      gOut.cancelScheduledValues(now);
      gOut.setValueAtTime(gOut.value, now);
      gOut.linearRampToValueAtTime(0, now + dur);

      gIn.cancelScheduledValues(now);
      gIn.setValueAtTime(0.001, now);
      gIn.linearRampToValueAtTime(this.volume, now + dur);

      await incoming.play();
      window.setTimeout(() => {
        if (this.activeIndex !== outgoingIndex) {
          outgoing.pause();
          outgoing.removeAttribute("src");
          outgoing.load();
        }
      }, dur * 1000 + 120);
    } else {
      if (outgoingIndex !== incomingIndex) {
        outgoing.pause();
        outgoing.removeAttribute("src");
        outgoing.load();
      }
      if (this.gains && this.ctx) {
        const now = this.ctx.currentTime;
        this.gains[incomingIndex].gain.cancelScheduledValues(now);
        this.gains[incomingIndex].gain.setValueAtTime(this.volume, now);
      } else {
        incoming.volume = this.volume;
      }
      await incoming.play();
    }
  }

  public async togglePlay(): Promise<void> {
    this.ensureWebAudio();
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
    for (const d of this.decks) {
      d.pause();
      d.removeAttribute("src");
      d.load();
    }
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
