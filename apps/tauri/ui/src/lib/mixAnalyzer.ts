import { api, isTauriRuntime } from "./api";
import { analyzeAudioBuffer, MIX_ANALYSIS_VERSION } from "./mixDsp";
import type { MixWorkerRequest, MixWorkerResponse } from "./mixDsp.worker";
import type { TrackMixProfile } from "./types";

export {
  analyzeAudioBuffer,
  detectBpmAndBeatGrid,
  detectIntroOutro,
  extractEnvelopes,
  MIX_ANALYSIS_VERSION,
  MIX_FRAME_RATE_HZ,
} from "./mixDsp";

const SLICE_WINDOW_SECS = 38;
const MAX_CACHED_SLICE_TRACKS = 4;

export interface AudioBufferSlice {
  buffer: AudioBuffer;
  offsetSecs: number;
  trackDurationSecs: number;
}

export interface TrackAudioSlices {
  intro: AudioBufferSlice;
  outro: AudioBufferSlice;
}

interface QueuedJob {
  trackId: string;
  streamUrl: string;
  priority: number;
  needSlices: boolean;
  resolve: (profile: TrackMixProfile | null) => void;
}

/**
 * Copies a time window [startSecs, endSecs] from a decoded AudioBuffer into a compact
 * standalone AudioBuffer so the full 4-minute buffer (~90 MB) can be garbage-collected immediately.
 */
function sliceAudioBuffer(
  ctx: BaseAudioContext,
  src: AudioBuffer,
  startSecs: number,
  endSecs: number,
): AudioBufferSlice {
  const sr = src.sampleRate;
  const clampedStart = Math.max(0, Math.min(src.duration, startSecs));
  const clampedEnd = Math.max(clampedStart + 0.1, Math.min(src.duration, endSecs));
  const startFrame = Math.floor(clampedStart * sr);
  const endFrame = Math.min(src.length, Math.ceil(clampedEnd * sr));
  const frameCount = Math.max(1, endFrame - startFrame);
  const channels = Math.min(2, Math.max(1, src.numberOfChannels));

  const out = ctx.createBuffer(channels, frameCount, sr);
  for (let c = 0; c < channels; c++) {
    const srcData = src.getChannelData(c);
    const dstData = out.getChannelData(c);
    dstData.set(srcData.subarray(startFrame, startFrame + frameCount));
  }
  return {
    buffer: out,
    offsetSecs: startFrame / sr,
    trackDurationSecs: src.duration,
  };
}

/**
 * Downmixes a decoded AudioBuffer into a transferable mono Float32Array (~22 kHz)
 * so the Web Worker processes 2x-3x fewer samples with zero main-thread blocking.
 */
function extractDownsampledMono(buffer: AudioBuffer, targetRate = 22050): {
  monoPcm: Float32Array;
  sampleRate: number;
} {
  const srcRate = buffer.sampleRate;
  const step = Math.max(1, Math.round(srcRate / targetRate));
  const outRate = srcRate / step;
  const srcLen = buffer.length;
  const outLen = Math.floor(srcLen / step);
  const mono = new Float32Array(outLen);
  const channels = buffer.numberOfChannels;

  if (channels === 1) {
    const ch0 = buffer.getChannelData(0);
    for (let i = 0, src = 0; i < outLen; i++, src += step) {
      mono[i] = ch0[src];
    }
  } else {
    const ch0 = buffer.getChannelData(0);
    const ch1 = buffer.getChannelData(1);
    for (let i = 0, src = 0; i < outLen; i++, src += step) {
      mono[i] = (ch0[src] + ch1[src]) * 0.5;
    }
  }
  return { monoPcm: mono, sampleRate: outRate };
}

/**
 * Sequential, memory-safe background analyzer & transition slice cache.
 * Decodes at most 1 track at a time, retains compact 38-second intro/outro AudioBuffer slices
 * for the Web Audio DSP Transition Bridge (Reverb, Beat Delay, Filter Sweep, Tempo Ramp & Limiter),
 * offloads DSP math to a Web Worker, and caches profiles in gyrotube-core.
 */
