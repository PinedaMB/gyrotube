import { getAndroidBridge, getAndroidWindow } from "../androidBridge";
import { api, isTauriRuntime } from "../api";
import { DualDeckAudioEngine, extractDominantRgb } from "../audioEngine";
import { NativeRustAudioEngine, type AudioEngineBackend } from "../nativeAudioEngine";
import { t } from "../i18n.svelte";
import {
  EQ_PRESETS,
  loadEqSettings,
  saveEqSettings,
  type EqPresetId,
  type EqSettings,
} from "../equalizer";
import {
  emptyStats,
  loadListeningStats,
  recordTrackPlay,
  saveListeningStats,
  type ListeningStats,
} from "../listeningStats";
import { hashToView, isSameView, type RouteState, viewToHash } from "../router";
import { parseYtmUrl } from "../ytmUrl";
import {
  readDownloadedPlaylistKeys,
  readHiddenTrackIds,
  readIntegrationsSettings,
  readLyricsAlign,
  readLyricsFontSize,
  readLyricsOffsets,
  readPinnedPlaylistIds,
  readRecentPlayedAt,
  readRecentSearches,
  readShowRomanization,
  readShowTranslation,
  readSpinDisplayName,
  readSponsorBlockEnabled,
  readTranslationLang,
  writeDownloadedPlaylistKeys,
  writeHiddenTrackIds,
  writeIntegrationsSettings,
  writeLyricsAlign,
  writeLyricsFontSize,
  writeLyricsOffsets,
  writePinnedPlaylistIds,
  writeRecentPlayedAt,
  writeRecentSearches,
  writeShowRomanization,
  writeShowTranslation,
  writeSpinDisplayName,
  writeSponsorBlockEnabled,
  writeTranslationLang,
} from "./localPrefs";
import { contextMenu } from "./contextMenu.svelte";
import { device } from "./device.svelte";
import { parseLibraryBackupImport, serializeLibraryBackup } from "./libraryBackup";
import { bindTauriPlayerEvents } from "./nativeEvents";
import { createYtmTrackStub, formatPlayerErrorMessage } from "./playerHelpers";
import {
  findActiveLyricIndex,
  moveQueueIndices,
  removeQueueAt,
  shuffleQueueKeepingCurrent,
  unshuffleQueue,
} from "./queueOps";
import {
  applyThemeToDocument,
  readChartCountry,
  readInitialSidebarOpen,
  readNavCollapsed,
  readThemeMode,
  readThemePalette,
  SessionPersister,
  writeChartCountry,
  writeNavCollapsed,
  writeSidebarCollapsed,
} from "./sessionPersistence";
import type {
  ActiveView,
  AppLanguage,
  Artist,
  AudioQuality,
  Genre,
  HomeFeed,
  IntegrationsSettings,
  LyricsAlign,
  LyricsData,
  LyricsFontSize,
  LyricsSourceOption,
  MainTab,
  RecentSearchEntry,
  RelatedFeed,
  SearchResults,
  SessionState,
  SpinStatus,
  SponsorSegment,
  ThemeMode,
  ThemePalette,
  Track,
  UserPlaylist,
  VideoStreamInfo,
} from "../types";

export interface ToastItem {
  id: number;
  message: string;
}

class PlayerStore {
  // Navigation & layout
  activeTab = $state<MainTab>("home");
  activeView = $state<ActiveView>({ kind: "home" });
  sidebarOpen = $state(readInitialSidebarOpen());
  navCollapsed = $state(readNavCollapsed());
  sidebarTab = $state<"player" | "lyrics" | "queue" | "related">("player");
  chartCountry = $state<string | null>(readChartCountry());
  relatedFeed = $state<RelatedFeed | null>(null);
  relatedTrackId = $state<string | null>(null);
  relatedLoading = $state(false);
  spinOpen = $state(false);
  partyMode = $state(false);
  playlistDraftTrack = $state<Track | null>(null);

  // Playback state
  queue = $state<Track[]>([]);
  unshuffledQueue = $state<Track[]>([]);
  currentIndex = $state<number | null>(null);
  sourceName = $state("Not playing");
  isPlaying = $state(false);
  isLoadingAudio = $state(false);
  positionSecs = $state(0);
  durationSecs = $state(0);
  qualityLabel = $state("YTM");
  volume = $state(1.0);
  shuffle = $state(false);
  repeat = $state(false);
  crossfadeSecs = $state(0);
  autoplayRelated = $state(true);
  backgroundPlayback = $state(true);
  audioQuality = $state<AudioQuality>("High");
  themeMode = $state<ThemeMode>(readThemeMode());
  themePalette = $state<ThemePalette>(readThemePalette());
  sponsorBlockEnabled = $state<boolean>(readSponsorBlockEnabled());
  language = $state<AppLanguage>("Auto");

  // Dynamic ambient color extracted from current cover
  ambientRgb = $state<[number, number, number] | null>(null);

  // Lyrics & per-track timing offsets
  lyrics = $state<LyricsData>({ type: "None" });
  lyricsLoading = $state(false);
  activeLyricIndex = $state<number | null>(null);
  lyricsOffsets = $state<Record<string, number>>(readLyricsOffsets());
  showRomanization = $state<boolean>(readShowRomanization());
  showTranslation = $state<boolean>(readShowTranslation());
  translationLangPref = $state<"es" | "en" | null>(readTranslationLang());
  lyricsFontSize = $state<LyricsFontSize>(readLyricsFontSize());
  lyricsAlign = $state<LyricsAlign>(readLyricsAlign());
  translatedLines = $state<string[] | null>(null);
  translationLoading = $state(false);
  lyricsSources = $state<LyricsSourceOption[]>([]);
  lyricsSourcesLoading = $state(false);
  activeLyricsSourceId = $state<string | null>(null);
  private lyricsSourcesTrackId: string | null = null;
  private translationGen = 0;

  // Opt-in desktop / scrobbling integrations
  integrations = $state<IntegrationsSettings>(readIntegrationsSettings());
  private scrobbleTrackId: string | null = null;
  private scrobbleStartedAt = 0;
  private scrobbleListenedSecs = 0;
  private scrobbleSubmitted = false;
  private scrobbleNowPlayingSent = false;

  // SponsorBlock segments for current track
  sponsorSegments = $state<SponsorSegment[]>([]);
  private skippedSponsorKeys = new Set<string>();

  // Related / suggested tracks for Autoplay
  suggestedTracks = $state<Track[]>([]);

  // User data
  favorites = $state<Track[]>([]);
  recentTracks = $state<Track[]>([]);
  userPlaylists = $state<UserPlaylist[]>([]);
  searchHistory = $state<string[]>([]);
  recentSearches = $state<RecentSearchEntry[]>(readRecentSearches());
  pinnedPlaylistIds = $state<string[]>(readPinnedPlaylistIds());
  recentPlayedAt = $state<Record<string, number>>(readRecentPlayedAt());
  hiddenTrackIds = $state<Set<string>>(readHiddenTrackIds());
  followedArtists = $state<Artist[]>([]);

  // Video mode (temporarily disabled — flip to `true` to re-enable video playback)
  public readonly videoFeatureEnabled = false;
  videoMode = $state(false);
  videoStream = $state<VideoStreamInfo | null>(null);
  videoLoading = $state(false);
  videoBuffering = $state(false);
  videoError = $state<string | null>(null);
  seekSeq = $state(0);
  counterpartTrack = $state<Track | null>(null);
  private videoGen = 0;
  private counterpartGen = 0;
  private counterpartCache = new Map<string, Track | null>();
  private awaitingVideoStartGen: number | null = null;
  private audioLoadedForGen = 0;
  private videoReadyEl: { gen: number; el: HTMLVideoElement } | null = null;
  private videoStartFallbackTimer: number | null = null;
  private videoSyncPauseActive = false;
  private videoSyncFallbackTimer: number | null = null;

  // Catalog data
  homeFeed = $state<HomeFeed | null>(null);
  homeLoading = $state(false);
  genres = $state<Genre[]>([]);
  genresLoading = $state(false);
  searchQuery = $state("");
  searchResults = $state<SearchResults | null>(null);
  searchLoading = $state(false);
  private searchCache = new Map<string, SearchResults>();
  private searchGen = 0;

  // SPIN status
  spinStatus = $state<SpinStatus>({
    local_active: false,
    local_url: "",
    local_qr_svg: null,
    allow_controls: false,
    guest_count: 0,
    added_count: 0,
    remote_role: "idle",
    remote_ticket: null,
    remote_qr_svg: null,
    remote_guest_count: 0,
  });
  spinDisplayName = $state<string>(readSpinDisplayName());
  spinRemoteConn = $state<{
    status: "idle" | "connecting" | "connected" | "reconnecting";
    hostName: string;
    allowControls: boolean;
  }>({
    status: "idle",
    hostName: "",
    allowControls: false,
  });

  // Network & offline downloads (Spotify-style auto-sync per playlist)
  isOffline = $state(typeof navigator !== "undefined" ? !navigator.onLine : false);
  downloadedTrackIds = $state<Set<string>>(new Set());
  downloadedPlaylistKeys = $state<Set<string>>(readDownloadedPlaylistKeys());
  downloadingTrackIds = $state<Set<string>>(new Set());
  offlineCoverBaseUrl = $state<string | null>(null);
  downloadingPlaylistKey = $state<string | null>(null);
  downloadProgress = $state<{ done: number; total: number } | null>(null);
  private downloadQueue: Track[] = [];
  private downloadWorkerRunning = false;

  private persistRecentSearches(): void {
    writeRecentSearches(this.recentSearches);
  }

  private persistPinnedPlaylistIds(): void {
    writePinnedPlaylistIds(this.pinnedPlaylistIds);
  }

  private persistRecentPlayedAt(): void {
    writeRecentPlayedAt(this.recentPlayedAt);
  }

  private persistHiddenTrackIds(): void {
    writeHiddenTrackIds(this.hiddenTrackIds);
  }

  private persistDownloadedPlaylistKeys(): void {
    writeDownloadedPlaylistKeys(this.downloadedPlaylistKeys);
  }

  // Equalizer & Listening stats (local-only)
  eqSettings = $state<EqSettings>(loadEqSettings());
  listeningStats = $state<ListeningStats>(loadListeningStats());
  private statsTrackId: string | null = null;
  private statsRecordedForTrack = false;
  private statsLastPos = 0;
  private statsUnsavedSecs = 0;

  // Toasts
  toasts = $state<ToastItem[]>([]);
  private toastSeq = 0;
  private loadGen = 0;
  private inFlightTransitionGen: number | null = null;
  private advancedFromGen = 0;
  private prefetchTimer: number | null = null;
  private lastLocalSpinUpdate = 0;
  private lastPositionPersistAt = 0;
  private persister = new SessionPersister();
  private routeDepth = 0;
  private routerBound = false;
  private engine: AudioEngineBackend;
  private audioDuckFactor = 1.0;

  currentTrack = $derived(
    this.currentIndex != null && this.currentIndex >= 0 && this.currentIndex < this.queue.length
      ? this.queue[this.currentIndex]
      : null,
  );

  nextTrack = $derived(
    this.currentIndex != null && this.currentIndex + 1 < this.queue.length
      ? this.queue[this.currentIndex + 1]
      : null,
  );

  currentLyricsOffset = $derived(
    this.currentTrack ? (this.lyricsOffsets[this.currentTrack.id] ?? 0) : 0,
  );

  private favoriteIdSet = $derived(new Set(this.favorites.map((t) => t.id)));
  private followedArtistIdSet = $derived(new Set(this.followedArtists.map((a) => a.id)));

  constructor() {
    const callbacks = {
      onTimeUpdate: (pos: number, dur: number) => {
        // While a new track is resolving, ignore trailing timeupdate ticks from the outgoing deck
        if (this.inFlightTransitionGen === this.loadGen && this.isLoadingAudio) {
          return;
        }
        const minDelta = !this.videoMode && this.lyrics.type === "Synced" ? 0.1 : 0.25;
        if (Math.abs(pos - this.positionSecs) >= minDelta || pos === 0) {
          this.positionSecs = pos;
        }
        if (dur > 0 && Math.abs(dur - this.durationSecs) >= 0.5) {
          this.durationSecs = dur;
          const cur = this.currentTrack;
          if (cur && cur.duration_secs == null) {
            this.backfillTrackDuration(cur.id, Math.round(dur));
          }
          this.syncAndroidForegroundService(this.isPlaying);
        }
        if (!this.videoMode) {
          this.updateActiveLyric(pos);
        }
        this.checkSponsorBlockSkip(pos, dur > 0 ? dur : this.durationSecs);
        this.trackListeningProgress(pos);
        const now = performance.now();
        if (this.isPlaying && !this.isLoadingAudio && pos > 1 && now - this.lastPositionPersistAt >= 5000) {
          this.lastPositionPersistAt = now;
          void this.persistSession();
        }
        if (this.spinStatus.local_active && this.isPlaying && !this.isLoadingAudio && now - this.lastLocalSpinUpdate >= 500) {
          this.lastLocalSpinUpdate = now;
          void this.syncSpinSnapshot();
        }
      },
      onPlayStateChange: (playing: boolean) => {
        this.isPlaying = playing;
        if (playing) this.isLoadingAudio = false;
        this.syncAndroidForegroundService(playing);
        void this.syncSpinSnapshot();
        void this.syncMprisState();
      },
      onEnded: () => {
        if (
          this.inFlightTransitionGen === this.loadGen ||
          this.advancedFromGen === this.loadGen
        ) {
          // A transition from this track is already in flight or already ran;
          // never double-advance, skip a track, or append a duplicate autoplay track.
          return;
        }
        this.advancedFromGen = this.loadGen;
        this.isPlaying = false;
        void this.syncSpinSnapshot();
        if (this.spinStatus.remote_role !== "guest") void this.handleTrackEnded();
      },
      onCrossfadeTrigger: () => {
        if (
          this.inFlightTransitionGen === this.loadGen ||
          this.advancedFromGen === this.loadGen
        ) {
          return;
        }
        if (this.spinStatus.remote_role !== "guest") {
          this.advancedFromGen = this.loadGen;
          void this.next(true);
        }
      },
      onError: (msg: string) => {
        this.isLoadingAudio = false;
        this.showToast(msg);
      },
    };
    this.engine =
      isTauriRuntime() && !device.isAndroid
        ? new NativeRustAudioEngine(callbacks)
        : new DualDeckAudioEngine(callbacks);
    this.engine.applyEqSettings(this.eqSettings);
  }

