<script lang="ts">
  import { Heart, Pause, Play, SkipBack, SkipForward } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly } from "svelte/transition";
  import AlbumView from "./components/AlbumView.svelte";
  import ArtistView from "./components/ArtistView.svelte";
  import CollectionView from "./components/CollectionView.svelte";
  import CollectionPlaylistModal from "./components/collection/CollectionPlaylistModal.svelte";
  import CoverImage from "./components/CoverImage.svelte";
  import ExploreView from "./components/ExploreView.svelte";
  import HeaderBar from "./components/HeaderBar.svelte";
  import HomeView from "./components/HomeView.svelte";
  import MobileNav from "./components/MobileNav.svelte";
  import NavSidebar from "./components/NavSidebar.svelte";
  import PartyMode from "./components/PartyMode.svelte";
  import SettingsModal from "./components/SettingsModal.svelte";
  import SidebarPlayer from "./components/SidebarPlayer.svelte";
  import SpinModal from "./components/spin/SpinModal.svelte";
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
      rgb ? `${rgb[0]}, ${rgb[1]}, ${rgb[2]}` : "33, 144, 164",
    );
  });
</script>

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

  {#if !player.sidebarOpen && player.currentTrack}
    <div
      transition:fly={{ y: 24, duration: 240, easing: cubicOut }}
      class="relative z-30 shrink-0 w-full flex items-center gap-2 sm:gap-2.5 px-3 sm:px-3.5 py-2 bg-[var(--bg-elevated)] border-t border-[var(--border)] md:fixed md:bottom-5 md:left-1/2 md:-translate-x-1/2 md:w-[min(560px,calc(100vw-24px))] md:rounded-full md:bg-[var(--bg-elevated)]/[0.97] md:border md:border-[var(--border-strong)] md:shadow-2xl"
    >
      <button
        class="flex-1 min-w-0 flex items-center gap-2.5 text-left"
        onclick={() => (player.sidebarOpen = true)}
      >
        <CoverImage
          class="w-10 h-10 !rounded-[8px] object-cover shrink-0"
          src={player.currentTrack.cover_url}
          alt=""
        />
        <div class="track-meta">
          <span class="track-title">{player.currentTrack.title}</span>
          <span class="track-sub">{player.currentTrack.artist}</span>
        </div>
      </button>
      <button
        class="icon-btn"
        class:fav-active={player.isFavorite(player.currentTrack.id)}
        title="Favorite"
        onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
      >
        <Heart
          size={17}
          fill={player.isFavorite(player.currentTrack.id) ? "currentColor" : "none"}
        />
      </button>
      <button class="icon-btn hidden sm:inline-flex" title="Previous" onclick={() => void player.prev()}>
        <SkipBack size={17} fill="currentColor" />
      </button>
      <button
        class="play-circle !w-10 !h-10"
        title="Play / Pause"
        onclick={() => void player.togglePlay()}
      >
        {#if player.isPlaying}
          <Pause size={17} fill="currentColor" />
        {:else}
          <Play size={17} fill="currentColor" class="translate-x-[1px]" />
        {/if}
      </button>
      <button class="icon-btn" title="Next" onclick={() => void player.next()}>
        <SkipForward size={17} fill="currentColor" />
      </button>
    </div>
  {/if}

  <MobileNav />

  {#if player.partyMode}
    <div transition:fade={{ duration: 220 }}>
      <PartyMode />
    </div>
  {/if}

  <SettingsModal />
  <SpinModal />
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
