<script lang="ts">
  import { Compass } from "lucide-svelte";
  import { api, isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { RelatedFeed } from "../lib/types";
  import Carousel from "./Carousel.svelte";
  import CoverImage from "./CoverImage.svelte";
  import OfflineState from "./OfflineState.svelte";
  import TrackRow from "./TrackRow.svelte";
  import HomeFeaturedHero from "./home/HomeFeaturedHero.svelte";
  import HomeListenAgain from "./home/HomeListenAgain.svelte";
  import HomePersonalMixes from "./home/HomePersonalMixes.svelte";
  import HomeShortcuts from "./home/HomeShortcuts.svelte";
  import HomeSkeleton from "./home/HomeSkeleton.svelte";
  import HomeTrendingCharts from "./home/HomeTrendingCharts.svelte";

  let greeting = $derived.by(() => {
    const h = new Date().getHours();
    if (h >= 5 && h < 12) return t("home.morning");
    if (h >= 12 && h < 19) return t("home.afternoon");
    return t("home.evening");
  });

  let listenAgain = $derived(player.recentTracks.slice(0, 6));
  let hasPersonalLibrary = $derived(
    player.recentTracks.length > 0 ||
      player.favorites.length > 0 ||
      player.userPlaylists.length > 0 ||
      player.followedArtists.length > 0,
  );

  // Personalized "Because you listened to..." feed seeded from recent/favorite track
  let seedTrack = $derived(player.recentTracks[0] ?? player.favorites[0] ?? null);
  let relatedFeed = $state<RelatedFeed | null>(null);
  let relatedSeedId = $state<string | null>(null);

  $effect(() => {
    const seed = seedTrack;
    if (!seed || player.isOffline || !isTauriRuntime()) return;
    if (relatedSeedId === seed.id && relatedFeed) return;
    relatedSeedId = seed.id;
    void api
      .getRelatedForTrack(seed.id)
      .then((feed) => {
        if (relatedSeedId === seed.id) {
          relatedFeed = feed;
        }
      })
      .catch(() => {
        // Non-fatal: personalized sections remain hidden if unavailable
      });
  });

  let featuredAlbum = $derived(player.homeFeed?.new_albums[0] ?? null);
</script>

<div class="page-clamp">
  {#if player.isOffline}
    <OfflineState />
  {:else}
    <div class="flex items-center justify-between gap-2 mb-5">
      <div>
        <h1 class="text-[28px] md:text-[32px] font-extrabold tracking-[-0.025em] leading-tight">
          {greeting}
        </h1>
        <p class="text-[13.5px] md:text-[14px] text-[var(--text-muted)] mt-0.5">
          {t("home.subtitle")}
        </p>
      </div>
      <button
        type="button"
        class="pill-btn shrink-0"
        onclick={() => {
          player.clearSearch();
          player.navigateTab("explore");
        }}
      >
        <Compass size={15} />
        <span>{t("home.discover_more")}</span>
      </button>
    </div>

    {#if hasPersonalLibrary}
      <HomeShortcuts />

      <HomeListenAgain tracks={listenAgain} />

      <HomePersonalMixes />

      {#if seedTrack && relatedFeed}
        {#if relatedFeed.albums.length > 0}
          <div class="mt-6">
            <Carousel title={t("home.because_you_listened", { title: seedTrack.title })}>
              {#each relatedFeed.albums.slice(0, 12) as album (album.id)}
                <button
                  type="button"
                  class="group p-2 rounded-2xl hover:bg-[var(--surface-hover)] text-left flex flex-col gap-2 transition-colors"
                  onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
                >
                  <div class="relative w-full rounded-xl overflow-hidden shadow-[0_6px_18px_rgba(15,23,42,0.18)] border border-[var(--border)]">
                    <CoverImage
                      class="media-cover !rounded-none transition-transform duration-300 group-hover:scale-[1.03]"
                      src={album.cover_url}
                      alt={album.title}
                    />
                  </div>
                  <div class="px-0.5 space-y-0.5 w-full">
                    <strong class="block w-full text-[14px] md:text-[14.5px] font-bold max-md:line-clamp-2 max-md:leading-snug max-md:whitespace-normal md:truncate">
                      {album.title}
                    </strong>
                    <span class="block text-[12.5px] md:text-[13px] text-[var(--text-muted)] truncate w-full">
                      {album.artist}
                    </span>
                  </div>
                </button>
              {/each}
            </Carousel>
          </div>
        {:else if relatedFeed.tracks.length > 0}
          <section class="mt-6 flex flex-col gap-3">
            <h2 class="section-title">
              {t("home.because_you_listened", { title: seedTrack.title })}
            </h2>
            <div class="track-list">
              {#each relatedFeed.tracks.slice(0, 6) as track, i (track.id + ":" + i)}
                <TrackRow
                  {track}
                  onPlay={() =>
                    player.replaceQueueAndPlay(
                      relatedFeed?.tracks ?? [],
                      i,
                      t("related.tracks"),
                    )}
                />
              {/each}
            </div>
          </section>
        {/if}

        {#if relatedFeed.artists.length > 0}
          <div class="mt-6">
            <Carousel title={t("related.artists")} minCardWidth={148}>
              {#each relatedFeed.artists.slice(0, 10) as artist (artist.id)}
                <button
                  type="button"
                  class="group p-2.5 rounded-2xl hover:bg-[var(--surface-hover)] flex flex-col items-center text-center gap-2 transition-colors"
                  onclick={() => player.openArtist(artist.id, artist.name)}
                >
                  <div class="w-full aspect-square rounded-full overflow-hidden shadow-[0_6px_18px_rgba(15,23,42,0.18)] border border-[var(--border)]">
                    <CoverImage
                      class="w-full h-full transition-transform duration-300 group-hover:scale-[1.04]"
                      src={artist.avatar_url}
                      alt={artist.name}
                      fallbackIcon="👤"
                    />
                  </div>
                  <strong class="w-full text-[13.5px] font-bold truncate">{artist.name}</strong>
                </button>
              {/each}
            </Carousel>
          </div>
        {/if}
      {/if}

      {#if player.followedArtists.length > 0}
        <div class="mt-6">
          <Carousel title={t("collection.followed_artists")} minCardWidth={148}>
            {#each player.followedArtists as artist (artist.id)}
              <button
                type="button"
                class="group p-2.5 rounded-2xl hover:bg-[var(--surface-hover)] flex flex-col items-center text-center gap-2 transition-colors"
                onclick={() => player.openArtist(artist.id, artist.name)}
              >
                <div class="w-full aspect-square rounded-full overflow-hidden shadow-[0_6px_18px_rgba(15,23,42,0.18)] border border-[var(--border)]">
                  <CoverImage
                    class="w-full h-full transition-transform duration-300 group-hover:scale-[1.04]"
                    src={artist.avatar_url}
                    alt={artist.name}
                    fallbackIcon="👤"
                  />
                </div>
                <strong class="w-full text-[13.5px] font-bold truncate">{artist.name}</strong>
              </button>
            {/each}
          </Carousel>
        </div>
      {/if}
    {:else}
      <!-- First-time user fallback before any local listening history exists -->
      {#if player.homeLoading && !player.homeFeed}
        <HomeSkeleton />
      {/if}

      {#if featuredAlbum}
        <HomeFeaturedHero album={featuredAlbum} />
      {/if}

      <HomeTrendingCharts />
    {/if}
  {/if}
</div>