  public async bootstrap(): Promise<void> {
    this.applyTheme(this.themeMode);
    this.initRouter();
    if (!isTauriRuntime()) {
      return;
    }
    try {
      const initial = await api.loadInitialState();
      this.favorites = initial.favorites;
      this.recentTracks = initial.recent_tracks;
      this.userPlaylists = initial.user_playlists;
      this.searchHistory = initial.search_history;
      if (this.recentSearches.length === 0 && initial.search_history.length > 0) {
        this.recentSearches = initial.search_history.slice(0, 10).map((term) => ({
          kind: "query",
          key: `q:${term.toLowerCase()}`,
          term,
        }));
        this.persistRecentSearches();
      }
      this.followedArtists = initial.followed_artists ?? [];

      const s = initial.session;
      this.sidebarOpen = !s.sidebar_collapsed;
      writeSidebarCollapsed(s.sidebar_collapsed);
      this.queue = s.queue;
      this.unshuffledQueue = s.unshuffled_queue;
      this.currentIndex = s.current_index;
      this.sourceName = s.source_name || "Not playing";
      this.volume = device.isAndroid ? 1.0 : (s.user_volume ?? 1.0);
      this.shuffle = s.shuffle;
      this.repeat = s.repeat;
      this.crossfadeSecs = s.crossfade_secs ?? 0;
      this.autoplayRelated = s.autoplay_related ?? true;
      this.audioQuality =
        s.audio_quality === "Normal" || s.audio_quality === "Low" ? s.audio_quality : "High";
      this.backgroundPlayback = s.background_playback ?? true;
      this.language = s.language ?? "Auto";
      this.themeMode = (s.theme_mode as ThemeMode) || "system";

      // On compact / mobile viewports, keep the player sheet closed on startup so Home is visible
      if (typeof window !== "undefined" && window.innerWidth < 1024) {
        this.sidebarOpen = false;
      }

      this.engine.setVolume(this.volume);
      this.engine.setCrossfadeSecs(this.crossfadeSecs);
      this.applyTheme(this.themeMode);

      // Restore metadata, ambient color, and lyrics for the saved track (without auto-playing)
      const restored = this.currentTrack;
      if (restored) {
        this.engine.updateMediaSession(restored, {
          onPlay: () => void this.play(),
          onPause: () => this.pause(),
          onPrev: () => void this.prev(),
          onNext: () => void this.next(),
          onSeek: (s) => this.seek(s),
        });
        void this.syncMprisState();
        this.videoMode = this.videoFeatureEnabled && Boolean(restored.is_video);
        this.probeCounterpartForCurrent(restored);
        void api.prefetchAudio(restored).catch(() => {
          // Playback retries if the background preparation fails.
        });
        this.durationSecs = restored.duration_secs ?? 0;
        const savedPos = Number.isFinite(s.position_secs) ? Math.max(0, s.position_secs) : 0;
        this.positionSecs =
          this.durationSecs > 2 && savedPos >= this.durationSecs - 2 ? 0 : savedPos;
        const cover = restored.cover_large_url || restored.cover_url;
        if (cover) {
          void extractDominantRgb(cover).then((rgb) => {
            if (this.currentTrack?.id === restored.id) this.ambientRgb = rgb;
          });
        }
        this.lyricsLoading = true;
        void api
          .getLyrics(restored.title, restored.artist, restored.duration_secs, restored.id)
          .then((data) => {
            if (this.currentTrack?.id === restored.id) {
              this.lyrics = data;
              if (this.positionSecs > 0) this.updateActiveLyric(this.positionSecs);
            }
          })
          .finally(() => {
            if (this.currentTrack?.id === restored.id) this.lyricsLoading = false;
          });
      }
      if (typeof window !== "undefined") {
        const flushPosition = () => {
          if (this.currentTrack && this.positionSecs > 0) {
            this.flushSessionNow();
          }
        };
        window.addEventListener("pagehide", flushPosition);
        window.addEventListener("beforeunload", flushPosition);
      }
    } catch (err) {
      this.showToast(String(err));
    }
    void this.loadHome();
    void this.loadGenres();
    void this.refreshDownloadedTracks();
    void this.refreshSpinStatus();
    void this.bindNativeEvents();
    this.bindAndroidSpinDeepLink();
  }

  private bindAndroidSpinDeepLink(): void {
    const win = getAndroidWindow();
    if (!win) return;
    win.__GYROTUBE_SPIN_LINK__ = (code: string) => {
      void this.joinSpinFromDeepLink(code);
    };
    const pending = getAndroidBridge()?.consumePendingSpinLink?.();
    if (pending && pending.trim()) {
      void this.joinSpinFromDeepLink(pending.trim());
    }
  }

  public setSpinDisplayName(name: string): void {
    const clean = name.slice(0, 28);
    this.spinDisplayName = clean;
    writeSpinDisplayName(clean);
    if (this.spinStatus.local_active || this.spinStatus.remote_role === "host") {
      void this.syncSpinSnapshot();
    }
  }

  public async joinSpinRemote(code: string): Promise<void> {
    const clean = code.trim();
    if (!clean || !isTauriRuntime()) return;
    this.spinStatus = await api.spinJoinRemote(clean);
    this.spinRemoteConn = {
      status: "connecting",
      hostName: "",
      allowControls: false,
    };
  }

  public async disconnectSpinRemote(): Promise<void> {
    if (!isTauriRuntime()) return;
    this.spinStatus = await api.spinStopRemote();
    this.spinRemoteConn = {
      status: "idle",
      hostName: "",
      allowControls: false,
    };
  }

  public async joinSpinFromDeepLink(code: string): Promise<void> {
    const clean = code.trim();
    if (!clean || !isTauriRuntime()) return;
    try {
      await this.joinSpinRemote(clean);
      this.showToast(t("toast.spin_connecting"));
    } catch (err) {
      this.showToast(String(err));
    }
  }

  private async bindNativeEvents(): Promise<void> {
    await bindTauriPlayerEvents({
      onPlay: () => void this.play(),
      onPause: () => this.pause(),
      onTogglePlay: () => void this.togglePlay(),
      onStop: () => this.pause(),
      onNext: () => void this.next(),
      onPrev: () => void this.prev(),
      onSeekTo: (secs) => this.seek(secs),
      onSeekBy: (offsetSecs) => {
        const cur = this.getExactPositionSecs();
        const dur = this.durationSecs || this.currentTrack?.duration_secs || 0;
        const next = dur > 0 ? Math.max(0, Math.min(dur, cur + offsetSecs)) : Math.max(0, cur + offsetSecs);
        this.seek(next);
      },
      onSetShuffle: (shuffle) => {
        if (this.shuffle !== shuffle) this.toggleShuffle();
      },
      onSetRepeat: (repeat) => {
        if (this.repeat !== repeat) this.toggleRepeat();
      },
      onSetVolume: (volume) => this.setVolume(volume),
      onMoveQueue: (from, to) => this.moveQueueItem(from, to),
      onEnqueueTrack: (track, guestName) => {
        this.enqueueTrack(track, true);
        this.showToast(t("toast.spin_guest_added", { guest: guestName, title: track.title }));
      },
      onRemoteSync: (payload, error) => {
        if (error) {
          if (this.spinStatus.remote_role === "guest") {
            this.spinRemoteConn = {
              ...this.spinRemoteConn,
              status: "reconnecting",
            };
          }
          this.showToast(error);
          return;
        }
        if (!payload) return;
        this.spinRemoteConn = {
          status: "connected",
          hostName: payload.host_name || "Host",
          allowControls: Boolean(payload.allow_controls),
        };
        if (!payload.track) {
          ++this.loadGen;
          this.currentIndex = null;
          this.queue = [];
          this.unshuffledQueue = [];
          this.isLoadingAudio = false;
          this.engine.stop();
          this.lyrics = { type: "None" };
          this.activeLyricIndex = null;
          return;
        }
        const driftMs = Math.max(0, Date.now() - payload.sent_at_ms);
        const targetPos = payload.position_secs + (payload.is_playing ? driftMs / 1000 : 0);
        const trackChanged = this.currentTrack?.id !== payload.track.id;
        const currentTrack = { ...payload.track, is_music: false };
        const upcoming = (payload.queue ?? []).map((item) => ({ ...item, is_music: false }));
        const syncedQueue = [currentTrack, ...upcoming];
        if (trackChanged || (!this.engine.activeDeck.src && !this.isLoadingAudio)) {
          this.engine.stop();
          this.queue = syncedQueue;
          this.unshuffledQueue = syncedQueue;
          this.sourceName = `SPIN · ${payload.host_name || "Host"}`;
          void this.playIndex(0, false, targetPos, !payload.is_playing);
        } else {
          // Keep upcoming queue synced even while the current song stays the same
          const queueIdsChanged =
            this.queue.length !== syncedQueue.length ||
            this.queue.some((item, idx) => item.id !== syncedQueue[idx]?.id);
          if (queueIdsChanged) {
            this.queue = syncedQueue;
            this.unshuffledQueue = syncedQueue;
          }
          if (!this.isLoadingAudio) {
            const deck = this.engine.activeDeck;
            if (
              deck.readyState >= 1 &&
              !deck.seeking &&
              Math.abs(deck.currentTime - targetPos) > (payload.is_playing ? 1.8 : 0.35)
            ) {
              this.seek(targetPos);
            }
            if (!payload.is_playing) {
              deck.pause();
            } else if (deck.paused) {
              void deck.play().catch((err) => this.showToast(String(err)));
            }
          }
        }
      },
    });
  }

  public applyTheme(mode: ThemeMode): void {
    this.themeMode = mode;
    applyThemeToDocument(mode, this.themePalette);
  }

  public setThemeMode(mode: ThemeMode): void {
    this.applyTheme(mode);
    void this.persistSession();
  }

  public setThemePalette(palette: ThemePalette): void {
    this.themePalette = palette;
    applyThemeToDocument(this.themeMode, palette);
  }

  public setSponsorBlockEnabled(enabled: boolean): void {
    this.sponsorBlockEnabled = enabled;
    writeSponsorBlockEnabled(enabled);
    if (!enabled) {
      this.sponsorSegments = [];
      this.skippedSponsorKeys.clear();
    } else if (this.currentTrack) {
      this.loadSponsorBlockForCurrent(this.loadGen);
    }
  }

  public adjustLyricsOffset(deltaSecs: number): void {
    const tid = this.currentTrack?.id;
    if (!tid) return;
    const current = this.lyricsOffsets[tid] ?? 0;
    const next = Math.max(-10, Math.min(10, Math.round((current + deltaSecs) * 10) / 10));
    const updated = { ...this.lyricsOffsets };
    if (next === 0) {
      delete updated[tid];
    } else {
      updated[tid] = next;
    }
    this.lyricsOffsets = updated;
    writeLyricsOffsets(updated);
    this.updateActiveLyric(this.positionSecs);
  }

  public resetLyricsOffset(): void {
    const tid = this.currentTrack?.id;
    if (!tid || !(tid in this.lyricsOffsets)) return;
    const updated = { ...this.lyricsOffsets };
    delete updated[tid];
    this.lyricsOffsets = updated;
    writeLyricsOffsets(updated);
    this.updateActiveLyric(this.positionSecs);
  }

  public setShowRomanization(show: boolean): void {
    this.showRomanization = show;
    writeShowRomanization(show);
  }

  public setLyricsFontSize(size: LyricsFontSize): void {
    this.lyricsFontSize = size;
    writeLyricsFontSize(size);
  }

  public setLyricsAlign(align: LyricsAlign): void {
    this.lyricsAlign = align;
    writeLyricsAlign(align);
  }

  public setShowTranslation(show: boolean): void {
    this.showTranslation = show;
    writeShowTranslation(show);
    if (show) {
      void this.loadTranslationForCurrent(true);
    } else {
      this.translatedLines = null;
      this.translationLoading = false;
    }
  }

  public setTranslationLang(lang: "es" | "en"): void {
    this.translationLangPref = lang;
    writeTranslationLang(lang);
    this.translatedLines = null;
    if (this.showTranslation) {
      void this.loadTranslationForCurrent(true);
    }
  }

