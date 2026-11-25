import type { AudioBufferSlice } from "./mixAnalyzer";
import type { TransitionPlan } from "./mixPlanner";

export interface BridgeTransitionOptions {
  outSlice: AudioBufferSlice;
  inSlice: AudioBufferSlice;
  outgoingPosSecs: number;
  plan: TransitionPlan;
  masterVolume: number;
  onProgress: (incomingPosSecs: number, incomingDurationSecs: number, progress01: number) => void;
  onHandoffReady: (handoffIncomingPosSecs: number) => void;
}

export interface BridgeStartResult {
  started: boolean;
  actualMixDur: number;
  handoffPosSecs: number;
}

interface ActiveBridgeSession {
  id: number;
  startedAtPerf: number;
  durationSecs: number;
  inStartSecs: number;
  inDurationSecs: number;
  inStartRate: number;
  masterGain: GainNode;
  inDryGain: GainNode;
  outSource: AudioBufferSourceNode;
  inSource: AudioBufferSourceNode;
  nodesToDisconnect: AudioNode[];
  progressTimer: number | null;
  handoffTimer: number | null;
  cleanupTimer: number | null;
  handoffFired: boolean;
  fadeOutStarted: boolean;
  onProgress: (incomingPosSecs: number, incomingDurationSecs: number, progress01: number) => void;
  onHandoffReady: (handoffIncomingPosSecs: number) => void;
}

function clamp(v: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, v));
}

/**
 * Computes the exact analytical track position reached by an AudioBufferSourceNode
 * whose playbackRate holds at `inStartRate` for [0, 0.55 * durationSecs] and then
 * ramps linearly from `inStartRate` to `1.0` over [0.55 * durationSecs, durationSecs].
 */
export function computeBridgeIncomingPosition(
  inStartSecs: number,
  inStartRate: number,
  durationSecs: number,
  elapsedSecs: number,
): number {
  const clampedElapsed = clamp(elapsedSecs, 0, durationSecs);
  const holdEnd = durationSecs * 0.55;
  if (clampedElapsed <= holdEnd) {
    return inStartSecs + clampedElapsed * inStartRate;
  }
  const dt = clampedElapsed - holdEnd;
  const rampSpan = Math.max(0.001, durationSecs - holdEnd);
  const u = clamp(dt / rampSpan, 0, 1);
  const curRate = inStartRate + (1.0 - inStartRate) * u;
  return inStartSecs + holdEnd * inStartRate + dt * (inStartRate + curRate) * 0.5;
}

/**
 * Generates a warm 2.1s stereo Hall/Plate Impulse Response with 16ms pre-delay
 * and high-frequency damping for lush DJ wash-out reverb transitions.
 */
function buildStereoReverbImpulse(ctx: BaseAudioContext): AudioBuffer {
  const sr = ctx.sampleRate;
  const duration = 2.1;
  const length = Math.floor(sr * duration);
  const preDelayFrames = Math.floor(sr * 0.016);
  const impulse = ctx.createBuffer(2, length, sr);

  // Deterministic PRNG so reverb tail is consistent and free of clicks
  let seed = 0x6d2b79f5;
  const nextRand = () => {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296 * 2 - 1;
  };

  const alphaLp = 1 - Math.exp((-2 * Math.PI * 3200) / sr);
  for (let c = 0; c < 2; c++) {
    const data = impulse.getChannelData(c);
    let lp = 0;
    for (let i = preDelayFrames; i < length; i++) {
      const normT = (i - preDelayFrames) / (length - preDelayFrames);
      const decay = Math.pow(1 - normT, 2.6) * Math.exp(-normT * 2.2);
      const white = nextRand();
      lp += alphaLp * (white - lp);
      data[i] = lp * decay * 0.45;
    }
  }
  return impulse;
}

/**
 * Soft-clipping analog-style tanh limiter curve (-1..+1) to prevent digital clipping
 * when both tracks + echo + reverb sum together at peak energy.
 */
