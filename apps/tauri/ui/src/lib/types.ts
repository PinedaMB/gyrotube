export interface Track {
  id: string;
  title: string;
  artist: string;
  artist_id?: string | null;
  album?: string | null;
  album_id?: string | null;
  duration_secs?: number | null;
  view_count?: number | null;
  cover_url?: string | null;
  cover_large_url?: string | null;
  verified_music?: boolean;
  is_video?: boolean;
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
  singles?: Album[];
  videos?: Track[];
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
  top_artists?: Artist[];
  country?: string | null;
  available_countries?: string[];
}

export interface RelatedFeed {
  tracks: Track[];
  albums: Album[];
  playlists: Playlist[];
  artists: Artist[];
}

export interface SearchResults {
  artists: Artist[];
  albums: Album[];
  playlists: Playlist[];
  tracks: Track[];
}

export interface SearchSuggestions {
  terms: string[];
  tracks: Track[];
  artists: Artist[];
  albums: Album[];
}

export type RecentSearchEntry =
  | { kind: "query"; key: string; term: string }
  | { kind: "artist"; key: string; artist: Artist }
  | { kind: "track"; key: string; track: Track }
  | { kind: "album"; key: string; album: Album };

export interface StreamInfo {
  url: string;
  quality_label: string;
}

export interface VideoStreamInfo {
  url: string;
  quality_label: string;
  width: number;
  height: number;
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
export type AudioQuality = "High" | "Normal" | "Low";
export type MainTab = "home" | "explore" | "collection";
export type CollectionSubView =
  | { kind: "favorites" }
  | { kind: "playlist"; id: string };

export type ActiveView =
  | { kind: "home" }
  | { kind: "explore" }
  | { kind: "search"; query?: string }
  | { kind: "collection"; sub?: CollectionSubView }
  | { kind: "settings" }
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
  audio_quality?: AudioQuality;
}

export interface InitialAppData {
  session: SessionState;
  favorites: Track[];
  recent_tracks: Track[];
  user_playlists: UserPlaylist[];
  search_history: string[];
  followed_artists?: Artist[];
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
  remote_guest_count: number;
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

export function formatPlays(count?: number | null): string {
  if (!count) return "";
  if (count >= 1_000_000_000) return `${(count / 1_000_000_000).toFixed(1)} B`;
  if (count >= 1_000_000) return `${(count / 1_000_000).toFixed(1)} M`;
  if (count >= 1_000) return `${(count / 1_000).toFixed(1)} K`;
  return `${count}`;
}
