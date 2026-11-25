<script lang="ts">
  import { LogicalSize } from "@tauri-apps/api/dpi";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { WifiOff } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly } from "svelte/transition";
  import { isTauriRuntime } from "./lib/api";
  import CollectionView from "./components/CollectionView.svelte";
  import CollectionPlaylistModal from "./components/collection/CollectionPlaylistModal.svelte";
  import ContextMenu from "./components/ContextMenu.svelte";
  import ExploreView from "./components/ExploreView.svelte";
  import GlobalTooltip from "./components/GlobalTooltip.svelte";
  import HeaderBar from "./components/HeaderBar.svelte";
  import HomeView from "./components/HomeView.svelte";
  import MiniPlayer from "./components/MiniPlayer.svelte";
  import MobileNav from "./components/MobileNav.svelte";
  import NavSidebar from "./components/NavSidebar.svelte";
  import SearchView from "./components/SearchView.svelte";
  import SidebarPlayer from "./components/SidebarPlayer.svelte";
  import { t } from "./lib/i18n.svelte";
  import { contextMenu } from "./lib/stores/contextMenu.svelte";
  import { device } from "./lib/stores/device.svelte";
  import { player } from "./lib/stores/player.svelte";
  import { updater } from "./lib/stores/updater.svelte";

  let savedMuteVolume = 1;
  const isDesktopTauri = isTauriRuntime() && !device.isAndroid;
  let windowEdgeToEdge = $state(false);

  function handleGlobalKeydown(e: KeyboardEvent) {
    if (isDesktopTauri && e.key === "F11") {
      e.preventDefault();
      const win = getCurrentWindow();
      void win
        .isFullscreen()
        .then((fs) => win.setFullscreen(!fs))
        .catch(() => {});
      return;
    }
    if (e.ctrlKey || e.metaKey || e.altKey) return;
    const target = e.target as HTMLElement | null;
    const tag = target?.tagName;
    if (
      tag === "INPUT" ||
      tag === "TEXTAREA" ||
      tag === "SELECT" ||
      target?.isContentEditable
    ) {
      return;
    }

    if (e.key === " " && tag !== "BUTTON" && tag !== "A" && target?.getAttribute("role") !== "button") {
      if (player.currentTrack) {
        e.preventDefault();
        void player.togglePlay();
      }
      return;
    }

    if (e.key === "ArrowRight" && player.currentTrack) {
      if (e.shiftKey) {
        e.preventDefault();
        void player.next();
      } else {
        e.preventDefault();
        player.seek(Math.min(player.durationSecs || 0, player.getExactPositionSecs() + 5));
      }
      return;
    }

    if (e.key === "ArrowLeft" && player.currentTrack) {
      if (e.shiftKey) {
        e.preventDefault();
        void player.prev();
      } else {
        e.preventDefault();
        player.seek(Math.max(0, player.getExactPositionSecs() - 5));
      }
      return;
    }

    if ((e.key === "m" || e.key === "M") && !device.isAndroid) {
      e.preventDefault();
      if (player.volume > 0.01) {
        savedMuteVolume = player.volume;
        player.setVolume(0);
      } else {
        player.setVolume(savedMuteVolume > 0.05 ? savedMuteVolume : 0.8);
      }
    }
  }

  onMount(() => {
    const startedAt = performance.now();
    const dismissSplash = () => {
      const splash = document.getElementById("app-splash");
      if (!splash || splash.classList.contains("splash-exit")) return;
      const elapsed = performance.now() - startedAt;
      const waitMs = Math.max(0, 520 - elapsed);
      window.setTimeout(() => {
        splash.classList.add("splash-exit");
        window.setTimeout(() => splash.remove(), 300);
      }, waitMs);
    };
    const safetyTimer = window.setTimeout(dismissSplash, 1400);
    void player.bootstrap().finally(() => {
      window.clearTimeout(safetyTimer);
      dismissSplash();
    });
    updater.init();

    const handleOffline = () => {
      player.isOffline = true;
    };

    let unlistenResize: (() => void) | undefined;
    let saveWindowTimer: number | undefined;
    const WINDOW_STATE_KEY = "gyrotube.windowState";
    if (isDesktopTauri) {
      const win = getCurrentWindow();
      let savedSize = { width: 1220, height: 760, maximized: false };
      try {
        const raw = localStorage.getItem(WINDOW_STATE_KEY);
        if (raw) {
          const parsed = JSON.parse(raw) as Partial<typeof savedSize>;
          if (
            typeof parsed.width === "number" &&
            typeof parsed.height === "number" &&
            parsed.width >= 360 &&
            parsed.height >= 480
          ) {
            savedSize = {
              width: Math.min(3840, Math.round(parsed.width)),
              height: Math.min(2160, Math.round(parsed.height)),
              maximized: Boolean(parsed.maximized),
            };
            void win.setSize(new LogicalSize(savedSize.width, savedSize.height)).catch(() => {});
          }
          if (savedSize.maximized) {
            void win.maximize().catch(() => {});
          }
        }
      } catch {
        // ignore invalid stored window state
      }

      const syncWindowFrame = (persist: boolean) => {
        void Promise.all([win.isMaximized(), win.isFullscreen(), win.innerSize(), win.scaleFactor()])
          .then(([max, fs, size, scale]) => {
            windowEdgeToEdge = max || fs;
            if (!persist || fs) return;
            const factor = scale > 0 ? scale : 1;
            const next = {
              width: max ? savedSize.width : Math.max(360, Math.round(size.width / factor)),
              height: max ? savedSize.height : Math.max(480, Math.round(size.height / factor)),
              maximized: max,
            };
            savedSize = next;
            try {
              localStorage.setItem(WINDOW_STATE_KEY, JSON.stringify(next));
            } catch {
              // ignore storage quota errors
            }
          })
          .catch(() => {});
      };
      syncWindowFrame(false);
      void win
        .onResized(() => {
          syncWindowFrame(false);
          window.clearTimeout(saveWindowTimer);
          saveWindowTimer = window.setTimeout(() => syncWindowFrame(true), 250);
        })
        .then((fn) => {
          unlistenResize = fn;
        })
        .catch(() => {});
    }
    const handleOnline = () => {
      const wasOffline = player.isOffline;
      player.isOffline = false;
      if (wasOffline) {
        player.showToast(t("network.back_online"));
        if (!player.homeFeed && !player.homeLoading) {
          void player.loadHome();
        }
        if (player.genres.length === 0 && !player.genresLoading) {
          void player.loadGenres();
        }
        void player.syncDownloadedPlaylists();
      }
    };
    const handleWheel = (e: WheelEvent) => {
      if (e.ctrlKey || e.metaKey) e.preventDefault();
    };
    const androidBridge = (
      window as unknown as {
        __GYROTUBE_ANDROID__?: { getSafeAreaInsets?: () => string };
      }
    ).__GYROTUBE_ANDROID__;
    if (androidBridge?.getSafeAreaInsets) {
      const [top, bottom] = androidBridge.getSafeAreaInsets().split(",");
      if (top) document.documentElement.style.setProperty("--safe-top", `${top}px`);
      if (bottom) document.documentElement.style.setProperty("--safe-bottom", `${bottom}px`);
    }
    player.isOffline = !navigator.onLine;
    window.addEventListener("offline", handleOffline);
    window.addEventListener("online", handleOnline);
    window.addEventListener("wheel", handleWheel, { passive: false });
    return () => {
      window.clearTimeout(saveWindowTimer);
      unlistenResize?.();
      window.removeEventListener("offline", handleOffline);
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("wheel", handleWheel);
      player.dispose();
    };
  });

  function playerFly(node: Element) {
    const phone = window.matchMedia("(width < 48rem)").matches;
    const reducedMotion = phone && window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    return fly(node, {
      x: phone ? 0 : 28,
      y: phone ? "100%" : 0,
      duration: reducedMotion ? 0 : 220,
      easing: cubicOut,
    });
  }

  let exploreMounted = $state(false);
  let collectionMounted = $state(false);
  let searchMounted = $state(false);
  let lastArtist = $state<{ artistId: string; artistName: string } | null>(null);
  let lastAlbum = $state<{
    id: string;
    title: string;
    coverUrl?: string | null;
    isPlaylist: boolean;
  } | null>(null);

  let miniPlayerVisible = $derived(!player.sidebarOpen && Boolean(player.currentTrack));

  let toastBottomClass = $derived(
    miniPlayerVisible
      ? "bottom-[calc(8.25rem+var(--safe-bottom))] md:bottom-[5.75rem]"
      : player.sidebarOpen
        ? "bottom-[calc(1.5rem+var(--safe-bottom))] md:bottom-5"
        : "bottom-[calc(4.5rem+var(--safe-bottom))] md:bottom-5",
  );

  $effect(() => {
    if (!device.isAndroid || typeof window === "undefined") return;
    const bridge = (
      window as unknown as {
        __GYROTUBE_ANDROID__?: { setSystemBarsTheme?: (bgHex: string, isDark: boolean) => void };
      }
    ).__GYROTUBE_ANDROID__;
    if (!bridge?.setSystemBarsTheme) return;
    const forceDarkIcons =
      player.partyMode || (!player.sidebarOpen && player.activeView.kind === "artist");
    if (forceDarkIcons) {
      bridge.setSystemBarsTheme("#131218", true);
    } else {
      const mode = player.themeMode;
      const prefersDark =
        typeof window.matchMedia === "function" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches;
      const isDark = mode === "dark" || (mode === "system" && prefersDark);
      bridge.setSystemBarsTheme(isDark ? "#1e1c26" : "#ffffff", isDark);
    }
  });

  $effect(() => {
    if (player.activeView.kind === "explore") {
      exploreMounted = true;
    } else if (player.activeView.kind === "collection") {
      collectionMounted = true;
    } else if (player.activeView.kind === "search") {
      searchMounted = true;
    } else if (player.activeView.kind === "artist") {
      if (
        !lastArtist ||
        lastArtist.artistId !== player.activeView.artistId ||
        lastArtist.artistName !== player.activeView.artistName
      ) {
        lastArtist = {
          artistId: player.activeView.artistId,
          artistName: player.activeView.artistName,
        };
      }
    } else if (player.activeView.kind === "album") {
      if (
        !lastAlbum ||
        lastAlbum.id !== player.activeView.id ||
        lastAlbum.isPlaylist !== Boolean(player.activeView.isPlaylist)
      ) {
        lastAlbum = {
          id: player.activeView.id,
          title: player.activeView.title,
          coverUrl: player.activeView.coverUrl,
          isPlaylist: Boolean(player.activeView.isPlaylist),
        };
      }
    }
  });

  $effect(() => {
    const rgb = player.ambientRgb;
    document.documentElement.style.setProperty(
      "--ambient-rgb",
      rgb ? `${rgb[0]}, ${rgb[1]}, ${rgb[2]}` : "16, 185, 129",
    );
  });

  $effect(() => {
    if (!isDesktopTauri) return;
    const track = player.currentTrack;
    const title =
      track && player.isPlaying
        ? `${track.title}${track.artist ? ` — ${track.artist}` : ""} · GyroTube`
        : "GyroTube";
    void getCurrentWindow().setTitle(title).catch(() => {});
  });