export class MixAnalysisQueue {
  private memCache = new Map<string, TrackMixProfile>();
  private sliceCache = new Map<string, TrackAudioSlices>();
  private inFlight = new Map<string, Promise<TrackMixProfile | null>>();
  private pending: QueuedJob[] = [];
  private workerRunning = false;
  private decodeCtx: BaseAudioContext | null = null;
  private activeAbort: AbortController | null = null;
  private dspWorker: Worker | null = null;
  private workerBroken = false;
  private workerSeq = 0;
  private workerCallbacks = new Map<number, (profile: TrackMixProfile | null) => void>();

  public getCached(trackId: string): TrackMixProfile | null {
    return this.memCache.get(trackId) ?? null;
  }

  public getSlices(trackId: string): TrackAudioSlices | null {
    const found = this.sliceCache.get(trackId);
    if (!found) return null;
    // Refresh LRU order
    this.sliceCache.delete(trackId);
    this.sliceCache.set(trackId, found);
    return found;
  }

  private storeSlices(trackId: string, ctx: BaseAudioContext, fullBuffer: AudioBuffer): void {
    const dur = fullBuffer.duration;
    const intro = sliceAudioBuffer(ctx, fullBuffer, 0, Math.min(dur, SLICE_WINDOW_SECS));
    const outro = sliceAudioBuffer(
      ctx,
      fullBuffer,
      Math.max(0, dur - SLICE_WINDOW_SECS),
      dur,
    );
    if (this.sliceCache.has(trackId)) {
      this.sliceCache.delete(trackId);
    }
    this.sliceCache.set(trackId, { intro, outro });
    while (this.sliceCache.size > MAX_CACHED_SLICE_TRACKS) {
      const oldestKey = this.sliceCache.keys().next().value;
      if (!oldestKey) break;
      this.sliceCache.delete(oldestKey);
    }
  }

  public analyze(
    trackId: string,
    streamUrl: string,
    priority = 0,
    needSlices = true,
  ): Promise<TrackMixProfile | null> {
    const mem = this.memCache.get(trackId);
    const hasSlices = !needSlices || this.sliceCache.has(trackId);
    if (mem && mem.version === MIX_ANALYSIS_VERSION && hasSlices) {
      return Promise.resolve(mem);
    }
    const existing = this.inFlight.get(trackId);
    if (existing) {
      return existing;
    }

    const promise = new Promise<TrackMixProfile | null>((resolve) => {
      this.pending.push({ trackId, streamUrl, priority, needSlices, resolve });
      this.pending.sort((a, b) => b.priority - a.priority);
      void this.drain();
    });
    this.inFlight.set(trackId, promise);
    void promise.finally(() => {
      if (this.inFlight.get(trackId) === promise) {
        this.inFlight.delete(trackId);
      }
    });
    return promise;
  }

  public cancelPending(): void {
    if (this.activeAbort) {
      this.activeAbort.abort();
      this.activeAbort = null;
    }
    const dropped = this.pending.splice(0, this.pending.length);
    for (const job of dropped) {
      job.resolve(null);
    }
  }

  private getDecodeContext(): BaseAudioContext | null {
    if (typeof window === "undefined") return null;
    if (!this.decodeCtx) {
      // Prefer OfflineAudioContext so background decoding never opens an OS audio output
      // stream (PulseAudio/PipeWire/Android AudioFocus).
      const OfflineCtx =
        window.OfflineAudioContext ||
        (window as unknown as { webkitOfflineAudioContext?: typeof OfflineAudioContext })
          .webkitOfflineAudioContext;
      if (OfflineCtx) {
        try {
          this.decodeCtx = new OfflineCtx(1, 1, 44100);
          return this.decodeCtx;
        } catch {
          // Fall back to standard AudioContext below
        }
      }
      const Ctx =
        window.AudioContext ||
        (window as unknown as { webkitAudioContext?: typeof AudioContext }).webkitAudioContext;
      if (!Ctx) return null;
      this.decodeCtx = new Ctx();
    }
    return this.decodeCtx;
  }

