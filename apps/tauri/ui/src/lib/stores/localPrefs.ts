import type { RecentSearchEntry } from "../types";

const KEYS = {
  recentSearches: "gyrotube.recent_searches_v1",
  pinnedPlaylists: "gyrotube.pinned_playlists_v1",
  recentPlayedAt: "gyrotube.recent_played_at_v1",
  hiddenTracks: "gyrotube.hidden_tracks_v1",
  downloadedPlaylists: "gyrotube.downloaded_playlists_v1",
  spinDisplayName: "gyrotube.spin_display_name_v1",
} as const;

function readJson<T>(key: string): T | null {
  if (typeof localStorage === "undefined") return null;
  try {
    const raw = localStorage.getItem(key);
    return raw ? (JSON.parse(raw) as T) : null;
  } catch {
    return null;
  }
}

function writeJson(key: string, value: unknown): void {
  if (typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(key, JSON.stringify(value));
  } catch {
    // ignore storage quota errors
  }
}

export function readRecentSearches(): RecentSearchEntry[] {
  const parsed = readJson<unknown>(KEYS.recentSearches);
  return Array.isArray(parsed) ? (parsed as RecentSearchEntry[]).slice(0, 10) : [];
}

export function writeRecentSearches(entries: RecentSearchEntry[]): void {
  writeJson(KEYS.recentSearches, entries);
}

export function readPinnedPlaylistIds(): string[] {
  const parsed = readJson<unknown>(KEYS.pinnedPlaylists);
  return Array.isArray(parsed)
    ? parsed.filter((x): x is string => typeof x === "string").slice(0, 4)
    : [];
}

export function writePinnedPlaylistIds(ids: string[]): void {
  writeJson(KEYS.pinnedPlaylists, ids);
}

export function readRecentPlayedAt(): Record<string, number> {
  const parsed = readJson<unknown>(KEYS.recentPlayedAt);
  return parsed && typeof parsed === "object" && !Array.isArray(parsed)
    ? (parsed as Record<string, number>)
    : {};
}

export function writeRecentPlayedAt(map: Record<string, number>): void {
  writeJson(KEYS.recentPlayedAt, map);
}

export function readHiddenTrackIds(): Set<string> {
  const parsed = readJson<unknown>(KEYS.hiddenTracks);
  return Array.isArray(parsed)
    ? new Set(parsed.filter((x): x is string => typeof x === "string"))
    : new Set();
}

export function writeHiddenTrackIds(ids: Set<string>): void {
  writeJson(KEYS.hiddenTracks, [...ids]);
}

export function readDownloadedPlaylistKeys(): Set<string> {
  const parsed = readJson<unknown>(KEYS.downloadedPlaylists);
  return Array.isArray(parsed)
    ? new Set(parsed.filter((x): x is string => typeof x === "string"))
    : new Set();
}

export function writeDownloadedPlaylistKeys(keys: Set<string>): void {
  writeJson(KEYS.downloadedPlaylists, [...keys]);
}

export function readSpinDisplayName(): string {
  if (typeof localStorage === "undefined") return "";
  try {
    return localStorage.getItem(KEYS.spinDisplayName) ?? "";
  } catch {
    return "";
  }
}

export function writeSpinDisplayName(name: string): void {
  if (typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(KEYS.spinDisplayName, name);
  } catch {
    // ignore
  }
}