  public getTranslationTargetLang(): "es" | "en" {
    if (this.translationLangPref) return this.translationLangPref;
    if (this.language === "Es") return "es";
    if (this.language === "En") return "en";
    if (typeof navigator !== "undefined" && navigator.language?.toLowerCase().startsWith("es")) {
      return "es";
    }
    return "en";
  }

  public async loadTranslationForCurrent(notifyIfUnchanged = false): Promise<void> {
    const track = this.currentTrack;
    if (!this.showTranslation || !track || !isTauriRuntime() || this.isOffline) {
      this.translatedLines = null;
      this.translationLoading = false;
      return;
    }
    if (this.lyrics.type !== "Synced" || this.lyrics.lines.length === 0) {
      this.translatedLines = null;
      this.translationLoading = false;
      return;
    }
    const gen = ++this.translationGen;
    const rawLines = this.lyrics.lines.map((l) => l.text);
    const targetLang = this.getTranslationTargetLang();
    const cacheKey = `${track.id}:${this.activeLyricsSourceId ?? "auto"}`;
    this.translationLoading = true;
    try {
      const translated = await api.translateLyrics(cacheKey, rawLines, targetLang);
      if (this.translationGen === gen && this.currentTrack?.id === track.id && this.showTranslation) {
        const hasDiff = translated.some((s) => s.trim().length > 0);
        this.translatedLines = hasDiff ? translated : null;
        if (!hasDiff && notifyIfUnchanged) {
          this.showToast(
            t("lyrics.translate_same_lang", {
              lang: targetLang === "es" ? "Español" : "English",
            }),
          );
        }
      }
    } catch (err) {
      console.error("[gyro] translateLyrics failed:", err);
      if (this.translationGen === gen) {
        this.translatedLines = null;
        if (notifyIfUnchanged) {
          this.showToast(`${t("lyrics.translate_error")}: ${String(err)}`);
        }
      }
    } finally {
      if (this.translationGen === gen) {
        this.translationLoading = false;
      }
    }
  }

  public async loadLyricsSources(): Promise<void> {
    const track = this.currentTrack;
    if (!track || !isTauriRuntime() || this.isOffline) return;
    if (this.lyricsSourcesTrackId === track.id && this.lyricsSources.length > 0) return;
    const expectedGen = this.loadGen;
    this.lyricsSourcesLoading = true;
    try {
      const list = await api.getLyricsSources(
        track.title,
        track.artist,
        track.duration_secs,
        track.id,
      );
      if (this.loadGen === expectedGen) {
        this.lyricsSources = list;
        this.lyricsSourcesTrackId = track.id;
      }
    } catch {
      if (this.loadGen === expectedGen) {
        this.lyricsSources = [];
      }
    } finally {
      if (this.loadGen === expectedGen) {
        this.lyricsSourcesLoading = false;
      }
    }
  }

  public selectLyricsSource(opt: LyricsSourceOption): void {
    this.lyrics = opt.data;
    this.activeLyricsSourceId = opt.id;
    this.activeLyricIndex = null;
    this.translatedLines = null;
    this.updateActiveLyric(this.positionSecs);
    if (this.showTranslation) {
      void this.loadTranslationForCurrent();
    }
  }

  public async copyFullLyrics(): Promise<void> {
    let text = "";
    if (this.lyrics.type === "Synced") {
      text = this.lyrics.lines.map((l) => l.text).join("\n").trim();
    } else if (this.lyrics.type === "Plain") {
      text = this.lyrics.text.trim();
    }
    if (!text) return;
    try {
      await navigator.clipboard.writeText(text);
      this.showToast(t("lyrics.copied"));
    } catch {
      // ignore clipboard errors
    }
  }

  public updateIntegrations(patch: Partial<IntegrationsSettings>): void {
    this.integrations = { ...this.integrations, ...patch };
    writeIntegrationsSettings(this.integrations);
    void this.syncDiscordRpc();
  }

  public setLanguage(lang: AppLanguage): void {
    this.language = lang;
    void this.persistSession();
  }

  public setCrossfadeSecs(secs: number): void {
    this.crossfadeSecs = Math.max(0, Math.min(10, Math.round(secs)));
    this.engine.setCrossfadeSecs(this.crossfadeSecs);
    void this.persistSession();
  }

  public setAutoplay(enabled: boolean): void {
    this.autoplayRelated = enabled;
    void this.persistSession();
  }

  public setAudioQuality(quality: AudioQuality): void {
    this.audioQuality = quality;
    void this.persistSession();
  }

  public setBackgroundPlayback(enabled: boolean): void {
    this.backgroundPlayback = enabled;
    void this.persistSession();
  }

  public setVolume(v: number): void {
    if (device.isAndroid) {
      this.volume = 1.0;
      this.engine.setVolume(this.audioDuckFactor);
      return;
    }
    this.volume = Math.max(0, Math.min(1, v));
    this.engine.setVolume(this.volume * this.audioDuckFactor);
    void this.persistSession();
    void this.syncMprisState();
  }

  private setAudioDucking(ducked: boolean): void {
    const next = ducked ? 0.25 : 1.0;
    if (this.audioDuckFactor === next) return;
    this.audioDuckFactor = next;
    const base = device.isAndroid ? 1.0 : this.volume;
    this.engine.setVolume(Math.max(0, Math.min(1, base * this.audioDuckFactor)));
  }

  public showToast(message: string): void {
    const clean = formatPlayerErrorMessage(message, this.isOffline);
    const id = ++this.toastSeq;
    this.toasts = [...this.toasts, { id, message: clean }];
    window.setTimeout(() => {
      this.toasts = this.toasts.filter((t) => t.id !== id);
    }, 3200);
  }

  public isTrackDownloaded(trackId: string): boolean {
    return this.downloadedTrackIds.has(trackId);
  }

  public isTrackDownloading(trackId: string): boolean {
    return this.downloadingTrackIds.has(trackId);
  }

  public isPlaylistMarkedDownloaded(playlistKey: string): boolean {
    return this.downloadedPlaylistKeys.has(playlistKey);
  }

  public offlineCoverUrl(trackId: string): string | null {
    if (!this.offlineCoverBaseUrl || !this.downloadedTrackIds.has(trackId)) return null;
    return `${this.offlineCoverBaseUrl}/${encodeURIComponent(trackId)}`;
  }

  public trackCoverUrl(track: Track | null | undefined, preferLarge = false): string | null {
    if (!track) return null;
    const local = this.offlineCoverUrl(track.id);
    if (local && this.isOffline) return local;
    const remote = preferLarge
      ? (track.cover_large_url || track.cover_url || null)
      : (track.cover_url || track.cover_large_url || null);
    return remote || local;
  }

  public async refreshDownloadedTracks(): Promise<void> {
    if (!isTauriRuntime()) return;
    try {
      const [ids, coverBase] = await Promise.all([
        api.listDownloadedTracks(),
        api.getOfflineCoverBaseUrl().catch(() => null),
      ]);
      this.downloadedTrackIds = new Set(ids);
      if (coverBase) this.offlineCoverBaseUrl = coverBase;
      if (!this.isOffline && this.downloadedPlaylistKeys.size > 0) {
        void this.syncDownloadedPlaylists();
      }
    } catch {
      // ignore
    }
  }

  private referencedDownloadedTrackIds(): Set<string> {
    const refs = new Set<string>();
    if (this.downloadedPlaylistKeys.has("__favorites__")) {
      for (const t of this.favorites) refs.add(t.id);
    }
    for (const pl of this.userPlaylists) {
      if (this.downloadedPlaylistKeys.has(pl.id)) {
        for (const t of pl.tracks) refs.add(t.id);
      }
    }
    return refs;
  }

  private async gcOrphanedDownloads(): Promise<void> {
    if (!isTauriRuntime()) return;
    const refs = this.referencedDownloadedTrackIds();
    const orphans = [...this.downloadedTrackIds].filter((id) => !refs.has(id));
    if (orphans.length === 0) return;
    try {
      await api.removeDownloadedTracks(orphans);
      const next = new Set(this.downloadedTrackIds);
      for (const id of orphans) next.delete(id);
      this.downloadedTrackIds = next;
    } catch {
      // ignore cleanup failures
    }
  }

  public enqueueOfflineDownloads(tracks: Track[]): void {
    if (!isTauriRuntime() || this.isOffline || tracks.length === 0) return;
    const queuedIds = new Set(this.downloadQueue.map((t) => t.id));
    for (const tr of tracks) {
      if (!this.downloadedTrackIds.has(tr.id) && !this.downloadingTrackIds.has(tr.id) && !queuedIds.has(tr.id)) {
        this.downloadQueue.push(tr);
        queuedIds.add(tr.id);
      }
    }
    if (!this.downloadWorkerRunning && this.downloadQueue.length > 0) {
      void this.runDownloadWorker();
    }
  }

  private async runDownloadWorker(): Promise<void> {
    if (this.downloadWorkerRunning) return;
    this.downloadWorkerRunning = true;
    try {
      while (this.downloadQueue.length > 0 && !this.isOffline) {
        const track = this.downloadQueue.shift();
        if (!track || this.downloadedTrackIds.has(track.id)) continue;
        const active = new Set(this.downloadingTrackIds);
        active.add(track.id);
        this.downloadingTrackIds = active;
        try {
          await api.downloadTrackOffline(track);
          const next = new Set(this.downloadedTrackIds);
          next.add(track.id);
          this.downloadedTrackIds = next;
        } catch {
          // Continue with remaining queued tracks
        } finally {
          const remaining = new Set(this.downloadingTrackIds);
          remaining.delete(track.id);
          this.downloadingTrackIds = remaining;
        }
      }
    } finally {
      this.downloadWorkerRunning = false;
    }
  }

  public async syncDownloadedPlaylists(): Promise<void> {
    if (!isTauriRuntime() || this.isOffline || this.downloadedPlaylistKeys.size === 0) return;
    const toSync: Track[] = [];
    if (this.downloadedPlaylistKeys.has("__favorites__")) {
      toSync.push(...this.favorites);
    }
    for (const pl of this.userPlaylists) {
      if (this.downloadedPlaylistKeys.has(pl.id)) {
        toSync.push(...pl.tracks);
      }
    }
    this.enqueueOfflineDownloads(toSync);
  }

  public async togglePlaylistDownload(
    playlistKey: string,
    tracks: Track[],
  ): Promise<"downloaded" | "removed" | "partial" | "noop"> {
    if (!isTauriRuntime()) return "noop";

    if (this.downloadedPlaylistKeys.has(playlistKey)) {
      const nextKeys = new Set(this.downloadedPlaylistKeys);
      nextKeys.delete(playlistKey);
      this.downloadedPlaylistKeys = nextKeys;
      this.persistDownloadedPlaylistKeys();
      // Remove pending tracks from queue if no longer referenced
      const refs = this.referencedDownloadedTrackIds();
      this.downloadQueue = this.downloadQueue.filter((t) => refs.has(t.id));
      await this.gcOrphanedDownloads();
      return "removed";
    }

    if (this.isOffline || tracks.length === 0) return "noop";
    const nextKeys = new Set(this.downloadedPlaylistKeys);
    nextKeys.add(playlistKey);
    this.downloadedPlaylistKeys = nextKeys;
    this.persistDownloadedPlaylistKeys();

    const res = await this.downloadPlaylistTracks(playlistKey, tracks);
    if (res.done === res.total) return "downloaded";
    return res.done > 0 ? "partial" : "noop";
  }

  public async downloadPlaylistTracks(
    playlistKey: string,
    tracks: Track[],
  ): Promise<{ done: number; total: number }> {
    if (!isTauriRuntime() || this.isOffline || this.downloadingPlaylistKey) {
      return { done: 0, total: tracks.length };
    }
    const pending = tracks.filter((tr) => !this.downloadedTrackIds.has(tr.id));
    if (pending.length === 0) {
      return { done: tracks.length, total: tracks.length };
    }
    this.downloadingPlaylistKey = playlistKey;
    let done = tracks.length - pending.length;
    const total = tracks.length;
    this.downloadProgress = { done, total };
    try {
      for (const tr of pending) {
        if (this.isOffline || !this.downloadedPlaylistKeys.has(playlistKey)) break;
        const active = new Set(this.downloadingTrackIds);
        active.add(tr.id);
        this.downloadingTrackIds = active;
        try {
          await api.downloadTrackOffline(tr);
          const next = new Set(this.downloadedTrackIds);
          next.add(tr.id);
          this.downloadedTrackIds = next;
          done += 1;
          this.downloadProgress = { done, total };
        } catch {
          // Continue downloading remaining tracks even if one fails
        } finally {
          const remaining = new Set(this.downloadingTrackIds);
          remaining.delete(tr.id);
          this.downloadingTrackIds = remaining;
        }
      }
      return { done, total };
    } finally {
      this.downloadingPlaylistKey = null;
      this.downloadProgress = null;
    }
  }

  public async removeDownloadedPlaylistTracks(tracks: Track[]): Promise<void> {
    if (!isTauriRuntime() || tracks.length === 0) return;
    const ids = tracks.map((t) => t.id).filter((id) => this.downloadedTrackIds.has(id));
    if (ids.length === 0) return;
    try {
      await api.removeDownloadedTracks(ids);
      const next = new Set(this.downloadedTrackIds);
      for (const id of ids) next.delete(id);
      this.downloadedTrackIds = next;
    } catch (err) {
      this.showToast(String(err));
    }
  }

