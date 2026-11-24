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
