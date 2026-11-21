import { invoke } from "@tauri-apps/api/core";
import type {
  ArtistDetails,
  Genre,
  HomeFeed,
  InitialAppData,
  LyricsData,
  Playlist,
  RelatedFeed,
  SearchResults,
  SessionState,
  SpinStatus,
  StreamInfo,
  Track,
  UserPlaylist,
} from "./types";

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

export function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__);
}

async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriRuntime()) {
    throw new Error(`Tauri runtime not available for command "${cmd}"`);
  }
  return invoke<T>(cmd, args);
}

export const api = {
  loadInitialState: (): Promise<InitialAppData> => call("load_initial_state"),
  saveSession: (session: SessionState): Promise<void> => call("save_session", { session }),
  saveFavorites: (favorites: Track[]): Promise<void> => call("save_favorites_cmd", { favorites }),
  saveRecentTracks: (tracks: Track[]): Promise<void> =>
    call("save_recent_tracks_cmd", { tracks }),
  saveUserPlaylists: (playlists: UserPlaylist[]): Promise<void> =>
    call("save_user_playlists_cmd", { playlists }),
  saveSearchHistory: (history: string[]): Promise<void> =>
    call("save_search_history_cmd", { history }),

  getHomeFeed: (forceRefresh = false, country: string | null = null): Promise<HomeFeed> =>
    call("get_home_feed", { forceRefresh, country }),
  getGenres: (forceRefresh = false): Promise<Genre[]> =>
    call("get_genres", { forceRefresh }),
  getGenrePlaylists: (genreId: string): Promise<[string, Playlist[]]> =>
    call("get_genre_playlists", { genreId }),
  searchAll: (query: string): Promise<SearchResults> => call("search_all", { query }),
  getSearchSuggestions: (query: string): Promise<string[]> =>
    call("get_search_suggestions", { query }),
  getArtist: (artistId: string, fallbackName: string): Promise<ArtistDetails> =>
    call("get_artist", { artistId, fallbackName }),
  getAlbumTracks: (albumId: string): Promise<[string, Track[]]> =>
    call("get_album_tracks", { albumId }),
  getPlaylistTracks: (playlistId: string): Promise<[string, Track[]]> =>
    call("get_playlist_tracks", { playlistId }),
  getRadioTracks: (trackId: string): Promise<Track[]> =>
    call("get_radio_tracks", { trackId }),
  getPlaylistRadio: (playlistId: string): Promise<Track[]> =>
    call("get_playlist_radio", { playlistId }),
  getRelatedForTrack: (trackId: string): Promise<RelatedFeed> =>
    call("get_related_for_track", { trackId }),
  getLyrics: (
    title: string,
    artist: string,
    durationSecs?: number | null,
    trackId?: string | null,
  ): Promise<LyricsData> =>
    call("get_lyrics", {
      trackId: trackId ?? null,
      title,
      artist,
      durationSecs: durationSecs ?? null,
    }),

  resolveAudioStream: (track: Track): Promise<StreamInfo> =>
    call("resolve_audio_stream", { track }),
  prefetchAudio: (track: Track): Promise<void> => call("prefetch_audio", { track }),
  cancelStaleStreams: (keepIds: string[]): Promise<void> =>
    call("cancel_stale_streams", { keepIds }),
  getCacheBytes: (): Promise<number> => call("get_cache_bytes"),
  clearAudioCache: (): Promise<void> => call("clear_audio_cache"),

  spinStatus: (): Promise<SpinStatus> => call("spin_status"),
  spinStartLocal: (): Promise<SpinStatus> => call("spin_start_local"),
  spinStopLocal: (): Promise<SpinStatus> => call("spin_stop_local"),
  spinSetAllowControls: (allow: boolean): Promise<SpinStatus> =>
    call("spin_set_allow_controls", { allow }),
  spinStartRemoteHost: (): Promise<SpinStatus> => call("spin_start_remote_host"),
  spinStopRemote: (): Promise<SpinStatus> => call("spin_stop_remote"),
  spinJoinRemote: (code: string): Promise<SpinStatus> => call("spin_join_remote", { code }),
  spinSyncSnapshot: (payload: {
    isPlaying: boolean;
    positionSecs: number;
    sourceName: string;
    currentIndex: number | null;
    queue: Track[];
  }): Promise<void> =>
    call("spin_sync_snapshot", {
      isPlaying: payload.isPlaying,
      positionSecs: payload.positionSecs,
      sourceName: payload.sourceName,
      currentIndex: payload.currentIndex,
      queue: payload.queue,
    }),
};
