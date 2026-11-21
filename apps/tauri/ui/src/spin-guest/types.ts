export interface JamTrackInfo {
  id: string;
  title: string;
  artist: string;
  album?: string | null;
  duration_secs?: number | null;
  cover_url?: string | null;
  is_current: boolean;
}

export interface JamSnapshot {
  active: boolean;
  url: string;
  host_name: string;
  allow_controls: boolean;
  is_playing: boolean;
  position_secs: number;
  now_playing: JamTrackInfo | null;
  queue: JamTrackInfo[];
  guest_count: number;
  added_count: number;
  guest_list: string[];
}

export interface SearchTrack {
  id: string;
  title: string;
  artist: string;
  album?: string | null;
  duration_secs?: number | null;
  cover_url?: string | null;
}

export function fmtTime(sec: number | null | undefined): string {
  const n = Math.max(0, Math.floor(sec || 0));
  const m = Math.floor(n / 60);
  const s = n % 60;
  return `${m}:${s < 10 ? "0" : ""}${s}`;
}
