import type { LyricWord } from "./types";

/**
 * Ported from `grapedjuice/ytm-rs` (`src/ui/karaoke.rs`, Better Lyrics `@braccato/core` 1.16):
 * - 150 ms visual lead for word-synced karaoke so the sweep lands on the sung beat.
 * - Soft-edged sweep ramp (-0.2..1.4 -> -0.1..1.5) over 1.6x word duration with 10% lead.
 * - Per-letter wave float + gentle glow that decays over max(1.2x, 1.2s).
 * - Long held words (>= 1.5s) get subtle emphasis scaling.
 */
export const RICHSYNC_LEAD_SECS = 0.15;
const SWIPE_LEAD = 0.1;
const SWIPE_DURATION = 1.6;
const SWIPE_START_FROM = -0.2;
const SWIPE_START_TO = 1.4;
const SWIPE_END_FROM = -0.1;
const SWIPE_END_TO = 1.5;
const LETTER_WAVE_CREST = 0.4;
const LONG_WORD_SECS = 1.5;
const GLOW_RATIO = 1.2;
const GLOW_MIN_SECS = 1.2;
const BASE_FLOAT_SECS = 0.46;

// Joining scripts (Arabic, Syriac, Thaana, N'Ko, Mandaic) must keep whole-word sweeps
// so cursive glyph shaping isn't broken by per-character spans.
const JOINING_SCRIPT_RE = /[\u0600-\u06FF\u0700-\u077F\u0780-\u07BF\u07C0-\u07FF\u0840-\u08FF]/;

export function hasJoiningScript(text: string): boolean {
  return JOINING_SCRIPT_RE.test(text);
}

function clamp01(v: number): number {
  return v <= 0 ? 0 : v >= 1 ? 1 : v;
}

function easeInOut(t: number): number {
  const x = clamp01(t);
  return x < 0.5 ? 2 * x * x : 1 - Math.pow(-2 * x + 2, 2) / 2;
}

function easeOut(t: number): number {
  const x = clamp01(t);
  return 1 - (1 - x) * (1 - x);
}

export type WordPhase = "pending" | "active" | "settled";

/**
 * Classifies a word at `tSecs` (already shifted by user offset + `RICHSYNC_LEAD_SECS`):
 * - `pending`: before the swipe lead begins (render dim, no per-letter DOM needed).
 * - `active`: during sweep, letter wave, or glow decay (render letter-by-letter wave).
 * - `settled`: fully lit and glow decayed (render crisp lit text, no per-letter DOM needed).
 */
export function getWordPhase(word: LyricWord, tSecs: number): WordPhase {
  const dur = Math.max(0.06, word.end_secs - word.start_secs);
  const e = tSecs - word.start_secs;
  if (e < -dur * SWIPE_LEAD) return "pending";
  const glowDur = Math.max(dur * GLOW_RATIO, GLOW_MIN_SECS);
  const activeWindow = Math.max(dur * SWIPE_DURATION, glowDur) + 0.08;
  if (e > activeWindow) return "settled";
  return "active";
}

export interface LetterFrame {
  /** 0..1 lit progress (drives opacity from dim 0.34 -> 1.0) */
  lit: number;
  /** Upward float in `em` (negative = up) */
  liftEm: number;
  /** Subtle scale for held/active letters */
  scale: number;
  /** 0..1 glow intensity */
  glow: number;
}

/**
 * Computes the per-letter animation state for character `charIdx` of `charCount`
 * within `word` at `tSecs` (matching `ytm-rs` `letter_waves` + `letter_float` + `glow_radius`).
 */
