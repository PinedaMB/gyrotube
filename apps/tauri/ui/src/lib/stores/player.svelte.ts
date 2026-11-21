import { api, isTauriRuntime } from "../api";
import { DualDeckAudioEngine, extractDominantRgb } from "../audioEngine";
import { hashToView, isSameView, type RouteState, viewToHash } from "../router";
import { contextMenu } from "./contextMenu.svelte";
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
  SessionPersister,
  writeChartCountry,
} from "./sessionPersistence";
import type {
  ActiveView,
  AppLanguage,
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
} from "../types";

export interface ToastItem {
  id: number;
  message: string;
}

class PlayerStore {
  // Navigation & layout
  activeTab = $state<MainTab>("home");
  activeView = $state<ActiveView>({ kind: "home" });
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
  themeMode = $state<ThemeMode>("system");
  language = $state<AppLanguage>("Auto");

  // Dynamic ambient color extracted from current cover
  ambientRgb = $state<[number, number, number] | null>(null);

  // Lyrics
  lyrics = $state<LyricsData>({ type: "None" });
  activeLyricIndex = $state<number | null>(null);

  // Related / suggested tracks for Autoplay
  suggestedTracks = $state<Track[]>([]);

  // User data
  favorites = $state<Track[]>([]);
  recentTracks = $state<Track[]>([]);
  userPlaylists = $state<UserPlaylist[]>([]);
  searchHistory = $state<string[]>([]);

  // Catalog data
  homeFeed = $state<HomeFeed | null>(null);
  homeLoading = $state(false);
  genres = $state<Genre[]>([]);
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

  // Toasts
  toasts = $state<ToastItem[]>([]);
  private toastSeq = 0;
  private loadGen = 0;
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