  public async clearAllDownloads(): Promise<void> {
    if (!isTauriRuntime()) return;
    this.downloadQueue = [];
    this.downloadedPlaylistKeys = new Set();
    this.persistDownloadedPlaylistKeys();
    await api.clearAllDownloads();
    this.downloadedTrackIds = new Set();
  }

  public exportLibraryJson(): string {
    return serializeLibraryBackup({
      favorites: $state.snapshot(this.favorites),
      userPlaylists: $state.snapshot(this.userPlaylists),
      followedArtists: $state.snapshot(this.followedArtists),
    });
  }

  public importLibraryJson(raw: string): {
    favoritesAdded: number;
    playlistsAdded: number;
    artistsAdded: number;
  } {
    const { newFavorites, newPlaylists, newArtists } = parseLibraryBackupImport(raw, {
      favorites: this.favorites,
      userPlaylists: this.userPlaylists,
      followedArtists: this.followedArtists,
    });

    if (newFavorites.length > 0) {
      this.favorites = [...this.favorites, ...newFavorites];
      if (isTauriRuntime()) void api.saveFavorites(this.favorites);
    }
    if (newPlaylists.length > 0) {
      this.userPlaylists = [...this.userPlaylists, ...newPlaylists];
      if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
    }
    if (newArtists.length > 0) {
      this.followedArtists = [...this.followedArtists, ...newArtists];
      if (isTauriRuntime()) void api.saveFollowedArtists(this.followedArtists);
    }

    return {
      favoritesAdded: newFavorites.length,
      playlistsAdded: newPlaylists.length,
      artistsAdded: newArtists.length,
    };
  }

  private isMobilePlayerHistoryEnabled(): boolean {
    if (typeof window === "undefined") return false;
    const isMobilePlatform =
      Boolean(getAndroidBridge()) ||
      /Android|iPhone|iPad|iPod/i.test(navigator.userAgent);
    return isMobilePlatform && window.innerWidth < 1024;
  }

  private initRouter(): void {
    if (typeof window === "undefined" || this.routerBound) return;
    this.routerBound = true;

    const initial = hashToView(window.location.hash);
    this.activeView = initial.view;
    this.activeTab = initial.tab;
    this.routeDepth = 0;
    const useMobilePlayerHistory = this.isMobilePlayerHistoryEnabled();
    if (useMobilePlayerHistory) {
      this.sidebarOpen = initial.playerOpen;
    }
    this.partyMode = initial.partyOpen;
    this.spinOpen = initial.spinOpen;
    const initialState: RouteState = {
      view: initial.view,
      tab: initial.tab,
      depth: 0,
      playerOpen: useMobilePlayerHistory ? initial.playerOpen : false,
      partyOpen: initial.partyOpen,
      spinOpen: initial.spinOpen,
    };
    window.history.replaceState(
      initialState,
      "",
      viewToHash(initial.view, {
        playerOpen: Boolean(initialState.playerOpen),
        partyOpen: Boolean(initialState.partyOpen),
        spinOpen: Boolean(initialState.spinOpen),
      }),
    );

    window.addEventListener("popstate", (event) => {
      // Dismiss transient context menus before navigating away
      if (contextMenu.open) {
        contextMenu.close();
        const plainView = $state.snapshot(this.activeView);
        const keep: RouteState = {
          view: plainView,
          tab: this.activeTab,
          depth: this.routeDepth + 1,
          playerOpen: this.isMobilePlayerHistoryEnabled() && this.sidebarOpen,
          partyOpen: this.partyMode,
          spinOpen: this.spinOpen,
        };
        this.routeDepth = keep.depth;
        window.history.pushState(keep, "", viewToHash(plainView, keep));
        return;
      }

      const st = event.state as RouteState | null;
      const parsed = hashToView(window.location.hash);
      const nextView = st?.view ?? parsed.view;
      const nextTab = st?.view ? st.tab : parsed.tab;
      if (!isSameView(this.activeView, nextView)) {
        this.activeView = nextView;
      }
      this.activeTab = nextTab;
      this.routeDepth = st?.view ? (st.depth ?? 0) : 0;
      if (this.isMobilePlayerHistoryEnabled()) {
        this.sidebarOpen = Boolean(st?.playerOpen ?? parsed.playerOpen);
      }
      this.partyMode = Boolean(st?.partyOpen ?? parsed.partyOpen);
      this.spinOpen = Boolean(st?.spinOpen ?? parsed.spinOpen);
      if (nextView.kind === "search") {
        if (nextView.query) {
          if (this.searchQuery !== nextView.query || !this.searchResults) {
            void this.runSearch(nextView.query, false);
          }
        } else if (this.searchQuery || this.searchResults) {
          this.clearSearch(false);
        }
      }
    });

    const androidWin = getAndroidWindow();
    if (androidWin) {
      androidWin.__GYROTUBE_DISMISS_OVERLAY__ = () => this.dismissTopOverlay();
    }
  }

  public toggleNavCollapsed(): void {
    this.navCollapsed = !this.navCollapsed;
    writeNavCollapsed(this.navCollapsed);
  }

  private pushOverlayState(overlays: { playerOpen?: boolean; partyOpen?: boolean; spinOpen?: boolean }): void {
    if (typeof window === "undefined") return;
    this.routeDepth += 1;
    const plainView = $state.snapshot(this.activeView);
    const state: RouteState = {
      view: plainView,
      tab: this.activeTab,
      depth: this.routeDepth,
      playerOpen: overlays.playerOpen ?? (this.isMobilePlayerHistoryEnabled() && this.sidebarOpen),
      partyOpen: overlays.partyOpen ?? this.partyMode,
      spinOpen: overlays.spinOpen ?? this.spinOpen,
    };
    window.history.pushState(state, "", viewToHash(plainView, state));
  }

  private replaceOverlayState(): void {
    if (typeof window === "undefined") return;
    const plainView = $state.snapshot(this.activeView);
    const state: RouteState = {
      view: plainView,
      tab: this.activeTab,
      depth: this.routeDepth,
      playerOpen: this.isMobilePlayerHistoryEnabled() && this.sidebarOpen,
      partyOpen: this.partyMode,
      spinOpen: this.spinOpen,
    };
    window.history.replaceState(state, "", viewToHash(plainView, state));
  }

  public openPlayerSheet(): void {
    if (this.sidebarOpen) return;
    this.sidebarOpen = true;
    if (this.isMobilePlayerHistoryEnabled()) {
      this.pushOverlayState({ playerOpen: true });
    } else {
      writeSidebarCollapsed(false);
      void this.persistSession();
    }
  }

  public closePlayerSheet(): void {
    if (!this.sidebarOpen) return;
    if (this.isMobilePlayerHistoryEnabled()) {
      const st = window.history.state as RouteState | null;
      if (st?.playerOpen && !st?.partyOpen && !st?.spinOpen && this.routeDepth > 0) {
        window.history.back();
        return;
      }
      this.sidebarOpen = false;
      this.replaceOverlayState();
      return;
    }
    this.sidebarOpen = false;
    writeSidebarCollapsed(true);
    void this.persistSession();
  }

  public togglePlayerSheet(): void {
    if (this.sidebarOpen) {
      this.closePlayerSheet();
    } else {
      this.openPlayerSheet();
    }
  }

  public openPartyMode(): void {
    if (this.videoMode || this.partyMode) return;
    this.partyMode = true;
    this.pushOverlayState({ partyOpen: true });
  }

  public closePartyMode(): void {
    if (!this.partyMode) return;
    const st = typeof window !== "undefined" ? (window.history.state as RouteState | null) : null;
    if (st?.partyOpen && this.routeDepth > 0) {
      window.history.back();
      return;
    }
    this.partyMode = false;
    this.replaceOverlayState();
  }

  public togglePartyMode(): void {
    if (this.partyMode) {
      this.closePartyMode();
    } else {
      this.openPartyMode();
    }
  }

  public openSpin(): void {
    if (this.spinOpen) return;
    this.spinOpen = true;
    this.pushOverlayState({ spinOpen: true });
    void this.refreshSpinStatus();
  }

  public closeSpin(): void {
    if (!this.spinOpen) return;
    const st = typeof window !== "undefined" ? (window.history.state as RouteState | null) : null;
    if (st?.spinOpen && this.routeDepth > 0) {
      window.history.back();
      return;
    }
    this.spinOpen = false;
    this.replaceOverlayState();
  }

  public toggleSpin(): void {
    if (this.spinOpen) {
      this.closeSpin();
    } else {
      this.openSpin();
    }
  }

  private dismissTopOverlay(): boolean {
    if (contextMenu.open) {
      contextMenu.close();
      return true;
    }
    if (this.partyMode) {
      this.closePartyMode();
      return true;
    }
    if (this.spinOpen) {
      this.closeSpin();
      return true;
    }
    return false;
  }

  private commitRoute(view: ActiveView, tab: MainTab): void {
    const changed = !isSameView(this.activeView, view);
    const isCompact = typeof window !== "undefined" && window.innerWidth < 1024;
    const wasCompactPlayer = isCompact && this.sidebarOpen;
    const useMobilePlayerHistory = this.isMobilePlayerHistoryEnabled();
    const currentState =
      typeof window !== "undefined" ? (window.history.state as RouteState | null) : null;
    const replaceOverlayEntry =
      (useMobilePlayerHistory && Boolean(currentState?.playerOpen)) ||
      Boolean(currentState?.partyOpen) ||
      Boolean(currentState?.spinOpen);

    if (!changed && replaceOverlayEntry && this.routeDepth > 0) {
      window.history.back();
      return;
    }

    if (wasCompactPlayer) {
      this.sidebarOpen = false;
    }
    this.partyMode = false;
    this.spinOpen = false;
    this.activeTab = tab;
    this.activeView = view;
    if (typeof window !== "undefined" && (changed || replaceOverlayEntry)) {
      const plainView = $state.snapshot(view);
      const state: RouteState = {
        view: plainView,
        tab,
        depth: replaceOverlayEntry ? this.routeDepth : this.routeDepth + 1,
        playerOpen: false,
        partyOpen: false,
        spinOpen: false,
      };
      if (replaceOverlayEntry) {
        window.history.replaceState(state, "", viewToHash(plainView));
      } else {
        this.routeDepth = state.depth;
        window.history.pushState(state, "", viewToHash(plainView));
      }
    }
  }

  public goBack(): void {
    if (typeof window !== "undefined" && this.routeDepth > 0) {
      window.history.back();
      return;
    }
    if (this.activeView.kind === "collection" && this.activeView.sub) {
      this.navigateTab("collection");
      return;
    }
    this.navigateTab(this.activeTab === "home" ? "home" : "explore");
  }

  public openLocalPlaylist(target: "favorites" | string): void {
    const sub =
      target === "favorites"
        ? ({ kind: "favorites" } as const)
        : ({ kind: "playlist", id: target } as const);
    this.commitRoute({ kind: "collection", sub }, "collection");
  }

  public navigateTab(tab: MainTab): void {
    this.commitRoute({ kind: tab }, tab);
  }

  public openSearch(query?: string): void {
    const clean = query?.trim();
    if (clean) {
      void this.runSearch(clean);
      return;
    }
    this.clearSearch(false);
    this.commitRoute({ kind: "search" }, this.activeTab);
  }

  public clearSearch(syncRoute = true): void {
    ++this.searchGen;
    this.searchQuery = "";
    this.searchResults = null;
    this.searchLoading = false;
    if (syncRoute && this.activeView.kind === "search" && this.activeView.query) {
      this.commitRoute({ kind: "search" }, this.activeTab);
    }
  }

  public openArtist(artistId: string, artistName: string): void {
    this.commitRoute({ kind: "artist", artistId, artistName }, "explore");
  }

  public async openArtistByNameOrId(
    artistId: string | null | undefined,
    artistName: string,
  ): Promise<void> {
    if (artistId) {
      this.openArtist(artistId, artistName);
      return;
    }
    const cleanName = artistName.split(/[,&]|feat\.?|ft\.?/i)[0]?.trim() || artistName.trim();
    if (!cleanName) return;
    if (!isTauriRuntime()) return;
    try {
      const res = await api.searchAll(cleanName);
      const lower = cleanName.toLowerCase();
      const match =
        res.artists.find((a) => a.name.trim().toLowerCase() === lower) ?? res.artists[0];
      if (match) {
        this.openArtist(match.id, match.name);
        return;
      }
      await this.runSearch(cleanName);
    } catch {
      await this.runSearch(cleanName);
    }
  }

  public openAlbum(id: string, title: string, coverUrl?: string | null, isPlaylist = false): void {
    this.commitRoute({ kind: "album", id, title, coverUrl, isPlaylist }, this.activeTab);
  }

  public openSettings(): void {
    if (this.activeView.kind === "settings") {
      this.goBack();
      return;
    }
    this.commitRoute({ kind: "settings" }, this.activeTab);
  }

  public setChartCountry(country: string | null): void {
    this.chartCountry = writeChartCountry(country);
  }

