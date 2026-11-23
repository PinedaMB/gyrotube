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
  import { STATION_PALETTE } from "../../lib/genrePalette";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Genre } from "../../lib/types";

  const STATION_COLORS = STATION_PALETTE;

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
      {#if player.activeView.kind !== "explore"}
        <button
          class="shrink-0 text-[12.5px] font-semibold text-[var(--accent-bright)] hover:underline"
          onclick={() => {
            player.clearSearch();
            player.navigateTab("explore");
          }}
        >
          {t("home.see_all")}
        </button>
      {/if}
    </div>

    <div class="grid grid-cols-2 lg:grid-cols-3 gap-2.5 md:gap-3.5">
      {#each stations as genre, idx (genre.id)}
        {@const c = stationColor(genre, idx)}
        {@const icon = stationIconKind(genre.name)}
        {@const bgUrl = stationCovers[genre.id]}
        <div
          class="group relative overflow-hidden rounded-2xl p-2.5 md:p-4 border border-[var(--border-strong)] hover:border-[var(--accent-border)] cursor-pointer transition-all duration-200 hover:-translate-y-0.5 flex flex-col justify-between md:min-h-[152px] bg-[#16151d] text-white shadow-[0_6px_20px_rgba(15,23,42,0.12)]"
          role="button"
          tabindex="0"
          onclick={() => void player.runSearch(genre.name)}
          onkeydown={(e) => {
            if (e.target !== e.currentTarget) return;
            if (e.key === "Enter") void player.runSearch(genre.name);
            else if (e.key === " ") e.preventDefault();
          }}
          onkeyup={(e) => {
            if (e.target === e.currentTarget && e.key === " ") {
              void player.runSearch(genre.name);
            }
          }}
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
                ? 'opacity-35 md:opacity-45'
                : 'opacity-0'}"
            />
          {/if}

          <!-- Color tint + high-contrast readability scrim -->
          <div
            class="absolute inset-0 pointer-events-none"
            style="background: linear-gradient(145deg, {c}cc 0%, {c}66 42%, rgba(16, 15, 22, 0.92) 100%);"
          ></div>
          <div
            class="absolute inset-0 pointer-events-none"
            style="background: linear-gradient(180deg, rgba(12, 12, 18, 0.16) 0%, rgba(12, 12, 18, 0.44) 55%, rgba(12, 12, 18, 0.78) 100%);"
          ></div>

          <!-- Card content -->
          <div class="relative z-10 flex items-center justify-end md:justify-between gap-2">
            <span
              class="hidden md:inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-black/45 text-white border border-white/15 backdrop-blur-xs"
            >
              <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: {c};"></span>
              {t("home.station_badge")}
            </span>
            <span
              class="w-7 h-7 rounded-full inline-flex items-center justify-center bg-black/35 border border-white/15 text-white/95"
            >
              {#if icon === "moon"}
                <Moon size={14} />
              {:else if icon === "car"}
                <Car size={14} />
              {:else if icon === "zap"}
                <Zap size={14} />
              {:else if icon === "heart"}
                <Heart size={14} fill="currentColor" />
              {:else if icon === "target"}
                <Target size={14} />
              {:else if icon === "headphones"}
                <Headphones size={14} />
              {:else}
                <Music size={14} />
              {/if}
            </span>
          </div>

          <div class="relative z-10 my-1 md:my-3">
            <h3 class="text-[16px] md:text-[18px] font-extrabold tracking-tight leading-snug text-white drop-shadow-[0_1px_6px_rgba(0,0,0,0.45)] break-words">
              {genre.name}
            </h3>
            <p class="hidden md:line-clamp-1 text-[12px] text-white/85 drop-shadow-[0_1px_4px_rgba(0,0,0,0.4)] mt-0.5">
              {t("home.station_sub")}
            </p>
          </div>

          <div class="relative z-10 flex items-center justify-between gap-2">
            <span
              class="inline-flex items-center gap-1 text-[12px] font-bold text-white/95 group-hover:translate-x-0.5 transition-transform drop-shadow-[0_1px_3px_rgba(0,0,0,0.45)]"
            >
              <span class="hidden md:inline">{t("home.explore_station")}</span>
              <ArrowRight size={13} />
            </span>
            <button
              type="button"
              class="w-11 h-11 md:w-9 md:h-9 rounded-full inline-flex items-center justify-center bg-black/45 hover:bg-black/65 border border-white/20 text-white shadow-md transition-all active:scale-95"
              title="{t('home.play')} {genre.name}"
              aria-label="{t('home.play')} {genre.name}"
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