</script>

<svelte:window oncontextmenu={(e) => contextMenu.openAppMenu(e)} onkeydown={handleGlobalKeydown} />

<div
  class="h-screen w-screen overflow-hidden flex flex-col bg-[var(--bg)] text-[var(--text)] {isDesktopTauri && !windowEdgeToEdge
    ? 'rounded-[14px] border border-[var(--border-strong)] shadow-[inset_0_1px_0_rgba(255,255,255,0.06)]'
    : ''}"
>
  <HeaderBar />

  {#if player.isOffline && player.activeView.kind === "collection"}
    <div
      role="alert"
      aria-live="assertive"
      transition:fly={{ y: -12, duration: 180, easing: cubicOut }}
      class="shrink-0 z-30 flex items-center justify-center gap-2 px-4 py-1.5 bg-[var(--danger)]/12 border-b border-[var(--danger)]/28 text-[var(--text)] text-[12.5px]"
    >
      <WifiOff size={14} class="shrink-0 text-[var(--danger)]" />
      <span class="font-semibold">{t("network.offline_mode")}</span>
      <span class="hidden sm:inline text-[var(--text-muted)]">· {t("network.offline_sub")}</span>
    </div>
  {/if}

  <div class="flex-1 min-h-0 flex overflow-hidden relative">
    <!-- Left navigation sidebar -->
    <NavSidebar />

    <!-- Center scrollable main view (static ambient layer behind scrollport for GPU-composited scrolling) -->
    <div class="flex-1 min-w-0 h-full relative overflow-hidden">
      <div class="pointer-events-none absolute inset-0 ambient-surface" aria-hidden="true"></div>
      <main class="relative z-10 w-full h-full overflow-hidden">
        {#if exploreMounted}
          <div
            class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y"
            class:hidden={player.activeView.kind !== "explore"}
          >
            <ExploreView />
          </div>
        {/if}

        {#if lastArtist}
          <div
            class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y"
            class:hidden={player.activeView.kind !== "artist"}
          >
            {#await import("./components/ArtistView.svelte") then { default: ArtistView }}
              <ArtistView
                artistId={lastArtist.artistId}
                artistName={lastArtist.artistName}
              />
            {/await}
          </div>
        {/if}

        {#if lastAlbum}
          <div
            class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y"
            class:hidden={player.activeView.kind !== "album"}
          >
            {#await import("./components/AlbumView.svelte") then { default: AlbumView }}
              <AlbumView
                id={lastAlbum.id}
                title={lastAlbum.title}
                coverUrl={lastAlbum.coverUrl}
                isPlaylist={lastAlbum.isPlaylist}
              />
            {/await}
          </div>
        {/if}

        <div
          class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y"
          class:hidden={player.activeView.kind !== "home"}
        >
          <HomeView />
        </div>

        {#if collectionMounted}
          <div
            class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y"
            class:hidden={player.activeView.kind !== "collection"}
          >
            <CollectionView />
          </div>
        {/if}

        {#if searchMounted}
          <div
            class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y"
            class:hidden={player.activeView.kind !== "search"}
          >
            <SearchView />
          </div>
        {/if}

        {#if player.activeView.kind === "settings"}
          <div class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y">
            {#await import("./components/SettingsView.svelte") then { default: SettingsView }}
              <SettingsView />
            {/await}
          </div>
        {/if}
      </main>
    </div>

    <!-- Right player sidebar (docked on lg+, full-height slide-over sheet on < lg) -->
    {#if player.sidebarOpen}
      <div
        class="lg:hidden fixed inset-0 z-40 bg-black/65"
        role="presentation"
        transition:fade={{ duration: 160 }}
        onclick={() => {
          player.closePlayerSheet();
        }}
      ></div>
      <div
        class="h-full shrink-0 max-lg:fixed max-lg:inset-y-0 max-lg:right-0 max-lg:z-50 max-md:inset-0"
        transition:playerFly
      >
        <SidebarPlayer />
      </div>
    {/if}
  </div>

  <MiniPlayer />

  <MobileNav />

  {#if player.partyMode}
    {#await import("./components/PartyMode.svelte") then { default: PartyMode }}
      <div transition:fade={{ duration: 220 }}>
        <PartyMode />
      </div>
    {/await}
  {/if}

  {#if player.spinOpen}
    {#await import("./components/spin/SpinModal.svelte") then { default: SpinModal }}
      <SpinModal />
    {/await}
  {/if}

  <ContextMenu />
  <GlobalTooltip />

  {#if player.playlistDraftTrack}
    <CollectionPlaylistModal
      open={true}
      editing={null}
      onClose={() => (player.playlistDraftTrack = null)}
      onSave={({ name, description, colorHex, iconName, coverPath }) => {
        const track = player.playlistDraftTrack;
        if (!track) return;
        player.createUserPlaylist(name, description, colorHex, iconName, [track], coverPath ?? null);
      }}
    />
  {/if}

  {#if player.toasts.length > 0}
    <div
      class="fixed {toastBottomClass} left-1/2 -translate-x-1/2 z-[60] max-w-[calc(100vw-2rem)] flex flex-col items-center gap-2 pointer-events-none transition-[bottom] duration-200"
      aria-live="polite"
    >
      {#each player.toasts as toast (toast.id)}
        <div
          transition:fly={{ y: 14, duration: 200, easing: cubicOut }}
          class="px-4 py-2 rounded-full text-[13px] font-medium text-center bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-xl"
        >
          {toast.message}
        </div>
      {/each}
    </div>
  {/if}
</div>