function buildSoftClipCurve(): Float32Array<ArrayBuffer> {
  const n = 2048;
  const buf = new ArrayBuffer(n * 4);
  const curve = new Float32Array(buf);
  const drive = 1.18;
  const norm = 1 / Math.tanh(drive);
  for (let i = 0; i < n; i++) {
    const x = (i * 2) / (n - 1) - 1;
    curve[i] = Math.tanh(x * drive) * norm * 0.98;
  }
  return curve;
}

/**
 * Sample-accurate Web Audio DSP Transition Bridge.
 * Fuses the outgoing track's outro slice and the incoming track's intro slice using:
 * - Continuous sample-level tempo morphing (AudioBufferSourceNode.playbackRate)
 * - DJ High-Pass / Low-Pass filter sweeps & sub-bass swap
 * - Beat-synced Dub Echo (DelayNode locked to 1/2 beat of the bridge BPM)
 * - Spatial Wash-Out Reverb (ConvolverNode)
 * - Master Bus Compressor & Brickwall Soft-Clipper Delimiter
 */
export class MixTransitionBridge {
  private ctx: AudioContext | null = null;
  private reverbImpulse: AudioBuffer | null = null;
  private softClipCurve: Float32Array<ArrayBuffer> | null = null;
  private active: ActiveBridgeSession | null = null;
  private seq = 0;

  private ensureContext(): AudioContext | null {
    if (typeof window === "undefined") return null;
    if (!this.ctx) {
      const Ctx =
        window.AudioContext ||
        (window as unknown as { webkitAudioContext?: typeof AudioContext }).webkitAudioContext;
      if (!Ctx) return null;
      this.ctx = new Ctx();
    }
    return this.ctx;
  }

  public get isActive(): boolean {
    return this.active !== null && !this.active.handoffFired;
  }

  public setVolume(volume: number): void {
    if (!this.active || !this.ctx) return;
    const clamped = clamp(volume, 0, 1);
    try {
      this.active.masterGain.gain.setTargetAtTime(clamped, this.ctx.currentTime, 0.02);
    } catch {
      this.active.masterGain.gain.value = clamped;
    }
  }

  public getCurrentIncomingPosSecs(): number | null {
    if (!this.active) return null;
    const elapsed = (performance.now() - this.active.startedAtPerf) / 1000;
    return computeBridgeIncomingPosition(
      this.active.inStartSecs,
      this.active.inStartRate,
      this.active.durationSecs,
      elapsed,
    );
  }

  public stop(): void {
    const session = this.active;
    if (!session) return;
    this.active = null;
    if (session.progressTimer !== null) window.clearInterval(session.progressTimer);
    if (session.handoffTimer !== null) window.clearTimeout(session.handoffTimer);
    if (session.cleanupTimer !== null) window.clearTimeout(session.cleanupTimer);
    try {
      session.outSource.stop();
    } catch {
      // ignore
    }
    try {
      session.inSource.stop();
    } catch {
      // ignore
    }
    // Explicitly disconnect all nodes (including the delay/feedback cycle) to prevent GC retention
    for (const node of session.nodesToDisconnect) {
      try {
        node.disconnect();
      } catch {
        // ignore
      }
    }
  }

  /**
   * Called by DualDeckAudioEngine the instant <audio> Deck B's play() resolves so the bridge's
   * dry incoming signal crossfades out cleanly with zero gap while Echo & Reverb tails ring out.
   */
  public beginHandoffFadeOut(): void {
    const session = this.active;
    if (!session || !this.ctx || session.fadeOutStarted) return;
    session.fadeOutStarted = true;
    const sessionId = session.id;
    const handoffNow = this.ctx.currentTime;
    try {
      session.inDryGain.gain.cancelScheduledValues(handoffNow);
      session.inDryGain.gain.setValueAtTime(session.inDryGain.gain.value, handoffNow);
      session.inDryGain.gain.linearRampToValueAtTime(0.0, handoffNow + 0.085);
      session.outSource.stop(handoffNow + 0.05);
      session.inSource.stop(handoffNow + 0.095);
    } catch {
      // ignore
    }
    if (session.cleanupTimer !== null) {
      window.clearTimeout(session.cleanupTimer);
    }
    session.cleanupTimer = window.setTimeout(() => {
      if (this.active?.id === sessionId) {
        this.stop();
      }
    }, 2200);
  }

