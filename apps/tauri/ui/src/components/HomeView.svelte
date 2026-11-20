<script lang="ts">
  import { RefreshCw } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import Carousel from "./Carousel.svelte";
  import CoverImage from "./CoverImage.svelte";
  import HomeFeaturedHero from "./home/HomeFeaturedHero.svelte";
  import HomeListenAgain from "./home/HomeListenAgain.svelte";
  import HomeQuickCards from "./home/HomeQuickCards.svelte";
  import HomeStationsGrid from "./home/HomeStationsGrid.svelte";
  import HomeTrendingCharts from "./home/HomeTrendingCharts.svelte";

  let greeting = $derived.by(() => {
    const h = new Date().getHours();
    if (h >= 5 && h < 12) return t("home.morning");
    if (h >= 12 && h < 19) return t("home.afternoon");
    return t("home.evening");
  });

  let listenAgain = $derived(player.recentTracks.slice(0, 6));
  let featuredAlbum = $derived(player.homeFeed?.new_albums[0] ?? null);
  let releaseAlbums = $derived(
    player.homeFeed
      ? player.homeFeed.new_albums.slice(player.homeFeed.new_albums.length > 1 ? 1 : 0, 16)
      : [],
  );
  let stations = $derived(player.genres.slice(0, 6));
</script>

<div class="page-clamp">
  <div class="flex items-center justify-between mb-4">
    <div>
      <h1 class="text-[28px] font-extrabold tracking-[-0.025em] leading-tight">{greeting}</h1>
      <p class="text-[13px] text-[var(--text-muted)] mt-0.5">{t("home.subtitle")}</p>
    </div>
    <button class="icon-btn" title={t("home.reload")} onclick={() => void player.loadHome(true)}>
      <RefreshCw size={16} class={player.homeLoading ? "animate-spin" : ""} />
    </button>
  </div>

  <HomeQuickCards />

  <HomeListenAgain tracks={listenAgain} />

  {#if featuredAlbum}
    <HomeFeaturedHero album={featuredAlbum} />
  {/if}

  {#if releaseAlbums.length > 0}
    <div class="mt-6">
      <Carousel title={t("home.new_releases")} id="new-releases">
        {#each releaseAlbums as album (album.id)}
          <button
            class="p-2.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] text-left flex flex-col gap-1.5 transition-colors"
            onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
          >
            <div class="relative w-full">
              <CoverImage class="media-cover" src={album.cover_url} alt={album.title} />
              {#if album.year}
                <span
                  class="absolute top-2 left-2 px-2 py-0.5 rounded-md text-[11px] font-bold bg-black/65 text-white numeric"
                >
                  {album.year}
                </span>
              {/if}
            </div>
            <strong class="text-[13.5px] truncate w-full">{album.title}</strong>
            <span class="text-[12px] text-[var(--text-muted)] truncate w-full">{album.artist}</span>
          </button>
        {/each}
      </Carousel>
    </div>
  {/if}

  <HomeTrendingCharts />

  <HomeStationsGrid {stations} />
</div>
