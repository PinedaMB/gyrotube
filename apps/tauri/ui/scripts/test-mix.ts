import assert from "node:assert/strict";
import {
  detectBpmAndBeatGrid,
  detectIntroOutro,
  extractEnvelopes,
  MIX_ANALYSIS_VERSION,
} from "../src/lib/mixDsp.ts";
import { planMixTransition } from "../src/lib/mixPlanner.ts";
import { computeBridgeIncomingPosition } from "../src/lib/mixTransitionBridge.ts";
import type { TrackMixProfile } from "../src/lib/types.ts";

/**
 * Generates a synthetic kick-drum track at the requested BPM with optional leading silence
 * and trailing fade-out for deterministic DSP verification.
 */
function makeSyntheticTrack(opts: {
  sampleRate: number;
  durationSecs: number;
  bpm: number;
  introSilenceSecs?: number;
  outroFadeSecs?: number;
  beatOffsetSecs?: number;
}): Float32Array {
  const {
    sampleRate,
    durationSecs,
    bpm,
    introSilenceSecs = 0,
    outroFadeSecs = 0,
    beatOffsetSecs = 0,
  } = opts;
  const totalSamples = Math.floor(sampleRate * durationSecs);
  const pcm = new Float32Array(totalSamples);
  const beatPeriod = 60 / bpm;

  for (let t = beatOffsetSecs; t < durationSecs; t += beatPeriod) {
    if (t < introSilenceSecs) continue;
    let amp = 0.85;
    if (outroFadeSecs > 0 && t > durationSecs - outroFadeSecs) {
      const rem = Math.max(0, durationSecs - t);
      amp *= Math.pow(rem / outroFadeSecs, 2);
    }
    const startSample = Math.floor(t * sampleRate);
    const kickLen = Math.floor(0.09 * sampleRate);
    for (let k = 0; k < kickLen && startSample + k < totalSamples; k++) {
      const env = Math.exp((-k / sampleRate) * 32);
      const freq = 55 + 90 * Math.exp((-k / sampleRate) * 45);
      pcm[startSample + k] += amp * env * Math.sin((2 * Math.PI * freq * k) / sampleRate);
    }
  }
  return pcm;
}

const sr = 22050;

// 1. Test 120 BPM track with 2.5 s leading silence and 10 s outro fade
const pcm120 = makeSyntheticTrack({
  sampleRate: sr,
  durationSecs: 60,
  bpm: 120,
  introSilenceSecs: 2.5,
  outroFadeSecs: 10,
  beatOffsetSecs: 0.0,
});
const env120 = extractEnvelopes(pcm120, sr);
const io120 = detectIntroOutro(env120.rms, 60, env120.frameRate);
const bpm120 = detectBpmAndBeatGrid(env120.onset, env120.frameRate);

assert.ok(bpm120.bpm !== null, "Expected BPM detection on 120 BPM kick track");
assert.ok(Math.abs(bpm120.bpm! - 120) <= 1.5, `Expected ~120 BPM, got ${bpm120.bpm}`);
assert.ok(bpm120.bpmConfidence >= 0.35, `Expected high confidence, got ${bpm120.bpmConfidence}`);
assert.ok(
  io120.introEndSecs >= 2.1 && io120.introEndSecs <= 2.9,
  `Expected intro ~2.5s, got ${io120.introEndSecs}`,
);
assert.ok(
  io120.outroStartSecs >= 48 && io120.outroStartSecs <= 56,
  `Expected outro start inside fade-out window, got ${io120.outroStartSecs}`,
);

// 2. Test 126 BPM track (+5% tempo difference -> compatible beatmatch!)
const pcm126 = makeSyntheticTrack({
  sampleRate: sr,
  durationSecs: 60,
  bpm: 126,
  introSilenceSecs: 1.5,
  beatOffsetSecs: 0.1,
});
const env126 = extractEnvelopes(pcm126, sr);
const io126 = detectIntroOutro(env126.rms, 60, env126.frameRate);
const bpm126 = detectBpmAndBeatGrid(env126.onset, env126.frameRate);

assert.ok(bpm126.bpm !== null && Math.abs(bpm126.bpm - 126) <= 1.5, `Expected ~126 BPM, got ${bpm126.bpm}`);

const profileA: TrackMixProfile = {
  version: MIX_ANALYSIS_VERSION,
  duration_secs: 60,
  intro_end_secs: io120.introEndSecs,
  outro_start_secs: io120.outroStartSecs,
  bpm: bpm120.bpm,
  bpm_confidence: bpm120.bpmConfidence,
  beat_offset_secs: bpm120.beatOffsetSecs,
  analyzed_at: 0,
};
const profileB: TrackMixProfile = {
  version: MIX_ANALYSIS_VERSION,
  duration_secs: 60,
  intro_end_secs: io126.introEndSecs,
  outro_start_secs: io126.outroStartSecs,
  bpm: bpm126.bpm,
  bpm_confidence: bpm126.bpmConfidence,
  beat_offset_secs: bpm126.beatOffsetSecs,
  analyzed_at: 0,
};

