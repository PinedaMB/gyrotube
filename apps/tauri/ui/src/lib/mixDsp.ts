import type { TrackMixProfile } from "./types";

export const MIX_ANALYSIS_VERSION = 3;
export const MIX_FRAME_RATE_HZ = 100; // 10 ms hop with 20 ms overlapping RMS window

const MIN_BPM = 70;
const MAX_BPM = 180;
const BPM_CONFIDENCE_THRESHOLD = 0.25;

export interface ExtractedEnvelopes {
  rms: Float32Array;
  onset: Float32Array;
  frameRate: number;
}

/**
 * Downsamples a mono PCM signal into 100 Hz full-band RMS and 3-band transient onset flux.
 * Splitting into Low (<150 Hz), Mid (150-2500 Hz), and High (>2500 Hz) ensures mastered/compressed
 * tracks with continuous basslines still produce crisp kick, snare, and hi-hat onset peaks.
 */
export function extractEnvelopes(
  monoPcm: Float32Array,
  sampleRate: number,
  frameRate = MIX_FRAME_RATE_HZ,
): ExtractedEnvelopes {
  const hop = Math.max(1, Math.round(sampleRate / frameRate));
  const numFrames = Math.floor(monoPcm.length / hop);
  const rms = new Float32Array(numFrames);
  const lowRms = new Float32Array(numFrames);
  const midRms = new Float32Array(numFrames);
  const highRms = new Float32Array(numFrames);
  const onset = new Float32Array(numFrames);

  if (numFrames === 0) {
    return { rms, onset, frameRate };
  }

  const sr = Math.max(8000, sampleRate);
  const alphaLow = 1 - Math.exp((-2 * Math.PI * 150) / sr);
  const alphaMid = 1 - Math.exp((-2 * Math.PI * 2500) / sr);
  let lpLow = 0;
  let lpMid = 0;

  for (let f = 0; f < numFrames; f++) {
    const start = f * hop;
    const end = Math.min(monoPcm.length, start + hop);
    let sumSq = 0;
    let lowSq = 0;
    let midSq = 0;
    let highSq = 0;
    for (let i = start; i < end; i++) {
      const s = monoPcm[i];
      sumSq += s * s;
      lpLow += alphaLow * (s - lpLow);
      lpMid += alphaMid * (s - lpMid);
      const low = lpLow;
      const mid = lpMid - lpLow;
      const high = s - lpMid;
      lowSq += low * low;
      midSq += mid * mid;
      highSq += high * high;
    }
    const count = Math.max(1, end - start);
    rms[f] = Math.sqrt(sumSq / count);
    lowRms[f] = Math.sqrt(lowSq / count);
    midRms[f] = Math.sqrt(midSq / count);
    highRms[f] = Math.sqrt(highSq / count);
  }

  // Smooth band RMS with a 3-frame (30 ms) triangular kernel to eliminate sub-bass
  // window-aliasing ripple while keeping 10 ms transient attack resolution.
  const smooth3 = (arr: Float32Array): Float32Array => {
    const out = new Float32Array(arr.length);
    if (arr.length === 0) return out;
    out[0] = arr[0];
    out[arr.length - 1] = arr[arr.length - 1];
    for (let i = 1; i < arr.length - 1; i++) {
      out[i] = arr[i - 1] * 0.25 + arr[i] * 0.5 + arr[i + 1] * 0.25;
    }
    return out;
  };
  const sLow = smooth3(lowRms);
  const sMid = smooth3(midRms);
  const sHigh = smooth3(highRms);

  // Half-wave rectified multi-band log-energy flux (mild compression preserves transients on mastered tracks)
  let prevLow = Math.log1p(12 * sLow[0]);
  let prevMid = Math.log1p(12 * sMid[0]);
  let prevHigh = Math.log1p(12 * sHigh[0]);
  for (let f = 1; f < numFrames; f++) {
    const curLow = Math.log1p(12 * sLow[f]);
    const curMid = Math.log1p(12 * sMid[f]);
    const curHigh = Math.log1p(12 * sHigh[f]);
    const lowFlux = Math.max(0, curLow - prevLow);
    const midFlux = Math.max(0, curMid - prevMid);
    const highFlux = Math.max(0, curHigh - prevHigh);
    onset[f] = lowFlux * 0.45 + midFlux * 0.35 + highFlux * 0.2;
    prevLow = curLow;
    prevMid = curMid;
    prevHigh = curHigh;
  }

  return { rms, onset, frameRate };
}

function rmsToDb(v: number): number {
  return 20 * Math.log10(Math.max(1e-7, v));
}

/**
 * Detects the rhythmic beat-drop intro cue point and phrase outro mix-out point
 * from the 100 Hz RMS and onset envelopes so Mix Mode skips dead intros/outros.
 */
