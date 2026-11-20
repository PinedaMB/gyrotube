<script lang="ts">
  import { ArrowRight, Heart, Plus } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration, formatSubscribers } from "../lib/types";
  import Carousel from "./Carousel.svelte";
  import CoverImage from "./CoverImage.svelte";

  type FilterTab = "all" | "songs" | "albums" | "artists" | "playlists";
  let filter = $state<FilterTab>("all");

  const PALETTE = [
    "#2190a4",
    "#e66100",
    "#3584e4",
    "#9141ac",
    "#2ec27e",
    "#e01b24",
    "#c061cb",
    "#f5c211",
    "#00b4d8",
    "#625df5",
  ];

  const MOOD_KEYWORDS = [
    "chill",
    "christmas",
    "commute",
    "energize",
    "feel good",
    "focus",
    "gaming",
    "party",
    "romance",
    "sad",
    "sleep",
    "workout",
  ];

  function colorFor(name: string): string {
    let h = 2166136261;
    for (let i = 0; i < name.length; i++) {
      h ^= name.toLowerCase().charCodeAt(i);
      h = Math.imul(h, 16777619);
    }
    return PALETTE[Math.abs(h) % PALETTE.length];
  }

  let moods = $derived(
    player.genres.filter((g) => MOOD_KEYWORDS.some((m) => g.name.toLowerCase().includes(m))),
  );
  let musicGenres = $derived(
    player.genres.filter((g) => !MOOD_KEYWORDS.some((m) => g.name.toLowerCase().includes(m))),
  );

  let results = $derived(player.searchResults);
  let topArtist = $derived(results?.artists[0] ?? null);
  let topFourSongs = $derived(results?.tracks.slice(0, 4) ?? []);
  let hideDuplicateArtists = $derived(
    filter === "all" && Boolean(topArtist) && (results?.artists.length ?? 0) === 1,
  );
</script>

