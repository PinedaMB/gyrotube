export interface Track {
  id: string;
  title: string;
  artist: string;
  album?: string | null;
  duration_secs?: number | null;
  cover_url?: string | null;
  cover_large_url?: string | null;
  verified_music?: boolean;
}

export interface Album {
  id: string;
  title: string;
  artist: string;
  year?: number | null;
  cover_url?: string | null;
}

export interface Playlist {
  id: string;
  name: string;
  subtitle: string;
  cover_url?: string | null;
}

export interface Artist {
  id: string;
  name: string;
  avatar_url?: string | null;
  subscriber_count?: number | null;
}

export interface ArtistDetails {
  name: string;
  header_url?: string | null;
  description?: string | null;
  subscriber_count?: number | null;
  tracks: Track[];
  albums: Album[];
  playlists: Playlist[];
  similar_artists: Artist[];
}

export interface Genre {
  id: string;
  name: string;
  color_hex: string;
}

export interface HomeFeed {
  trending: Track[];
  top: Track[];
  new_albums: Album[];
}

export interface SearchResults {
  artists: Artist[];
  albums: Album[];
  playlists: Playlist[];
  tracks: Track[];
}

export interface StreamInfo {
  url: string;
  quality_label: string;
}

export interface SyncedLyricLine {
  time_secs: number;
  text: string;
}

export type LyricsData =
  | { type: "Synced"; lines: SyncedLyricLine[] }
  | { type: "Plain"; text: string }
  | { type: "None" };

export interface UserPlaylist {
  id: string;
  name: string;
  description: string;
  color_hex?: string | null;
  icon_name?: string | null;
  cover_path?: string | null;
  tracks: Track[];
}

export type ThemeMode = "system" | "dark" | "light";
export type AppLanguage = "Auto" | "Es" | "En";
export type MainTab = "home" | "explore" | "collection";
export type ActiveView =
  | { kind: "home" }
  | { kind: "explore" }
  | { kind: "collection" }
  | { kind: "artist"; artistId: string; artistName: string }
  | { kind: "album"; id: string; title: string; coverUrl?: string | null; isPlaylist?: boolean };

export interface SessionState {
  sidebar_collapsed: boolean;
  queue: Track[];
  unshuffled_queue: Track[];
  current_index: number | null;
  source_name: string;
  user_volume: number;
  shuffle: boolean;
  repeat: boolean;
  position_secs: number;
  crossfade_secs: number;
  language: AppLanguage;
  background_playback: boolean;
  autoplay_related: boolean;
  theme_mode: ThemeMode;
}

export interface InitialAppData {
  session: SessionState;
  favorites: Track[];
  recent_tracks: Track[];
  user_playlists: UserPlaylist[];
  search_history: string[];
}

export interface SpinStatus {
  local_active: boolean;
  local_url: string;
  local_qr_svg: string | null;
  allow_controls: boolean;
  guest_count: number;
  added_count: number;
  remote_role: "idle" | "host" | "guest";
  remote_ticket: string | null;
  remote_qr_svg: string | null;
}

export function formatDuration(secs?: number | null): string {
  if (secs == null || !Number.isFinite(secs) || secs < 0) return "--:--";
  const total = Math.floor(secs);
  const mins = Math.floor(total / 60);
  const rem = total % 60;
  return `${mins}:${rem.toString().padStart(2, "0")}`;
}

export function formatSubscribers(count?: number | null): string {
  if (!count) return "";
  if (count >= 1_000_000) return `${(count / 1_000_000).toFixed(1)} M subscribers`;
  if (count >= 1_000) return `${(count / 1_000).toFixed(1)} K subscribers`;
  return `${count} subscribers`;
}
