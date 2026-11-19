<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";
</script>

<div class="page-clamp">
  <h1 class="page-title">Collection</h1>
  <p class="muted caption">Your local playlists, recent history, and favorite songs</p>

  <section class="section">
    <div class="section-head">
      <h2 class="section-title">Local playlists</h2>
    </div>

    <div class="playlists-grid">
      <div
        class="playlist-card fav-card"
        role="button"
        tabindex="0"
        onclick={() =>
          player.favorites.length &&
          player.replaceQueueAndPlay(player.favorites, 0, "Favorites")}
        onkeydown={(e) =>
          e.key === "Enter" &&
          player.favorites.length &&
          player.replaceQueueAndPlay(player.favorites, 0, "Favorites")}
      >
        <span class="pl-badge fav-badge">♥</span>
        <div class="track-meta">
          <strong>Favorites</strong>
          <span class="muted small">{player.favorites.length} songs</span>
        </div>
        <button
          class="icon-btn pl-play"
          disabled={player.favorites.length === 0}
          title="Play favorites"
        >
          ▶
        </button>
      </div>

      {#each player.userPlaylists as pl (pl.id)}
        <div
          class="playlist-card"
          role="button"
          tabindex="0"
          onclick={() =>
            pl.tracks.length && player.replaceQueueAndPlay(pl.tracks, 0, pl.name)}
          onkeydown={(e) =>
            e.key === "Enter" &&
            pl.tracks.length &&
            player.replaceQueueAndPlay(pl.tracks, 0, pl.name)}
        >
          <span class="pl-badge" style="background: {pl.color_hex || 'var(--accent)'};">♪</span>
          <div class="track-meta">
            <strong>{pl.name}</strong>
            <span class="muted small">{pl.tracks.length} songs</span>
          </div>
          <button class="icon-btn pl-play" disabled={pl.tracks.length === 0} title="Play playlist">
            ▶
          </button>
        </div>
      {/each}
    </div>
  </section>

  <section class="section">
    <div class="section-head">
      <h2 class="section-title">Recently played</h2>
      {#if player.recentTracks.length > 0}
        <button
          class="pill-btn"
          onclick={() => player.replaceQueueAndPlay(player.recentTracks, 0, "Recent")}
        >
          ▶ Play all
        </button>
      {/if}
    </div>

    {#if player.recentTracks.length === 0}
      <p class="muted">Songs you play will appear here.</p>
    {:else}
      <div class="track-list">
        {#each player.recentTracks as track, i (track.id)}
          {@const active = player.currentTrack?.id === track.id}
          <div
            class="track-row"
            class:active
            role="button"
            tabindex="0"
            onclick={() => player.replaceQueueAndPlay(player.recentTracks, i, "Recent")}
            onkeydown={(e) =>
              e.key === "Enter" && player.replaceQueueAndPlay(player.recentTracks, i, "Recent")}
          >
            {#if track.cover_url}
              <img class="track-cover" src={track.cover_url} alt="" />
            {/if}
            <div class="track-meta">
              <span class="track-title">{track.title}</span>
              <span class="track-sub">{track.artist}{track.album ? ` • ${track.album}` : ""}</span>
            </div>
            <span class="muted numeric dur">{formatDuration(track.duration_secs)}</span>
            <button
              class="icon-btn"
              class:fav-active={player.isFavorite(track.id)}
              title="Favorite"
              onclick={(e) => {
                e.stopPropagation();
                player.toggleFavorite(track);
              }}
            >
              ♥
            </button>
            <button
              class="icon-btn"
              title="Add to queue"
              onclick={(e) => {
                e.stopPropagation();
                player.enqueueTrack(track);
              }}
            >
              +
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </section>
</div>

<style>
  .page-title {
    margin: 0;
    font-size: 28px;
    font-weight: 850;
  }

  .caption {
    margin: 3px 0 0;
    font-size: 13px;
  }

  .section {
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .playlists-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 12px;
  }

  .playlist-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    cursor: pointer;
    transition: border-color 150ms ease, background-color 150ms ease;
  }

  .playlist-card:hover {
    background: var(--surface-hover);
    border-color: var(--accent-border);
  }

  .playlist-card:hover .pl-play:not(:disabled) {
    background: var(--accent);
    color: #ffffff;
  }

  .fav-card {
    background: linear-gradient(135deg, rgba(224, 27, 36, 0.16), var(--surface) 68%);
    border-color: rgba(224, 27, 36, 0.24);
  }

  .pl-badge {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    color: #ffffff;
    flex-shrink: 0;
  }

  .fav-badge {
    background: linear-gradient(135deg, #e01b24, #ff5b7f);
  }

  .small {
    font-size: 12px;
  }

  .dur {
    font-size: 12px;
  }

  @media (max-width: 780px) {
    .playlists-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
