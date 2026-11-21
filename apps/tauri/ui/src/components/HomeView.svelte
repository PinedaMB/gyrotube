<script lang="ts">
  import { Globe, RefreshCw } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import { formatSubscribers } from "../lib/types";
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

  const POPULAR_COUNTRIES: { code: string; label: string }[] = [
    { code: "", label: "Global" },
    { code: "MX", label: "México" },
    { code: "US", label: "United States" },
    { code: "ES", label: "España" },
    { code: "AR", label: "Argentina" },
    { code: "CO", label: "Colombia" },
    { code: "CL", label: "Chile" },
    { code: "PE", label: "Perú" },
    { code: "BR", label: "Brasil" },
    { code: "GB", label: "UK" },
    { code: "FR", label: "France" },
    { code: "DE", label: "Deutschland" },
    { code: "JP", label: "Japan" },
    { code: "KR", label: "Korea" },
  ];

  let listenAgain = $derived(player.recentTracks.slice(0, 6));
  let featuredAlbum = $derived(player.homeFeed?.new_albums[0] ?? null);
  let releaseAlbums = $derived(
    player.homeFeed
      ? player.homeFeed.new_albums.slice(player.homeFeed.new_albums.length > 1 ? 1 : 0, 16)
      : [],
  );
  let topArtists = $derived(player.homeFeed?.top_artists ?? []);
  let stations = $derived(player.genres.slice(0, 6));
</script>

<div class="page-clamp">
  <div class="flex items-center justify-between gap-2 mb-4">
    <div>
      <h1 class="text-[28px] font-extrabold tracking-[-0.025em] leading-tight">{greeting}</h1>
      <p class="text-[13px] text-[var(--text-muted)] mt-0.5">{t("home.subtitle")}</p>
    </div>
    <div class="flex items-center gap-1.5 shrink-0">
      <label
        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[12px] font-semibold bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--border-strong)] transition-colors cursor-pointer"
      >
        <Globe size={13} class="text-[var(--text-muted)] shrink-0" />
        <select
          class="bg-transparent outline-none cursor-pointer text-[12px] font-semibold"
          value={player.chartCountry ?? ""}
          onchange={(e) => player.setChartCountry((e.currentTarget as HTMLSelectElement).value || null)}
        >
          {#each POPULAR_COUNTRIES as c}
            <option value={c.code} class="bg-[var(--bg-elevated)] text-[var(--text)]">
              {c.code === "" ? t("home.country_global") : c.label}
            </option>
          {/each}
        </select>
      </label>
      <button class="icon-btn" title={t("home.reload")} onclick={() => void player.loadHome(true)}>
        <RefreshCw size={16} class={player.homeLoading ? "animate-spin" : ""} />
      </button>
    </div>
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
            class="p-2 md:p-2.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] text-left flex flex-col gap-1.5 transition-colors"
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
            <strong class="w-full text-[13.5px] max-md:line-clamp-2 max-md:leading-snug max-md:whitespace-normal md:truncate">{album.title}</strong>
            <span class="text-[12px] text-[var(--text-muted)] truncate w-full">{album.artist}</span>
          </button>
        {/each}
      </Carousel>
    </div>
  {/if}

  <HomeTrendingCharts />

  {#if topArtists.length > 0}
    <div class="mt-6">
      <Carousel title={t("home.top_artists")} id="top-artists">
        {#each topArtists as artist (artist.id)}
          <button
            class="p-3 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] flex flex-col items-center text-center gap-2 transition-colors"
            onclick={() => player.openArtist(artist.id, artist.name)}
          >
            <CoverImage
              class="w-full aspect-square rounded-full"
              src={artist.avatar_url}
              alt={artist.name}
              fallbackIcon="👤"
            />
            <strong class="w-full text-[13.5px] truncate">{artist.name}</strong>
            {#if artist.subscriber_count}
              <span class="text-[11.5px] text-[var(--text-muted)] truncate w-full">
                {formatSubscribers(artist.subscriber_count)}
              </span>
            {/if}
          </button>
        {/each}
      </Carousel>
    </div>
  {/if}

  <HomeStationsGrid {stations} />
</div>
