import type { TrackMixProfile } from "./types";

export type MixTransitionMode = "beatmatch" | "smart" | "crossfade";

export interface TransitionPlan {
  mode: MixTransitionMode;
  triggerAtSecs: number;
  incomingStartSecs: number;
  durationSecs: number;
  /** Outgoing track target playbackRate during the transition (morphs 1.0 -> outgoingTargetPlaybackRate) */
  outgoingTargetPlaybackRate: number;
  /** Incoming track initial playbackRate at mix entry (morphs initialPlaybackRate -> 1.0) */
  initialPlaybackRate: number;
  useBassSwap: boolean;
  outBpm: number | null;
  inBpm: number | null;
}

export interface MixPlanOptions {
  outgoingDurationSecs: number;
  mixMode: boolean;
  crossfadeSecs: number;
}

const MIN_BEATMATCH_CONFIDENCE = 0.22;
// Keep per-deck rate shift within ±7% (~1.1 semitones) so turntable-style tempo ramps
// sound natural on vocals while bridging up to ±14% total BPM difference across both decks.
const MAX_SINGLE_DECK_STRETCH = 0.07;
const MAX_COMBINED_RATIO_DELTA = 0.14;

function clamp(v: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, v));
}

/**
 * Finds the best tempo ratio (including 1:1, 2:1, 1:2, 4:3, and 3:4 polyrhythmic DJ ratios)
 * and splits the tempo adaptation across BOTH decks so songs up to ±25% apart lock together
 * without either song exceeding a natural ±12% pitch-preserved tempo shift.
 */
function computeDualDeckTempoBridge(
  outBpm: number,
  inBpm: number,
): { outTargetRate: number; inStartRate: number; effectiveRatio: number } | null {
  if (outBpm <= 0 || inBpm <= 0) return null;
  // Candidate multipliers for incoming BPM (1x direct, 2x double-time, 0.5x half-time)
  const multipliers = [1.0, 2.0, 0.5];
  let bestRatio = outBpm / inBpm;
  let bestDelta = Math.abs(bestRatio - 1);

  for (const m of multipliers) {
    const r = outBpm / (inBpm * m);
    const d = Math.abs(r - 1);
    if (d < bestDelta) {
      bestRatio = r;
      bestDelta = d;
    }
  }

  if (bestDelta > MAX_COMBINED_RATIO_DELTA) {
    return null;
  }

  // If within ±4.5%, keep outgoing steady at 1.0 and only adjust incoming deck
  if (bestDelta <= 0.045) {
    return {
      outTargetRate: 1.0,
      inStartRate: clamp(bestRatio, 1 - MAX_SINGLE_DECK_STRETCH, 1 + MAX_SINGLE_DECK_STRETCH),
      effectiveRatio: bestRatio,
    };
  }

  // For larger BPM gaps (7%..25%), meet in the geometric middle!
  // Outgoing moves toward incoming (1.0 -> 1/sqrt(ratio)), incoming starts at sqrt(ratio) -> 1.0.
  const halfRatio = Math.sqrt(bestRatio);
  const outTargetRate = clamp(
    1 / halfRatio,
    1 - MAX_SINGLE_DECK_STRETCH,
    1 + MAX_SINGLE_DECK_STRETCH,
  );
  const inStartRate = clamp(
    bestRatio * outTargetRate,
    1 - MAX_SINGLE_DECK_STRETCH,
    1 + MAX_SINGLE_DECK_STRETCH,
  );

  return {
    outTargetRate,
    inStartRate,
    effectiveRatio: bestRatio,
  };
}

function snapToNearestBeat(targetSecs: number, beatOffsetSecs: number, beatPeriodSecs: number): number {
  const k = Math.round((targetSecs - beatOffsetSecs) / beatPeriodSecs);
  return Math.max(0, beatOffsetSecs + k * beatPeriodSecs);
}

function snapToNextBeat(minSecs: number, beatOffsetSecs: number, beatPeriodSecs: number): number {
  const k = Math.ceil((minSecs - beatOffsetSecs) / beatPeriodSecs);
  return Math.max(0, beatOffsetSecs + Math.max(0, k) * beatPeriodSecs);
}

/**
 * Computes a complete Spotify-style Automix / DJ transition plan between two tracks.
 */
