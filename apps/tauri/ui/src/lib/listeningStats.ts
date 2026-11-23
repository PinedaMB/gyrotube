import type { Track } from "./types";

export interface TrackPlayStat {
  track: Track;
  plays: number;
}

export interface ArtistPlayStat {
  name: string;
  id: string | null;
  plays: number;
  avatarUrl?: string | null;
}

export interface ListeningStats {
  totalSeconds: number;
  totalPlays: number;
  tracks: Record<string, TrackPlayStat>;
  artists: Record<string, ArtistPlayStat>;
}

const STORAGE_KEY = "gyrotube.listening_stats_v1";
const MAX_TRACKS = 300;
const MAX_ARTISTS = 150;

export function emptyStats(): ListeningStats {
  return { totalSeconds: 0, totalPlays: 0, tracks: {}, artists: {} };
}

export function loadListeningStats(): ListeningStats {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return emptyStats();
    const parsed = JSON.parse(raw) as Partial<ListeningStats>;
    return {
      totalSeconds: Number(parsed.totalSeconds) || 0,
      totalPlays: Number(parsed.totalPlays) || 0,
      tracks: parsed.tracks && typeof parsed.tracks === "object" ? parsed.tracks : {},
      artists: parsed.artists && typeof parsed.artists === "object" ? parsed.artists : {},
    };
  } catch {
    return emptyStats();
  }
}

export function saveListeningStats(stats: ListeningStats): void {
  try {
    // Keep storage bounded by trimming least-played entries if oversized
    const trackEntries = Object.entries(stats.tracks);
    if (trackEntries.length > MAX_TRACKS) {
      trackEntries.sort((a, b) => b[1].plays - a[1].plays);
      stats.tracks = Object.fromEntries(trackEntries.slice(0, MAX_TRACKS));
    }
    const artistEntries = Object.entries(stats.artists);
    if (artistEntries.length > MAX_ARTISTS) {
      artistEntries.sort((a, b) => b[1].plays - a[1].plays);
      stats.artists = Object.fromEntries(artistEntries.slice(0, MAX_ARTISTS));
    }
    localStorage.setItem(STORAGE_KEY, JSON.stringify(stats));
  } catch {
    // ignore quota errors
  }
}

export function recordTrackPlay(stats: ListeningStats, track: Track): ListeningStats {
  const existingTrack = stats.tracks[track.id];
  const primaryArtist = (track.artist.split(",")[0] ?? track.artist).trim() || "Unknown";
  const artistKey = primaryArtist.toLowerCase();
  const existingArtist = stats.artists[artistKey];

  return {
    totalSeconds: stats.totalSeconds,
    totalPlays: stats.totalPlays + 1,
    tracks: {
      ...stats.tracks,
      [track.id]: {
        track,
        plays: (existingTrack?.plays ?? 0) + 1,
      },
    },
    artists: {
      ...stats.artists,
      [artistKey]: {
        name: existingArtist?.name ?? primaryArtist,
        id: existingArtist?.id ?? track.artist_id ?? null,
        plays: (existingArtist?.plays ?? 0) + 1,
        avatarUrl: existingArtist?.avatarUrl ?? track.cover_url ?? null,
      },
    },
  };
}

export function topPlayedTracks(stats: ListeningStats, limit = 5): TrackPlayStat[] {
  return Object.values(stats.tracks)
    .sort((a, b) => b.plays - a.plays)
    .slice(0, limit);
}

export function topPlayedArtists(stats: ListeningStats, limit = 5): ArtistPlayStat[] {
  return Object.values(stats.artists)
    .sort((a, b) => b.plays - a.plays)
    .slice(0, limit);
}

export function formatListeningTime(totalSeconds: number, isSpanish: boolean): string {
  const mins = Math.floor(totalSeconds / 60);
  if (mins < 60) {
    return `${Math.max(1, mins)} min`;
  }
  const hours = Math.floor(mins / 60);
  const remMins = mins % 60;
  if (remMins === 0) return isSpanish ? `${hours} h` : `${hours}h`;
  return isSpanish ? `${hours} h ${remMins} min` : `${hours}h ${remMins}m`;
}
