<script lang="ts">
  import { onMount } from "svelte";
  import AlbumView from "./components/AlbumView.svelte";
  import ArtistView from "./components/ArtistView.svelte";
  import CollectionView from "./components/CollectionView.svelte";
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

<div class="shell">
  <HeaderBar />

  <div class="workspace">
    {#if player.sidebarOpen}
      <SidebarPlayer />
    {/if}

    <main class="main-pane ambient-surface scroll-y">
      <nav class="main-tabs">
        <button class:active={player.activeTab === "home"} onclick={() => player.navigateTab("home")}>
          ⌂ Home
        </button>
        <button class:active={player.activeTab === "explore"} onclick={() => player.navigateTab("explore")}>
          ⌕ Explore
        </button>
        <button
          class:active={player.activeTab === "collection"}
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
    <div class="floating-mini">
      <button class="mini-info" onclick={() => (player.sidebarOpen = true)}>
        {#if player.currentTrack.cover_url}
          <img class="mini-cover" src={player.currentTrack.cover_url} alt="" />
        {/if}
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
      <button class="play-circle mini-play" title="Play / Pause" onclick={() => void player.togglePlay()}>
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
    <div class="toast-stack" aria-live="polite">
      {#each player.toasts as toast (toast.id)}
        <div class="toast">{toast.message}</div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .shell {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .workspace {
    flex: 1;
    min-height: 0;
    display: flex;
    overflow: hidden;
  }

  .main-pane {
    flex: 1;
    min-width: 0;
    height: 100%;
    position: relative;
  }

  .main-tabs {
    display: flex;
    justify-content: center;
    gap: 6px;
    padding: 12px 16px 4px;
  }

  .main-tabs button {
    padding: 7px 16px;
    border-radius: 999px;
    font-size: 13.5px;
    font-weight: 600;
    color: var(--text-muted);
    transition: background-color 140ms ease, color 140ms ease;
  }

  .main-tabs button:hover {
    background: var(--surface);
    color: var(--text);
  }

  .main-tabs button.active {
    background: var(--surface-hover);
    color: var(--text);
  }

  .floating-mini {
    position: fixed;
    right: 18px;
    bottom: 18px;
    z-index: 25;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px 8px 10px;
    border-radius: 20px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-strong);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.52);
    max-width: 360px;
  }

  .mini-info {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
    max-width: 170px;
    text-align: left;
  }

  .mini-cover {
    width: 40px;
    height: 40px;
    border-radius: 8px;
    object-fit: cover;
    flex-shrink: 0;
  }

  .mini-play {
    width: 36px;
    height: 36px;
    font-size: 13px;
  }

  .toast-stack {
    position: fixed;
    left: 50%;
    bottom: 22px;
    transform: translateX(-50%);
    z-index: 80;
    display: flex;
    flex-direction: column;
    gap: 8px;
    pointer-events: none;
  }

  .toast {
    padding: 10px 18px;
    border-radius: 999px;
    background: var(--bg-elevated);
    border: 1px solid var(--accent-border);
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.45);
    font-size: 13px;
    font-weight: 600;
  }
</style>
