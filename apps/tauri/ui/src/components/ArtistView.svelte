<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { player } from "../lib/stores/player.svelte";
  import type { ArtistDetails } from "../lib/types";
  import ArtistDiscography from "./artist/ArtistDiscography.svelte";
  import ArtistHero from "./artist/ArtistHero.svelte";
  import ArtistPopularTracks from "./artist/ArtistPopularTracks.svelte";

  let { artistId, artistName }: { artistId: string; artistName: string } = $props();

  let details = $state<ArtistDetails | null>(null);
  let loading = $state(false);

  $effect(() => {
    const id = artistId;
    const name = artistName;
    details = null;
    if (!isTauriRuntime()) return;
    loading = true;
    void api
      .getArtist(id, name)
      .then((res) => {
        if (artistId === id) details = res;
      })
      .catch((err) => player.showToast(String(err)))
      .finally(() => {
        if (artistId === id) loading = false;
      });
  });
</script>

<div class="w-full pb-28">
  <ArtistHero {artistName} {details} />

  <div class="w-full max-w-[1280px] mx-auto px-6 space-y-8">
    {#if loading}
      <div class="space-y-6 animate-pulse pt-2" aria-busy="true">
        <div class="h-5 w-40 rounded bg-white/[0.08]"></div>
        <div class="divide-y divide-white/[0.06]">
          {#each [0, 1, 2, 3, 4] as _}
            <div class="flex items-center gap-4 py-3">
              <div class="w-10 h-10 rounded-lg bg-white/[0.08]"></div>
              <div class="flex-1 space-y-2">
                <div class="h-3.5 w-48 rounded bg-white/[0.09]"></div>
                <div class="h-3 w-32 rounded bg-white/[0.05]"></div>
              </div>
              <div class="h-3 w-28 rounded bg-white/[0.05] hidden lg:block"></div>
              <div class="h-3 w-10 rounded bg-white/[0.06]"></div>
            </div>
          {/each}
        </div>
      </div>
    {:else if details}
      <ArtistPopularTracks artistName={details.name} tracks={details.tracks} />
      <ArtistDiscography {details} />
    {/if}
  </div>
</div>