export function detectIntroOutro(
  rms: Float32Array,
  durationSecs: number,
  frameRate = MIX_FRAME_RATE_HZ,
  onset?: Float32Array,
): { introEndSecs: number; outroStartSecs: number } {
  if (rms.length === 0 || durationSecs <= 0) {
    return { introEndSecs: 0, outroStartSecs: Math.max(0, durationSecs - 16) };
  }

  // Estimate 90th-percentile loudness reference across audible frames
  const sampleStep = Math.max(1, Math.floor(rms.length / 2000));
  const samples: number[] = [];
  for (let i = 0; i < rms.length; i += sampleStep) {
    if (rms[i] > 1e-5) samples.push(rms[i]);
  }
  if (samples.length === 0) {
    return { introEndSecs: 0, outroStartSecs: Math.max(0, durationSecs - 16) };
  }
  samples.sort((a, b) => a - b);
  const peakRef = samples[Math.min(samples.length - 1, Math.floor(samples.length * 0.9))];
  const peakRefDb = rmsToDb(peakRef);

  const windowFrames = Math.max(1, Math.round(2.0 * frameRate));
  const prefix = new Float64Array(rms.length + 1);
  for (let i = 0; i < rms.length; i++) {
    prefix[i + 1] = prefix[i] + rms[i];
  }

  const winSamples: number[] = [];
  const winStep = Math.max(1, Math.floor(rms.length / 400));
  for (let f = windowFrames; f < rms.length; f += winStep) {
    const avg = (prefix[f] - prefix[f - windowFrames]) / windowFrames;
    if (avg > 1e-5) winSamples.push(avg);
  }
  winSamples.sort((a, b) => a - b);
  const sustainRef =
    winSamples.length > 0
      ? winSamples[Math.min(winSamples.length - 1, Math.floor(winSamples.length * 0.9))]
      : peakRef;
  const sustainRefDb = rmsToDb(sustainRef);

  // 1) Intro cue: find where real groove/energy enters (not just a quiet -26 dB pad at 0.05s).
  // First find the earliest audible sound (-24 dB), then look up to 18 s ahead for where
  // 0.8 s sustained loudness reaches within -10.5 dB of the track's main body.
  const firstSoundThreshDb = peakRefDb - 24;
  const grooveThreshDb = sustainRefDb - 10.5;
  const maxIntroSecs = Math.min(18, durationSecs * 0.16);
  const maxIntroFrame = Math.min(rms.length - 1, Math.floor(maxIntroSecs * frameRate));
  const shortWin = Math.max(1, Math.round(0.8 * frameRate));

  let firstSoundFrame = 0;
  for (let i = 0; i <= maxIntroFrame; i++) {
    if (rmsToDb(rms[i]) >= firstSoundThreshDb) {
      firstSoundFrame = i;
      break;
    }
  }

  let grooveFrame = firstSoundFrame;
  for (let i = firstSoundFrame; i <= maxIntroFrame; i++) {
    const hi = Math.min(rms.length, i + shortWin);
    const localAvg = (prefix[hi] - prefix[i]) / Math.max(1, hi - i);
    if (rmsToDb(localAvg) >= grooveThreshDb) {
      grooveFrame = i;
      break;
    }
  }

  // Snap grooveFrame to the strongest nearby transient onset within ±0.35 s
  if (onset && onset.length === rms.length) {
    const searchLo = Math.max(firstSoundFrame, grooveFrame - Math.round(0.2 * frameRate));
    const searchHi = Math.min(maxIntroFrame, grooveFrame + Math.round(0.45 * frameRate));
    let bestOnsetFrame = grooveFrame;
    let bestOnsetVal = -1;
    for (let i = searchLo; i <= searchHi; i++) {
      if (onset[i] > bestOnsetVal) {
        bestOnsetVal = onset[i];
        bestOnsetFrame = i;
      }
    }
    if (bestOnsetVal > 0.02) {
      grooveFrame = bestOnsetFrame;
    }
  }

  const introEndSecs = Math.max(0, Math.min(maxIntroSecs, grooveFrame / frameRate - 0.04));

  // 2) Outro mix-out point: find where the final phrase winds down (-4.5 dB from sustainRef)
  // or at least 16-24 s before the end of full-length songs so mixes happen continuously.
  const tailThreshDb = peakRefDb - 22;
  let lastAudibleFrame = rms.length - 1;
  while (lastAudibleFrame > 0 && rmsToDb(rms[lastAudibleFrame]) < tailThreshDb) {
    lastAudibleFrame--;
  }
  const audibleEndSecs = Math.min(durationSecs, (lastAudibleFrame + 1) / frameRate);

  const maxOutroCut = Math.min(30, durationSecs * 0.26);
  const minOutroSecs = Math.max(introEndSecs + 8, durationSecs - maxOutroCut);
  const dropThreshDb = sustainRefDb - 4.5;

  const scanStartFrame = Math.max(
    windowFrames,
    Math.floor(Math.max(minOutroSecs, durationSecs - 32) * frameRate),
  );
  let outroFrame = lastAudibleFrame;
  for (let f = lastAudibleFrame; f >= scanStartFrame; f--) {
    const winStart = Math.max(0, f - windowFrames);
    const avgRms = (prefix[f] - prefix[winStart]) / Math.max(1, f - winStart);
    if (rmsToDb(avgRms) >= dropThreshDb) {
      outroFrame = Math.min(lastAudibleFrame, f + Math.round(0.2 * frameRate));
      break;
    }
    outroFrame = f;
  }

  const rawOutroSecs = outroFrame / frameRate;
  const outroStartSecs = Math.max(minOutroSecs, Math.min(audibleEndSecs, rawOutroSecs));

  return {
    introEndSecs: Number(introEndSecs.toFixed(3)),
    outroStartSecs: Number(outroStartSecs.toFixed(3)),
  };
}

