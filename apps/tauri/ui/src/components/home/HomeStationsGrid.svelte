<script lang="ts">
  import {
    ArrowRight,
    Car,
    Headphones,
    Heart,
    Moon,
    Music,
    Play,
    Target,
    Zap,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Genre } from "../../lib/types";

  const STATION_COLORS = [
    "#e05263",
    "#2ec27e",
    "#9141ac",
    "#c061cb",
    "#e66100",
    "#2190a4",
  ];

  let { stations }: { stations: Genre[] } = $props();
  let sectionEl = $state<HTMLElement | null>(null);
  let loadingStationId = $state<string | null>(null);
  let stationCovers = $state<Record<string, string>>({});
  let stationFirstPlaylist = $state<Record<string, string>>({});
  let loadedCovers = $state<Record<string, boolean>>({});
  let fetchedIds = new Set<string>();
  let inView = $state(false);

  function resizeCover(url: string): string {
    if (url.includes("googleusercontent.com") && /=w\d+-h\d+/.test(url)) {
      return url.replace(/=w\d+-h\d+[^&?]*/, "=w360-h360-l90-rj");
    }
    return url;
  }

  onMount(() => {
    if (!sectionEl || typeof IntersectionObserver === "undefined") {
      inView = true;
      return;
    }
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries.some((e) => e.isIntersecting)) {
          inView = true;
          observer.disconnect();
        }
      },
      { rootMargin: "220px" },
    );
    observer.observe(sectionEl);
    return () => observer.disconnect();
  });

  $effect(() => {
    if (!inView || !isTauriRuntime() || stations.length === 0) return;
    for (const genre of stations) {
      if (fetchedIds.has(genre.id)) continue;
      fetchedIds.add(genre.id);
      void api
        .getGenrePlaylists(genre.id)
        .then(([_title, playlists]) => {
          const first = playlists.find((p) => p.cover_url) ?? playlists[0];
          if (first) {
            stationFirstPlaylist = { ...stationFirstPlaylist, [genre.id]: first.id };
            if (first.cover_url) {
              stationCovers = { ...stationCovers, [genre.id]: resizeCover(first.cover_url) };
            }
          }
        })
        .catch(() => {});
    }
  });

  function stationColor(g: Genre, index: number): string {
    if (g.color_hex && g.color_hex.startsWith("#") && g.color_hex.length >= 7) {
      return g.color_hex;
    }
    return STATION_COLORS[index % STATION_COLORS.length];
  }

  function stationIconKind(
    name: string,
  ): "moon" | "car" | "zap" | "heart" | "target" | "headphones" | "music" {
    const n = name.toLowerCase();
    if (n.includes("chill") || n.includes("sleep") || n.includes("relax")) return "moon";
    if (n.includes("commute") || n.includes("drive") || n.includes("travel")) return "car";
    if (n.includes("energ") || n.includes("workout") || n.includes("party")) return "zap";
    if (n.includes("feel") || n.includes("roman") || n.includes("love")) return "heart";
    if (n.includes("focus") || n.includes("study")) return "target";
    if (n.includes("christmas") || n.includes("holiday") || n.includes("mix")) return "headphones";
    return "music";
  }

  async function playStation(g: Genre) {
    if (!isTauriRuntime() || loadingStationId) return;
    loadingStationId = g.id;
    try {
      let playlistId = stationFirstPlaylist[g.id];
      if (!playlistId) {
        const [_title, playlists] = await api.getGenrePlaylists(g.id);
        playlistId = playlists[0]?.id;
      }
      if (playlistId) {
        const [name, tracks] = await api.getPlaylistTracks(playlistId);
        if (tracks.length > 0) {
          player.replaceQueueAndPlay(tracks, 0, name || g.name);
          return;
        }
      }
      await player.runSearch(g.name);
    } catch (err) {
      player.showToast(String(err));
    } finally {
      loadingStationId = null;
    }
  }
</script>

