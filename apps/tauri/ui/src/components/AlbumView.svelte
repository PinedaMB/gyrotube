<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../lib/types";

  let {
    id,
    title,
    coverUrl = null,
    isPlaylist = false,
  }: { id: string; title: string; coverUrl?: string | null; isPlaylist?: boolean } = $props();

  let tracks = $state<Track[]>([]);
  let resolvedTitle = $state("");
  let loading = $state(false);

  $effect(() => {
    const targetId = id;
    const isPl = isPlaylist;
    resolvedTitle = title;
    tracks = [];
    if (!isTauriRuntime()) return;
    loading = true;
    const req = isPl ? api.getPlaylistTracks(targetId) : api.getAlbumTracks(targetId);
    void req
      .then(([name, list]) => {
        if (id === targetId) {
          if (name) resolvedTitle = name;
          tracks = list;
        }
      })
      .catch((err) => player.showToast(String(err)))
      .finally(() => {
        if (id === targetId) loading = false;
      });
  });

  let totalMins = $derived(
    Math.round(tracks.reduce((sum, t) => sum + (t.duration_secs ?? 0), 0) / 60),
  );
</script>

<div class="page-clamp">
  <div class="album-hero">
    <button
      class="icon-btn back-btn"
      title="Back"
      onclick={() => player.navigateTab(player.activeTab)}
    >
      ←
    </button>
    <div class="hero-row">
      {#if coverUrl || tracks[0]?.cover_url}
        <img class="album-cover" src={coverUrl || tracks[0]?.cover_url || ""} alt={resolvedTitle} />
      {/if}
      <div class="album-meta">
        <span class="type-pill">{isPlaylist ? "PLAYLIST" : "ALBUM"}</span>
        <h1>{resolvedTitle}</h1>
        {#if tracks[0]?.artist}
          <p class="artist">{tracks[0].artist}</p>
        {/if}
        <p class="muted small">{tracks.length} songs{totalMins ? ` • ${totalMins} min` : ""}</p>
      </div>
    </div>
  </div>

  <div class="album-actions">
    <button
      class="play-circle"
      disabled={tracks.length === 0}
      title="Play all"
      onclick={() => player.replaceQueueAndPlay(tracks, 0, resolvedTitle)}
    >
      ▶
    </button>
    <button
      class="pill-btn"
      disabled={tracks.length === 0}
      onclick={() => {
        const shuffled = [...tracks].sort(() => Math.random() - 0.5);
        player.replaceQueueAndPlay(shuffled, 0, resolvedTitle);
      }}
    >
      ⇄ Shuffle
    </button>
  </div>

  {#if loading}
    <p class="muted">Loading tracks…</p>
  {:else if tracks.length > 0}
    <div class="track-list">
      {#each tracks as track, i (track.id + ":" + i)}
        {@const active = player.currentTrack?.id === track.id}
        <div
          class="track-row"
          class:active
          role="button"
          tabindex="0"
          onclick={() => player.replaceQueueAndPlay(tracks, i, resolvedTitle)}
          onkeydown={(e) =>
            e.key === "Enter" && player.replaceQueueAndPlay(tracks, i, resolvedTitle)}
        >
          <span class="num numeric" class:active-num={active}>{i + 1}</span>
          <div class="track-meta">
            <span class="track-title">{track.title}</span>
            <span class="track-sub">{track.artist}</span>
          </div>
          <span class="muted numeric dur">{formatDuration(track.duration_secs)}</span>
          <button
            class="icon-btn"
            class:fav-active={player.isFavorite(track.id)}
            onclick={(e) => {
              e.stopPropagation();
              player.toggleFavorite(track);
            }}
          >
            ♥
          </button>
          <button
            class="icon-btn"
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
</div>

<style>
  .album-hero {
    padding: 20px 24px 22px;
    border-radius: 18px;
    border: 1px solid var(--border);
    background: linear-gradient(180deg, rgba(var(--ambient-rgb), 0.28), rgba(var(--ambient-rgb), 0.08));
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.28);
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .back-btn {
    background: rgba(0, 0, 0, 0.35);
    color: #ffffff;
  }

  .hero-row {
    display: flex;
    align-items: flex-end;
    gap: 22px;
  }

  .album-cover {
    width: 184px;
    height: 184px;
    border-radius: 14px;
    object-fit: cover;
    box-shadow: 0 14px 36px rgba(0, 0, 0, 0.44);
    flex-shrink: 0;
  }

  .album-meta {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .type-pill {
    align-self: flex-start;
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.08em;
    padding: 3px 10px;
    border-radius: 999px;
    background: var(--surface-hover);
  }

  .album-meta h1 {
    margin: 0;
    font-size: 32px;
    font-weight: 850;
  }

  .artist {
    margin: 0;
    font-weight: 600;
  }

  .small {
    margin: 0;
    font-size: 12.5px;
  }

  .album-actions {
    margin: 16px 0;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .num {
    width: 24px;
    text-align: right;
    font-size: 12.5px;
    color: var(--text-muted);
  }

  .active-num {
    color: var(--accent-bright);
    font-weight: 800;
  }

  .dur {
    font-size: 12px;
  }

  @media (max-width: 600px) {
    .hero-row {
      flex-direction: column;
      align-items: center;
      text-align: center;
    }
    .type-pill {
      align-self: center;
    }
  }
</style>