  constructor() {
    this.engine = new DualDeckAudioEngine({
      onTimeUpdate: (pos, dur) => {
        const minDelta = this.lyrics.type === "Synced" ? 0.1 : 0.25;
        if (Math.abs(pos - this.positionSecs) >= minDelta || pos === 0) {
          this.positionSecs = pos;
        }
        if (dur > 0 && Math.abs(dur - this.durationSecs) >= 0.5) {
          this.durationSecs = dur;
        }
        this.updateActiveLyric(pos);
        const now = performance.now();
        if (this.spinStatus.local_active && this.isPlaying && !this.isLoadingAudio && now - this.lastLocalSpinUpdate >= 500) {
          this.lastLocalSpinUpdate = now;
          void this.syncSpinSnapshot();
        }
      },
      onPlayStateChange: (playing) => {
        this.isPlaying = playing;
        if (playing) this.isLoadingAudio = false;
        void this.syncSpinSnapshot();
      },
      onEnded: () => {
        this.isPlaying = false;
        void this.syncSpinSnapshot();
        if (this.spinStatus.remote_role !== "guest") void this.handleTrackEnded();
      },
      onCrossfadeTrigger: () => {
        if (this.spinStatus.remote_role !== "guest") void this.next(true);
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

      const s = initial.session;
      this.sidebarOpen = !s.sidebar_collapsed;
      this.queue = s.queue;
      this.unshuffledQueue = s.unshuffled_queue;
      this.currentIndex = s.current_index;
      this.sourceName = s.source_name || "Not playing";
      this.volume = s.user_volume ?? 1.0;
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
        void api
          .getLyrics(restored.title, restored.artist, restored.duration_secs, restored.id)
          .then((data) => {
            if (this.currentTrack?.id === restored.id) this.lyrics = data;
          });
      }
    } catch (err) {
      this.showToast(String(err));
    }
    void this.loadHome();
    void this.loadGenres();
    void this.refreshSpinStatus();
    void this.bindNativeEvents();
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
    this.volume = Math.max(0, Math.min(1, v));
    this.engine.setVolume(this.volume);
    void this.persistSession();
  }

  public showToast(message: string): void {
    const id = ++this.toastSeq;
    this.toasts = [...this.toasts, { id, message }];
    window.setTimeout(() => {
      this.toasts = this.toasts.filter((t) => t.id !== id);
    }, 3200);
  }

  private initRouter(): void {
    if (typeof window === "undefined" || this.routerBound) return;
    this.routerBound = true;

    const initial = hashToView(window.location.hash);
    this.activeView = initial.view;
    this.activeTab = initial.tab;
    this.routeDepth = 0;
    const initialState: RouteState = {
      view: initial.view,
      tab: initial.tab,
      depth: 0,
    };
    window.history.replaceState(initialState, "", viewToHash(initial.view));

    window.addEventListener("popstate", (event) => {
      // On Android / desktop Back button, dismiss open overlays before navigating away
      if (this.dismissTopOverlay()) {
        const keep: RouteState = {
          view: this.activeView,
          tab: this.activeTab,
          depth: this.routeDepth + 1,
        };
        this.routeDepth = keep.depth;
        window.history.pushState(keep, "", viewToHash(this.activeView));
        return;
      }

      const st = event.state as RouteState | null;
      if (st?.view) {
        this.activeView = st.view;
        this.activeTab = st.tab;
        this.routeDepth = st.depth ?? 0;
      } else {
        const parsed = hashToView(window.location.hash);
        this.activeView = parsed.view;
        this.activeTab = parsed.tab;
        this.routeDepth = 0;
      }
      document.querySelector("main")?.scrollTo({ top: 0 });
    });
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
    if (this.sidebarOpen && typeof window !== "undefined" && window.innerWidth < 1024) {
      this.sidebarOpen = false;
      void this.persistSession();
      return true;
    }
    return false;
  }

  private commitRoute(view: ActiveView, tab: MainTab): void {
    const changed = !isSameView(this.activeView, view);
    this.activeTab = tab;
    this.activeView = view;
    if (typeof window !== "undefined" && changed) {
      this.routeDepth += 1;
      const state: RouteState = { view, tab, depth: this.routeDepth };
      window.history.pushState(state, "", viewToHash(view));
    }
    document.querySelector("main")?.scrollTo({ top: 0 });
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
    if (!isTauriRuntime()) return;
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
    if (!isTauriRuntime()) return;
    try {
      this.genres = await api.getGenres(forceRefresh);
    } catch {
      // Non-fatal
    }
  }

  public async runSearch(raw: string): Promise<void> {
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

  public toggleFavorite(track: Track): void {
    if (this.isFavorite(track.id)) {
      this.favorites = this.favorites.filter((t) => t.id !== track.id);
    } else {
      this.favorites = [track, ...this.favorites];
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
    if (!current || !isTauriRuntime()) return;
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

  public async playIndex(index: number, useCrossfade = false, startPositionSecs = 0, startPaused = false): Promise<void> {
    if (index < 0 || index >= this.queue.length) return;
    const track = this.queue[index];
    const gen = ++this.loadGen;
    this.currentIndex = index;
    this.isLoadingAudio = true;
    this.positionSecs = 0;
    this.durationSecs = track.duration_secs ?? 0;
    this.lyrics = { type: "None" };
    this.activeLyricIndex = null;

    // Record recent
    this.recentTracks = [track, ...this.recentTracks.filter((t) => t.id !== track.id)].slice(0, 30);
    if (isTauriRuntime()) void api.saveRecentTracks(this.recentTracks);

    // Extract ambient cover color
    const cover = track.cover_large_url || track.cover_url;
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
      this.isLoadingAudio = false;
      return;
    }

    try {
      void api.cancelStaleStreams([track.id]);
      const stream = await api.resolveAudioStream(track);
      if (this.loadGen !== gen) return;
      this.qualityLabel = stream.quality_label;
      await this.engine.loadAndPlay(stream.url, startPositionSecs, useCrossfade, startPaused);
      if (this.loadGen !== gen) return;
      if (startPaused) this.isLoadingAudio = false;
      void this.syncSpinSnapshot();

      // Load lyrics & related suggestions asynchronously
      void api.getLyrics(track.title, track.artist, track.duration_secs, track.id).then((data) => {
        if (this.loadGen === gen) this.lyrics = data;
      });
      if (this.sidebarTab === "related") {
        void this.loadRelatedForCurrent(true);
      }
      if (this.spinStatus.remote_role === "guest") return;
      void api.getRadioTracks(track.id).then((tracks) => {
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
        } else {
          this.suggestedTracks = related.slice(0, 12);
        }
      });
    } catch (err) {
      if (this.loadGen === gen) {
        this.isLoadingAudio = false;
        this.showToast(String(err));
      }
    }
  }

  public async togglePlay(): Promise<void> {
    if (!this.currentTrack || this.currentIndex == null) return;
    if (!this.engine.activeDeck.src) {
      await this.playIndex(this.currentIndex);
      return;
    }
    await this.engine.togglePlay();
  }

  public seek(secs: number): void {
    this.positionSecs = secs;
    this.engine.seek(secs);
    this.updateActiveLyric(secs);
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

  private updateActiveLyric(pos: number): void {
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