{#if stations.length > 0}
  <section bind:this={sectionEl} class="mt-6 flex flex-col gap-3">
    <div class="flex items-center justify-between gap-3">
      <h2 class="section-title">{t("home.stations_title")}</h2>
      <button
        class="shrink-0 text-[12.5px] font-semibold text-[var(--accent-bright)] hover:underline"
        onclick={() => {
          player.clearSearch();
          player.navigateTab("explore");
        }}
      >
        {t("home.see_all")}
      </button>
    </div>

    <div class="grid grid-cols-2 lg:grid-cols-3 gap-2.5 md:gap-3.5">
      {#each stations as genre, idx (genre.id)}
        {@const c = stationColor(genre, idx)}
        {@const icon = stationIconKind(genre.name)}
        {@const bgUrl = stationCovers[genre.id]}
        <div
          class="group relative overflow-hidden rounded-2xl p-2.5 md:p-4 border border-white/10 hover:border-white/20 cursor-pointer transition-all duration-200 hover:-translate-y-0.5 flex flex-col justify-between md:min-h-[152px] bg-[var(--surface)]"
          role="button"
          tabindex="0"
          onclick={() => void player.runSearch(genre.name)}
          onkeydown={(e) => e.target === e.currentTarget && e.key === "Enter" && void player.runSearch(genre.name)}
        >
          <!-- Background playlist artwork (lazy loaded) -->
          {#if bgUrl}
            <img
              src={bgUrl}
              alt=""
              loading="lazy"
              decoding="async"
              referrerpolicy="no-referrer"
              onload={() => (loadedCovers = { ...loadedCovers, [genre.id]: true })}
              class="absolute inset-0 w-full h-full object-cover scale-105 group-hover:scale-110 transition-all duration-500 pointer-events-none {loadedCovers[genre.id]
                ? 'opacity-20 md:opacity-35'
                : 'opacity-0'}"
            />
          {/if}

          <!-- Color tint + readability scrim -->
          <div
            class="absolute inset-0 pointer-events-none"
            style="background: linear-gradient(155deg, {c}80 0%, {c}44 45%, rgba(18, 18, 22, 0.88) 100%);"
          ></div>

          <!-- Card content -->
          <div class="relative z-10 flex items-center justify-end md:justify-between gap-2">
            <span
              class="hidden md:inline px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-black/55"
              style="color: {c};"
            >
              {t("home.station_badge")}
            </span>
            <span class="opacity-90" style="color: {c};">
              {#if icon === "moon"}
                <Moon size={16} />
              {:else if icon === "car"}
                <Car size={16} />
              {:else if icon === "zap"}
                <Zap size={16} />
              {:else if icon === "heart"}
                <Heart size={16} fill="currentColor" />
              {:else if icon === "target"}
                <Target size={16} />
              {:else if icon === "headphones"}
                <Headphones size={16} />
              {:else}
                <Music size={16} />
              {/if}
            </span>
          </div>

          <div class="relative z-10 my-1 md:my-3">
            <h3 class="text-[16px] md:text-[18px] font-extrabold tracking-tight leading-snug drop-shadow-sm break-words">
              {genre.name}
            </h3>
            <p class="hidden md:line-clamp-1 text-[12px] text-white/75 mt-0.5">
              {t("home.station_sub")}
            </p>
          </div>

          <div class="relative z-10 flex items-center justify-between gap-2">
            <span
              class="inline-flex items-center gap-1 text-[12px] font-bold group-hover:translate-x-0.5 transition-transform"
              style="color: {c};"
            >
              <span class="hidden md:inline">{t("home.explore_station")}</span>
              <ArrowRight size={13} />
            </span>
            <button
              class="w-11 h-11 md:w-9 md:h-9 rounded-full inline-flex items-center justify-center bg-black/45 hover:bg-black/65 border border-white/15 text-white shadow-md transition-all active:scale-95"
              title="Play {genre.name}"
              disabled={loadingStationId === genre.id}
              onclick={(e) => {
                e.stopPropagation();
                void playStation(genre);
              }}
            >
              {#if loadingStationId === genre.id}
                <span class="w-4 h-4 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>
              {:else}
                <Play size={15} fill="currentColor" class="translate-x-[1px]" />
              {/if}
            </button>
          </div>
        </div>
      {/each}
    </div>
  </section>
{/if}