export function computeLetterFrame(
  word: LyricWord,
  charIdx: number,
  charCount: number,
  tSecs: number,
  reducedMotion: boolean,
): LetterFrame {
  const dur = Math.max(0.06, word.end_secs - word.start_secs);
  const e = tSecs - word.start_secs;
  const sd = dur * SWIPE_DURATION;
  const lead = dur * SWIPE_LEAD;
  const n = Math.max(1, charCount);

  // Soft-edged sweep position in [0, 1] space across the word
  const sweepP = clamp01((e + lead) / sd);
  const edgeStart = SWIPE_START_FROM + (SWIPE_START_TO - SWIPE_START_FROM) * sweepP;
  const edgeEnd = SWIPE_END_FROM + (SWIPE_END_TO - SWIPE_END_FROM) * sweepP;

  const charCenter = (charIdx + 0.5) / n;
  const lit =
    edgeEnd > edgeStart
      ? clamp01((edgeEnd - charCenter) / (edgeEnd - edgeStart))
      : charCenter <= edgeEnd
        ? 1
        : 0;

  // Glow decays smoothly over max(1.2x, 1.2s) once the letter is lit
  const glowDur = Math.max(dur * GLOW_RATIO, GLOW_MIN_SECS);
  const glowDecay = e >= 0 && e < glowDur ? 1 - easeOut(e / glowDur) : 0;
  const glow = lit * glowDecay;

  if (reducedMotion) {
    return { lit, liftEm: 0, scale: 1, glow: glow * 0.5 };
  }

  // Per-letter wave delay & duration (matching `planLetterWave` in ytm-rs)
  const begin = clamp01((charIdx / n - SWIPE_END_FROM) / (SWIPE_END_TO - SWIPE_END_FROM)) * sd;
  const finish =
    clamp01(((charIdx + 1) / n - SWIPE_START_FROM) / (SWIPE_START_TO - SWIPE_START_FROM)) * sd;
  const letterDur = Math.max(0.02, finish - begin);
  const touched = begin - lead;
  const rise = BASE_FLOAT_SECS * LETTER_WAVE_CREST;
  const waveDelay = touched + letterDur - rise > touched ? touched : touched + letterDur - rise;
  const waveDur =
    touched + letterDur - rise > touched ? letterDur / LETTER_WAVE_CREST : BASE_FLOAT_SECS;

  const wp = (e - waveDelay) / Math.max(0.05, waveDur);
  if (wp <= 0) {
    return { lit, liftEm: 0, scale: 1, glow };
  }

  const p = Math.min(1, wp);
  const isLong = dur >= LONG_WORD_SECS;
  const crestEm = isLong ? -0.105 : -0.075;
  const settleEm = -0.02;
  const peakScale = isLong ? 1.055 : 1.02;

  if (p < LETTER_WAVE_CREST) {
    const k = easeInOut(p / LETTER_WAVE_CREST);
    return {
      lit,
      liftEm: crestEm * k,
      scale: 1 + (peakScale - 1) * k,
      glow,
    };
  }
  const k = easeOut((p - LETTER_WAVE_CREST) / (1 - LETTER_WAVE_CREST));
  return {
    lit,
    liftEm: crestEm + (settleEm - crestEm) * k,
    scale: peakScale + (1 - peakScale) * k,
    glow,
  };
}

/**
 * Soft-edged whole-word gradient sweep (for joining scripts like Arabic where
 * splitting into per-character spans breaks cursive shaping).
 */
export function computeFeatheredSweepGradient(word: LyricWord, tSecs: number): string {
  const dur = Math.max(0.06, word.end_secs - word.start_secs);
  const e = tSecs - word.start_secs;
  const p = clamp01((e + dur * SWIPE_LEAD) / (dur * SWIPE_DURATION));
  const startPct = Math.round((SWIPE_START_FROM + (SWIPE_START_TO - SWIPE_START_FROM) * p) * 100);
  const endPct = Math.round((SWIPE_END_FROM + (SWIPE_END_TO - SWIPE_END_FROM) * p) * 100);
  return `linear-gradient(90deg, var(--text) 0%, var(--text) ${startPct}%, color-mix(in srgb, var(--text) 34%, transparent) ${endPct}%, color-mix(in srgb, var(--text) 34%, transparent) 100%)`;
}

/**
 * Formats inline style for a single letter span in the active karaoke wave.
 */
export function letterInlineStyle(frame: LetterFrame, isParty: boolean): string {
  const opacity = (0.34 + 0.66 * frame.lit).toFixed(3);
  const glowPx = frame.glow > 0.03 ? (frame.glow * (isParty ? 14 : 10)).toFixed(1) : "";
  const glowAlpha = frame.glow > 0.03 ? Math.round(frame.glow * (isParty ? 62 : 48)) : 0;
  const shadow = glowPx
    ? `text-shadow: 0 0 ${glowPx}px color-mix(in srgb, var(--accent-bright) ${glowAlpha}%, transparent);`
    : "";
  const lift = Math.abs(frame.liftEm) > 0.002 ? `translate: 0 ${frame.liftEm.toFixed(3)}em;` : "";
  const scale = Math.abs(frame.scale - 1) > 0.002 ? `scale: ${frame.scale.toFixed(3)};` : "";
  return `opacity: ${opacity}; ${lift} ${scale} ${shadow}`;
}