/**
 * Estimates BPM, confidence (0..1), and beat grid phase offset from the 100 Hz 3-band onset envelope.
 * Uses the core middle section of the song (where drums/groove are strongest) and harmonic
 * comb-autocorrelation (lag + 2*lag + 3*lag) for reliable detection on mastered music.
 */
export function detectBpmAndBeatGrid(
  onset: Float32Array,
  frameRate = MIX_FRAME_RATE_HZ,
): { bpm: number | null; bpmConfidence: number; beatOffsetSecs: number } {
  const n = onset.length;
  if (n < frameRate * 8) {
    return { bpm: null, bpmConfidence: 0, beatOffsetSecs: 0 };
  }

  // Adaptive local-mean subtraction (0.6 s window) to isolate rhythmic peaks from sustained energy
  const halfWin = Math.round(frameRate * 0.3);
  const prefix = new Float64Array(n + 1);
  for (let i = 0; i < n; i++) prefix[i + 1] = prefix[i] + onset[i];

  const norm = new Float32Array(n);
  let energySum = 0;
  for (let i = 0; i < n; i++) {
    const lo = Math.max(0, i - halfWin);
    const hi = Math.min(n, i + halfWin + 1);
    const localMean = (prefix[hi] - prefix[lo]) / (hi - lo);
    const val = Math.max(0, onset[i] - localMean);
    norm[i] = val;
    energySum += val * val;
  }
  if (energySum / n < 5e-4) {
    return { bpm: null, bpmConfidence: 0, beatOffsetSecs: 0 };
  }

  // Analyze up to 60 s from the core body of the track (skipping quiet intro/outro)
  const maxBodyFrames = frameRate * 60;
  let bodyStart = 0;
  let bodyEnd = n;
  if (n > maxBodyFrames) {
    const margin = Math.min(Math.floor(n * 0.15), Math.floor((n - maxBodyFrames) / 2));
    bodyStart = margin;
    bodyEnd = Math.min(n, bodyStart + maxBodyFrames);
  }
  const signal = norm.subarray(bodyStart, bodyEnd);
  const len = signal.length;

  const minLag = Math.max(1, Math.floor((frameRate * 60) / MAX_BPM));
  const maxLag = Math.min(Math.floor(len / 4), Math.ceil((frameRate * 60) / MIN_BPM));
  if (maxLag <= minLag + 2) {
    return { bpm: null, bpmConfidence: 0, beatOffsetSecs: 0 };
  }

  // Compute raw autocorrelation up to 4 * maxLag so continuous-BPM comb scoring is O(1)
  const maxHarmonicLag = Math.min(Math.floor(len / 2), maxLag * 4 + 2);
  const acf = new Float64Array(maxHarmonicLag + 1);
  for (let lag = Math.max(1, minLag - 1); lag <= maxHarmonicLag; lag++) {
    let sum = 0;
    const limit = len - lag;
    for (let i = 0; i < limit; i++) {
      sum += signal[i] * signal[i + lag];
    }
    acf[lag] = sum / Math.max(1, limit);
  }

  const acfAt = (lag: number): number => {
    if (lag < 1 || lag >= maxHarmonicLag) return 0;
    const lo = Math.floor(lag);
    const frac = lag - lo;
    return acf[lo] * (1 - frac) + acf[lo + 1] * frac;
  };

  // Scan in continuous 0.5 BPM steps (eliminates integer-lag quantization error at high BPMs)
  const bpmStep = 0.5;
  const candCount = Math.floor((MAX_BPM - MIN_BPM) / bpmStep) + 1;
  const combScores = new Float64Array(candCount);
  const weightedScores = new Float64Array(candCount);

  let sumScore = 0;
  for (let i = 0; i < candCount; i++) {
    const bpmCand = MIN_BPM + i * bpmStep;
    const lag = (frameRate * 60) / bpmCand;
    const comb =
      acfAt(lag) + 0.6 * acfAt(lag * 2) + 0.35 * acfAt(lag * 3) + 0.2 * acfAt(lag * 4);
    combScores[i] = comb;
    sumScore += comb;

    // Gentle Gaussian prior around 120 BPM (sigma = 0.55 octaves) to prefer musical octave
    const octDist = Math.log2(bpmCand / 120);
    const prior = Math.exp(-0.5 * (octDist / 0.55) * (octDist / 0.55));
    weightedScores[i] = comb * (0.72 + 0.28 * prior);
  }

  let bestIdx = 0;
  for (let i = 1; i < candCount; i++) {
    if (weightedScores[i] > weightedScores[bestIdx]) {
      bestIdx = i;
    }
  }

  // Parabolic interpolation around best BPM candidate
  let bpm = MIN_BPM + bestIdx * bpmStep;
  if (bestIdx > 0 && bestIdx < candCount - 1) {
    const y0 = weightedScores[bestIdx - 1];
    const y1 = weightedScores[bestIdx];
    const y2 = weightedScores[bestIdx + 1];
    const denom = y0 - 2 * y1 + y2;
    if (Math.abs(denom) > 1e-12) {
      const delta = Math.max(-0.5, Math.min(0.5, (0.5 * (y0 - y2)) / denom));
      bpm += delta * bpmStep;
    }
  }

  // Z-score confidence of best comb peak vs distribution of all candidate BPMs
  const meanScore = sumScore / candCount;
  let varSum = 0;
  for (let i = 0; i < candCount; i++) {
    const d = combScores[i] - meanScore;
    varSum += d * d;
  }
  const stdScore = Math.sqrt(varSum / candCount);
  const zScore = stdScore > 1e-12 ? (combScores[bestIdx] - meanScore) / stdScore : 0;
  const bpmConfidence = Math.max(0, Math.min(1, (zScore - 1.2) / 2.8));

  if (bpmConfidence < BPM_CONFIDENCE_THRESHOLD) {
    return {
      bpm: null,
      bpmConfidence: Number(bpmConfidence.toFixed(3)),
      beatOffsetSecs: 0,
    };
  }

  const periodFrames = (frameRate * 60) / bpm;
  const bins = Math.max(8, Math.round(periodFrames));
  const phaseHist = new Float64Array(bins);

  // Fold full track's onset envelope into beat-period phase bins
  for (let i = 0; i < n; i++) {
    const phase = ((i % periodFrames) / periodFrames) * bins;
    const bin = Math.floor(phase) % bins;
    phaseHist[bin] += norm[i];
  }

  // Smooth circularly with [0.25, 0.5, 0.25] kernel
  let bestBin = 0;
  let bestVal = -1;
  for (let b = 0; b < bins; b++) {
    const prev = phaseHist[(b - 1 + bins) % bins];
    const cur = phaseHist[b];
    const next = phaseHist[(b + 1) % bins];
    const smoothed = prev * 0.25 + cur * 0.5 + next * 0.25;
    if (smoothed > bestVal) {
      bestVal = smoothed;
      bestBin = b;
    }
  }

  const beatOffsetSecs = (bestBin / bins) * (60 / bpm);

  return {
    bpm: Number(bpm.toFixed(2)),
    bpmConfidence: Number(bpmConfidence.toFixed(3)),
    beatOffsetSecs: Number(beatOffsetSecs.toFixed(4)),
  };
}