export function planMixTransition(
  outgoing: TrackMixProfile | null,
  incoming: TrackMixProfile | null,
  opts: MixPlanOptions,
): TransitionPlan | null {
  const outDur = opts.outgoingDurationSecs > 0 ? opts.outgoingDurationSecs : (outgoing?.duration_secs ?? 0);
  if (outDur <= 4) return null;

  // Classic manual crossfade when Mix Mode is off
  if (!opts.mixMode) {
    if (opts.crossfadeSecs <= 0 || outDur <= opts.crossfadeSecs + 2) return null;
    return {
      mode: "crossfade",
      triggerAtSecs: Math.max(0, outDur - opts.crossfadeSecs),
      incomingStartSecs: 0,
      durationSecs: opts.crossfadeSecs,
      outgoingTargetPlaybackRate: 1.0,
      initialPlaybackRate: 1.0,
      useBassSwap: false,
      outBpm: outgoing?.bpm ?? null,
      inBpm: incoming?.bpm ?? null,
    };
  }

  // Mix Mode active: default to 11 s phrase overlap so transitions feel continuous and musical
  const baseDur = clamp(opts.crossfadeSecs > 0 ? Math.max(8, opts.crossfadeSecs) : 11, 8, 14);

  // Level 2: Attempt dual-deck BPM beatmatch & tempo morphing
  if (
    outgoing?.bpm &&
    incoming?.bpm &&
    outgoing.bpm_confidence >= MIN_BEATMATCH_CONFIDENCE &&
    incoming.bpm_confidence >= MIN_BEATMATCH_CONFIDENCE
  ) {
    const bridge = computeDualDeckTempoBridge(outgoing.bpm, incoming.bpm);
    if (bridge !== null) {
      const outBeatPeriod = 60 / outgoing.bpm;
      const inBeatPeriod = 60 / incoming.bpm;

      // Quantize mix length to a musical bar multiple (12, 16, or 24 beats)
      const beatChoices = [8, 12, 16, 24];
      let mixDur = baseDur;
      let bestDist = Infinity;
      for (const beats of beatChoices) {
        const secs = beats * outBeatPeriod;
        if (secs < 5.0 || secs > 14.5) continue;
        const dist = Math.abs(secs - baseDur);
        if (dist < bestDist) {
          bestDist = dist;
          mixDur = secs;
        }
      }
      mixDur = Math.min(mixDur, Math.max(5.0, outDur * 0.25));

      // Anchor the mix so it rides the track's real outro without chopping off the final chorus.
      // Aim for the mix to finish near the end of the outro (or outDur - 0.5s).
      const targetMixEnd = Math.min(outDur - 0.5, outgoing.outro_start_secs + mixDur * 0.75);
      const rawAnchor = targetMixEnd - mixDur;
      const minTrigger = Math.max(4, outDur - Math.min(24, outDur * 0.22));
      const maxTrigger = Math.max(minTrigger, outDur - mixDur - 0.5);
      let triggerAt = snapToNearestBeat(
        clamp(rawAnchor, minTrigger, maxTrigger),
        outgoing.beat_offset_secs,
        outBeatPeriod,
      );
      if (triggerAt > maxTrigger) {
        triggerAt = Math.max(minTrigger, triggerAt - outBeatPeriod);
      }

      const rawIntro = Math.max(0, incoming.intro_end_secs - 0.04);
      const maxIntro = Math.min(18, incoming.duration_secs * 0.18);
      const incomingStart = clamp(
        snapToNextBeat(rawIntro, incoming.beat_offset_secs, inBeatPeriod),
        0,
        maxIntro,
      );

      return {
        mode: "beatmatch",
        triggerAtSecs: Number(triggerAt.toFixed(3)),
        incomingStartSecs: Number(incomingStart.toFixed(3)),
        durationSecs: Number(mixDur.toFixed(3)),
        outgoingTargetPlaybackRate: Number(bridge.outTargetRate.toFixed(4)),
        initialPlaybackRate: Number(bridge.inStartRate.toFixed(4)),
        useBassSwap: true,
        outBpm: outgoing.bpm,
        inBpm: incoming.bpm,
      };
    }
  }

  // Level 1: Smart Mix fallback (ride natural outro without cutting off vocals)
  const mixDur = Math.min(baseDur, Math.max(4, outDur * 0.22));
  const minTrigger = Math.max(4, outDur - Math.min(22, outDur * 0.2));
  const maxTrigger = Math.max(minTrigger, outDur - mixDur - 0.4);
  const targetMixEnd = outgoing
    ? Math.min(outDur - 0.4, outgoing.outro_start_secs + mixDur * 0.75)
    : outDur - 0.4;
  const triggerAt = clamp(targetMixEnd - mixDur, minTrigger, maxTrigger);
  const incomingStart = incoming
    ? clamp(incoming.intro_end_secs, 0, Math.min(18, incoming.duration_secs * 0.18))
    : 0;

  return {
    mode: "smart",
    triggerAtSecs: Number(triggerAt.toFixed(3)),
    incomingStartSecs: Number(incomingStart.toFixed(3)),
    durationSecs: Number(mixDur.toFixed(3)),
    outgoingTargetPlaybackRate: 1.0,
    initialPlaybackRate: 1.0,
    useBassSwap: true,
    outBpm: outgoing?.bpm ?? null,
    inBpm: incoming?.bpm ?? null,
  };
}