  public async loadHome(forceRefresh = false): Promise<void> {
    if (!isTauriRuntime() || this.isOffline) return;
    this.homeLoading = true;
    try {
      this.homeFeed = await api.getHomeFeed(forceRefresh, this.chartCountry);
    } catch (err) {
      this.showToast(String(err));
    } finally {
      this.homeLoading = false;
    }
  }

  public async loadRelatedForCurrent(force = false): Promise<void> {
    const cur = this.currentTrack;
    if (!cur || !isTauriRuntime()) {
      this.relatedFeed = null;
      this.relatedTrackId = null;
      return;
    }
    if (!force && this.relatedTrackId === cur.id && this.relatedFeed) return;
    this.relatedLoading = true;
    this.relatedTrackId = cur.id;
    try {
      const feed = await api.getRelatedForTrack(cur.id);
      if (this.currentTrack?.id === cur.id) {
        this.relatedFeed = feed;
      }
    } catch {
      if (this.currentTrack?.id === cur.id) {
        this.relatedFeed = { tracks: [], albums: [], playlists: [], artists: [] };
      }
    } finally {
      if (this.currentTrack?.id === cur.id) {
        this.relatedLoading = false;
      }
    }
  }

  public async loadGenres(forceRefresh = false): Promise<void> {
    if (!isTauriRuntime() || this.isOffline) return;
    this.genresLoading = true;
    try {
      this.genres = await api.getGenres(forceRefresh);
    } catch {
      // Non-fatal
    } finally {
      this.genresLoading = false;
    }
  }

  public async runSearch(raw: string, pushRoute = true): Promise<void> {
    if (this.isOffline) return;
    const q = raw.trim();
    if (!q) return;
    const gen = ++this.searchGen;
    this.searchQuery = q;
    if (pushRoute) {
      this.commitRoute({ kind: "search", query: q }, this.activeTab);
    }
    this.recordRecentSearch({ kind: "query", key: `q:${q.toLowerCase()}`, term: q });
    if (isTauriRuntime()) {
      const cacheKey = q.toLowerCase();
      const cached = this.searchCache.get(cacheKey);
      if (cached) {
        this.searchResults = cached;
        this.searchLoading = false;
        return;
      }
      this.searchLoading = true;
      try {
        const res = await api.searchAll(q);
        if (this.searchGen !== gen) return;
        if (this.searchCache.size >= 12) {
          const oldest = this.searchCache.keys().next().value;
          if (oldest !== undefined) this.searchCache.delete(oldest);
        }
        this.searchCache.set(cacheKey, res);
        this.searchResults = res;
      } catch (err) {
        if (this.searchGen === gen) this.showToast(String(err));
      } finally {
        if (this.searchGen === gen) this.searchLoading = false;
      }
    }
  }

  public recordRecentSearch(entry: RecentSearchEntry): void {
    this.recentSearches = [
      entry,
      ...this.recentSearches.filter((item) => item.key !== entry.key),
    ].slice(0, 10);
    this.persistRecentSearches();
    if (entry.kind === "query") {
      this.searchHistory = [
        entry.term,
        ...this.searchHistory.filter((h) => h.toLowerCase() !== entry.term.toLowerCase()),
      ].slice(0, 8);
      if (isTauriRuntime()) void api.saveSearchHistory(this.searchHistory);
    }
  }

  public removeRecentSearch(key: string): void {
    const removed = this.recentSearches.find((item) => item.key === key);
    this.recentSearches = this.recentSearches.filter((item) => item.key !== key);
    this.persistRecentSearches();
    if (removed?.kind === "query") {
      this.searchHistory = this.searchHistory.filter(
        (item) => item.toLowerCase() !== removed.term.toLowerCase(),
      );
      if (isTauriRuntime()) void api.saveSearchHistory(this.searchHistory);
    }
  }

  public clearSearchHistory(): void {
    this.searchHistory = [];
    this.recentSearches = [];
    this.persistRecentSearches();
    if (isTauriRuntime()) void api.saveSearchHistory([]);
  }

  public removeSearchHistoryItem(term: string): void {
    this.removeRecentSearch(`q:${term.toLowerCase()}`);
  }

  public get sortedUserPlaylists(): UserPlaylist[] {
    if (this.pinnedPlaylistIds.length === 0) return this.userPlaylists;
    const pinnedSet = new Set(this.pinnedPlaylistIds);
    const pinned = this.pinnedPlaylistIds
      .map((id) => this.userPlaylists.find((p) => p.id === id))
      .filter((p): p is UserPlaylist => p != null);
    const rest = this.userPlaylists.filter((p) => !pinnedSet.has(p.id));
    return [...pinned, ...rest];
  }

  public isPlaylistPinned(id: string): boolean {
    return this.pinnedPlaylistIds.includes(id);
  }

  public togglePinPlaylist(id: string): void {
    if (this.pinnedPlaylistIds.includes(id)) {
      this.pinnedPlaylistIds = this.pinnedPlaylistIds.filter((x) => x !== id);
      this.persistPinnedPlaylistIds();
      return;
    }
    if (this.pinnedPlaylistIds.length >= 4) {
      this.showToast(t("collection.pin_max"));
      return;
    }
    this.pinnedPlaylistIds = [id, ...this.pinnedPlaylistIds];
    this.persistPinnedPlaylistIds();
  }

  public isTrackHidden(trackId: string): boolean {
    return this.hiddenTrackIds.has(trackId);
  }

  public toggleHideTrack(track: Track): void {
    const next = new Set(this.hiddenTrackIds);
    if (next.has(track.id)) {
      next.delete(track.id);
      this.hiddenTrackIds = next;
      this.persistHiddenTrackIds();
      this.showToast(t("ctx.track_unhidden", { title: track.title }));
    } else {
      next.add(track.id);
      this.hiddenTrackIds = next;
      this.persistHiddenTrackIds();
      this.showToast(t("ctx.track_hidden", { title: track.title }));
      if (this.currentTrack?.id === track.id && this.queue.length > 1) {
        void this.next(false);
      }
    }
  }

  public removeRecentTrack(trackId: string): void {
    this.recentTracks = this.recentTracks.filter((t) => t.id !== trackId);
    if (trackId in this.recentPlayedAt) {
      const next = { ...this.recentPlayedAt };
      delete next[trackId];
      this.recentPlayedAt = next;
      this.persistRecentPlayedAt();
    }
    if (isTauriRuntime()) void api.saveRecentTracks(this.recentTracks);
  }

  public clearRecentTracks(): void {
    this.recentTracks = [];
    this.recentPlayedAt = {};
    this.persistRecentPlayedAt();
    if (isTauriRuntime()) void api.saveRecentTracks([]);
  }

  public isFavorite(trackId: string): boolean {
    return this.favoriteIdSet.has(trackId);
  }

  public isFollowingArtist(artistId: string): boolean {
    return this.followedArtistIdSet.has(artistId);
  }

  public toggleFollowArtist(artist: Artist): void {
    const exists = this.followedArtistIdSet.has(artist.id);
    this.followedArtists = exists
      ? this.followedArtists.filter((a) => a.id !== artist.id)
      : [artist, ...this.followedArtists];
    if (isTauriRuntime()) void api.saveFollowedArtists(this.followedArtists);
  }

  public get canSwitchVideoMode(): boolean {
    return this.counterpartTrack != null;
  }

  private probeCounterpartForCurrent(track: Track): void {
    const gen = ++this.counterpartGen;
    if (!isTauriRuntime()) {
      this.counterpartTrack = null;
      return;
    }
    const wantVideo = !track.is_video;
    const cacheKey = `${track.id}:${wantVideo ? "v" : "s"}`;
    if (this.counterpartCache.has(cacheKey)) {
      this.counterpartTrack = this.counterpartCache.get(cacheKey) ?? null;
      return;
    }
    this.counterpartTrack = null;
    void api
      .findCounterpartTrack(track, wantVideo)
      .then((counterpart) => {
        const valid = counterpart && counterpart.id !== track.id ? counterpart : null;
        this.counterpartCache.set(cacheKey, valid);
        if (valid) {
          this.counterpartCache.set(`${valid.id}:${wantVideo ? "s" : "v"}`, track);
        }
        if (this.counterpartGen === gen && this.currentTrack?.id === track.id) {
          this.counterpartTrack = valid;
        }
      })
      .catch(() => {
        if (this.counterpartGen === gen && this.currentTrack?.id === track.id) {
          this.counterpartTrack = null;
        }
      });
  }

  public setVideoMode(enabled: boolean): void {
    if (!this.videoFeatureEnabled) {
      this.videoMode = false;
      this.videoStream = null;
      this.videoLoading = false;
      this.videoError = null;
      return;
    }
    if (this.videoMode === enabled) return;
    const cur = this.currentTrack;
    const idx = this.currentIndex;

    if (!cur || idx == null || !isTauriRuntime()) {
      this.videoMode = false;
      this.videoStream = null;
      this.videoLoading = false;
      this.videoError = null;
      return;
    }

    const needSwitch = Boolean(cur.is_video) !== enabled;
    if (!needSwitch) {
      this.videoMode = enabled;
      if (!enabled) {
        this.clearVideoStartWait();
        this.clearVideoSyncPause(false);
        this.videoStream = null;
        this.videoLoading = false;
        this.videoError = null;
        if (this.lyrics.type === "None") {
          void this.loadLyricsForCurrent();
        }
      } else {
        this.partyMode = false;
        if (this.sidebarTab === "lyrics") {
          this.sidebarTab = "player";
        }
        this.activeLyricIndex = null;
        this.lyricsLoading = false;
        // Pause audio briefly while the video loads so both start in lockstep
        // with a unified loading state.
        const wasPlaying = this.isPlaying;
        this.clearVideoStartWait();
        this.clearVideoSyncPause(false);
        if (wasPlaying) {
          const gen = this.loadGen;
          this.audioLoadedForGen = gen;
          this.awaitingVideoStartGen = gen;
          this.isLoadingAudio = true;
          this.videoBuffering = true;
          this.engine.pause();
          this.videoStartFallbackTimer = window.setTimeout(() => {
            if (this.awaitingVideoStartGen === gen) {
              this.notifyVideoUnavailable();
            }
          }, 6500);
        }
        void this.loadVideoForCurrent();
      }
      return;
    }

    const cacheKey = `${cur.id}:${enabled ? "v" : "s"}`;
    const counterpart = this.counterpartTrack ?? this.counterpartCache.get(cacheKey) ?? null;
    if (!counterpart || counterpart.id === cur.id) {
      // Video without an official studio counterpart: allow switching to Song mode
      // (audio-only with cover art, matching YouTube Music) without interrupting playback.
      if (!enabled && cur.is_video) {
        ++this.videoGen;
        this.videoMode = false;
        this.videoStream = null;
        this.videoLoading = false;
        this.videoError = null;
        if (this.lyrics.type === "None") {
          void this.loadLyricsForCurrent();
        }
      }
      return;
    }

    ++this.videoGen;
    this.counterpartCache.set(cacheKey, counterpart);
    this.counterpartCache.set(`${counterpart.id}:${enabled ? "s" : "v"}`, cur);
    this.videoMode = enabled;
    if (enabled) {
      this.partyMode = false;
      if (this.sidebarTab === "lyrics") {
        this.sidebarTab = "player";
      }
      this.activeLyricIndex = null;
      this.lyricsLoading = false;
    }
    const wasPlaying = this.isPlaying;
    const resumePos = this.positionSecs;
    const nextQueue = [...this.queue];
    nextQueue[idx] = counterpart;
    this.queue = nextQueue;
    void this.playIndex(idx, false, resumePos, !wasPlaying);
  }

  public getExactPositionSecs(): number {
    const deck = this.engine.activeDeck;
    if (deck && deck.src && Number.isFinite(deck.currentTime)) {
      return deck.currentTime;
    }
    return this.positionSecs;
  }

  private clearVideoStartWait(): void {
    this.awaitingVideoStartGen = null;
    this.videoReadyEl = null;
    if (this.videoStartFallbackTimer !== null) {
      window.clearTimeout(this.videoStartFallbackTimer);
      this.videoStartFallbackTimer = null;
    }
  }

  private clearVideoSyncPause(resumePlayback: boolean): void {
    const wasPausedForSync = this.videoSyncPauseActive;
    this.videoSyncPauseActive = false;
    this.videoBuffering = false;
    if (this.videoSyncFallbackTimer !== null) {
      window.clearTimeout(this.videoSyncFallbackTimer);
      this.videoSyncFallbackTimer = null;
    }
    if (wasPausedForSync && resumePlayback) {
      const videoEl = this.videoReadyEl?.gen === this.loadGen ? this.videoReadyEl.el : null;
      if (videoEl) {
        void videoEl.play().catch(() => {});
      }
      void this.engine.play().catch(() => {
        this.isLoadingAudio = false;
      });
    } else if (wasPausedForSync) {
      this.isLoadingAudio = false;
    }
  }

