<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration, formatSubscribers, type ArtistDetails } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";

  let { artistId, artistName }: { artistId: string; artistName: string } = $props();

  let details = $state<ArtistDetails | null>(null);
  let loading = $state(false);
  let expanded = $state(false);

  $effect(() => {
    const id = artistId;
    const name = artistName;
    details = null;
    expanded = false;
    if (!isTauriRuntime()) return;
    loading = true;
    void api
      .getArtist(id, name)
      .then((res) => {
        if (artistId === id) details = res;
      })
      .catch((err) => player.showToast(String(err)))
      .finally(() => {
        if (artistId === id) loading = false;
      });
  });

  let visibleTracks = $derived(
    details ? (expanded ? details.tracks : details.tracks.slice(0, 5)) : [],
  );
</script>

<div class="page-clamp">
  <div class="hero">
    {#if details?.header_url}
      <img class="hero-bg" src={details.header_url} alt="" />
    {/if}
    <div class="hero-scrim">
      <button class="icon-btn back-btn" title="Back" onclick={() => player.navigateTab("explore")}>
        ←
      </button>
      <div class="hero-bottom">
        <span class="verified">✓ Artist</span>
        <h1>{details?.name ?? artistName}</h1>
        {#if details?.subscriber_count}
          <p class="muted">{formatSubscribers(details.subscriber_count)}</p>
        {/if}
        <div class="hero-actions">
          <button
            class="pill-btn primary"
            disabled={!details?.tracks.length}
            onclick={() =>
              details && player.replaceQueueAndPlay(details.tracks, 0, details.name)}
          >
            ▶ Play
          </button>
        </div>
      </div>
    </div>
  </div>

  {#if loading}
    <p class="muted">Loading artist…</p>
  {:else if details}
    {#if visibleTracks.length > 0}
      <section class="section">
        <h2 class="section-title">Popular songs</h2>
        <div class="track-list">
          {#each visibleTracks as track, i (track.id)}
            {@const active = player.currentTrack?.id === track.id}
            <div
              class="track-row"
              class:active
              role="button"
              tabindex="0"
              onclick={() => details && player.replaceQueueAndPlay(details.tracks, i, details.name)}
              onkeydown={(e) =>
                e.key === "Enter" &&
                details &&
                player.replaceQueueAndPlay(details.tracks, i, details.name)}
            >
              <CoverImage class="track-cover" src={track.cover_url} alt="" />
              <div class="track-meta">
                <span class="track-title">{track.title}</span>
                <span class="track-sub">{track.artist}{track.album ? ` • ${track.album}` : ""}</span>
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
        {#if details.tracks.length > 5}
          <button class="pill-btn toggle-btn" onclick={() => (expanded = !expanded)}>
            {expanded ? "▴ Show less" : `▾ Show ${details.tracks.length - 5} more`}
          </button>
        {/if}
      </section>
    {/if}

    {#if details.albums.length > 0}
      <section class="section">
        <h2 class="section-title">Albums</h2>
        <div class="rail-scroll">
          {#each details.albums as album (album.id)}
            <button
              class="media-card"
              onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
            >
              <CoverImage class="media-cover" src={album.cover_url} alt={album.title} />
              <strong class="media-title">{album.title}</strong>
              <span class="media-sub muted">{album.year ?? album.artist}</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    {#if details.description}
      <section class="section">
        <div class="bio-card">
          <h3>About the artist</h3>
          <p class="muted">{details.description}</p>
        </div>
      </section>
    {/if}
  {/if}
</div>

<style>
  .hero {
    position: relative;
    height: 300px;
    border-radius: 18px;
    overflow: hidden;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .hero-bg {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: 50% 35%;
  }

  .hero-scrim {
    position: relative;
    z-index: 1;
    height: 100%;
    padding: 16px 24px 22px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    background: linear-gradient(180deg, rgba(0, 0, 0, 0.22), rgba(0, 0, 0, 0.82));
  }

  .back-btn {
    background: rgba(0, 0, 0, 0.45);
    color: #ffffff;
  }

  .hero-bottom h1 {
    margin: 4px 0;
    font-size: 36px;
    font-weight: 850;
    color: #ffffff;
  }

  .verified {
    font-size: 12px;
    font-weight: 700;
    color: var(--accent-bright);
  }

  .hero-actions {
    margin-top: 10px;
    display: flex;
    gap: 10px;
  }

  .section {
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .toggle-btn {
    align-self: flex-start;
  }

  .media-card {
    width: 176px;
    flex-shrink: 0;
    padding: 10px;
    border-radius: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .media-title,
  .media-sub {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bio-card {
    padding: 18px 20px;
    border-radius: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .bio-card h3 {
    margin: 0 0 8px;
    font-size: 15px;
  }

  .bio-card p {
    margin: 0;
    font-size: 13.5px;
    line-height: 1.6;
  }

  .dur {
    font-size: 12px;
  }
</style>
