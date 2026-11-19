<script lang="ts">
  import { onMount } from "svelte";
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
      <SidebarPlayer />
    {/if}

    <main class="flex-1 min-w-0 h-full overflow-y-auto ambient-surface scroll-y">
      <nav class="flex items-center justify-center gap-2 pt-3.5 pb-1 px-4">
        <button
          class="px-4 py-1.5 rounded-full text-[13px] font-semibold transition-colors {player.activeTab === 'home'
            ? 'bg-[var(--surface-hover)] text-[var(--text)] shadow-sm'
            : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
          onclick={() => player.navigateTab("home")}
        >
          ⌂ Home
        </button>
        <button
          class="px-4 py-1.5 rounded-full text-[13px] font-semibold transition-colors {player.activeTab === 'explore'
            ? 'bg-[var(--surface-hover)] text-[var(--text)] shadow-sm'
            : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
          onclick={() => player.navigateTab("explore")}
        >
          ⌕ Explore
        </button>
        <button
          class="px-4 py-1.5 rounded-full text-[13px] font-semibold transition-colors {player.activeTab === 'collection'
            ? 'bg-[var(--surface-hover)] text-[var(--text)] shadow-sm'
            : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
          onclick={() => player.navigateTab("collection")}
        >
          ▤ Collection
        </button>
      </nav>

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
    </main>

    {#if player.spinOpen}
      <SpinSidebar />
    {/if}
  </div>

  {#if !player.sidebarOpen && player.currentTrack}
    <div
      class="fixed bottom-5 left-1/2 -translate-x-1/2 z-40 w-[min(560px,calc(100vw-32px))] flex items-center gap-2.5 px-3.5 py-2 rounded-full bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-2xl"
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
        ♥
      </button>
      <button class="icon-btn" title="Previous" onclick={() => void player.prev()}>⏮</button>
      <button
        class="play-circle !w-10 !h-10"
        title="Play / Pause"
        onclick={() => void player.togglePlay()}
      >
        {#if player.isPlaying}❚❚{:else}▶{/if}
      </button>
      <button class="icon-btn" title="Next" onclick={() => void player.next()}>⏭</button>
    </div>
  {/if}

  {#if player.partyMode}
    <PartyMode />
  {/if}

  <SettingsModal />

  {#if player.toasts.length > 0}
    <div
      class="fixed bottom-5 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-2 pointer-events-none"
      aria-live="polite"
    >
      {#each player.toasts as toast (toast.id)}
        <div
          class="px-4 py-2 rounded-full text-[13px] font-medium bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-xl"
        >
          {toast.message}
        </div>
      {/each}
    </div>
  {/if}
</div>
