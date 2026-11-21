<script lang="ts">
  import { Globe, RefreshCw } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import { formatSubscribers } from "../lib/types";
  import Carousel from "./Carousel.svelte";
  import CoverImage from "./CoverImage.svelte";
  import SelectDropdown from "./SelectDropdown.svelte";
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

  let countryOptions = $derived(
    POPULAR_COUNTRIES.map((c) => ({
      value: c.code,
      label: c.code === "" ? t("home.country_global") : c.label,
    })),
  );
  let countryChanged = $derived(
    (player.chartCountry ?? null) !== (player.homeFeed?.country ?? null),
  );

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
      <h1 class="text-[28px] md:text-[32px] font-extrabold tracking-[-0.025em] leading-tight">{greeting}</h1>
      <p class="text-[13.5px] md:text-[14px] text-[var(--text-muted)] mt-0.5">{t("home.subtitle")}</p>
    </div>
    <div class="flex items-center gap-1.5 shrink-0">
      <SelectDropdown
        value={player.chartCountry ?? ""}
        options={countryOptions}
        ariaLabel={t("home.country_global")}
        onChange={(val) => player.setChartCountry(val || null)}
      >
        {#snippet leading()}
          <Globe size={14} />
        {/snippet}
      </SelectDropdown>
      <button
        type="button"
        class="icon-btn relative"
        class:active={countryChanged}
        title={t("home.reload")}
        onclick={() => void player.loadHome(true)}
      >
        <RefreshCw size={16} class={player.homeLoading ? "animate-spin" : ""} />
        {#if countryChanged && !player.homeLoading}
          <span class="w-2 h-2 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1"></span>
        {/if}
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
            type="button"
            class="group p-2 rounded-2xl hover:bg-white/[0.06] text-left flex flex-col gap-2 transition-colors"
            onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
          >
            <div class="relative w-full rounded-xl overflow-hidden shadow-[0_8px_22px_rgba(0,0,0,0.35)] border border-white/[0.07]">
              <CoverImage
                class="media-cover !rounded-none transition-transform duration-300 group-hover:scale-[1.03]"
                src={album.cover_url}
                alt={album.title}
              />
              {#if album.year}
                <span
                  class="absolute top-2 left-2 px-2 py-0.5 rounded-md text-[10.5px] font-bold bg-black/70 text-white numeric"
                >
                  {album.year}
                </span>
              {/if}
            </div>
            <div class="px-0.5 space-y-0.5 w-full">
              <strong class="block w-full text-[14px] md:text-[14.5px] font-bold max-md:line-clamp-2 max-md:leading-snug max-md:whitespace-normal md:truncate">{album.title}</strong>
              <span class="block text-[12.5px] md:text-[13px] text-[var(--text-muted)] truncate w-full">{album.artist}</span>
            </div>
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
            type="button"
            class="group p-2.5 rounded-2xl hover:bg-white/[0.06] flex flex-col items-center text-center gap-2 transition-colors"
            onclick={() => player.openArtist(artist.id, artist.name)}
          >
            <div class="w-full aspect-square rounded-full overflow-hidden shadow-[0_8px_22px_rgba(0,0,0,0.35)] border border-white/[0.08]">
              <CoverImage
                class="w-full h-full transition-transform duration-300 group-hover:scale-[1.04]"
                src={artist.avatar_url}
                alt={artist.name}
                fallbackIcon="👤"
              />
            </div>
            <strong class="w-full text-[13.5px] font-bold truncate">{artist.name}</strong>
            {#if artist.subscriber_count}
              <span class="text-[11.5px] text-[var(--text-muted)] truncate w-full -mt-1">
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
