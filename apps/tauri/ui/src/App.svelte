<script lang="ts">
  import { WifiOff } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly } from "svelte/transition";
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
  import SidebarPlayer from "./components/SidebarPlayer.svelte";
  import { t } from "./lib/i18n.svelte";
  import { contextMenu } from "./lib/stores/contextMenu.svelte";
  import { player } from "./lib/stores/player.svelte";

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

    const handleOffline = () => {
      player.isOffline = true;
    };
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
      }
    };
    player.isOffline = !navigator.onLine;
    window.addEventListener("offline", handleOffline);
    window.addEventListener("online", handleOnline);
    return () => {
      window.removeEventListener("offline", handleOffline);
      window.removeEventListener("online", handleOnline);
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
      ? "bottom-[calc(8.25rem+env(safe-area-inset-bottom))] md:bottom-[5.75rem]"
      : player.sidebarOpen
        ? "bottom-[calc(1.5rem+env(safe-area-inset-bottom))] md:bottom-5"
        : "bottom-[calc(4.5rem+env(safe-area-inset-bottom))] md:bottom-5",
  );

  $effect(() => {
    if (player.activeView.kind === "explore") {
      exploreMounted = true;
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
</script>

<svelte:window oncontextmenu={(e) => contextMenu.openAppMenu(e)} />

<div class="h-screen w-screen overflow-hidden flex flex-col bg-[var(--bg)] text-[var(--text)]">
  <HeaderBar />

  {#if player.isOffline}
    <div
      role="alert"
      aria-live="assertive"
      transition:fly={{ y: -12, duration: 180, easing: cubicOut }}
      class="shrink-0 z-30 flex items-center justify-center gap-2.5 px-4 py-2 bg-[var(--danger)]/15 border-b border-[var(--danger)]/35 text-[var(--text)] text-[13px]"
    >
      <WifiOff size={15} class="shrink-0 text-[var(--danger)]" />
      <span class="font-semibold">{t("network.offline_title")}</span>
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

        {#if player.activeView.kind === "home"}
          <div class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y">
            <HomeView />
          </div>
        {:else if player.activeView.kind === "collection"}
          <div class="w-full h-full overflow-y-auto overflow-x-hidden scroll-y">
            <CollectionView />
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

  {#if player.settingsOpen}
    {#await import("./components/SettingsModal.svelte") then { default: SettingsModal }}
      <SettingsModal />
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
      onSave={({ name, description, colorHex, iconName }) => {
        const track = player.playlistDraftTrack;
        if (!track) return;
        player.createUserPlaylist(name, description, colorHex, iconName, [track]);
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
