<script lang="ts">
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly } from "svelte/transition";
  import AlbumView from "./components/AlbumView.svelte";
  import ArtistView from "./components/ArtistView.svelte";
  import CollectionView from "./components/CollectionView.svelte";
  import CollectionPlaylistModal from "./components/collection/CollectionPlaylistModal.svelte";
  import ContextMenu from "./components/ContextMenu.svelte";
  import ExploreView from "./components/ExploreView.svelte";
  import HeaderBar from "./components/HeaderBar.svelte";
  import HomeView from "./components/HomeView.svelte";
  import MiniPlayer from "./components/MiniPlayer.svelte";
  import MobileNav from "./components/MobileNav.svelte";
  import NavSidebar from "./components/NavSidebar.svelte";
  import PartyMode from "./components/PartyMode.svelte";
  import SettingsModal from "./components/SettingsModal.svelte";
  import SidebarPlayer from "./components/SidebarPlayer.svelte";
  import SpinModal from "./components/spin/SpinModal.svelte";
  import { contextMenu } from "./lib/stores/contextMenu.svelte";
  import { player } from "./lib/stores/player.svelte";

  onMount(() => {
    void player.bootstrap();
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

  let viewKey = $derived(
    player.activeView.kind === "artist"
      ? `artist:${player.activeView.artistId}`
      : player.activeView.kind === "album"
        ? `album:${player.activeView.id}`
        : player.activeView.kind,
  );

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

  <div class="flex-1 min-h-0 flex overflow-hidden relative">
    <!-- Left navigation sidebar -->
    <NavSidebar />

    <!-- Center scrollable main view (static ambient layer behind scrollport for GPU-composited scrolling) -->
    <div class="flex-1 min-w-0 h-full relative overflow-hidden">
      <div class="pointer-events-none absolute inset-0 ambient-surface" aria-hidden="true"></div>
      <main class="relative z-10 w-full h-full overflow-y-auto overflow-x-hidden scroll-y">
        {#key viewKey}
          <div>
            {#if player.activeView.kind === "home"}
              <HomeView />
            {:else if player.activeView.kind === "explore"}
              <ExploreView />
            {:else if player.activeView.kind === "collection"}
              <CollectionView />
            {:else if player.activeView.kind === "artist"}
              <ArtistView
                artistId={player.activeView.artistId}
                artistName={player.activeView.artistName}
              />
            {:else if player.activeView.kind === "album"}
              <AlbumView
                id={player.activeView.id}
                title={player.activeView.title}
                coverUrl={player.activeView.coverUrl}
                isPlaylist={player.activeView.isPlaylist}
              />
            {/if}
          </div>
        {/key}
      </main>
    </div>

    <!-- Right player sidebar (docked on lg+, full-height slide-over sheet on < lg) -->
    {#if player.sidebarOpen}
      <div
        class="lg:hidden fixed inset-0 z-40 bg-black/65"
        role="presentation"
        transition:fade={{ duration: 160 }}
        onclick={() => {
          player.sidebarOpen = false;
          void player.persistSession();
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
    <div transition:fade={{ duration: 220 }}>
      <PartyMode />
    </div>
  {/if}

  <SettingsModal />
  <SpinModal />
  <ContextMenu />
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
      class="fixed bottom-5 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-2 pointer-events-none"
      aria-live="polite"
    >
      {#each player.toasts as toast (toast.id)}
        <div
          transition:fly={{ y: 14, duration: 200, easing: cubicOut }}
          class="px-4 py-2 rounded-full text-[13px] font-medium bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-xl"
        >
          {toast.message}
        </div>
      {/each}
    </div>
  {/if}
</div>
