import {
  detectBpmAndBeatGrid,
  detectIntroOutro,
  extractEnvelopes,
  MIX_ANALYSIS_VERSION,
} from "./mixDsp";
import type { TrackMixProfile } from "./types";

export interface MixWorkerRequest {
  id: number;
  monoPcm: Float32Array;
  sampleRate: number;
  durationSecs: number;
}

export interface MixWorkerResponse {
  id: number;
  profile: TrackMixProfile | null;
}

self.onmessage = (e: MessageEvent<MixWorkerRequest>) => {
  const { id, monoPcm, sampleRate, durationSecs } = e.data;
  try {
    const { rms, onset, frameRate } = extractEnvelopes(monoPcm, sampleRate);
    const { introEndSecs, outroStartSecs } = detectIntroOutro(rms, durationSecs, frameRate, onset);
    const { bpm, bpmConfidence, beatOffsetSecs } = detectBpmAndBeatGrid(onset, frameRate);
    const profile: TrackMixProfile = {
      version: MIX_ANALYSIS_VERSION,
      duration_secs: Number(durationSecs.toFixed(3)),
      intro_end_secs: introEndSecs,
      outro_start_secs: outroStartSecs,
      bpm,
      bpm_confidence: bpmConfidence,
      beat_offset_secs: beatOffsetSecs,
      analyzed_at: Math.floor(Date.now() / 1000),
    };
    const resp: MixWorkerResponse = { id, profile };
    self.postMessage(resp);
  } catch {
    const resp: MixWorkerResponse = { id, profile: null };
    self.postMessage(resp);
  }
};