  private startAudioAndVideoTogether(videoEl: HTMLVideoElement | null): void {
    const gen = this.loadGen;
    this.clearVideoStartWait();
    if (videoEl) {
      this.videoReadyEl = { gen, el: videoEl };
    }
    const deck = this.engine.activeDeck;

    const launchBoth = () => {
      if (this.loadGen !== gen) return;
      this.videoBuffering = false;
      if (videoEl) {
        videoEl.playbackRate = 1;
        void videoEl.play().catch(() => {});
      }
      void deck.play().catch(() => {
        this.isLoadingAudio = false;
      });
    };

    if (videoEl && Number.isFinite(deck.currentTime)) {
      const target = Math.max(0, deck.currentTime);
      const clamped = Number.isFinite(videoEl.duration)
        ? Math.min(target, Math.max(0, videoEl.duration - 0.1))
        : target;
      if (Math.abs(videoEl.currentTime - clamped) > 0.08) {
        // Wait for the <video> seek to settle on the exact frame before starting
        // audio so both begin in lockstep without an initial offset.
        this.isLoadingAudio = true;
        this.videoBuffering = true;
        let done = false;
        const finish = () => {
          if (done) return;
          done = true;
          videoEl.removeEventListener("seeked", finish);
          window.clearTimeout(timeoutId);
          launchBoth();
        };
        const timeoutId = window.setTimeout(finish, 1500);
        videoEl.addEventListener("seeked", finish, { once: true });
        try {
          videoEl.currentTime = clamped;
          return;
        } catch {
          finish();
          return;
        }
      }
    }

    launchBoth();
  }

  public notifyVideoReady(videoEl: HTMLVideoElement): void {
    this.videoReadyEl = { gen: this.loadGen, el: videoEl };
    if (
      this.awaitingVideoStartGen === this.loadGen &&
      this.audioLoadedForGen === this.loadGen
    ) {
      this.startAudioAndVideoTogether(videoEl);
    } else if (this.videoSyncPauseActive) {
      this.clearVideoSyncPause(true);
    }
  }

  public notifyVideoBuffering(buffering: boolean): void {
    if (!this.videoMode) {
      this.videoBuffering = false;
      return;
    }
    if (buffering) {
      if (this.awaitingVideoStartGen === this.loadGen) {
        this.videoBuffering = true;
        return;
      }
      if (!this.isPlaying || this.videoSyncPauseActive) return;
      // Pause audio briefly while <video> refills its buffer so both stay in sync.
      this.videoSyncPauseActive = true;
      this.videoBuffering = true;
      this.isLoadingAudio = true;
      this.engine.pause();
      this.videoSyncFallbackTimer = window.setTimeout(() => {
        this.clearVideoSyncPause(true);
      }, 4000);
    } else {
      if (this.videoSyncPauseActive) {
        this.clearVideoSyncPause(true);
      } else {
        this.videoBuffering = false;
      }
    }
  }

  public notifyVideoUnavailable(): void {
    if (
      this.awaitingVideoStartGen === this.loadGen &&
      this.audioLoadedForGen === this.loadGen
    ) {
      this.startAudioAndVideoTogether(null);
    } else if (this.awaitingVideoStartGen === this.loadGen) {
      this.clearVideoStartWait();
      this.videoBuffering = false;
    } else if (this.videoSyncPauseActive) {
      this.clearVideoSyncPause(true);
    }
  }

  public async loadVideoForCurrent(): Promise<void> {
    const track = this.currentTrack;
    if (!this.videoFeatureEnabled || !track || !this.videoMode || !isTauriRuntime()) {
      this.videoStream = null;
      this.videoLoading = false;
      this.videoError = null;
      return;
    }
    const gen = ++this.videoGen;
    this.videoLoading = true;
    this.videoError = null;
    this.videoStream = null;
    const isLinuxDesktop =
      typeof navigator !== "undefined" &&
      !device.isAndroid &&
      /Linux/i.test(navigator.userAgent);
    const maxHeight =
      (typeof window !== "undefined" && window.innerWidth < 768) || isLinuxDesktop ? 720 : 1080;
    try {
      const info = await api.resolveVideoStream(track.id, maxHeight);
      if (this.videoGen === gen && this.currentTrack?.id === track.id && this.videoMode) {
        this.videoStream = info;
      }
    } catch (err) {
      if (this.videoGen === gen) {
        this.videoError = String(err);
        this.notifyVideoUnavailable();
      }
    } finally {
      if (this.videoGen === gen) {
        this.videoLoading = false;
      }
    }
  }

  private backfillTrackDuration(trackId: string, durationSecs: number): void {
    if (durationSecs <= 0) return;
    const patch = (list: Track[]): [Track[], boolean] => {
      let changed = false;
      const next = list.map((t) => {
        if (t.id === trackId && t.duration_secs == null) {
          changed = true;
          return { ...t, duration_secs: durationSecs };
        }
        return t;
      });
      return [next, changed];
    };
    const [nextQueue, qChanged] = patch(this.queue);
    if (qChanged) this.queue = nextQueue;
    const [nextUnshuffled, uChanged] = patch(this.unshuffledQueue);
    if (uChanged) this.unshuffledQueue = nextUnshuffled;
    const [nextRecent, rChanged] = patch(this.recentTracks);
    if (rChanged) {
      this.recentTracks = nextRecent;
      if (isTauriRuntime()) void api.saveRecentTracks(this.recentTracks);
    }
    const [nextFavs, fChanged] = patch(this.favorites);
    if (fChanged) {
      this.favorites = nextFavs;
      if (isTauriRuntime()) void api.saveFavorites(this.favorites);
    }
    if (qChanged || uChanged) void this.persistSession();
  }

  public toggleFavorite(track: Track): void {
    const removing = this.isFavorite(track.id);
    if (removing) {
      this.favorites = this.favorites.filter((t) => t.id !== track.id);
    } else {
      const withDuration =
        track.duration_secs == null && this.currentTrack?.id === track.id && this.durationSecs > 0
          ? { ...track, duration_secs: Math.round(this.durationSecs) }
          : track;
      this.favorites = [withDuration, ...this.favorites];
    }
    if (isTauriRuntime()) void api.saveFavorites(this.favorites);
    if (this.downloadedPlaylistKeys.has("__favorites__")) {
      if (removing) {
        void this.gcOrphanedDownloads();
      } else {
        this.enqueueOfflineDownloads([track]);
      }
    }
  }

  public replaceQueueAndPlay(tracks: Track[], index: number, source: string): void {
    if (!tracks.length) return;
    this.queue = [...tracks];
    this.unshuffledQueue = [...tracks];
    this.sourceName = source;
    this.shuffle = false;
    void this.playIndex(Math.max(0, Math.min(index, tracks.length - 1)));
  }

  private sendRemoteControl(action: "play_pause" | "next" | "prev"): void {
    if (!this.spinRemoteConn.allowControls) {
      this.showToast(t("toast.spin_controls_disabled"));
      return;
    }
    void api.spinRemoteControl(action).catch((err) => this.showToast(String(err)));
  }

  private sendRemoteAddTrack(track: Track): void {
    const host = this.spinRemoteConn.hostName || "Host";
    void api
      .spinRemoteAddTrack(track, this.spinDisplayName)
      .then(() => {
        this.showToast(t("toast.spin_sent_to_host", { title: track.title, host }));
      })
      .catch((err) => this.showToast(String(err)));
  }

  public playNext(track: Track): void {
    if (this.spinStatus.remote_role === "guest") {
      this.sendRemoteAddTrack(track);
      return;
    }
    if (this.currentIndex == null || this.queue.length === 0) {
      this.replaceQueueAndPlay([track], 0, track.title);
      return;
    }
    const insertAt = Math.min(this.currentIndex + 1, this.queue.length);
    const next = [...this.queue];
    next.splice(insertAt, 0, track);
    this.queue = next;
    this.showToast(t("toast.playing_next", { title: track.title }));
    void this.persistSession();
    void this.syncSpinSnapshot();
  }

  public enqueueTrack(track: Track, silent = false): void {
    if (this.spinStatus.remote_role === "guest") {
      this.sendRemoteAddTrack(track);
      return;
    }
    this.queue = [...this.queue, track];
    if (!silent) {
      this.showToast(t("toast.added_to_queue", { title: track.title }));
    }
    if (this.currentIndex == null) {
      void this.playIndex(0);
    } else {
      void this.persistSession();
      void this.syncSpinSnapshot();
      void this.syncMprisState();
    }
  }

  public async startRadioFromCurrent(): Promise<void> {
    const current = this.currentTrack;
    if (!current || !isTauriRuntime() || this.isOffline) return;
    try {
      const radio = await api.getRadioTracks(current.id);
      const unique = radio.filter((t) => t.id !== current.id);
      if (unique.length === 0) {
        this.showToast(t("toast.radio_empty"));
        return;
      }
      this.queue = [current, ...unique];
      this.unshuffledQueue = [...this.queue];
      this.currentIndex = 0;
      this.shuffle = false;
      this.sourceName = `Radio · ${current.title}`;
      void this.persistSession();
      void this.syncSpinSnapshot();
      this.showToast(t("toast.radio_started", { count: unique.length }));
    } catch (err) {
      this.showToast(String(err));
    }
  }

  public removeQueueIndex(idx: number): void {
    const res = removeQueueAt(this.queue, this.currentIndex, idx);
    if (!res) return;
    this.queue = res.queue;
    this.currentIndex = res.currentIndex;
    if (res.shouldStop) this.engine.stop();
    if (res.playIndexAfter != null) void this.playIndex(res.playIndexAfter);
    void this.persistSession();
    void this.syncSpinSnapshot();
  }

  public moveQueueItem(from: number, to: number): void {
    const res = moveQueueIndices(this.queue, this.currentIndex, from, to);
    if (!res) return;
    this.queue = res.queue;
    this.currentIndex = res.currentIndex;
    void this.persistSession();
    void this.syncSpinSnapshot();
  }

  public clearQueue(): void {
    const current = this.currentTrack;
    if (current) {
      this.queue = [current];
      this.unshuffledQueue = [current];
      this.currentIndex = 0;
      this.shuffle = false;
      void this.persistSession();
      void this.syncSpinSnapshot();
      this.showToast(t("toast.queue_cleared"));
      return;
    }
    this.queue = [];
    this.unshuffledQueue = [];
    this.currentIndex = null;
    this.engine.stop();
    this.lyrics = { type: "None" };
    this.ambientRgb = null;
    void this.persistSession();
  }

  private schedulePrefetchNext(expectedGen: number): void {
    if (this.prefetchTimer !== null) {
      window.clearTimeout(this.prefetchTimer);
      this.prefetchTimer = null;
    }
    if (!isTauriRuntime()) return;
    const upcoming =
      this.nextTrack ?? (this.autoplayRelated ? (this.suggestedTracks[0] ?? null) : null);
    if (!upcoming) return;
    this.prefetchTimer = window.setTimeout(() => {
      this.prefetchTimer = null;
      if (this.loadGen !== expectedGen) return;
      const target =
        this.nextTrack ?? (this.autoplayRelated ? (this.suggestedTracks[0] ?? null) : null);
      if (!target) return;
      void api.prefetchAudio(target).catch(() => {
        // Ignore prefetch failures; normal playback will resolve on demand.
      });
    }, 3000);
  }