  private getWorker(): Worker | null {
    if (this.workerBroken || typeof Worker === "undefined") return null;
    if (!this.dspWorker) {
      try {
        this.dspWorker = new Worker(new URL("./mixDsp.worker.ts", import.meta.url), {
          type: "module",
        });
        this.dspWorker.onmessage = (e: MessageEvent<MixWorkerResponse>) => {
          const cb = this.workerCallbacks.get(e.data.id);
          if (cb) {
            this.workerCallbacks.delete(e.data.id);
            cb(e.data.profile);
          }
        };
        this.dspWorker.onerror = () => {
          this.workerBroken = true;
          for (const cb of this.workerCallbacks.values()) cb(null);
          this.workerCallbacks.clear();
        };
      } catch {
        this.workerBroken = true;
        return null;
      }
    }
    return this.dspWorker;
  }

  private runDspInWorker(buffer: AudioBuffer): Promise<TrackMixProfile | null> {
    const worker = this.getWorker();
    if (!worker) {
      return Promise.resolve(analyzeAudioBuffer(buffer));
    }
    const { monoPcm, sampleRate } = extractDownsampledMono(buffer, 22050);
    const id = ++this.workerSeq;
    return new Promise<TrackMixProfile | null>((resolve) => {
      this.workerCallbacks.set(id, resolve);
      const req: MixWorkerRequest = {
        id,
        monoPcm,
        sampleRate,
        durationSecs: buffer.duration,
      };
      worker.postMessage(req, [monoPcm.buffer]);
    });
  }

  private async drain(): Promise<void> {
    if (this.workerRunning) return;
    this.workerRunning = true;
    try {
      while (this.pending.length > 0) {
        const job = this.pending.shift()!;
        const result = await this.processJob(job);
        job.resolve(result);
      }
    } finally {
      this.workerRunning = false;
    }
  }

  private async processJob(job: QueuedJob): Promise<TrackMixProfile | null> {
    let cachedProfile: TrackMixProfile | null = this.memCache.get(job.trackId) ?? null;

    // 1. Check persistent Rust cache first
    if (!cachedProfile && isTauriRuntime()) {
      try {
        const persisted = await api.getMixProfile(job.trackId);
        if (persisted && persisted.version === MIX_ANALYSIS_VERSION) {
          cachedProfile = persisted;
          this.memCache.set(job.trackId, persisted);
        }
      } catch {
        // Continue to decode on cache miss/error
      }
    }

    if (cachedProfile && (!job.needSlices || this.sliceCache.has(job.trackId))) {
      return cachedProfile;
    }

    // 2. Yield briefly (1.5 s) before fetching so active playback never contends
    // with the analyzer for initial loopback Range chunks.
    const abort = new AbortController();
    this.activeAbort = abort;
    try {
      await new Promise<void>((resolve) => {
        const timer = window.setTimeout(resolve, 1500);
        abort.signal.addEventListener(
          "abort",
          () => {
            window.clearTimeout(timer);
            resolve();
          },
          { once: true },
        );
      });
      if (abort.signal.aborted) return cachedProfile;

      const ctx = this.getDecodeContext();
      if (!ctx) return cachedProfile;

      const resp = await fetch(job.streamUrl, { signal: abort.signal });
      if (!resp.ok) return cachedProfile;
      const arrayBuf = await resp.arrayBuffer();
      if (abort.signal.aborted || arrayBuf.byteLength === 0) return cachedProfile;

      const audioBuffer = await ctx.decodeAudioData(arrayBuf.slice(0));
      if (abort.signal.aborted) return cachedProfile;

      // Extract compact 38s intro & outro slices for the Web Audio DSP Transition Bridge
      this.storeSlices(job.trackId, ctx, audioBuffer);

      if (cachedProfile) {
        return cachedProfile;
      }

      const profile = await this.runDspInWorker(audioBuffer);
      if (!profile || abort.signal.aborted) return null;

      this.memCache.set(job.trackId, profile);
      if (isTauriRuntime()) {
        void api.saveMixProfile(job.trackId, profile).catch(() => {});
      }
      return profile;
    } catch {
      return cachedProfile;
    } finally {
      if (this.activeAbort === abort) {
        this.activeAbort = null;
      }
    }
  }
}
