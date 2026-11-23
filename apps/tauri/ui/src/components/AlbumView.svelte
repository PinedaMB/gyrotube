<script module lang="ts">
  import type { Track } from "../lib/types";

  interface CachedAlbum {
    title: string;
    tracks: Track[];
    rgb: [number, number, number] | null;
  }

  const ALBUM_CACHE_MAX = 20;
  const albumCache = new Map<string, CachedAlbum>();

  function cacheAlbum(key: string, data: CachedAlbum): void {
    if (albumCache.has(key)) albumCache.delete(key);
    albumCache.set(key, data);
    if (albumCache.size > ALBUM_CACHE_MAX) {
      const oldestKey = albumCache.keys().next().value;
      if (oldestKey !== undefined) albumCache.delete(oldestKey);
    }
  }
</script>

<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { extractDominantRgb } from "../lib/audioEngine";
  import { player } from "../lib/stores/player.svelte";
  import AlbumSidebarHero from "./album/AlbumSidebarHero.svelte";
  import AlbumTrackList from "./album/AlbumTrackList.svelte";
  import OfflineState from "./OfflineState.svelte";

  let {
    id,
    title,
    coverUrl = null,
    isPlaylist = false,
  }: { id: string; title: string; coverUrl?: string | null; isPlaylist?: boolean } = $props();

  let tracks = $state<Track[]>([]);
  let resolvedTitle = $state("");
  let loading = $state(false);
  let albumRgb = $state<[number, number, number] | null>(null);
  let rootEl = $state<HTMLDivElement | null>(null);
  let loadedKey: string | null = null;

  function scrollPaneTop(behavior: ScrollBehavior = "auto") {
    rootEl?.closest(".scroll-y")?.scrollTo({ top: 0, behavior });
  }

  $effect(() => {
    const targetId = id;
    const isPl = isPlaylist;
    const key = `${isPl ? "pl" : "al"}:${targetId}`;
    if (loadedKey === key && (tracks.length > 0 || loading)) return;
    loadedKey = key;
    scrollPaneTop("auto");

    const cached = albumCache.get(key);
    if (cached) {
      resolvedTitle = cached.title || title;
      tracks = cached.tracks;
      albumRgb = cached.rgb;
      loading = false;
      return;
    }

    resolvedTitle = title;
    tracks = [];
    albumRgb = null;

    if (coverUrl) {
      void extractDominantRgb(coverUrl).then((rgb) => {
        if (loadedKey === key) {
          albumRgb = rgb;
          const entry = albumCache.get(key);
          if (entry) entry.rgb = rgb;
        }
      });
    }

    if (!isTauriRuntime()) return;
    loading = true;
    const req = isPl ? api.getPlaylistTracks(targetId) : api.getAlbumTracks(targetId);
    void req
      .then(([name, list]) => {
        const finalTitle = name || title;
        cacheAlbum(key, { title: finalTitle, tracks: list, rgb: loadedKey === key ? albumRgb : null });
        if (loadedKey === key) {
          if (name) resolvedTitle = name;
          tracks = list;
          if (!coverUrl && list[0]?.cover_url) {
            void extractDominantRgb(list[0].cover_url).then((rgb) => {
              if (loadedKey === key) {
                albumRgb = rgb;
                const entry = albumCache.get(key);
                if (entry) entry.rgb = rgb;
              }
            });
          }
        }
      })
      .catch((err) => {
        if (loadedKey === key) loadedKey = null;
        player.showToast(String(err));
      })
      .finally(() => {
        if (loadedKey === key) loading = false;
      });
  });

  let glowStyle = $derived(
    albumRgb
      ? `background-image: radial-gradient(ellipse 75% 48% at 24% 26%, rgba(${albumRgb[0]}, ${albumRgb[1]}, ${albumRgb[2]}, 0.22) 0%, rgba(${albumRgb[0]}, ${albumRgb[1]}, ${albumRgb[2]}, 0.07) 48%, transparent 78%);`
      : "",
  );
</script>

<div bind:this={rootEl} class="w-full min-h-full transition-all duration-500" style={player.isOffline ? "" : glowStyle}>
  {#if player.isOffline}
    <div class="page-clamp">
      <OfflineState />
    </div>
  {:else}
    <div class="w-full max-w-[1280px] mx-auto px-4 sm:px-8 pt-[max(1rem,env(safe-area-inset-top))] sm:pt-6 pb-36 md:pb-28">
      <div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-8 lg:gap-12 items-start">
        <AlbumSidebarHero
          {id}
          title={resolvedTitle}
          {coverUrl}
          {isPlaylist}
          {tracks}
          {loading}
        />

        <AlbumTrackList
          title={resolvedTitle}
          {tracks}
          {loading}
        />
      </div>
    </div>
  {/if}
</div>
