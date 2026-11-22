import { api, isTauriRuntime } from "../api";
import { DualDeckAudioEngine, extractDominantRgb } from "../audioEngine";
import { hashToView, isSameView, type RouteState, viewToHash } from "../router";
import { contextMenu } from "./contextMenu.svelte";
import { device } from "./device.svelte";
import { bindTauriPlayerEvents } from "./nativeEvents";
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
  readThemeMode,
  SessionPersister,
  writeChartCountry,
} from "./sessionPersistence";
import type {
  ActiveView,
  AppLanguage,
  Artist,
  Genre,
  HomeFeed,
  LyricsData,
  MainTab,
  RelatedFeed,
  SearchResults,
  SessionState,
  SpinStatus,
  ThemeMode,
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
  collectionDetailOpen = $state(false);
  sidebarOpen = $state(true);
  navCollapsed = $state(false);
  sidebarTab = $state<"player" | "lyrics" | "queue" | "related">("player");
  chartCountry = $state<string | null>(readChartCountry());
  relatedFeed = $state<RelatedFeed | null>(null);
  relatedTrackId = $state<string | null>(null);
  relatedLoading = $state(false);
  spinOpen = $state(false);
  partyMode = $state(false);
  settingsOpen = $state(false);
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
  themeMode = $state<ThemeMode>(readThemeMode());
  language = $state<AppLanguage>("Auto");

  // Dynamic ambient color extracted from current cover
  ambientRgb = $state<[number, number, number] | null>(null);

  // Lyrics
  lyrics = $state<LyricsData>({ type: "None" });
  lyricsLoading = $state(false);
  activeLyricIndex = $state<number | null>(null);

  // Related / suggested tracks for Autoplay
  suggestedTracks = $state<Track[]>([]);

  // User data
  favorites = $state<Track[]>([]);
  recentTracks = $state<Track[]>([]);
  userPlaylists = $state<UserPlaylist[]>([]);
  searchHistory = $state<string[]>([]);
  followedArtists = $state<Artist[]>([]);

  // Video mode (hybrid synced <video muted> alongside master audio deck)
  videoMode = $state(false);
  videoStream = $state<VideoStreamInfo | null>(null);
  videoLoading = $state(false);
  videoError = $state<string | null>(null);
  counterpartTrack = $state<Track | null>(null);
  private videoGen = 0;
  private counterpartGen = 0;
  private counterpartCache = new Map<string, Track | null>();
  private awaitingVideoStartGen: number | null = null;
  private audioLoadedForGen = 0;
  private videoReadyEl: { gen: number; el: HTMLVideoElement } | null = null;
  private videoStartFallbackTimer: number | null = null;

  // Catalog data
  homeFeed = $state<HomeFeed | null>(null);
  homeLoading = $state(false);
  genres = $state<Genre[]>([]);
  genresLoading = $state(false);
  searchQuery = $state("");
  searchResults = $state<SearchResults | null>(null);
  searchLoading = $state(false);

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
  });

  // Network & offline downloads
  isOffline = $state(typeof navigator !== "undefined" ? !navigator.onLine : false);
  downloadedTrackIds = $state<Set<string>>(new Set());
  offlineCoverBaseUrl = $state<string | null>(null);
  downloadingPlaylistKey = $state<string | null>(null);
  downloadProgress = $state<{ done: number; total: number } | null>(null);

  // Toasts
  toasts = $state<ToastItem[]>([]);
  private toastSeq = 0;
  private loadGen = 0;
  private inFlightTransitionGen: number | null = null;
  private advancedFromGen = 0;
  private prefetchTimer: number | null = null;
  private lastLocalSpinUpdate = 0;
  private persister = new SessionPersister();
  private routeDepth = 0;
  private routerBound = false;
  private engine: DualDeckAudioEngine;

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

  private favoriteIdSet = $derived(new Set(this.favorites.map((t) => t.id)));
  private followedArtistIdSet = $derived(new Set(this.followedArtists.map((a) => a.id)));

  constructor() {
    this.engine = new DualDeckAudioEngine({
      onTimeUpdate: (pos, dur) => {
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
        const now = performance.now();
        if (this.spinStatus.local_active && this.isPlaying && !this.isLoadingAudio && now - this.lastLocalSpinUpdate >= 500) {
          this.lastLocalSpinUpdate = now;
          void this.syncSpinSnapshot();
        }
      },
      onPlayStateChange: (playing) => {
        this.isPlaying = playing;
        if (playing) this.isLoadingAudio = false;
        this.syncAndroidForegroundService(playing);
        void this.syncSpinSnapshot();
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
      onError: (msg) => {
        this.isLoadingAudio = false;
        this.showToast(msg);
      },
    });
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
      this.followedArtists = initial.followed_artists ?? [];

      const s = initial.session;
      this.sidebarOpen = !s.sidebar_collapsed;
      this.queue = s.queue;
      this.unshuffledQueue = s.unshuffled_queue;
      this.currentIndex = s.current_index;
      this.sourceName = s.source_name || "Not playing";
      this.volume = device.isAndroid ? 1.0 : (s.user_volume ?? 1.0);
      this.shuffle = s.shuffle;
      this.repeat = s.repeat;
      this.crossfadeSecs = s.crossfade_secs ?? 0;
      this.autoplayRelated = s.autoplay_related ?? true;
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
        this.videoMode = Boolean(restored.is_video);
        this.probeCounterpartForCurrent(restored);
        void api.prefetchAudio(restored).catch(() => {
          // Playback retries if the background preparation fails.
        });
        this.durationSecs = restored.duration_secs ?? 0;
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
            if (this.currentTrack?.id === restored.id) this.lyrics = data;
          })
          .finally(() => {
            if (this.currentTrack?.id === restored.id) this.lyricsLoading = false;
          });
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
    if (typeof window === "undefined") return;
    const win = window as unknown as {
      __GYROTUBE_ANDROID__?: {
        consumePendingSpinLink?: () => string;
      };
      __GYROTUBE_SPIN_LINK__?: (code: string) => void;
    };
    win.__GYROTUBE_SPIN_LINK__ = (code: string) => {
      void this.joinSpinFromDeepLink(code);
    };
    const pending = win.__GYROTUBE_ANDROID__?.consumePendingSpinLink?.();
    if (pending && pending.trim()) {
      void this.joinSpinFromDeepLink(pending.trim());
    }
  }

  public async joinSpinFromDeepLink(code: string): Promise<void> {
    const clean = code.trim();
    if (!clean || !isTauriRuntime()) return;
    try {
      this.spinStatus = await api.spinJoinRemote(clean);
      this.showToast("SPIN · Connecting…");
    } catch (err) {
      this.showToast(String(err));
    }
  }

  private async bindNativeEvents(): Promise<void> {
    await bindTauriPlayerEvents({
      onTogglePlay: () => void this.togglePlay(),
      onNext: () => void this.next(),
      onPrev: () => void this.prev(),
      onMoveQueue: (from, to) => this.moveQueueItem(from, to),
      onEnqueueTrack: (track, guestName) => {
        this.enqueueTrack(track);
        this.showToast(`${guestName} added ${track.title}`);
      },
      onRemoteSync: (payload, error) => {
        if (error) {
          this.showToast(error);
          return;
        }
        if (!payload) return;
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
        if (trackChanged || (!this.engine.activeDeck.src && !this.isLoadingAudio)) {
          this.engine.stop();
          const track = { ...payload.track, is_music: false };
          this.queue = [track];
          this.unshuffledQueue = [track];
          this.sourceName = `SPIN · ${payload.host_name || "Host"}`;
          void this.playIndex(0, false, targetPos, !payload.is_playing);
        } else if (!this.isLoadingAudio) {
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
      },
    });
  }

  public applyTheme(mode: ThemeMode): void {
    this.themeMode = mode;
    applyThemeToDocument(mode);
  }

  public setThemeMode(mode: ThemeMode): void {
    this.applyTheme(mode);
    void this.persistSession();
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

  public setBackgroundPlayback(enabled: boolean): void {
    this.backgroundPlayback = enabled;
    void this.persistSession();
  }

  public setVolume(v: number): void {
    if (device.isAndroid) {
      this.volume = 1.0;
      this.engine.setVolume(1.0);
      return;
    }
    this.volume = Math.max(0, Math.min(1, v));
    this.engine.setVolume(this.volume);
    void this.persistSession();
  }

  private formatErrorMessage(raw: string): string {
    const msg = raw.trim();
    const lower = msg.toLowerCase();
    const isEs =
      this.language === "Es" ||
      (this.language === "Auto" &&
        typeof navigator !== "undefined" &&
        (navigator.language || "").toLowerCase().startsWith("es"));

    if (
      this.isOffline ||
      lower.includes("dns error") ||
      lower.includes("failed to lookup address") ||
      lower.includes("error sending request") ||
      lower.includes("network error") ||
      lower.includes("connection refused") ||
      lower.includes("connect error")
    ) {
      return isEs ? "Sin conexión a internet" : "No internet connection";
    }
    if (lower.includes("timed out") || lower.includes("timeout")) {
      return isEs ? "Tiempo de espera agotado. Intenta de nuevo." : "Request timed out. Please try again.";
    }
    if (lower.includes("http 403") || lower.includes("http 410") || lower.includes("unavailable")) {
      return isEs ? "Esta canción no está disponible en este momento" : "This track is currently unavailable";
    }
    if (lower.includes("not supported") || lower.includes("audio error (code")) {
      return isEs ? "No se pudo reproducir el audio de esta canción" : "Could not play audio for this track";
    }
    return msg.length > 140 ? `${msg.slice(0, 137)}…` : msg;
  }

  public showToast(message: string): void {
    const clean = this.formatErrorMessage(message);
    const id = ++this.toastSeq;
    this.toasts = [...this.toasts, { id, message: clean }];
    window.setTimeout(() => {
      this.toasts = this.toasts.filter((t) => t.id !== id);
    }, 3200);
  }

  public isTrackDownloaded(trackId: string): boolean {
    return this.downloadedTrackIds.has(trackId);
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
    } catch {
      // ignore
    }
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
        if (this.isOffline) break;
        try {
          await api.downloadTrackOffline(tr);
          const next = new Set(this.downloadedTrackIds);
          next.add(tr.id);
          this.downloadedTrackIds = next;
          done += 1;
          this.downloadProgress = { done, total };
        } catch {
          // Continue downloading remaining tracks even if one fails
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

  private isMobilePlayerHistoryEnabled(): boolean {
    if (typeof window === "undefined") return false;
    const win = window as unknown as { __GYROTUBE_ANDROID__?: unknown };
    const isMobilePlatform =
      Boolean(win.__GYROTUBE_ANDROID__) ||
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
    const initialState: RouteState = {
      view: initial.view,
      tab: initial.tab,
      depth: 0,
      playerOpen: useMobilePlayerHistory ? initial.playerOpen : false,
    };
    window.history.replaceState(
      initialState,
      "",
      viewToHash(initial.view, Boolean(initialState.playerOpen)),
    );

    window.addEventListener("popstate", (event) => {
      // On Android / desktop Back button, dismiss open modals before navigating away
      if (this.dismissTopOverlay()) {
        const keepPlayer = this.isMobilePlayerHistoryEnabled() && this.sidebarOpen;
        const plainView = $state.snapshot(this.activeView);
        const keep: RouteState = {
          view: plainView,
          tab: this.activeTab,
          depth: this.routeDepth + 1,
          playerOpen: keepPlayer,
        };
        this.routeDepth = keep.depth;
        window.history.pushState(keep, "", viewToHash(plainView, keepPlayer));
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
    });

    (
      window as unknown as { __GYROTUBE_DISMISS_OVERLAY__?: () => boolean }
    ).__GYROTUBE_DISMISS_OVERLAY__ = () => this.dismissTopOverlay();
  }

  public openPlayerSheet(): void {
    if (this.sidebarOpen) return;
    this.sidebarOpen = true;
    if (this.isMobilePlayerHistoryEnabled()) {
      this.routeDepth += 1;
      const plainView = $state.snapshot(this.activeView);
      const state: RouteState = {
        view: plainView,
        tab: this.activeTab,
        depth: this.routeDepth,
        playerOpen: true,
      };
      window.history.pushState(state, "", viewToHash(plainView, true));
    } else {
      void this.persistSession();
    }
  }

  public closePlayerSheet(): void {
    if (!this.sidebarOpen) return;
    if (this.isMobilePlayerHistoryEnabled()) {
      const st = window.history.state as RouteState | null;
      if (st?.playerOpen && this.routeDepth > 0) {
        window.history.back();
        return;
      }
      this.sidebarOpen = false;
      const plainView = $state.snapshot(this.activeView);
      const nextState: RouteState = {
        view: plainView,
        tab: this.activeTab,
        depth: this.routeDepth,
        playerOpen: false,
      };
      window.history.replaceState(nextState, "", viewToHash(plainView, false));
      return;
    }
    this.sidebarOpen = false;
    void this.persistSession();
  }

  public togglePlayerSheet(): void {
    if (this.sidebarOpen) {
      this.closePlayerSheet();
    } else {
      this.openPlayerSheet();
    }
  }

  private dismissTopOverlay(): boolean {
    if (contextMenu.open) {
      contextMenu.close();
      return true;
    }
    if (this.partyMode) {
      this.partyMode = false;
      return true;
    }
    if (this.settingsOpen) {
      this.settingsOpen = false;
      return true;
    }
    if (this.spinOpen) {
      this.spinOpen = false;
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
    const replacePlayerEntry = useMobilePlayerHistory && Boolean(currentState?.playerOpen);

    // Navigating from the open mobile player to the exact view already underneath:
    // pop the `?player=1` entry so we don't duplicate the underlying view in history.
    if (!changed && replacePlayerEntry && this.routeDepth > 0) {
      window.history.back();
      return;
    }

    if (wasCompactPlayer) {
      this.sidebarOpen = false;
    }
    this.activeTab = tab;
    this.activeView = view;
    if (typeof window !== "undefined" && (changed || replacePlayerEntry)) {
      const plainView = $state.snapshot(view);
      if (replacePlayerEntry) {
        const state: RouteState = {
          view: plainView,
          tab,
          depth: this.routeDepth,
          playerOpen: false,
        };
        window.history.replaceState(state, "", viewToHash(plainView, false));
      } else {
        this.routeDepth += 1;
        const state: RouteState = {
          view: plainView,
          tab,
          depth: this.routeDepth,
          playerOpen: false,
        };
        window.history.pushState(state, "", viewToHash(plainView, false));
      }
    }
  }

  public goBack(): void {
    if (typeof window !== "undefined" && this.routeDepth > 0) {
      window.history.back();
      return;
    }
    this.navigateTab(this.activeTab === "home" ? "home" : "explore");
  }

  public navigateTab(tab: MainTab): void {
    if (tab === "explore" && this.activeTab === "explore" && this.activeView.kind === "explore") {
      this.clearSearch();
    }
    this.commitRoute({ kind: tab }, tab);
  }

  public clearSearch(): void {
    this.searchQuery = "";
    this.searchResults = null;
    this.searchLoading = false;
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

  public async runSearch(raw: string): Promise<void> {
    if (this.isOffline) return;
    const q = raw.trim();
    if (!q) return;
    this.searchQuery = q;
    this.commitRoute({ kind: "explore" }, "explore");
    this.searchHistory = [q, ...this.searchHistory.filter((h) => h.toLowerCase() !== q.toLowerCase())].slice(0, 8);
    if (isTauriRuntime()) {
      void api.saveSearchHistory(this.searchHistory);
      this.searchLoading = true;
      try {
        this.searchResults = await api.searchAll(q);
      } catch (err) {
        this.showToast(String(err));
      } finally {
        this.searchLoading = false;
      }
    }
  }

  public clearSearchHistory(): void {
    this.searchHistory = [];
    if (isTauriRuntime()) void api.saveSearchHistory([]);
  }

  public removeSearchHistoryItem(term: string): void {
    this.searchHistory = this.searchHistory.filter((item) => item !== term);
    if (isTauriRuntime()) void api.saveSearchHistory(this.searchHistory);
  }

  public removeRecentTrack(trackId: string): void {
    this.recentTracks = this.recentTracks.filter((t) => t.id !== trackId);
    if (isTauriRuntime()) void api.saveRecentTracks(this.recentTracks);
  }

  public clearRecentTracks(): void {
    this.recentTracks = [];
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
        void this.loadVideoForCurrent();
      }
      return;
    }

    const cacheKey = `${cur.id}:${enabled ? "v" : "s"}`;
    const counterpart = this.counterpartTrack ?? this.counterpartCache.get(cacheKey) ?? null;
    if (!counterpart || counterpart.id === cur.id) {
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

  private startAudioAndVideoTogether(videoEl: HTMLVideoElement | null): void {
    this.clearVideoStartWait();
    const deck = this.engine.activeDeck;
    if (videoEl && Number.isFinite(deck.currentTime)) {
      const target = Math.max(0, deck.currentTime);
      const clamped = Number.isFinite(videoEl.duration)
        ? Math.min(target, Math.max(0, videoEl.duration - 0.1))
        : target;
      if (Math.abs(videoEl.currentTime - clamped) > 0.08 && !videoEl.seeking) {
        try {
          videoEl.currentTime = clamped;
        } catch {
          // ignore
        }
      }
      videoEl.playbackRate = 1;
      void videoEl.play().catch(() => {});
    }
    void deck.play().catch(() => {
      this.isLoadingAudio = false;
    });
  }

  public notifyVideoReady(videoEl: HTMLVideoElement): void {
    this.videoReadyEl = { gen: this.loadGen, el: videoEl };
    if (
      this.awaitingVideoStartGen === this.loadGen &&
      this.audioLoadedForGen === this.loadGen
    ) {
      this.startAudioAndVideoTogether(videoEl);
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
    }
  }

  public async loadVideoForCurrent(): Promise<void> {
    const track = this.currentTrack;
    if (!track || !this.videoMode || !isTauriRuntime()) {
      this.videoStream = null;
      this.videoLoading = false;
      this.videoError = null;
      return;
    }
    const gen = ++this.videoGen;
    this.videoLoading = true;
    this.videoError = null;
    this.videoStream = null;
    const maxHeight = typeof window !== "undefined" && window.innerWidth < 768 ? 720 : 1080;
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
    if (this.isFavorite(track.id)) {
      this.favorites = this.favorites.filter((t) => t.id !== track.id);
    } else {
      const withDuration =
        track.duration_secs == null && this.currentTrack?.id === track.id && this.durationSecs > 0
          ? { ...track, duration_secs: Math.round(this.durationSecs) }
          : track;
      this.favorites = [withDuration, ...this.favorites];
    }
    if (isTauriRuntime()) void api.saveFavorites(this.favorites);
  }

  public replaceQueueAndPlay(tracks: Track[], index: number, source: string): void {
    if (!tracks.length) return;
    this.queue = [...tracks];
    this.unshuffledQueue = [...tracks];
    this.sourceName = source;
    this.shuffle = false;
    void this.playIndex(Math.max(0, Math.min(index, tracks.length - 1)));
  }

  public enqueueTrack(track: Track): void {
    this.queue = [...this.queue, track];
    this.showToast(`Added to queue: ${track.title}`);
    if (this.currentIndex == null) {
      void this.playIndex(0);
    } else {
      void this.persistSession();
      void this.syncSpinSnapshot();
    }
  }

  public async startRadioFromCurrent(): Promise<void> {
    const current = this.currentTrack;
    if (!current || !isTauriRuntime() || this.isOffline) return;
    try {
      const radio = await api.getRadioTracks(current.id);
      const unique = radio.filter((t) => t.id !== current.id);
      if (unique.length === 0) {
        this.showToast("No related radio tracks found");
        return;
      }
      this.queue = [current, ...unique];
      this.unshuffledQueue = [...this.queue];
      this.currentIndex = 0;
      this.shuffle = false;
      this.sourceName = `Radio · ${current.title}`;
      void this.persistSession();
      void this.syncSpinSnapshot();
      this.showToast(`Radio started · ${unique.length} songs queued`);
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
      this.showToast("Queue cleared");
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
    this.videoStream = null;
    this.videoError = null;
    this.videoMode = Boolean(track.is_video);
    if (track.is_video) {
      this.partyMode = false;
      if (this.sidebarTab === "lyrics") {
        this.sidebarTab = "player";
      }
    }
    this.probeCounterpartForCurrent(track);

    // Record recent
    this.recentTracks = [track, ...this.recentTracks.filter((t) => t.id !== track.id)].slice(0, 30);
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
      onPlay: () => void this.togglePlay(),
      onPause: () => void this.togglePlay(),
      onPrev: () => void this.prev(),
      onNext: () => void this.next(),
      onSeek: (s) => this.seek(s),
    });

    void this.persistSession();

    if (!isTauriRuntime()) {
      if (this.inFlightTransitionGen === gen) this.inFlightTransitionGen = null;
      this.isLoadingAudio = false;
      return;
    }

    try {
      const keepIds =
        useCrossfade && prevTrackId && prevTrackId !== track.id
          ? [track.id, prevTrackId]
          : [track.id];
      void api.cancelStaleStreams(keepIds);
      this.clearVideoStartWait();

      const waitForVideo = this.videoMode && !startPaused;
      if (this.videoMode) {
        this.lyricsLoading = false;
        if (waitForVideo) {
          this.awaitingVideoStartGen = gen;
          this.videoStartFallbackTimer = window.setTimeout(() => {
            if (this.awaitingVideoStartGen === gen) {
              this.notifyVideoUnavailable();
            }
          }, 6500);
        }
        // Resolve video stream in parallel with audio so both buffer together
        void this.loadVideoForCurrent();
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
      this.schedulePrefetchNext(gen);

      if (waitForVideo) {
        if (this.videoReadyEl && this.videoReadyEl.gen === gen) {
          this.startAudioAndVideoTogether(this.videoReadyEl.el);
        } else if (this.videoError != null || this.awaitingVideoStartGen !== gen) {
          this.startAudioAndVideoTogether(null);
        }
      } else if (!this.videoMode) {
        // Load lyrics asynchronously only in song mode
        void this.loadLyricsForCurrent(gen);
      }
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
        this.isLoadingAudio = false;
        this.showToast(String(err));
      }
    }
  }

  public async togglePlay(): Promise<void> {
    if (!this.currentTrack || this.currentIndex == null) return;
    if (!this.engine.activeDeck.src) {
      if (this.isLoadingAudio) return;
      await this.playIndex(this.currentIndex);
      return;
    }
    await this.engine.togglePlay();
  }

  public seek(secs: number): void {
    this.positionSecs = secs;
    this.engine.seek(secs);
    this.updateActiveLyric(secs);
    this.syncAndroidForegroundService(this.isPlaying);
    void this.syncSpinSnapshot();
  }

  public async prev(): Promise<void> {
    if (this.positionSecs > 3) {
      this.seek(0);
      return;
    }
    if (this.currentIndex != null && this.currentIndex > 0) {
      await this.playIndex(this.currentIndex - 1);
    }
  }

  public async next(useCrossfade = false): Promise<void> {
    if (this.currentIndex == null) return;
    if (this.currentIndex + 1 < this.queue.length) {
      await this.playIndex(this.currentIndex + 1, useCrossfade);
    } else if (this.repeat && this.queue.length > 0) {
      await this.playIndex(0, useCrossfade);
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
    if (this.queue.length <= 1 || this.currentIndex == null) return;
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
  }

  public toggleRepeat(): void {
    this.repeat = !this.repeat;
    void this.persistSession();
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
        }
      })
      .finally(() => {
        if (this.loadGen === expectedGen) this.lyricsLoading = false;
      });
  }

  private updateActiveLyric(pos: number): void {
    if (this.videoMode) return;
    const found = findActiveLyricIndex(this.lyrics, pos);
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
    if (typeof window === "undefined") return;
    const win = window as unknown as {
      __GYROTUBE_ANDROID__?: {
        setPlaybackActive: (
          playing: boolean,
          title: string,
          artist: string,
          coverUrl: string,
          hasTrack: boolean,
          positionMs: number,
          durationMs: number,
        ) => void;
      };
      __GYROTUBE_MEDIA_ACTION__?: (action: string) => void;
    };
    if (!win.__GYROTUBE_MEDIA_ACTION__) {
      win.__GYROTUBE_MEDIA_ACTION__ = (action: string) => {
        if (action === "prev") void this.prev();
        else if (action === "next") void this.next();
        else if (action === "toggle") void this.togglePlay();
        else if (action === "play" && !this.isPlaying) void this.togglePlay();
        else if (action === "pause" && this.isPlaying) void this.togglePlay();
        else if (action.startsWith("seek:")) {
          const ms = Number(action.slice(5));
          if (Number.isFinite(ms) && ms >= 0) {
            this.seek(ms / 1000);
          }
        }
      };
    }
    const bridge = win.__GYROTUBE_ANDROID__;
    if (!bridge) return;
    const cur = this.currentTrack;
    // Keep the foreground service & WebView alive during track-to-track transitions
    const keepPlaying = playing || this.inFlightTransitionGen !== null;
    const posMs = Math.max(0, Math.round(this.getExactPositionSecs() * 1000));
    const durSecs = this.durationSecs > 0 ? this.durationSecs : (cur?.duration_secs ?? 0);
    const durMs = Math.max(0, Math.round(durSecs * 1000));
    bridge.setPlaybackActive(
      keepPlaying,
      cur?.title ?? "GyroTube",
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
  ): UserPlaylist {
    const clean = name.trim() || "New playlist";
    const pl: UserPlaylist = {
      id: `pl-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
      name: clean,
      description: description.trim(),
      color_hex: colorHex,
      icon_name: iconName,
      cover_path: null,
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
  ): void {
    const clean = name.trim();
    if (!clean) return;
    this.userPlaylists = this.userPlaylists.map((p) =>
      p.id === id
        ? { ...p, name: clean, description: description.trim(), color_hex: colorHex, icon_name: iconName }
        : p,
    );
    if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
  }

  public deleteUserPlaylist(id: string): void {
    this.userPlaylists = this.userPlaylists.filter((p) => p.id !== id);
    if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
  }

  public addTrackToPlaylist(playlistId: string, track: Track): void {
    let added = false;
    this.userPlaylists = this.userPlaylists.map((p) => {
      if (p.id !== playlistId) return p;
      if (p.tracks.some((t) => t.id === track.id)) return p;
      added = true;
      return { ...p, tracks: [...p.tracks, track] };
    });
    if (added) {
      if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
      this.showToast(`Added "${track.title}" to playlist`);
    }
  }

  public removeTrackFromPlaylist(playlistId: string, trackId: string): void {
    this.userPlaylists = this.userPlaylists.map((p) =>
      p.id === playlistId ? { ...p, tracks: p.tracks.filter((t) => t.id !== trackId) } : p,
    );
    if (isTauriRuntime()) void api.saveUserPlaylists(this.userPlaylists);
  }

  public async persistSession(): Promise<void> {
    this.persister.schedule(
      (): SessionState => ({
        sidebar_collapsed: !this.sidebarOpen,
        queue: this.queue,
        unshuffled_queue: this.unshuffledQueue,
        current_index: this.currentIndex,
        source_name: this.sourceName,
        user_volume: this.volume,
        shuffle: this.shuffle,
        repeat: this.repeat,
        position_secs: this.positionSecs,
        crossfade_secs: this.crossfadeSecs,
        language: this.language,
        background_playback: this.backgroundPlayback,
        autoplay_related: this.autoplayRelated,
        theme_mode: this.themeMode,
      }),
    );
  }
}

export const player = new PlayerStore();
