<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { extractDominantRgb } from "../lib/audioEngine";
  import { player } from "../lib/stores/player.svelte";
  import type { Track } from "../lib/types";
  import AlbumSidebarHero from "./album/AlbumSidebarHero.svelte";
  import AlbumTrackList from "./album/AlbumTrackList.svelte";

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

  $effect(() => {
    const targetId = id;
    const isPl = isPlaylist;
    resolvedTitle = title;
    tracks = [];
    albumRgb = null;

    if (coverUrl) {
      void extractDominantRgb(coverUrl).then((rgb) => {
        if (id === targetId) albumRgb = rgb;
      });
    }

    if (!isTauriRuntime()) return;
    loading = true;
    const req = isPl ? api.getPlaylistTracks(targetId) : api.getAlbumTracks(targetId);
    void req
      .then(([name, list]) => {
        if (id === targetId) {
          if (name) resolvedTitle = name;
          tracks = list;
          if (!coverUrl && list[0]?.cover_url) {
            void extractDominantRgb(list[0].cover_url).then((rgb) => {
              if (id === targetId) albumRgb = rgb;
            });
          }
        }
      })
      .catch((err) => player.showToast(String(err)))
      .finally(() => {
        if (id === targetId) loading = false;
      });
  });

  let glowStyle = $derived(
    albumRgb
      ? `background-image: radial-gradient(ellipse 75% 48% at 24% 26%, rgba(${albumRgb[0]}, ${albumRgb[1]}, ${albumRgb[2]}, 0.22) 0%, rgba(${albumRgb[0]}, ${albumRgb[1]}, ${albumRgb[2]}, 0.07) 48%, transparent 78%);`
      : "",
  );
</script>

<div class="w-full min-h-full transition-all duration-500" style={glowStyle}>
  <div class="w-full max-w-[1280px] mx-auto px-8 pt-6 pb-28">
    <div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-12 items-start">
      <AlbumSidebarHero
        title={resolvedTitle}
        {coverUrl}
        {isPlaylist}
        {tracks}
      />

      <AlbumTrackList
        title={resolvedTitle}
        {isPlaylist}
        {tracks}
        {loading}
      />
    </div>
  </div>
</div>