  public async playIndex(index: number, useCrossfade = false, startPositionSecs = 0, startPaused = false): Promise<void> {
    if (index < 0 || index >= this.queue.length) return;
    const prevTrackId = this.currentTrack?.id ?? null;
    const track = this.queue[index];
    const gen = ++this.loadGen;
    this.inFlightTransitionGen = gen;
    if (!useCrossfade) {
      this.engine.stop();
    }
    this.currentIndex = index;
    this.syncAndroidForegroundService(!startPaused);
    this.isLoadingAudio = true;
    this.positionSecs = startPositionSecs;
    this.durationSecs = track.duration_secs ?? 0;
    this.lyrics = { type: "None" };
    this.activeLyricIndex = null;
    this.lyricsLoading = !(this.videoFeatureEnabled && Boolean(track.is_video));
    this.translatedLines = null;
    this.translationLoading = false;
    this.lyricsSources = [];
    this.lyricsSourcesTrackId = null;
    this.activeLyricsSourceId = null;
    this.sponsorSegments = [];
    this.skippedSponsorKeys.clear();
    this.videoStream = null;
    this.videoError = null;
    this.videoMode = this.videoFeatureEnabled && Boolean(track.is_video);
    if (this.videoMode) {
      this.partyMode = false;
      if (this.sidebarTab === "lyrics") {
        this.sidebarTab = "player";
      }
    }
    this.probeCounterpartForCurrent(track);

    // Record recent + timestamp for date-grouped history
    this.recentTracks = [track, ...this.recentTracks.filter((t) => t.id !== track.id)].slice(0, 30);
    this.recentPlayedAt = { ...this.recentPlayedAt, [track.id]: Date.now() };
    this.persistRecentPlayedAt();
    if (isTauriRuntime()) void api.saveRecentTracks(this.recentTracks);

    // Extract ambient cover color
    const cover = this.trackCoverUrl(track, true);
    if (cover) {
      void extractDominantRgb(cover).then((rgb) => {
        if (this.loadGen === gen) this.ambientRgb = rgb;
      });
    } else {
      this.ambientRgb = null;
    }

    this.engine.updateMediaSession(track, {
      onPlay: () => void this.play(),
      onPause: () => this.pause(),
      onPrev: () => void this.prev(),
      onNext: () => void this.next(),
      onSeek: (s) => this.seek(s),
    });
    void this.syncMprisState();

    void this.persistSession();

    if (!isTauriRuntime()) {
      if (this.inFlightTransitionGen === gen) this.inFlightTransitionGen = null;
      this.isLoadingAudio = false;
      this.lyricsLoading = false;
      return;
    }

    try {
      const keepIds =
        useCrossfade && prevTrackId && prevTrackId !== track.id
          ? [track.id, prevTrackId]
          : [track.id];
      void api.cancelStaleStreams(keepIds);
      this.clearVideoStartWait();
      this.clearVideoSyncPause(false);

      const waitForVideo = this.videoMode && !startPaused;
      if (this.videoMode) {
        this.lyricsLoading = false;
        if (waitForVideo) {
          this.awaitingVideoStartGen = gen;
          this.videoBuffering = true;
          this.videoStartFallbackTimer = window.setTimeout(() => {
            if (this.awaitingVideoStartGen === gen) {
              this.notifyVideoUnavailable();
            }
          }, 6500);
        }
        // Resolve video stream in parallel with audio so both buffer together
        void this.loadVideoForCurrent();
      } else {
        this.videoBuffering = false;
        // Fetch lyrics in parallel with audio stream so the loading skeleton shows immediately
        void this.loadLyricsForCurrent(gen);
      }

      const stream = await api.resolveAudioStream(track);
      if (this.loadGen !== gen) return;
      this.qualityLabel = stream.quality_label;
      await this.engine.loadAndPlay(
        stream.url,
        startPositionSecs,
        useCrossfade,
        startPaused || waitForVideo,
      );
      if (this.loadGen !== gen) return;
      if (this.inFlightTransitionGen === gen) this.inFlightTransitionGen = null;
      this.audioLoadedForGen = gen;
      if (startPaused) this.isLoadingAudio = false;
      void this.syncSpinSnapshot();
      void this.syncMprisState();
      this.schedulePrefetchNext(gen);

      if (waitForVideo) {
        if (this.videoReadyEl && this.videoReadyEl.gen === gen) {
          this.startAudioAndVideoTogether(this.videoReadyEl.el);
        } else if (this.videoError != null || this.awaitingVideoStartGen !== gen) {
          this.startAudioAndVideoTogether(null);
        }
      }
      this.loadSponsorBlockForCurrent(gen);
      if (this.sidebarTab === "related") {
        void this.loadRelatedForCurrent(true);
      }
      if (this.spinStatus.remote_role === "guest") return;
      void api
        .getRadioTracks(track.id)
        .then((tracks) => {
          if (this.loadGen !== gen) return;
          const existing = new Set(this.queue.map((t) => t.id));
          const related = tracks.filter((t) => !existing.has(t.id));
          if (this.autoplayRelated && this.queue.length === 1 && related.length > 0) {
            const autoQueued = related.slice(0, 12);
            this.queue = [...this.queue, ...autoQueued];
            this.unshuffledQueue = [...this.queue];
            this.suggestedTracks = related.slice(12, 24);
            void this.persistSession();
            void this.syncSpinSnapshot();
            this.schedulePrefetchNext(gen);
          } else {
            this.suggestedTracks = related.slice(0, 12);
            this.schedulePrefetchNext(gen);
          }
        })
        .catch(() => {
          // Ignore radio fetch errors; playback continues with current queue.
        });
    } catch (err) {
      if (this.loadGen === gen) {
        if (this.inFlightTransitionGen === gen) this.inFlightTransitionGen = null;
        this.clearVideoStartWait();
        this.clearVideoSyncPause(false);
        this.isLoadingAudio = false;
        this.showToast(String(err));
      }
    }
  }

  public async play(): Promise<void> {
    if (this.spinStatus.remote_role === "guest") {
      if (!this.isPlaying) this.sendRemoteControl("play_pause");
      return;
    }
    if (!this.currentTrack || this.currentIndex == null) return;
    if (!this.engine.activeDeck.src) {
      if (this.isLoadingAudio) return;
      const resumePos = this.positionSecs > 0.5 ? this.positionSecs : 0;
      await this.playIndex(this.currentIndex, false, resumePos);
      return;
    }
    await this.engine.play();
  }

  public pause(): void {
    if (this.spinStatus.remote_role === "guest") {
      if (this.isPlaying) this.sendRemoteControl("play_pause");
      return;
    }
    this.engine.pause();
    void this.persistSession();
  }

  public async togglePlay(): Promise<void> {
    if (this.spinStatus.remote_role === "guest") {
      this.sendRemoteControl("play_pause");
      return;
    }
    if (!this.currentTrack || this.currentIndex == null) return;
    if (!this.engine.activeDeck.src) {
      if (this.isLoadingAudio) return;
      const resumePos = this.positionSecs > 0.5 ? this.positionSecs : 0;
      await this.playIndex(this.currentIndex, false, resumePos);
      return;
    }
    if (this.videoSyncPauseActive || this.awaitingVideoStartGen === this.loadGen) {
      this.clearVideoStartWait();
      this.clearVideoSyncPause(false);
      this.isLoadingAudio = false;
      this.engine.pause();
      void this.persistSession();
      return;
    }
    await this.engine.togglePlay();
    if (!this.isPlaying) void this.persistSession();
  }

  public seek(secs: number): void {
    const wasPlaying = this.isPlaying || this.videoSyncPauseActive;
    this.positionSecs = secs;
    this.seekSeq += 1;
    // Allow re-triggering segments that lie ahead of a manual seek
    for (const seg of this.sponsorSegments) {
      if (secs < seg.start - 0.25) {
        this.skippedSponsorKeys.delete(`${seg.start}:${seg.end}`);
      }
    }
    this.engine.seek(secs);
    // In video mode while playing, pause audio briefly until the <video> finishes
    // seeking to the new keyframe so audio and video resume together.
    if (this.videoMode && wasPlaying && this.videoReadyEl?.gen === this.loadGen) {
      const videoEl = this.videoReadyEl.el;
      if (videoEl.readyState >= 1) {
        const clamped = Number.isFinite(videoEl.duration)
          ? Math.min(Math.max(0, secs), Math.max(0, videoEl.duration - 0.1))
          : Math.max(0, secs);
        if (Math.abs(videoEl.currentTime - clamped) > 0.15) {
          this.clearVideoSyncPause(false);
          this.videoSyncPauseActive = true;
          this.videoBuffering = true;
          this.isLoadingAudio = true;
          this.engine.pause();
          this.videoSyncFallbackTimer = window.setTimeout(() => {
            this.clearVideoSyncPause(true);
          }, 1500);
          try {
            videoEl.currentTime = clamped;
          } catch {
            this.clearVideoSyncPause(true);
          }
        }
      }
    }
    this.updateActiveLyric(secs);
    this.syncAndroidForegroundService(this.isPlaying);
    void this.syncSpinSnapshot();
    void this.syncMprisState(true);
    void this.persistSession();
  }

  public async prev(): Promise<void> {
    if (this.spinStatus.remote_role === "guest") {
      this.sendRemoteControl("prev");
      return;
    }
    if (this.positionSecs > 3) {
      this.seek(0);
      return;
    }
    if (this.currentIndex != null && this.currentIndex > 0) {
      await this.playIndex(this.currentIndex - 1);
    }
  }

  public async next(useCrossfade = false): Promise<void> {
    if (this.spinStatus.remote_role === "guest") {
      this.sendRemoteControl("next");
      return;
    }
    if (this.currentIndex == null) return;
    let nextIdx = this.currentIndex + 1;
    while (nextIdx < this.queue.length && this.hiddenTrackIds.has(this.queue[nextIdx].id)) {
      nextIdx++;
    }
    if (nextIdx < this.queue.length) {
      await this.playIndex(nextIdx, useCrossfade);
    } else if (this.repeat && this.queue.length > 0) {
      const firstVisible = this.queue.findIndex((t) => !this.hiddenTrackIds.has(t.id));
      await this.playIndex(firstVisible >= 0 ? firstVisible : 0, useCrossfade);
    } else if (this.autoplayRelated && this.suggestedTracks.length > 0) {
      const nextTrack = this.suggestedTracks[0];
      this.suggestedTracks = this.suggestedTracks.slice(1);
      this.queue = [...this.queue, nextTrack];
      this.unshuffledQueue = [...this.unshuffledQueue, nextTrack];
      await this.playIndex(this.queue.length - 1, useCrossfade);
    }
  }

  private async handleTrackEnded(): Promise<void> {
    if (this.currentIndex == null) return;
    if (this.currentIndex + 1 < this.queue.length || this.repeat || this.autoplayRelated) {
      await this.next(false);
    }
  }

  public toggleShuffle(): void {
    this.shuffle = !this.shuffle;
    if (this.queue.length <= 1 || this.currentIndex == null) {
      void this.syncMprisState();
      return;
    }
    if (this.shuffle) {
      const res = shuffleQueueKeepingCurrent(this.queue, this.currentIndex);
      this.unshuffledQueue = res.unshuffledQueue;
      this.queue = res.queue;
      this.currentIndex = res.currentIndex;
    } else if (this.unshuffledQueue.length > 0) {
      const res = unshuffleQueue(this.unshuffledQueue, this.queue[this.currentIndex]);
      this.queue = res.queue;
      this.currentIndex = res.currentIndex;
    }
    void this.persistSession();
    void this.syncSpinSnapshot();
    void this.syncMprisState();
  }

  public toggleRepeat(): void {
    this.repeat = !this.repeat;
    void this.persistSession();
    void this.syncMprisState();
  }

  public async syncMprisState(seeked = false): Promise<void> {
    if (!isTauriRuntime() || device.isAndroid) return;
    const track = this.currentTrack;
    const duration = this.durationSecs > 0 ? this.durationSecs : (track?.duration_secs ?? 0);
    const canGoNext =
      this.currentIndex != null &&
      (this.currentIndex + 1 < this.queue.length ||
        this.repeat ||
        (this.autoplayRelated && this.suggestedTracks.length > 0));
    const canGoPrevious =
      this.currentIndex != null && (this.currentIndex > 0 || this.positionSecs > 3);
    try {
      await api.mprisSyncState({
        track_id: track?.id ?? null,
        title: track?.title ?? null,
        artist: track?.artist ?? null,
        album: track?.album ?? null,
        art_url: this.trackCoverUrl(track, true),
        duration_secs: duration,
        position_secs: this.getExactPositionSecs(),
        is_playing: this.isPlaying,
        can_go_next: canGoNext,
        can_go_previous: canGoPrevious,
        shuffle: this.shuffle,
        repeat: this.repeat,
        volume: this.volume,
        seeked,
      });
    } catch {
      // ignore on platforms without MPRIS
    }
    void this.syncDiscordRpc();
  }

  public async syncDiscordRpc(): Promise<void> {
    if (!isTauriRuntime() || device.isAndroid) return;
    const cfg = this.integrations;
    const track = this.currentTrack;
    const duration = this.durationSecs > 0 ? this.durationSecs : (track?.duration_secs ?? 0);
    try {
      await api.discordRpcSync({
        enabled: cfg.discordRpcEnabled,
        client_id: cfg.discordClientId,
        title: track?.title ?? null,
        artist: track?.artist ?? null,
        album: track?.album ?? null,
        art_url: track?.cover_large_url ?? track?.cover_url ?? null,
        duration_secs: duration,
        position_secs: this.getExactPositionSecs(),
        is_playing: this.isPlaying,
      });
    } catch {
      // ignore Discord RPC errors silently
    }
  }

  private submitScrobble(track: Track, nowPlaying: boolean, timestamp?: number): void {
    if (!isTauriRuntime() || this.isOffline) return;
    const cfg = this.integrations;
    if (cfg.listenBrainzEnabled && cfg.listenBrainzToken.trim()) {
      void api
        .scrobbleListenBrainz(cfg.listenBrainzToken, track, nowPlaying, timestamp ?? null)
        .catch(() => {});
    }
    if (
      cfg.lastFmEnabled &&
      cfg.lastFmApiKey.trim() &&
      cfg.lastFmApiSecret.trim() &&
      cfg.lastFmSessionKey.trim()
    ) {
      void api
        .scrobbleLastFm(
          {
            api_key: cfg.lastFmApiKey,
            api_secret: cfg.lastFmApiSecret,
            session_key: cfg.lastFmSessionKey,
          },
          track,
          nowPlaying,
          timestamp ?? null,
        )
        .catch(() => {});
    }
  }

  private loadLyricsForCurrent(expectedGen = this.loadGen): Promise<void> {
    const track = this.currentTrack;
    if (!track || this.videoMode || !isTauriRuntime()) {
      this.lyricsLoading = false;
      return Promise.resolve();
    }
    this.lyricsLoading = true;
    return api
      .getLyrics(track.title, track.artist, track.duration_secs, track.id)
      .then((data) => {
        if (this.loadGen === expectedGen && !this.videoMode) {
          this.lyrics = data;
          this.updateActiveLyric(this.positionSecs);
          if (this.showTranslation && data.type === "Synced") {
            void this.loadTranslationForCurrent();
          }
        }
      })
      .finally(() => {
        if (this.loadGen === expectedGen) this.lyricsLoading = false;
      });
  }