/**
 * Full DSP analysis on a decoded Web Audio AudioBuffer.
 */
export function analyzeAudioBuffer(buffer: AudioBuffer): TrackMixProfile {
  const length = buffer.length;
  const channels = buffer.numberOfChannels;
  const mono = new Float32Array(length);

  if (channels === 1) {
    mono.set(buffer.getChannelData(0));
  } else {
    const inv = 1 / channels;
    for (let c = 0; c < channels; c++) {
      const data = buffer.getChannelData(c);
      for (let i = 0; i < length; i++) {
        mono[i] += data[i] * inv;
      }
    }
  }

  const { rms, onset, frameRate } = extractEnvelopes(mono, buffer.sampleRate);
  const { introEndSecs, outroStartSecs } = detectIntroOutro(rms, buffer.duration, frameRate, onset);
  const { bpm, bpmConfidence, beatOffsetSecs } = detectBpmAndBeatGrid(onset, frameRate);

  return {
    version: MIX_ANALYSIS_VERSION,
    duration_secs: Number(buffer.duration.toFixed(3)),
    intro_end_secs: introEndSecs,
    outro_start_secs: outroStartSecs,
    bpm,
    bpm_confidence: bpmConfidence,
    beat_offset_secs: beatOffsetSecs,
    analyzed_at: Math.floor(Date.now() / 1000),
  };
}
