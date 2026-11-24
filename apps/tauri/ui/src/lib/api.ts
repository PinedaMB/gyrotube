import { invoke } from "@tauri-apps/api/core";
import type {
  Artist,
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
  VideoStreamInfo,
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
  saveFollowedArtists: (artists: Artist[]): Promise<void> =>
    call("save_followed_artists_cmd", { artists }),

  getHomeFeed: (forceRefresh = false, country: string | null = null): Promise<HomeFeed> =>
    call("get_home_feed", { forceRefresh, country }),
  getGenres: (forceRefresh = false): Promise<Genre[]> =>
    call("get_genres", { forceRefresh }),
  getGenrePlaylists: (genreId: string): Promise<[string, Playlist[]]> =>
    call("get_genre_playlists", { genreId }),
  searchAll: (query: string): Promise<SearchResults> => call("search_all", { query }),
  getSearchSuggestions: (query: string): Promise<import("./types").SearchSuggestions> =>
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
  resolveVideoStream: (trackId: string, maxHeight = 720): Promise<VideoStreamInfo> =>
    call("resolve_video_stream", { trackId, maxHeight }),
  findCounterpartTrack: (track: Track, wantVideo: boolean): Promise<Track | null> =>
    call("find_counterpart_track", { track, wantVideo }),
  prefetchAudio: (track: Track): Promise<void> => call("prefetch_audio", { track }),
  cancelStaleStreams: (keepIds: string[]): Promise<void> =>
    call("cancel_stale_streams", { keepIds }),
  getCacheBytes: (): Promise<number> => call("get_cache_bytes"),
  clearAudioCache: (): Promise<void> => call("clear_audio_cache"),
  listDownloadedTracks: (): Promise<string[]> => call("list_downloaded_tracks"),
  downloadTrackOffline: (track: Track): Promise<string> =>
    call("download_track_offline", { track }),
  removeDownloadedTracks: (trackIds: string[]): Promise<void> =>
    call("remove_downloaded_tracks", { trackIds }),
  getDownloadsBytes: (): Promise<number> => call("get_downloads_bytes"),
  clearAllDownloads: (): Promise<void> => call("clear_all_downloads"),
  getOfflineCoverBaseUrl: (): Promise<string> => call("get_offline_cover_base_url"),

  spinStatus: (): Promise<SpinStatus> => call("spin_status"),
  spinStartLocal: (): Promise<SpinStatus> => call("spin_start_local"),
  spinStopLocal: (): Promise<SpinStatus> => call("spin_stop_local"),
  spinSetAllowControls: (allow: boolean): Promise<SpinStatus> =>
    call("spin_set_allow_controls", { allow }),
  spinStartRemoteHost: (): Promise<SpinStatus> => call("spin_start_remote_host"),
  spinStopRemote: (): Promise<SpinStatus> => call("spin_stop_remote"),
  spinJoinRemote: (code: string): Promise<SpinStatus> => call("spin_join_remote", { code }),
  spinRemoteControl: (action: "play_pause" | "next" | "prev"): Promise<void> =>
    call("spin_remote_control", { action }),
  spinRemoteAddTrack: (track: Track, guestName?: string): Promise<void> =>
    call("spin_remote_add_track", {
      track,
      guestName: guestName?.trim() || null,
    }),
  spinSyncSnapshot: (payload: {
    isPlaying: boolean;
    positionSecs: number;
    sourceName: string;
    currentIndex: number | null;
    queue: Track[];
    displayName?: string;
  }): Promise<void> =>
    call("spin_sync_snapshot", {
      isPlaying: payload.isPlaying,
      positionSecs: payload.positionSecs,
      sourceName: payload.sourceName,
      currentIndex: payload.currentIndex,
      queue: payload.queue,
      displayName: payload.displayName?.trim() || null,
    }),
  getAppVersion: (): Promise<string> => call("get_app_version"),
  checkForUpdates: (): Promise<{
    current_version: string;
    latest_version: string | null;
    update_available: boolean;
    can_auto_install: boolean;
    release_url: string;
    download_url: string | null;
    notes: string | null;
  }> => call("check_for_updates"),
  installUpdate: (): Promise<void> => call("install_update"),
};