  public async startTransition(opts: BridgeTransitionOptions): Promise<BridgeStartResult> {
    const fail: BridgeStartResult = { started: false, actualMixDur: 0, handoffPosSecs: 0 };
    this.stop();
    const ctx = this.ensureContext();
    if (!ctx) return fail;

    if (ctx.state === "suspended") {
      try {
        await ctx.resume();
      } catch {
        return fail;
      }
    }
    if (ctx.state !== "running") return fail;

    const { outSlice, inSlice, outgoingPosSecs, plan, masterVolume } = opts;
    const outOffsetInSlice = outgoingPosSecs - outSlice.offsetSecs;
    const inOffsetInSlice = plan.incomingStartSecs - inSlice.offsetSecs;

    if (
      outOffsetInSlice < -0.25 ||
      outOffsetInSlice >= outSlice.buffer.duration - 2.0 ||
      inOffsetInSlice < -0.25 ||
      inOffsetInSlice >= inSlice.buffer.duration - 3.0
    ) {
      return fail;
    }

    const safeOutOffset = clamp(outOffsetInSlice, 0, outSlice.buffer.duration - 1.0);
    const safeInOffset = clamp(inOffsetInSlice, 0, inSlice.buffer.duration - 1.0);
    const outTargetRate = clamp(plan.outgoingTargetPlaybackRate || 1.0, 0.92, 1.08);
    const inStartRate = clamp(plan.initialPlaybackRate || 1.0, 0.92, 1.08);

    // Ensure mixDur fits comfortably inside BOTH outSlice AND inSlice (with headroom for handoff)
    // so outSource NEVER hits end-of-buffer while still audible!
    const maxOutWallSecs = (outSlice.buffer.duration - safeOutOffset - 0.2) / Math.max(1, outTargetRate);
    const maxInWallSecs = (inSlice.buffer.duration - safeInOffset - 2.0) / Math.max(1, inStartRate);
    const maxSafeMixDur = Math.min(14.0, maxInWallSecs, maxOutWallSecs);
    const mixDur = clamp(plan.durationSecs, 3.5, maxSafeMixDur);
    if (mixDur < 3.5 || maxSafeMixDur < 3.5) {
      return fail;
    }

    if (!this.reverbImpulse) {
      this.reverbImpulse = buildStereoReverbImpulse(ctx);
    }
    if (!this.softClipCurve) {
      this.softClipCurve = buildSoftClipCurve();
    }

    const now = ctx.currentTime;
    const sessionId = ++this.seq;

    // 1. Master Delimiter / Bus Glue Compressor + Soft-Clipper
    const mixBus = ctx.createGain();
    mixBus.gain.setValueAtTime(1.0, now);

    const busCompressor = ctx.createDynamicsCompressor();
    busCompressor.threshold.setValueAtTime(-11, now);
    busCompressor.knee.setValueAtTime(8, now);
    busCompressor.ratio.setValueAtTime(12, now);
    busCompressor.attack.setValueAtTime(0.002, now);
    busCompressor.release.setValueAtTime(0.14, now);

    const softClipper = ctx.createWaveShaper();
    softClipper.curve = this.softClipCurve;
    softClipper.oversample = "2x";

    const masterGain = ctx.createGain();
    // 60ms smooth micro-fade-in so taking over from <audio> Deck A is seamless
    masterGain.gain.setValueAtTime(0.001, now);
    masterGain.gain.linearRampToValueAtTime(clamp(masterVolume, 0, 1), now + 0.06);

    mixBus.connect(busCompressor);
    busCompressor.connect(softClipper);
    softClipper.connect(masterGain);
    masterGain.connect(ctx.destination);

    // 2. Outgoing Deck: Tempo Morph + DJ High-Pass Filter Sweep + Beat Dub Delay + Wash-Out Reverb
    const outSource = ctx.createBufferSource();
    outSource.buffer = outSlice.buffer;
    outSource.playbackRate.setValueAtTime(1.0, now);
    outSource.playbackRate.linearRampToValueAtTime(outTargetRate, now + mixDur * 0.42);

    const outHighPass = ctx.createBiquadFilter();
    outHighPass.type = "highpass";
    outHighPass.Q.setValueAtTime(0.85, now);
    outHighPass.frequency.setValueAtTime(20, now);
    outHighPass.frequency.setValueAtTime(25, now + mixDur * 0.25);
    outHighPass.frequency.exponentialRampToValueAtTime(420, now + mixDur * 0.68);
    outHighPass.frequency.exponentialRampToValueAtTime(950, now + mixDur);

    const outDryGain = ctx.createGain();
    outDryGain.gain.setValueAtTime(1.0, now);
    outDryGain.gain.linearRampToValueAtTime(0.88, now + mixDur * 0.32);
    outDryGain.gain.linearRampToValueAtTime(0.68, now + mixDur * 0.65);
    outDryGain.gain.linearRampToValueAtTime(0.001, now + mixDur * 0.96);

    outSource.connect(outHighPass);
    outHighPass.connect(outDryGain);
    outDryGain.connect(mixBus);

    // 2a. Beat-Synced Dub Echo (1/2 beat delay synced to bridge BPM)
    const bridgeBpm = clamp(
      plan.outBpm ? plan.outBpm * outTargetRate : plan.inBpm ? plan.inBpm * inStartRate : 120,
      72,
      176,
    );
    const echoDelaySecs = clamp((60 / bridgeBpm) * 0.5, 0.16, 0.44);
    const echoSend = ctx.createGain();
    echoSend.gain.setValueAtTime(0.0, now);
    echoSend.gain.setValueAtTime(0.02, now + mixDur * 0.35);
    echoSend.gain.linearRampToValueAtTime(0.38, now + mixDur * 0.78);
    echoSend.gain.linearRampToValueAtTime(0.0, now + mixDur);

    const delayNode = ctx.createDelay(1.0);
    delayNode.delayTime.setValueAtTime(echoDelaySecs, now);

    const echoBandPass = ctx.createBiquadFilter();
    echoBandPass.type = "bandpass";
    echoBandPass.frequency.setValueAtTime(1050, now);
    echoBandPass.Q.setValueAtTime(0.85, now);

    const echoFeedback = ctx.createGain();
    echoFeedback.gain.setValueAtTime(0.43, now);
    echoFeedback.gain.setValueAtTime(0.43, now + mixDur);
    echoFeedback.gain.linearRampToValueAtTime(0.0, now + mixDur + 2.0);

    const echoWetGain = ctx.createGain();
    echoWetGain.gain.setValueAtTime(0.75, now);

    outHighPass.connect(echoSend);
    echoSend.connect(delayNode);
    delayNode.connect(echoBandPass);
    echoBandPass.connect(echoFeedback);
    echoFeedback.connect(delayNode);
    echoBandPass.connect(echoWetGain);
    echoWetGain.connect(mixBus);

    // 2b. Spatial Wash-Out Reverb
    const reverbSend = ctx.createGain();
    reverbSend.gain.setValueAtTime(0.0, now);
    reverbSend.gain.setValueAtTime(0.04, now + mixDur * 0.3);
    reverbSend.gain.linearRampToValueAtTime(0.45, now + mixDur * 0.85);
    reverbSend.gain.linearRampToValueAtTime(0.0, now + mixDur);

    const convolver = ctx.createConvolver();
    convolver.buffer = this.reverbImpulse;

    const reverbWetGain = ctx.createGain();
    reverbWetGain.gain.setValueAtTime(0.85, now);

    outHighPass.connect(reverbSend);
    reverbSend.connect(convolver);
    convolver.connect(reverbWetGain);
    reverbWetGain.connect(mixBus);

    // 3. Incoming Deck: Beat-Locked Tempo Glide + Warm Filter Reveal + Sub-Bass Swap
    const inSource = ctx.createBufferSource();
    inSource.buffer = inSlice.buffer;
    inSource.playbackRate.setValueAtTime(inStartRate, now);
    inSource.playbackRate.setValueAtTime(inStartRate, now + mixDur * 0.55);
    inSource.playbackRate.linearRampToValueAtTime(1.0, now + mixDur);

    const inLowPass = ctx.createBiquadFilter();
    inLowPass.type = "lowpass";
    inLowPass.Q.setValueAtTime(0.75, now);
    inLowPass.frequency.setValueAtTime(1250, now);
    inLowPass.frequency.exponentialRampToValueAtTime(20000, now + mixDur * 0.34);

    const inLowShelf = ctx.createBiquadFilter();
    inLowShelf.type = "lowshelf";
    inLowShelf.frequency.setValueAtTime(165, now);
    inLowShelf.gain.setValueAtTime(-15, now);
    inLowShelf.gain.setValueAtTime(-14, now + mixDur * 0.25);
    inLowShelf.gain.linearRampToValueAtTime(0, now + mixDur * 0.56);

    const inDryGain = ctx.createGain();
    // 15ms zero-crossing micro fade-in prevents any click on non-zero-crossing cue points
    inDryGain.gain.setValueAtTime(0.001, now);
    inDryGain.gain.linearRampToValueAtTime(0.2, now + 0.015);
    inDryGain.gain.linearRampToValueAtTime(0.92, now + mixDur * 0.3);
    inDryGain.gain.linearRampToValueAtTime(1.0, now + mixDur * 0.68);
    inDryGain.gain.setValueAtTime(1.0, now + mixDur);

    inSource.connect(inLowPass);
    inLowPass.connect(inLowShelf);
    inLowShelf.connect(inDryGain);
    inDryGain.connect(mixBus);

    // Start both sample-accurate sources simultaneously on the exact audio clock tick
    outSource.start(now, safeOutOffset);
    inSource.start(now, safeInOffset);

    const handoffPosSecs = computeBridgeIncomingPosition(
      plan.incomingStartSecs,
      inStartRate,
      mixDur,
      mixDur,
    );

    const startedAtPerf = performance.now();
    const session: ActiveBridgeSession = {
      id: sessionId,
      startedAtPerf,
      durationSecs: mixDur,
      inStartSecs: plan.incomingStartSecs,
      inDurationSecs: inSlice.trackDurationSecs,
      inStartRate,
      masterGain,
      inDryGain,
      outSource,
      inSource,
      nodesToDisconnect: [
        outSource,
        inSource,
        outHighPass,
        outDryGain,
        echoSend,
        delayNode,
        echoBandPass,
        echoFeedback,
        echoWetGain,
        reverbSend,
        convolver,
        reverbWetGain,
        inLowPass,
        inLowShelf,
        inDryGain,
        mixBus,
        busCompressor,
        softClipper,
        masterGain,
      ],
      progressTimer: null,
      handoffTimer: null,
      cleanupTimer: null,
      handoffFired: false,
      fadeOutStarted: false,
      onProgress: opts.onProgress,
      onHandoffReady: opts.onHandoffReady,
    };
    this.active = session;

    session.progressTimer = window.setInterval(() => {
      if (this.active?.id !== sessionId || session.handoffFired) return;
      const elapsed = (performance.now() - startedAtPerf) / 1000;
      const p01 = clamp(elapsed / mixDur, 0, 1);
      const pos = computeBridgeIncomingPosition(
        session.inStartSecs,
        session.inStartRate,
        mixDur,
        elapsed,
      );
      session.onProgress(pos, session.inDurationSecs, p01);
    }, 50);

    // Trigger handoff to <audio> Deck B at mixDur; safety fallback fades out after 450ms
    // even if <audio> play() is delayed, while Echo/Reverb tails ring out for 2.2s.
    session.handoffTimer = window.setTimeout(() => {
      if (this.active?.id !== sessionId || session.handoffFired) return;
      session.handoffFired = true;
      if (session.progressTimer !== null) {
        window.clearInterval(session.progressTimer);
        session.progressTimer = null;
      }
      session.onHandoffReady(handoffPosSecs);

      session.cleanupTimer = window.setTimeout(() => {
        if (this.active?.id === sessionId && !session.fadeOutStarted) {
          this.beginHandoffFadeOut();
        }
      }, 450);
    }, Math.round(mixDur * 1000));

    return { started: true, actualMixDur: mixDur, handoffPosSecs };
  }
}
