import type { LyricsData } from "./types";

export const SECTION_HEADER_RE = /^\[[^\]]{1,40}\]$/;

export interface PlainStanzaLine {
  text: string;
  isHeader: boolean;
  selectableIdx: number | null;
}

export interface ParsedPlainLyrics {
  stanzas: PlainStanzaLine[][];
  selectable: string[];
}

export function parsePlainStanzas(text: string): ParsedPlainLyrics {
  const selectable: string[] = [];
  const rawBlocks = text
    .trim()
    .split(/\r?\n\s*\r?\n+/)
    .map((block) =>
      block
        .split(/\r?\n/)
        .map((l) => l.trim())
        .filter((l) => l.length > 0),
    )
    .filter((lines) => lines.length > 0);

  const stanzas: PlainStanzaLine[][] = rawBlocks.map((block) =>
    block.map((line) => {
      if (SECTION_HEADER_RE.test(line)) {
        return { text: line.slice(1, -1).trim(), isHeader: true, selectableIdx: null };
      }
      const selectableIdx = selectable.length;
      selectable.push(line);
      return { text: line, isHeader: false, selectableIdx };
    }),
  );

  return { stanzas, selectable };
}

export function isWaitingForFirstLyric(
  lyrics: LyricsData,
  activeIndex: number | null,
  positionSecs: number,
): boolean {
  if (lyrics.type !== "Synced" || lyrics.lines.length === 0) return false;
  const first = lyrics.lines[0];
  return activeIndex == null && first.time_secs >= 3 && positionSecs < first.time_secs;
}

/**
 * Asymmetric forward-reading opacity curve: upcoming lines stay clearer
 * than already-sung lines so the listener can comfortably read ahead.
 */
export function computeLyricOpacity(idx: number, activeIndex: number | null): number {
  if (activeIndex == null) {
    if (idx === 0) return 0.84;
    if (idx === 1) return 0.56;
    return 0.34;
  }
  if (idx === activeIndex) return 1.0;
  const delta = idx - activeIndex;
  if (delta === 1) return 0.58;
  if (delta === 2) return 0.38;
  if (delta > 2) return 0.24;
  if (delta === -1) return 0.34;
  return 0.2;
}

export const MAX_SELECTED_LYRIC_LINES = 5;
const INSTRUMENTAL_ONLY_RE = /^[\s♪♫♬♩•·…\-—]+$/;

export function isSelectableLyricText(text: string): boolean {
  return text.trim().length > 0 && !INSTRUMENTAL_ONLY_RE.test(text);
}

export function buildContiguousLyricRange(
  from: number,
  to: number,
  isSelectable: (idx: number) => boolean,
): number[] {
  const start = Math.min(from, to);
  const end = Math.max(from, to);
  const out: number[] = [];
  for (let i = start; i <= end; i++) {
    if (isSelectable(i)) out.push(i);
  }
  return out;
}

export function toggleContiguousLyricSelection(
  current: number[],
  idx: number,
  isSelectable: (idx: number) => boolean,
  maxLines = MAX_SELECTED_LYRIC_LINES,
): number[] {
  if (!isSelectable(idx)) return current;
  if (current.length === 0) return [idx];

  const sorted = [...current].sort((a, b) => a - b);
  const min = sorted[0];
  const max = sorted[sorted.length - 1];

  if (sorted.includes(idx)) {
    if (sorted.length === 1) return [];
    if (idx === min) return sorted.slice(1);
    if (idx === max) return sorted.slice(0, -1);
    return sorted.slice(0, sorted.indexOf(idx) + 1);
  }

  const candidate = buildContiguousLyricRange(Math.min(min, idx), Math.max(max, idx), isSelectable);
  return candidate.length <= maxLines ? candidate : [idx];
}
