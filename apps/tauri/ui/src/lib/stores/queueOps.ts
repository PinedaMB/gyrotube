import type { LyricsData, Track } from "../types";

export function shuffleQueueKeepingCurrent(
  queue: Track[],
  currentIndex: number,
): { queue: Track[]; unshuffledQueue: Track[]; currentIndex: number } {
  const current = queue[currentIndex];
  const rest = queue.filter((_, i) => i !== currentIndex);
  for (let i = rest.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [rest[i], rest[j]] = [rest[j], rest[i]];
  }
  return {
    queue: [current, ...rest],
    unshuffledQueue: [...queue],
    currentIndex: 0,
  };
}

export function unshuffleQueue(
  unshuffledQueue: Track[],
  currentTrack: Track,
): { queue: Track[]; currentIndex: number } {
  const idx = unshuffledQueue.findIndex((t) => t.id === currentTrack.id);
  return {
    queue: [...unshuffledQueue],
    currentIndex: idx >= 0 ? idx : 0,
  };
}

export function moveQueueIndices(
  queue: Track[],
  currentIndex: number | null,
  from: number,
  to: number,
): { queue: Track[]; currentIndex: number | null } | null {
  if (from === to || from < 0 || to < 0 || from >= queue.length || to >= queue.length) {
    return null;
  }
  const copy = [...queue];
  const [moved] = copy.splice(from, 1);
  copy.splice(to, 0, moved);

  let nextIdx = currentIndex;
  if (currentIndex === from) {
    nextIdx = to;
  } else if (currentIndex != null) {
    if (from < currentIndex && to >= currentIndex) nextIdx = currentIndex - 1;
    else if (from > currentIndex && to <= currentIndex) nextIdx = currentIndex + 1;
  }
  return { queue: copy, currentIndex: nextIdx };
}

export function removeQueueAt(
  queue: Track[],
  currentIndex: number | null,
  idx: number,
): {
  queue: Track[];
  currentIndex: number | null;
  shouldStop: boolean;
  playIndexAfter: number | null;
} | null {
  if (idx < 0 || idx >= queue.length) return null;
  const nextQueue = queue.filter((_, i) => i !== idx);
  if (currentIndex == null) {
    return { queue: nextQueue, currentIndex: null, shouldStop: false, playIndexAfter: null };
  }
  if (nextQueue.length === 0) {
    return { queue: nextQueue, currentIndex: null, shouldStop: true, playIndexAfter: null };
  }
  if (idx === currentIndex) {
    return {
      queue: nextQueue,
      currentIndex,
      shouldStop: false,
      playIndexAfter: Math.min(idx, nextQueue.length - 1),
    };
  }
  return {
    queue: nextQueue,
    currentIndex: idx < currentIndex ? currentIndex - 1 : currentIndex,
    shouldStop: false,
    playIndexAfter: null,
  };
}

export function findActiveLyricIndex(lyrics: LyricsData, posSecs: number): number | null {
  if (lyrics.type !== "Synced") return null;
  const lines = lyrics.lines;
  for (let i = lines.length - 1; i >= 0; i--) {
    if (lines[i].time_secs <= posSecs + 0.15) {
      return i;
    }
  }
  return null;
}
