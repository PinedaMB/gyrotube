import { api, isTauriRuntime } from "../api";
import { DualDeckAudioEngine, extractDominantRgb } from "../audioEngine";
import type {
  ActiveView,
  AppLanguage,
  Genre,
  HomeFeed,
  LyricsData,
  MainTab,
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
  sidebarTab = $state<"player" | "lyrics" | "queue">("player");
  spinOpen = $state(false);
  partyMode = $state(false);
  settingsOpen = $state(false);

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

  constructor() {
    this.engine = new DualDeckAudioEngine({
      onTimeUpdate: (pos, dur) => {
        this.positionSecs = pos;
        if (dur > 0) this.durationSecs = dur;
        this.updateActiveLyric(pos);
      },
      onPlayStateChange: (playing) => {
        this.isPlaying = playing;
        if (playing) this.isLoadingAudio = false;
      },
      onEnded: () => {
        void this.handleTrackEnded();
      },
      onCrossfadeTrigger: () => {
        void this.next(true);
      },
      onError: (msg) => {
        this.isLoadingAudio = false;
        this.showToast(msg);
      },
    });
  }

  public async bootstrap(): Promise<void> {
    this.applyTheme(this.themeMode);
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

      this.engine.setVolume(this.volume);
      this.engine.setCrossfadeSecs(this.crossfadeSecs);
      this.applyTheme(this.themeMode);
    } catch (err) {
      this.showToast(String(err));
    }
    void this.loadHome();
    void this.loadGenres();
    void this.refreshSpinStatus();
  }

  public applyTheme(mode: ThemeMode): void {
    this.themeMode = mode;
    if (typeof document === "undefined") return;
    const root = document.documentElement;
    if (mode === "system") {
      root.removeAttribute("data-theme");
    } else {
      root.setAttribute("data-theme", mode);
    }
  }

  public setThemeMode(mode: ThemeMode): void {
    this.applyTheme(mode);
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

  public navigateTab(tab: MainTab): void {
    this.activeTab = tab;
    this.activeView = { kind: tab };
  }

  public openArtist(artistId: string, artistName: string): void {
    this.activeTab = "explore";
    this.activeView = { kind: "artist", artistId, artistName };
  }

  public openAlbum(id: string, title: string, coverUrl?: string | null, isPlaylist = false): void {
    this.activeView = { kind: "album", id, title, coverUrl, isPlaylist };
  }

  public async loadHome(forceRefresh = false): Promise<void> {
    if (!isTauriRuntime()) return;
    this.homeLoading = true;
    try {
      this.homeFeed = await api.getHomeFeed(forceRefresh);
    } catch (err) {
      this.showToast(String(err));
    } finally {
      this.homeLoading = false;
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
    this.activeTab = "explore";
    this.activeView = { kind: "explore" };
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

  public isFavorite(trackId: string): boolean {
    return this.favorites.some((t) => t.id === trackId);
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
    }
  }

  public removeQueueIndex(idx: number): void {
    if (idx < 0 || idx >= this.queue.length) return;
    const nextQueue = this.queue.filter((_, i) => i !== idx);
    this.queue = nextQueue;
    if (this.currentIndex == null) return;
    if (nextQueue.length === 0) {
      this.currentIndex = null;
      this.engine.stop();
    } else if (idx === this.currentIndex) {
      const nextIdx = Math.min(idx, nextQueue.length - 1);
      void this.playIndex(nextIdx);
    } else if (idx < this.currentIndex) {
      this.currentIndex -= 1;
    }
    void this.persistSession();
  }

  public moveQueueItem(from: number, to: number): void {
    if (from === to || from < 0 || to < 0 || from >= this.queue.length || to >= this.queue.length) {
      return;
    }
    const copy = [...this.queue];
    const [moved] = copy.splice(from, 1);
    copy.splice(to, 0, moved);
    this.queue = copy;
    if (this.currentIndex === from) {
      this.currentIndex = to;
    } else if (this.currentIndex != null) {
      if (from < this.currentIndex && to >= this.currentIndex) this.currentIndex -= 1;
      else if (from > this.currentIndex && to <= this.currentIndex) this.currentIndex += 1;
    }
    void this.persistSession();
  }

  public clearQueue(): void {
    this.queue = [];
    this.unshuffledQueue = [];
    this.currentIndex = null;
    this.engine.stop();
    this.lyrics = { type: "None" };
    this.ambientRgb = null;
    void this.persistSession();
  }

  public async playIndex(index: number, useCrossfade = false): Promise<void> {
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
      await this.engine.loadAndPlay(stream.url, 0, useCrossfade);

      // Load lyrics & related suggestions asynchronously
      void api.getLyrics(track.title, track.artist, track.duration_secs).then((data) => {
        if (this.loadGen === gen) this.lyrics = data;
      });
      void api.getRadioTracks(track.id).then((tracks) => {
        if (this.loadGen === gen) {
          const existing = new Set(this.queue.map((t) => t.id));
          this.suggestedTracks = tracks.filter((t) => !existing.has(t.id)).slice(0, 12);
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
    if (!this.currentTrack) return;
    await this.engine.togglePlay();
  }

  public seek(secs: number): void {
    this.positionSecs = secs;
    this.engine.seek(secs);
    this.updateActiveLyric(secs);
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
    const current = this.queue[this.currentIndex];
    if (this.shuffle) {
      this.unshuffledQueue = [...this.queue];
      const rest = this.queue.filter((_, i) => i !== this.currentIndex);
      for (let i = rest.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [rest[i], rest[j]] = [rest[j], rest[i]];
      }
      this.queue = [current, ...rest];
      this.currentIndex = 0;
    } else if (this.unshuffledQueue.length > 0) {
      const idx = this.unshuffledQueue.findIndex((t) => t.id === current.id);
      this.queue = [...this.unshuffledQueue];
      this.currentIndex = idx >= 0 ? idx : 0;
    }
    void this.persistSession();
  }

  public toggleRepeat(): void {
    this.repeat = !this.repeat;
    void this.persistSession();
  }

  private updateActiveLyric(pos: number): void {
    if (this.lyrics.type !== "Synced") return;
    const lines = this.lyrics.lines;
    let found: number | null = null;
    for (let i = lines.length - 1; i >= 0; i--) {
      if (lines[i].time_secs <= pos + 0.15) {
        found = i;
        break;
      }
    }
    if (found !== this.activeLyricIndex) {
      this.activeLyricIndex = found;
    }
  }

  public async refreshSpinStatus(): Promise<void> {
    if (!isTauriRuntime()) return;
    try {
      this.spinStatus = await api.spinStatus();
    } catch {
      // ignore
    }
  }

  public async persistSession(): Promise<void> {
    if (!isTauriRuntime()) return;
    const session: SessionState = {
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
    };
    try {
      await api.saveSession(session);
    } catch {
      // ignore
    }
  }
}

export const player = new PlayerStore();