<div class="page-clamp">
  {#if player.searchLoading}
    <div class="space-y-6 animate-pulse" aria-busy="true" aria-label="Searching">
      <!-- Filter pills skeleton -->
      <div class="flex items-center gap-2">
        {#each [64, 92, 96, 86, 98] as w}
          <div class="h-8 rounded-full bg-white/[0.07]" style="width: {w}px;"></div>
        {/each}
      </div>

      <!-- Top result + Top 4 songs skeleton -->
      <div class="grid grid-cols-1 md:grid-cols-[minmax(240px,300px)_minmax(0,1fr)] gap-5">
        <div class="space-y-2.5">
          <div class="h-4 w-24 rounded-md bg-white/[0.07]"></div>
          <div class="rounded-2xl p-5 bg-[var(--surface)] border border-[var(--border)] space-y-4">
            <div class="w-20 h-20 rounded-full bg-white/[0.08]"></div>
            <div class="h-6 w-40 rounded-lg bg-white/[0.09]"></div>
            <div class="flex items-center justify-between pt-2">
              <div class="h-5 w-28 rounded-full bg-white/[0.07]"></div>
              <div class="w-10 h-10 rounded-full bg-white/[0.09]"></div>
            </div>
          </div>
        </div>

        <div class="space-y-2.5">
          <div class="h-4 w-20 rounded-md bg-white/[0.07]"></div>
          <div class="track-list">
            {#each [0, 1, 2, 3] as _}
              <div class="flex items-center gap-3 px-3.5 py-2.5 border-b border-[var(--border)] last:border-b-0">
                <div class="w-11 h-11 rounded-lg bg-white/[0.08] shrink-0"></div>
                <div class="flex-1 space-y-2">
                  <div class="h-3.5 w-44 rounded bg-white/[0.09]"></div>
                  <div class="h-3 w-32 rounded bg-white/[0.06]"></div>
                </div>
                <div class="h-3 w-9 rounded bg-white/[0.06]"></div>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Albums carousel skeleton -->
      <div class="space-y-3">
        <div class="h-5 w-28 rounded-md bg-white/[0.08]"></div>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3">
          {#each [0, 1, 2, 3, 4] as _}
            <div class="rounded-2xl p-2.5 bg-[var(--surface)] border border-[var(--border)] space-y-2.5">
              <div class="aspect-square w-full rounded-xl bg-white/[0.08]"></div>
              <div class="h-3.5 w-4/5 rounded bg-white/[0.08]"></div>
              <div class="h-3 w-1/2 rounded bg-white/[0.05]"></div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else if results}
    <div class="filters">
      <button class="back-explore" onclick={() => player.clearSearch()} title={t("explore.back")}>
        {t("explore.back")}
      </button>
      <button class:active={filter === "all"} onclick={() => (filter = "all")}>{t("explore.all")}</button>
      <button class:active={filter === "songs"} onclick={() => (filter = "songs")}>
        {t("explore.songs")}{#if results.tracks.length}&nbsp;· {results.tracks.length}{/if}
      </button>
      <button class:active={filter === "albums"} onclick={() => (filter = "albums")}>
        {t("explore.albums")}{#if results.albums.length}&nbsp;· {results.albums.length}{/if}
      </button>
      <button class:active={filter === "artists"} onclick={() => (filter = "artists")}>
        {t("explore.artists")}{#if results.artists.length}&nbsp;· {results.artists.length}{/if}
      </button>
      <button class:active={filter === "playlists"} onclick={() => (filter = "playlists")}>
        {t("explore.playlists")}{#if results.playlists.length}&nbsp;· {results.playlists.length}{/if}
      </button>
    </div>

    {#if filter === "all" && (topArtist || topFourSongs.length > 0)}
      <div class="overview-grid">
        {#if topArtist}
          <section class="overview-col">
            <h3 class="col-title">Top result</h3>
            <div
              class="top-result-card"
              role="button"
              tabindex="0"
              onclick={() => player.openArtist(topArtist.id, topArtist.name)}
              onkeydown={(e) => e.key === "Enter" && player.openArtist(topArtist.id, topArtist.name)}
            >
              <CoverImage class="top-avatar" src={topArtist.avatar_url} alt={topArtist.name} fallbackIcon="👤" />
              <h2 class="top-name">{topArtist.name}</h2>
              <div class="top-bottom">
                <div class="top-meta">
                  <span class="type-pill">Artist</span>
                  {#if topArtist.subscriber_count}
                    <span class="muted">{formatSubscribers(topArtist.subscriber_count)}</span>
                  {/if}
                </div>
                <button
                  class="play-circle small-play"
                  title="Play"
                  onclick={(e) => {
                    e.stopPropagation();
                    if (topFourSongs[0]) {
                      player.replaceQueueAndPlay([topFourSongs[0]], 0, `Search: ${player.searchQuery}`);
                    }
                  }}
                >
                  ▶
                </button>
              </div>
            </div>
          </section>
        {/if}

        {#if topFourSongs.length > 0}
          <section class="overview-col songs-col">
            <div class="col-head">
              <h3 class="col-title">Songs</h3>
              {#if results.tracks.length > 4}
                <button class="see-all" onclick={() => (filter = "songs")}>
                  See all <ArrowRight size={13} aria-hidden="true" />
                </button>
              {/if}
            </div>
            <div class="track-list">
              {#each topFourSongs as track (track.id)}
                {@const active = player.currentTrack?.id === track.id}
                <div
                  class="track-row"
                  class:active
                  role="button"
                  tabindex="0"
                  onclick={() => player.replaceQueueAndPlay([track], 0, `Search: ${player.searchQuery}`)}
                  onkeydown={(e) =>
                    e.key === "Enter" &&
                    player.replaceQueueAndPlay([track], 0, `Search: ${player.searchQuery}`)}
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
                    title="Favorite"
                    onclick={(e) => {
                      e.stopPropagation();
                      player.toggleFavorite(track);
                    }}
                  >
                    <Heart size={16} fill={player.isFavorite(track.id) ? "currentColor" : "none"} />
                  </button>
                  <button
                    class="icon-btn"
                    title="Add to queue"
                    onclick={(e) => {
                      e.stopPropagation();
                      player.enqueueTrack(track);
                    }}
                  >
                    <Plus size={16} />
                  </button>
                </div>
              {/each}
            </div>
          </section>
        {/if}
      </div>
    {/if}

    {#if (filter === "all" || filter === "artists") && results.artists.length > 0 && !hideDuplicateArtists}
      <div class="section">
        <Carousel title="Artists" minCardWidth={144}>
          {#each results.artists as artist (artist.id)}
            <button class="artist-card" onclick={() => player.openArtist(artist.id, artist.name)}>
              <CoverImage class="artist-avatar" src={artist.avatar_url} alt={artist.name} fallbackIcon="👤" />
              <strong>{artist.name}</strong>
              {#if artist.subscriber_count}
                <span class="muted small">{formatSubscribers(artist.subscriber_count)}</span>
              {/if}
            </button>
          {/each}
        </Carousel>
      </div>
    {/if}

    {#if (filter === "all" || filter === "albums") && results.albums.length > 0}
      <div class="section">
        <Carousel title="Albums">
          {#each results.albums as album (album.id)}
            <button
              class="media-card"
              onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
            >
              <CoverImage class="media-cover" src={album.cover_url} alt={album.title} />
              <strong class="media-title">{album.title}</strong>
              <span class="media-sub muted">{album.artist}</span>
            </button>
          {/each}
        </Carousel>
      </div>
    {/if}

    {#if (filter === "all" || filter === "playlists") && results.playlists.length > 0}
      <div class="section">
        <Carousel title="Playlists">
          {#each results.playlists as pl (pl.id)}
            <button
              class="media-card"
              onclick={() => player.openAlbum(pl.id, pl.name, pl.cover_url, true)}
            >
              <CoverImage class="media-cover" src={pl.cover_url} alt={pl.name} />
              <strong class="media-title">{pl.name}</strong>
              <span class="media-sub muted">{pl.subtitle}</span>
            </button>
          {/each}
        </Carousel>
      </div>
    {/if}

    {#if filter === "songs" && results.tracks.length > 0}
      <section class="section">
        <h2 class="section-title">Songs</h2>
        <div class="track-list">
          {#each results.tracks as track (track.id)}
            {@const active = player.currentTrack?.id === track.id}
            <div
              class="track-row"
              class:active
              role="button"
              tabindex="0"
              onclick={() => player.replaceQueueAndPlay([track], 0, `Search: ${player.searchQuery}`)}
              onkeydown={(e) =>
                e.key === "Enter" &&
                player.replaceQueueAndPlay([track], 0, `Search: ${player.searchQuery}`)}
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
                title="Favorite"
                onclick={(e) => {
                  e.stopPropagation();
                  player.toggleFavorite(track);
                }}
              >
                <Heart size={16} fill={player.isFavorite(track.id) ? "currentColor" : "none"} />
              </button>
              <button
                class="icon-btn"
                title="Add to queue"
                onclick={(e) => {
                  e.stopPropagation();
                  player.enqueueTrack(track);
                }}
              >
                <Plus size={16} />
              </button>
            </div>
          {/each}
        </div>
      </section>
    {/if}
  {:else}
    <h1 class="explore-title">{t("explore.title")}</h1>
    <p class="muted caption">{t("explore.subtitle")}</p>

    {#if moods.length > 0}
      <section class="section">
        <h2 class="section-title">{t("explore.moods")}</h2>
        <div class="genre-grid">
          {#each moods as m (m.id)}
            {@const c = colorFor(m.name)}
            <button
              class="mood-tile"
              style="background: linear-gradient(135deg, {c}33, var(--surface) 68%); border-color: {c}44;"
              onclick={() => void player.runSearch(m.name)}
            >
              <span class="mood-dot" style="background: {c}33; color: {c};">♪</span>
              <span class="genre-name">{m.name}</span>
              <span class="muted">›</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    {#if musicGenres.length > 0}
      <section class="section">
        <h2 class="section-title">{t("explore.genres")}</h2>
        <div class="genre-grid">
          {#each musicGenres as g (g.id)}
            {@const c = colorFor(g.name)}
            <button
              class="genre-chip"
              style="background: linear-gradient(90deg, {c}26, var(--surface) 62%);"
              onclick={() => void player.runSearch(g.name)}
            >
              <span class="genre-bar" style="background: {c};"></span>
              <span class="genre-name">{g.name}</span>
              <span class="muted">›</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</div>

<style>
  .explore-title {
    margin: 0;
    font-size: 28px;
    font-weight: 850;
  }

  .caption {
    margin: 3px 0 0;
    font-size: 13px;
  }

  .filters {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 18px;
  }

  .filters button {
    padding: 6px 14px;
    border-radius: 999px;
    font-size: 13px;
    font-weight: 600;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .filters button.active {
    background: var(--accent-soft);
    border-color: var(--accent-border);
    color: var(--accent-bright);
  }

  .overview-grid {
    display: grid;
    grid-template-columns: 300px minmax(0, 1fr);
    gap: 18px;
    margin-bottom: 22px;
  }

  .overview-col {
    min-width: 0;
  }

  .col-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .col-title {
    margin: 0 0 10px;
    font-size: 17px;
    font-weight: 800;
  }

  .see-all {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 12.5px;
    font-weight: 600;
    color: var(--text-muted);
    padding: 4px 10px;
    border-radius: 999px;
  }

  .see-all:hover {
    color: var(--accent-bright);
    background: var(--accent-soft);
  }

  .top-result-card {
    padding: 18px;
    border-radius: 16px;
    background: linear-gradient(145deg, var(--accent-soft), var(--surface) 60%);
    border: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 12px;
    cursor: pointer;
    transition: border-color 150ms ease, box-shadow 150ms ease;
  }

  .top-result-card:hover {
    border-color: var(--accent-border);
    box-shadow: 0 10px 26px rgba(0, 0, 0, 0.28);
  }

  .top-name {
    margin: 0;
    font-size: 22px;
    font-weight: 850;
  }

  .top-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }

  .top-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
  }

  .type-pill {
    padding: 3px 10px;
    border-radius: 999px;
    font-size: 11px;
    font-weight: 700;
    background: var(--surface-hover);
  }

  .small-play {
    width: 42px;
    height: 42px;
  }

  .section {
    margin-top: 22px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .artist-card {
    width: 148px;
    flex-shrink: 0;
    padding: 10px;
    border-radius: 14px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    text-align: center;
  }

  .artist-card:hover {
    background: var(--surface);
  }

  .small {
    font-size: 11.5px;
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

  .media-title {
    font-size: 13.5px;
  }

  .media-sub {
    font-size: 12px;
  }

  .genre-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 10px;
  }

  .mood-tile,
  .genre-chip {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 14px;
    border-radius: 12px;
    border: 1px solid var(--border);
    text-align: left;
    font-weight: 600;
  }

  .mood-dot {
    width: 32px;
    height: 32px;
    border-radius: 10px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .genre-bar {
    width: 4px;
    height: 22px;
    border-radius: 999px;
    flex-shrink: 0;
  }

  .genre-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 13.5px;
  }

  .dur {
    font-size: 12px;
  }

  @media (max-width: 780px) {
    .overview-grid {
      grid-template-columns: minmax(0, 1fr);
    }
    .genre-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
</style>