const beatmatchPlan = planMixTransition(profileA, profileB, {
  outgoingDurationSecs: 60,
  mixMode: true,
  crossfadeSecs: 8,
});
assert.ok(beatmatchPlan !== null);
assert.equal(beatmatchPlan.mode, "beatmatch");
assert.equal(beatmatchPlan.useBassSwap, true);
assert.ok(
  beatmatchPlan.outgoingTargetPlaybackRate >= 1.0 && beatmatchPlan.outgoingTargetPlaybackRate <= 1.07,
  `Expected outgoing rate in [1.0, 1.07], got ${beatmatchPlan.outgoingTargetPlaybackRate}`,
);
assert.ok(
  beatmatchPlan.initialPlaybackRate >= 0.93 && beatmatchPlan.initialPlaybackRate < 1.0,
  `Expected incoming rate in [0.93, 1.0), got ${beatmatchPlan.initialPlaybackRate}`,
);
assert.ok(beatmatchPlan.incomingStartSecs >= 1.2, `Expected intro skip, got ${beatmatchPlan.incomingStartSecs}`);

// 2b. Larger tempo jump (102.5 BPM -> 114 BPM, ~11% difference) -> Dual-deck tempo bridge!
const bridgePlan = planMixTransition(
  { ...profileA, bpm: 102.5, bpm_confidence: 1.0, duration_secs: 205 },
  { ...profileB, bpm: 114.0, bpm_confidence: 0.7, duration_secs: 213 },
  {
    outgoingDurationSecs: 205,
    mixMode: true,
    crossfadeSecs: 0,
  },
);
assert.ok(bridgePlan !== null);
assert.equal(bridgePlan.mode, "beatmatch");
assert.ok(
  bridgePlan.outgoingTargetPlaybackRate > 1.03 && bridgePlan.outgoingTargetPlaybackRate <= 1.07,
  `Expected outgoing track to speed up into bridge tempo, got ${bridgePlan.outgoingTargetPlaybackRate}`,
);
assert.ok(
  bridgePlan.initialPlaybackRate >= 0.93 && bridgePlan.initialPlaybackRate < 0.97,
  `Expected incoming track to start slowed at bridge tempo, got ${bridgePlan.initialPlaybackRate}`,
);

// 3. Incompatible BPM (80 vs 112 -> 29% gap) -> graceful fallback to Level 1 "smart"
const profileSlow: TrackMixProfile = {
  ...profileA,
  bpm: 80,
  bpm_confidence: 0.9,
};
const profileFast: TrackMixProfile = {
  ...profileB,
  bpm: 112,
  bpm_confidence: 0.9,
};
const smartPlan = planMixTransition(profileSlow, profileFast, {
  outgoingDurationSecs: 180,
  mixMode: true,
  crossfadeSecs: 0,
});
assert.ok(smartPlan !== null);
assert.equal(smartPlan.mode, "smart");
assert.equal(smartPlan.initialPlaybackRate, 1.0);
assert.equal(smartPlan.useBassSwap, true);

// 4. Mastered/compressed track with continuous sub-bassline + kick/snare/hi-hat at 128 BPM
const pcmMastered = new Float32Array(sr * 70);
const beat128 = 60 / 128;
for (let i = 0; i < pcmMastered.length; i++) {
  const t = i / sr;
  // Continuous loud sub-bass drone (what broke single-band lowpass detection!)
  let s = 0.35 * Math.sin(2 * Math.PI * 65 * t) + 0.15 * Math.sin(2 * Math.PI * 130 * t);
  const beatPos = t % beat128;
  // Kick on every beat
  if (beatPos < 0.08) {
    s += 0.65 * Math.exp(-beatPos * 35) * Math.sin(2 * Math.PI * (55 + 85 * Math.exp(-beatPos * 40)) * beatPos);
  }
  // Snare/clap on beats 2 & 4 (mid/high transient)
  const beatIdx = Math.floor(t / beat128);
  if (beatIdx % 2 === 1 && beatPos < 0.06) {
    s += 0.45 * Math.exp(-beatPos * 50) * Math.sin(2 * Math.PI * 1100 * beatPos);
  }
  // Soft-clip mastering limiter
  pcmMastered[i] = Math.tanh(s * 1.4);
}
const envMastered = extractEnvelopes(pcmMastered, sr);
const bpmMastered = detectBpmAndBeatGrid(envMastered.onset, envMastered.frameRate);
assert.ok(
  bpmMastered.bpm !== null && Math.abs(bpmMastered.bpm - 128) <= 1.5,
  `Expected ~128 BPM on compressed mastered track, got ${bpmMastered.bpm} (conf=${bpmMastered.bpmConfidence})`,
);

// 5. Steady sine tone (no rhythm) -> bpm: null
const tone = new Float32Array(sr * 20);
for (let i = 0; i < tone.length; i++) tone[i] = 0.4 * Math.sin((2 * Math.PI * 220 * i) / sr);
const envTone = extractEnvelopes(tone, sr);
const bpmTone = detectBpmAndBeatGrid(envTone.onset, envTone.frameRate);
assert.equal(bpmTone.bpm, null, `Expected null BPM for steady sine tone, got ${bpmTone.bpm}`);

// 6. Test analytical DSP bridge handoff position integration
const handoffPos = computeBridgeIncomingPosition(2.0, 0.9, 10.0, 10.0);
// 0..5.5s at 0.9x = 4.95s; 5.5..10.0s (4.5s) ramping 0.9->1.0 (avg 0.95x) = 4.275s -> total 2.0 + 9.225 = 11.225s
assert.ok(
  Math.abs(handoffPos - 11.225) < 1e-4,
  `Expected exact analytical handoff pos 11.225s, got ${handoffPos}`,
);

console.log("✓ Mix DSP & planner unit tests passed:", {
  detected120: bpm120.bpm,
  detected126: bpm126.bpm,
  detectedMastered128: bpmMastered.bpm,
  masteredConf: bpmMastered.bpmConfidence,
  beatmatchRate: beatmatchPlan.initialPlaybackRate,
  beatmatchDuration: beatmatchPlan.durationSecs,
});
