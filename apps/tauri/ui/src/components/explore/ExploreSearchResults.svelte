<script lang="ts">
  import { Play } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatSubscribers, type SearchResults } from "../../lib/types";
  import Carousel from "../Carousel.svelte";
  import CoverImage from "../CoverImage.svelte";
  import TrackRow from "../TrackRow.svelte";

  type FilterTab = "all" | "songs" | "albums" | "artists" | "playlists";

  let { results }: { results: SearchResults } = $props();
  let filter = $state<FilterTab>("all");
  let filterBarEl = $state<HTMLDivElement | null>(null);
  let lastResults: SearchResults | null = null;

  $effect(() => {
    if (results !== lastResults) {
      lastResults = results;
      filter = "all";
      filterBarEl?.closest(".scroll-y")?.scrollTo({ top: 0 });
    }
  });

  let topArtist = $derived(results.artists[0] ?? null);
  let topFourSongs = $derived(results.tracks.slice(0, 4));
  let hideDuplicateArtists = $derived(
    filter === "all" && Boolean(topArtist) && results.artists.length === 1,
  );
</script>

<div bind:this={filterBarEl} class="flex flex-wrap gap-2 mb-5">
  <button
    type="button"
    class="hidden md:inline-flex px-3.5 py-1.5 rounded-full text-[13px] font-semibold bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] transition-colors"
    onclick={() => player.goBack()}
    title={t("nav.back")}
  >
    ← {t("nav.back")}
  </button>
  {#each [
    { id: "all", label: t("explore.all"), count: 0 },
    { id: "songs", label: t("explore.songs"), count: results.tracks.length },
    { id: "albums", label: t("explore.albums"), count: results.albums.length },
    { id: "artists", label: t("explore.artists"), count: results.artists.length },
    { id: "playlists", label: t("explore.playlists"), count: results.playlists.length },
  ] as tab (tab.id)}
    <button
      type="button"
      aria-pressed={filter === tab.id}
      class="px-3.5 py-1.5 rounded-full text-[13px] font-semibold border transition-colors {filter === tab.id
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'bg-[var(--surface)] border-[var(--border)] hover:bg-[var(--surface-hover)]'}"
      onclick={() => (filter = tab.id as FilterTab)}
    >
      {tab.label}{#if tab.count}&nbsp;· {tab.count}{/if}
    </button>
  {/each}
</div>

{#if filter === "all" && (topArtist || topFourSongs.length > 0)}
  <div class="grid grid-cols-1 md:grid-cols-[300px_minmax(0,1fr)] gap-5 mb-6">
    {#if topArtist}
      <section class="min-w-0">
        <h2 class="m-0 mb-2.5 text-[17px] font-extrabold">{t("explore.top_result")}</h2>
        <div
          class="p-[18px] rounded-2xl border border-[var(--border)] hover:border-[var(--accent-border)] hover:shadow-[0_10px_26px_rgba(0,0,0,0.28)] flex flex-col gap-3 cursor-pointer transition-all"
          style="background: linear-gradient(145deg, var(--accent-soft), var(--surface) 60%);"
          role="button"
          tabindex="0"
          onclick={() => player.openArtist(topArtist.id, topArtist.name)}
          onkeydown={(e) => {
            if (e.target !== e.currentTarget) return;
            if (e.key === "Enter") player.openArtist(topArtist.id, topArtist.name);
            else if (e.key === " ") e.preventDefault();
          }}
          onkeyup={(e) => {
            if (e.target === e.currentTarget && e.key === " ") {
              player.openArtist(topArtist.id, topArtist.name);
            }
          }}
        >
          <CoverImage class="top-avatar" src={topArtist.avatar_url} alt="" fallbackIcon="👤" />
          <h3 class="m-0 text-[22px] font-extrabold truncate">{topArtist.name}</h3>
          <div class="flex items-center justify-between gap-2.5">
            <div class="flex items-center gap-2 text-[12px] min-w-0">
              <span class="px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-[var(--surface-hover)] shrink-0">
                {t("explore.artist_pill")}
              </span>
              {#if topArtist.subscriber_count}
                <span class="muted truncate">{formatSubscribers(topArtist.subscriber_count)}</span>
              {/if}
            </div>
            <button
              type="button"
              class="play-circle !w-[42px] !h-[42px] shrink-0"
              title={t("home.play")}
              aria-label={`${t("home.play")} ${topArtist.name}`}
              onclick={(e) => {
                e.stopPropagation();
                if (topFourSongs[0]) {
                  player.replaceQueueAndPlay([topFourSongs[0]], 0, `Search: ${player.searchQuery}`);
                }
              }}
            >
              <Play size={17} fill="currentColor" class="translate-x-[1px]" aria-hidden="true" />
            </button>
          </div>
        </div>
      </section>
    {/if}

    {#if topFourSongs.length > 0}
      <section class="min-w-0">
        <div class="flex items-center justify-between mb-2.5">
          <h2 class="m-0 text-[17px] font-extrabold">{t("explore.songs")}</h2>
          {#if results.tracks.length > 4}
            <button
              type="button"
              class="inline-flex items-center gap-1 text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--accent-bright)] hover:bg-[var(--accent-soft)] px-2.5 py-1 rounded-full transition-colors"
              onclick={() => (filter = "songs")}
            >
              {t("home.see_all")}
            </button>
          {/if}
        </div>
        <div class="track-list">
          {#each topFourSongs as track (track.id)}
            <TrackRow {track} onPlay={() => player.replaceQueueAndPlay([track], 0, `Search: ${player.searchQuery}`)} />
          {/each}
        </div>
      </section>
    {/if}
  </div>
{/if}

{#if (filter === "all" || filter === "artists") && results.artists.length > 0 && !hideDuplicateArtists}
  <div class="mt-6">
    <Carousel title={t("explore.artists")} minCardWidth={154}>
      {#each results.artists as artist (artist.id)}
        <button
          type="button"
          class="w-[154px] shrink-0 p-2.5 rounded-2xl hover:bg-[var(--surface)] flex flex-col items-center gap-2 text-center transition-colors"
          onclick={() => player.openArtist(artist.id, artist.name)}
        >
          <CoverImage class="artist-avatar" src={artist.avatar_url} alt={artist.name} fallbackIcon="👤" />
          <strong class="text-[14px] font-bold truncate max-w-full">{artist.name}</strong>
          {#if artist.subscriber_count}
            <span class="muted text-[12px] truncate max-w-full">{formatSubscribers(artist.subscriber_count)}</span>
          {/if}
        </button>
      {/each}
    </Carousel>
  </div>
{/if}

{#if (filter === "all" || filter === "albums") && results.albums.length > 0}
  <div class="mt-6">
    <Carousel title={t("explore.albums")}>
      {#each results.albums as album (album.id)}
        <button
          type="button"
          class="group w-[184px] shrink-0 p-2 rounded-2xl hover:bg-[var(--surface)] text-left flex flex-col gap-2 transition-colors"
          onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
        >
          <CoverImage class="media-cover !rounded-xl shadow-[0_6px_18px_rgba(0,0,0,0.3)]" src={album.cover_url} alt={album.title} />
          <div class="px-0.5 min-w-0">
            <strong class="block text-[14px] font-bold truncate">{album.title}</strong>
            <span class="block text-[12.5px] muted truncate mt-0.5">{album.artist}</span>
          </div>
        </button>
      {/each}
    </Carousel>
  </div>
{/if}

{#if (filter === "all" || filter === "playlists") && results.playlists.length > 0}
  <div class="mt-6">
    <Carousel title={t("explore.playlists")}>
      {#each results.playlists as pl (pl.id)}
        <button
          type="button"
          class="group w-[184px] shrink-0 p-2 rounded-2xl hover:bg-[var(--surface)] text-left flex flex-col gap-2 transition-colors"
          onclick={() => player.openAlbum(pl.id, pl.name, pl.cover_url, true)}
        >
          <CoverImage class="media-cover !rounded-xl shadow-[0_6px_18px_rgba(0,0,0,0.3)]" src={pl.cover_url} alt={pl.name} />
          <div class="px-0.5 min-w-0">
            <strong class="block text-[14px] font-bold truncate">{pl.name}</strong>
            <span class="block text-[12.5px] muted truncate mt-0.5">{pl.subtitle}</span>
          </div>
        </button>
      {/each}
    </Carousel>
  </div>
{/if}

{#if filter === "songs" && results.tracks.length > 0}
  <section class="mt-6 space-y-3">
    <h2 class="section-title">{t("explore.songs")}</h2>
    <div class="track-list">
      {#each results.tracks as track (track.id)}
        <TrackRow {track} onPlay={() => player.replaceQueueAndPlay([track], 0, `Search: ${player.searchQuery}`)} />
      {/each}
    </div>
  </section>
{/if}
