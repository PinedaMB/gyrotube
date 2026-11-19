<script lang="ts">
  import { Compass, Heart, Home, Library, Pause, Play, SkipBack, SkipForward } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly } from "svelte/transition";
  import AlbumView from "./components/AlbumView.svelte";
  import ArtistView from "./components/ArtistView.svelte";
  import CollectionView from "./components/CollectionView.svelte";
  import CoverImage from "./components/CoverImage.svelte";
  import ExploreView from "./components/ExploreView.svelte";
  import HeaderBar from "./components/HeaderBar.svelte";
  import HomeView from "./components/HomeView.svelte";
  import PartyMode from "./components/PartyMode.svelte";
  import SettingsModal from "./components/SettingsModal.svelte";
  import SidebarPlayer from "./components/SidebarPlayer.svelte";
  import SpinSidebar from "./components/SpinSidebar.svelte";
  import { player } from "./lib/stores/player.svelte";

  onMount(() => {
    void player.bootstrap();
  });

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
    {#if player.sidebarOpen}
      <div class="h-full shrink-0" transition:fly={{ x: -24, duration: 220, easing: cubicOut }}>
        <SidebarPlayer />
      </div>
    {/if}

    <main class="flex-1 min-w-0 h-full overflow-y-auto ambient-surface scroll-y">
      <nav class="flex items-center justify-center gap-2 pt-3.5 pb-1 px-4">
        <button
          class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-all duration-200 active:scale-95 {player.activeTab === 'home'
            ? 'bg-[var(--surface-hover)] text-[var(--text)] shadow-sm'
            : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
          onclick={() => player.navigateTab("home")}
        >
          <Home size={15} />
          Home
        </button>
        <button
          class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-all duration-200 active:scale-95 {player.activeTab === 'explore'
            ? 'bg-[var(--surface-hover)] text-[var(--text)] shadow-sm'
            : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
          onclick={() => player.navigateTab("explore")}
        >
          <Compass size={15} />
          Explore
        </button>
        <button
          class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold transition-all duration-200 active:scale-95 {player.activeTab === 'collection'
            ? 'bg-[var(--surface-hover)] text-[var(--text)] shadow-sm'
            : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
          onclick={() => player.navigateTab("collection")}
        >
          <Library size={15} />
          Collection
        </button>
      </nav>

      {#key viewKey}
        <div in:fly={{ y: 10, duration: 220, easing: cubicOut }}>
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

    {#if player.spinOpen}
      <div class="h-full shrink-0" transition:fly={{ x: 24, duration: 220, easing: cubicOut }}>
        <SpinSidebar />
      </div>
    {/if}
  </div>

  {#if !player.sidebarOpen && player.currentTrack}
    <div
      transition:fly={{ y: 24, duration: 240, easing: cubicOut }}
      class="fixed bottom-5 left-1/2 -translate-x-1/2 z-40 w-[min(560px,calc(100vw-32px))] flex items-center gap-2.5 px-3.5 py-2 rounded-full bg-[var(--bg-elevated)]/95 backdrop-blur-xl border border-[var(--border-strong)] shadow-2xl"
    >
      <button
        class="flex-1 min-w-0 flex items-center gap-2.5 text-left"
        onclick={() => (player.sidebarOpen = true)}
      >
        <CoverImage
          class="w-10 h-10 rounded-lg object-cover shrink-0"
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
      <button class="icon-btn" title="Previous" onclick={() => void player.prev()}>
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

  {#if player.partyMode}
    <div transition:fade={{ duration: 220 }}>
      <PartyMode />
    </div>
  {/if}

  <SettingsModal />

  {#if player.toasts.length > 0}
    <div
      class="fixed bottom-5 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-2 pointer-events-none"
      aria-live="polite"
    >
      {#each player.toasts as toast (toast.id)}
        <div
          transition:fly={{ y: 14, duration: 200, easing: cubicOut }}
          class="px-4 py-2 rounded-full text-[13px] font-medium bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-xl"
        >
          {toast.message}
        </div>
      {/each}
    </div>
  {/if}
</div>
