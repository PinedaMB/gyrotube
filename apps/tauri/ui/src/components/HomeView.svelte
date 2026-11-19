<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";

  let chartTab = $state<"trending" | "top">("trending");

  let greeting = $derived.by(() => {
    const h = new Date().getHours();
    if (h >= 5 && h < 12) return "Good morning";
    if (h >= 12 && h < 19) return "Good afternoon";
    return "Good evening";
  });

  let listenAgain = $derived(player.recentTracks.slice(0, 6));
  let featuredAlbum = $derived(player.homeFeed?.new_albums[0] ?? null);
  let releaseAlbums = $derived(
    player.homeFeed
      ? player.homeFeed.new_albums.slice(player.homeFeed.new_albums.length > 1 ? 1 : 0, 16)
      : [],
  );
  let chartTracks = $derived(
    chartTab === "top"
      ? (player.homeFeed?.top ?? [])
      : (player.homeFeed?.trending ?? []),
  );
</script>

<div class="page-clamp">
  <div class="home-head">
    <div>
      <h1 class="greeting">{greeting}</h1>
      <p class="muted caption">Your music ready to play</p>
    </div>
    <button class="icon-btn" title="Reload home" onclick={() => void player.loadHome(true)}>↻</button>
  </div>

  <section class="quick-grid">
    <button class="quick-card q1" onclick={() => document.getElementById("new-releases")?.scrollIntoView({ behavior: "smooth" })}>
      <span class="q-badge">◎</span>
      <span class="q-text"><strong>New releases</strong><small class="muted">Recent albums &amp; singles</small></span>
    </button>
    <button class="quick-card q2" onclick={() => document.getElementById("trending-hits")?.scrollIntoView({ behavior: "smooth" })}>
      <span class="q-badge">☰</span>
      <span class="q-text"><strong>Top charts</strong><small class="muted">Trending &amp; Top Music</small></span>
    </button>
    <button class="quick-card q3" onclick={() => player.navigateTab("explore")}>
      <span class="q-badge">⌕</span>
      <span class="q-text"><strong>Moods</strong><small class="muted">Explore genres &amp; vibes</small></span>
    </button>
    <button class="quick-card q4" onclick={() => player.navigateTab("collection")}>
      <span class="q-badge">♥</span>
      <span class="q-text"><strong>Collection</strong><small class="muted">Your favorite songs</small></span>
    </button>
  </section>

  {#if listenAgain.length > 0}
    <section class="section">
      <h2 class="section-title">Listen again</h2>
      <div class="listen-grid">
        {#each listenAgain as track, i (track.id)}
          {@const active = player.currentTrack?.id === track.id}
          <button
            class="listen-tile"
            class:playing={active}
            onclick={() => player.replaceQueueAndPlay(listenAgain, i, "Listen again")}
          >
            {#if track.cover_url}
              <img class="tile-cover" src={track.cover_url} alt="" />
            {/if}
            <div class="track-meta">
              <span class="track-title">{track.title}</span>
              <span class="track-sub">{track.artist}</span>
            </div>
            <span class="play-badge" aria-hidden="true">{active ? "🔊" : "▶"}</span>
          </button>
        {/each}
      </div>
    </section>
  {/if}

  {#if featuredAlbum}
    <section class="section">
      <div class="hero-card">
        {#if featuredAlbum.cover_url}
          <img class="hero-cover" src={featuredAlbum.cover_url} alt={featuredAlbum.title} />
        {/if}
        <div class="hero-info">
          <span class="hero-badge">FEATURED</span>
          <h2 class="hero-title">{featuredAlbum.title}</h2>
          <p class="muted">{featuredAlbum.artist}</p>
          <div class="hero-actions">
            <button
              class="pill-btn primary"
              onclick={() => player.openAlbum(featuredAlbum.id, featuredAlbum.title, featuredAlbum.cover_url)}
            >
              ▶ Play
            </button>
            <button
              class="pill-btn"
              onclick={() => player.openAlbum(featuredAlbum.id, featuredAlbum.title, featuredAlbum.cover_url)}
            >
              Open album
            </button>
          </div>
        </div>
      </div>
    </section>
  {/if}

  {#if releaseAlbums.length > 0}
    <section class="section" id="new-releases">
      <h2 class="section-title">New releases</h2>
      <div class="rail-scroll">
        {#each releaseAlbums as album (album.id)}
          <button
            class="media-card"
            onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
          >
            <div class="media-cover-wrap">
              {#if album.cover_url}
                <img class="media-cover" src={album.cover_url} alt={album.title} />
              {/if}
              {#if album.year}
                <span class="year-badge numeric">{album.year}</span>
              {/if}
            </div>
            <strong class="media-title">{album.title}</strong>
            <span class="media-sub muted">{album.artist}</span>
          </button>
        {/each}
      </div>
    </section>
  {/if}

  {#if chartTracks.length > 0}
    <section class="section" id="trending-hits">
      <div class="section-head">
        <h2 class="section-title">Trending hits</h2>
        <div class="pill-group">
          <button class:active={chartTab === "trending"} onclick={() => (chartTab = "trending")}>Trending</button>
          <button class:active={chartTab === "top"} onclick={() => (chartTab = "top")}>Top Music</button>
        </div>
      </div>

      <div class="track-list">
        {#each chartTracks as track, i (track.id)}
          {@const active = player.currentTrack?.id === track.id}
          <div
            class="track-row"
            class:active
            role="button"
            tabindex="0"
            onclick={() => player.replaceQueueAndPlay(chartTracks, i, "Trending hits")}
            onkeydown={(e) => e.key === "Enter" && player.replaceQueueAndPlay(chartTracks, i, "Trending hits")}
          >
            <span class="rank numeric">{active ? "🔊" : String(i + 1).padStart(2, "0")}</span>
            {#if track.cover_url}
              <img class="track-cover" src={track.cover_url} alt="" />
            {/if}
            <div class="track-meta">
              <span class="track-title">{track.title}</span>
              <span class="track-sub">{track.artist}</span>
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
    </section>
  {:else if !player.homeLoading}
    <div class="empty-home">
      <p class="muted">Connect Tauri runtime (`cargo tauri dev`) to load live YouTube Music feeds.</p>
    </div>
  {/if}
</div>

<style>
  .home-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 18px;
  }

  .greeting {
    margin: 0;
    font-size: 28px;
    font-weight: 850;
    letter-spacing: -0.025em;
  }

  .caption {
    margin: 3px 0 0;
    font-size: 13px;
  }

  .quick-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 10px;
    margin-bottom: 24px;
  }

  .quick-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 14px;
    border: 1px solid var(--border);
    text-align: left;
    transition: transform 140ms ease, border-color 140ms ease;
  }

  .quick-card:hover {
    transform: translateY(-1px);
  }

  .q1 { background: linear-gradient(135deg, rgba(224, 27, 36, 0.16), var(--surface) 70%); }
  .q2 { background: linear-gradient(135deg, rgba(53, 132, 228, 0.16), var(--surface) 70%); }
  .q3 { background: linear-gradient(135deg, rgba(192, 97, 203, 0.16), var(--surface) 70%); }
  .q4 { background: linear-gradient(135deg, rgba(46, 194, 126, 0.16), var(--surface) 70%); }

  .q-badge {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    flex-shrink: 0;
    background: var(--surface-hover);
  }

  .q-text {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .q-text strong,
  .q-text small {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .q-text strong {
    font-size: 14px;
  }

  .q-text small {
    font-size: 11.5px;
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
    gap: 12px;
  }

  .listen-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 10px;
  }

  .listen-tile {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 12px;
    background: var(--surface);
    border: 1px solid var(--border);
    text-align: left;
    transition: background-color 140ms ease, border-color 140ms ease;
  }

  .listen-tile:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }

  .listen-tile.playing {
    background: var(--accent-soft);
    border-color: var(--accent-border);
  }

  .listen-tile.playing .track-title {
    color: var(--accent-bright);
  }

  .tile-cover {
    width: 46px;
    height: 46px;
    border-radius: 8px;
    object-fit: cover;
    flex-shrink: 0;
  }

  .play-badge {
    width: 30px;
    height: 30px;
    border-radius: 999px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    color: var(--text-muted);
    flex-shrink: 0;
    transition: background-color 140ms ease, color 140ms ease, box-shadow 140ms ease;
  }

  .listen-tile:hover .play-badge {
    background: var(--accent);
    color: #ffffff;
    box-shadow: 0 4px 12px rgba(33, 144, 164, 0.42);
  }

  .listen-tile.playing .play-badge {
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .hero-card {
    display: flex;
    align-items: center;
    gap: 24px;
    padding: 20px 24px;
    border-radius: 18px;
    background: linear-gradient(135deg, rgba(var(--ambient-rgb), 0.24), var(--surface) 65%);
    border: 1px solid var(--border);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.28);
  }

  .hero-cover {
    width: 148px;
    height: 148px;
    border-radius: 14px;
    object-fit: cover;
    box-shadow: 0 14px 36px rgba(0, 0, 0, 0.45);
    flex-shrink: 0;
  }

  .hero-info {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .hero-badge {
    align-self: flex-start;
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.08em;
    padding: 3px 10px;
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .hero-title {
    margin: 0;
    font-size: 24px;
    font-weight: 850;
  }

  .hero-actions {
    display: flex;
    gap: 10px;
    margin-top: 6px;
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
    transition: background-color 140ms ease, border-color 140ms ease;
  }

  .media-card:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }

  .media-cover-wrap {
    position: relative;
  }

  .media-cover {
    width: 156px;
    height: 156px;
    border-radius: 10px;
    object-fit: cover;
    display: block;
  }

  .year-badge {
    position: absolute;
    top: 8px;
    left: 8px;
    padding: 2px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
    background: rgba(0, 0, 0, 0.65);
    color: #ffffff;
  }

  .media-title,
  .media-sub {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .media-title {
    font-size: 13.5px;
  }

  .media-sub {
    font-size: 12px;
  }

  .pill-group {
    display: flex;
    gap: 4px;
    padding: 3px;
    border-radius: 999px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .pill-group button {
    padding: 5px 12px;
    border-radius: 999px;
    font-size: 12.5px;
    font-weight: 600;
    color: var(--text-muted);
  }

  .pill-group button.active {
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .rank {
    width: 24px;
    text-align: center;
    font-size: 12.5px;
    color: var(--text-muted);
  }

  .dur {
    font-size: 12px;
  }

  .empty-home {
    padding: 32px 0;
    text-align: center;
  }

  @media (max-width: 900px) {
    .quick-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
    .listen-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 560px) {
    .listen-grid {
      grid-template-columns: 1fr;
    }
    .hero-card {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