  private loadSponsorBlockForCurrent(expectedGen = this.loadGen): void {
    const track = this.currentTrack;
    if (!track || !this.sponsorBlockEnabled || this.isOffline || !isTauriRuntime()) {
      this.sponsorSegments = [];
      return;
    }
    void api
      .getSponsorBlockSegments(track.id)
      .then((segments) => {
        if (this.loadGen === expectedGen && this.sponsorBlockEnabled) {
          this.sponsorSegments = segments;
        }
      })
      .catch(() => {
        if (this.loadGen === expectedGen) this.sponsorSegments = [];
      });
  }

  private checkSponsorBlockSkip(pos: number, dur: number): void {
    if (
      !this.sponsorBlockEnabled ||
      this.sponsorSegments.length === 0 ||
      this.isLoadingAudio ||
      this.spinStatus.remote_role === "guest"
    ) {
      return;
    }
    for (const seg of this.sponsorSegments) {
      if (pos >= seg.start && pos < seg.end - 0.35) {
        const key = `${seg.start}:${seg.end}`;
        if (this.skippedSponsorKeys.has(key)) continue;
        this.skippedSponsorKeys.add(key);
        if (dur > 0 && seg.end >= dur - 0.75) {
          this.showToast(t("player.sponsorblock_skipped"));
          void this.handleTrackEnded();
        } else {
          this.seek(seg.end);
          this.showToast(t("player.sponsorblock_skipped"));
        }
        return;
      }
    }
  }

  private updateActiveLyric(pos: number): void {
    if (this.videoMode) return;
    const found = findActiveLyricIndex(this.lyrics, pos + this.currentLyricsOffset);
    if (found !== this.activeLyricIndex) {
      this.activeLyricIndex = found;
    }
  }

  public async refreshSpinStatus(): Promise<void> {
    if (!isTauriRuntime()) return;
    try {
      this.spinStatus = await api.spinStatus();
      void this.syncSpinSnapshot();
    } catch {
      // ignore
    }
  }

  private syncAndroidForegroundService(playing: boolean): void {
    const win = getAndroidWindow();
    if (!win) return;
    if (!win.__GYROTUBE_MEDIA_ACTION__) {
      win.__GYROTUBE_MEDIA_ACTION__ = (action: string) => {
        if (action === "prev") void this.prev();
        else if (action === "next") void this.next();
        else if (action === "toggle") void this.togglePlay();
        else if (action === "play" && !this.isPlaying) void this.togglePlay();
        else if (action === "pause" && this.isPlaying) void this.togglePlay();
        else if (action === "duck") this.setAudioDucking(true);
        else if (action === "unduck") this.setAudioDucking(false);
        else if (action.startsWith("seek:")) {
          const ms = Number(action.slice(5));
          if (Number.isFinite(ms) && ms >= 0) {
            this.seek(ms / 1000);
          }
        }
      };
    }
    const bridge = getAndroidBridge();
    if (!bridge?.setPlaybackActive) return;
    const cur = this.currentTrack;
    // Keep the foreground service & WebView alive during track-to-track transitions
    const keepPlaying = playing || this.inFlightTransitionGen !== null;
    const posMs = Math.max(0, Math.round(this.getExactPositionSecs() * 1000));
    const durSecs = this.durationSecs > 0 ? this.durationSecs : (cur?.duration_secs ?? 0);
    const durMs = Math.max(0, Math.round(durSecs * 1000));
    bridge.setPlaybackActive(
      keepPlaying,
      cur?.title ?? "Gyro",
      cur?.artist ?? "",
      this.trackCoverUrl(cur, false) ?? "",
      Boolean(cur),
      posMs,
      durMs,
    );
  }

  public async syncSpinSnapshot(): Promise<void> {
    if (!isTauriRuntime()) return;
    if (!this.spinStatus.local_active && this.spinStatus.remote_role !== "host") return;
    try {
      await api.spinSyncSnapshot({
        isPlaying: this.isPlaying,
        positionSecs: this.positionSecs,
        sourceName: this.sourceName,
        currentIndex: this.currentIndex,
        queue: this.queue,
        displayName: this.spinDisplayName || undefined,
      });
    } catch {
      // ignore
    }
  }

  public createUserPlaylist(
    name: string,
    description = "",
    colorHex: string | null = "#10b981",
    iconName: string | null = null,
    tracks: Track[] = [],
    coverPath: string | null = null,
  ): UserPlaylist {
    const clean = name.trim() || "New playlist";
    const pl: UserPlaylist = {
      id: `pl-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
      name: clean,
      description: description.trim(),
      color_hex: colorHex,
      icon_name: iconName,
      cover_path: coverPath,
      tracks,
    };
    this.userPlaylists = [pl, ...this.userPlaylists];
    if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
    return pl;
  }

  public updateUserPlaylist(
    id: string,
    name: string,
    description: string,
    colorHex: string | null,
    iconName: string | null,
    coverPath?: string | null,
  ): void {
    const clean = name.trim();
    if (!clean) return;
    this.userPlaylists = this.userPlaylists.map((p) =>
      p.id === id
        ? {
            ...p,
            name: clean,
            description: description.trim(),
            color_hex: colorHex,
            icon_name: iconName,
            cover_path: coverPath !== undefined ? coverPath : p.cover_path,
          }
        : p,
    );
    if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
  }

  public deleteUserPlaylist(id: string): void {
    this.userPlaylists = this.userPlaylists.filter((p) => p.id !== id);
    if (this.pinnedPlaylistIds.includes(id)) {
      this.pinnedPlaylistIds = this.pinnedPlaylistIds.filter((x) => x !== id);
      this.persistPinnedPlaylistIds();
    }
    if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
    if (this.downloadedPlaylistKeys.has(id)) {
      const next = new Set(this.downloadedPlaylistKeys);
      next.delete(id);
      this.downloadedPlaylistKeys = next;
      this.persistDownloadedPlaylistKeys();
      void this.gcOrphanedDownloads();
    }
  }

  public addTrackToPlaylist(playlistId: string, track: Track): void {
    let added = false;
    let duplicateName: string | null = null;
    this.userPlaylists = this.userPlaylists.map((p) => {
      if (p.id !== playlistId) return p;
      if (p.tracks.some((t) => t.id === track.id)) {
        duplicateName = p.name;
        return p;
      }
      added = true;
      return { ...p, tracks: [...p.tracks, track] };
    });
    if (duplicateName) {
      this.showToast(
        t("collection.duplicate_in_playlist", { title: track.title, playlist: duplicateName }),
      );
      return;
    }
    if (added) {
      if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
      if (this.downloadedPlaylistKeys.has(playlistId)) {
        this.enqueueOfflineDownloads([track]);
      }
      this.showToast(t("toast.added_to_playlist", { title: track.title }));
    }
  }

  public movePlaylistTrack(playlistId: string, from: number, to: number): void {
    if (from === to) return;
    let changed = false;
    this.userPlaylists = this.userPlaylists.map((p) => {
      if (p.id !== playlistId) return p;
      if (from < 0 || to < 0 || from >= p.tracks.length || to >= p.tracks.length) return p;
      const copy = [...p.tracks];
      const [moved] = copy.splice(from, 1);
      copy.splice(to, 0, moved);
      changed = true;
      return { ...p, tracks: copy };
    });
    if (changed && isTauriRuntime()) {
      void api.saveUserPlaylists(this.userPlaylists);
    }
  }

  public tryOpenYtmLink(raw: string): boolean {
    const parsed = parseYtmUrl(raw);
    if (!parsed || !isTauriRuntime() || this.isOffline) return false;

    if (parsed.kind === "playlist") {
      this.openAlbum(parsed.id, "Playlist", null, true);
      return true;
    }
    if (parsed.kind === "album") {
      this.openAlbum(parsed.id, t("album.badge_album"));
      return true;
    }
    if (parsed.kind === "artist") {
      this.openArtist(parsed.id, t("explore.artist_pill"));
      return true;
    }
    if (parsed.kind === "track") {
      this.replaceQueueAndPlay([createYtmTrackStub(parsed.id)], 0, "YouTube Music");
      return true;
    }
    return false;
  }

  public setEqEnabled(enabled: boolean): void {
    this.eqSettings = { ...this.eqSettings, enabled };
    saveEqSettings(this.eqSettings);
    this.engine.applyEqSettings(this.eqSettings);
  }

  public setEqNormalize(normalize: boolean): void {
    this.eqSettings = { ...this.eqSettings, normalize };
    saveEqSettings(this.eqSettings);
    this.engine.applyEqSettings(this.eqSettings);
  }

  public setEqPreset(preset: Exclude<EqPresetId, "custom">): void {
    const bands: [number, number, number, number, number] = [...EQ_PRESETS[preset]];
    this.eqSettings = { ...this.eqSettings, enabled: true, preset, bands };
    saveEqSettings(this.eqSettings);
    this.engine.applyEqSettings(this.eqSettings);
  }

  public setEqBand(index: number, db: number): void {
    if (index < 0 || index >= 5) return;
    const clamped = Math.max(-12, Math.min(12, Math.round(db)));
    const bands: [number, number, number, number, number] = [...this.eqSettings.bands];
    bands[index] = clamped;
    this.eqSettings = { ...this.eqSettings, enabled: true, preset: "custom", bands };
    saveEqSettings(this.eqSettings);
    this.engine.applyEqSettings(this.eqSettings);
  }

  private trackListeningProgress(pos: number): void {
    const cur = this.currentTrack;
    if (!cur || !this.isPlaying) return;
    if (this.statsTrackId !== cur.id) {
      this.statsTrackId = cur.id;
      this.statsRecordedForTrack = false;
      this.statsLastPos = pos;
      this.scrobbleTrackId = cur.id;
      this.scrobbleStartedAt = Math.floor(Date.now() / 1000);
      this.scrobbleListenedSecs = 0;
      this.scrobbleSubmitted = false;
      this.scrobbleNowPlayingSent = true;
      this.submitScrobble(cur, true);
      return;
    }
    if (!this.scrobbleNowPlayingSent && this.scrobbleTrackId === cur.id) {
      this.scrobbleNowPlayingSent = true;
      this.submitScrobble(cur, true);
    }
    const delta = pos - this.statsLastPos;
    this.statsLastPos = pos;
    if (delta > 0 && delta < 5) {
      this.statsUnsavedSecs += delta;
      this.scrobbleListenedSecs += delta;
      if (this.statsUnsavedSecs >= 10) {
        const add = Math.floor(this.statsUnsavedSecs);
        this.statsUnsavedSecs -= add;
        this.listeningStats = {
          ...this.listeningStats,
          totalSeconds: this.listeningStats.totalSeconds + add,
        };
        saveListeningStats(this.listeningStats);
      }
    }
    if (!this.statsRecordedForTrack && pos >= 30) {
      this.statsRecordedForTrack = true;
      this.listeningStats = recordTrackPlay(this.listeningStats, cur);
      saveListeningStats(this.listeningStats);
    }
    if (!this.scrobbleSubmitted && this.scrobbleTrackId === cur.id) {
      const dur = this.durationSecs > 0 ? this.durationSecs : (cur.duration_secs ?? 0);
      const threshold = dur > 0 ? Math.max(30, Math.min(dur * 0.5, 240)) : 30;
      if (this.scrobbleListenedSecs >= threshold) {
        this.scrobbleSubmitted = true;
        this.submitScrobble(cur, false, this.scrobbleStartedAt || Math.floor(Date.now() / 1000));
      }
    }
  }

  public updateArtistStatAvatar(artistName: string, artistId: string | null, avatarUrl: string): void {
    const key = artistName.trim().toLowerCase();
    const existing = this.listeningStats.artists[key];
    if (!existing) return;
    if (existing.avatarUrl === avatarUrl && (existing.id || !artistId)) return;
    this.listeningStats = {
      ...this.listeningStats,
      artists: {
        ...this.listeningStats.artists,
        [key]: {
          ...existing,
          id: existing.id ?? artistId,
          avatarUrl,
        },
      },
    };
    saveListeningStats(this.listeningStats);
  }

  public resetListeningStats(): void {
    this.listeningStats = emptyStats();
    saveListeningStats(this.listeningStats);
    this.showToast(t("stats.reset_done"));
  }

  public removeTrackFromPlaylist(playlistId: string, trackId: string): void {
    this.userPlaylists = this.userPlaylists.map((p) =>
      p.id === playlistId ? { ...p, tracks: p.tracks.filter((t) => t.id !== trackId) } : p,
    );
    if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
    if (this.downloadedPlaylistKeys.has(playlistId)) {
      void this.gcOrphanedDownloads();
    }
  }

  private buildSessionSnapshot(): SessionState {
    return {
      sidebar_collapsed: !this.sidebarOpen,
      queue: this.queue,
      unshuffled_queue: this.unshuffledQueue,
      current_index: this.currentIndex,
      source_name: this.sourceName,
      user_volume: this.volume,
      shuffle: this.shuffle,
      repeat: this.repeat,
      position_secs: this.getExactPositionSecs(),
      crossfade_secs: this.crossfadeSecs,
      language: this.language,
      background_playback: this.backgroundPlayback,
      autoplay_related: this.autoplayRelated,
      theme_mode: this.themeMode,
      audio_quality: this.audioQuality,
    };
  }

  public async persistSession(): Promise<void> {
    this.persister.schedule(() => this.buildSessionSnapshot());
  }

  public flushSessionNow(): void {
    this.persister.flushNow(() => this.buildSessionSnapshot());
  }
}

export const player = new PlayerStore();
