<script lang="ts">
  import { Heart, Play } from "lucide-svelte";
  import { api, isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { ArtistDetails } from "../lib/types";
  import ArtistAllSongsPage from "./artist/ArtistAllSongsPage.svelte";
  import ArtistDiscography from "./artist/ArtistDiscography.svelte";
  import ArtistHero from "./artist/ArtistHero.svelte";
  import ArtistPopularTracks from "./artist/ArtistPopularTracks.svelte";

  let { artistId, artistName }: { artistId: string; artistName: string } = $props();

  let details = $state<ArtistDetails | null>(null);
  let loading = $state(false);
  let showingAllSongs = $state(false);

  let savedByArtist = $derived.by(() => {
    const targetName = (details?.name ?? artistName).trim().toLowerCase();
    return player.favorites.filter(
      (tr) =>
        tr.artist_id === artistId ||
        (targetName.length > 0 && tr.artist.toLowerCase().includes(targetName)),
    );
  });

  $effect(() => {
    const id = artistId;
    const name = artistName;
    details = null;
    showingAllSongs = false;
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

  function openAllSongs() {
    showingAllSongs = true;
    document.querySelector("main")?.scrollTo({ top: 0, behavior: "smooth" });
  }

  function backToArtist() {
    showingAllSongs = false;
    document.querySelector("main")?.scrollTo({ top: 0, behavior: "smooth" });
  }
</script>

{#if showingAllSongs && details}
  <ArtistAllSongsPage
    artistName={details.name}
    tracks={details.tracks}
    onBack={backToArtist}
  />
{:else}
  <div class="w-full pb-28">
    <ArtistHero {artistId} {artistName} {details} />

    <div class="w-full max-w-[1400px] mx-auto px-6 space-y-8">
      {#if savedByArtist.length > 0}
        <div class="flex items-center justify-between gap-4 px-4 py-3 rounded-2xl bg-[var(--surface)] border border-[var(--border)]">
          <div class="flex items-center gap-3 min-w-0">
            <span class="w-10 h-10 rounded-xl bg-[var(--fav-coral)]/18 text-[var(--fav-coral)] flex items-center justify-center shrink-0">
              <Heart size={18} fill="currentColor" />
            </span>
            <div class="min-w-0">
              <div class="text-[11px] font-bold uppercase tracking-[0.07em] text-[var(--accent-bright)]">
                {t("artist.in_library")}
              </div>
              <div class="text-[14px] font-semibold truncate">
                {savedByArtist.length === 1
                  ? t("artist.in_library_one")
                  : t("artist.in_library_many", { count: savedByArtist.length })}
              </div>
            </div>
          </div>
          <button
            type="button"
            class="pill-btn primary shrink-0"
            onclick={() =>
              player.replaceQueueAndPlay(
                savedByArtist,
                0,
                `${details?.name ?? artistName} · ${t("collection.favorites")}`,
              )}
          >
            <Play size={14} fill="currentColor" />
            {t("artist.play")}
          </button>
        </div>
      {/if}
      {#if loading}
        <div class="space-y-6 animate-pulse pt-2" aria-busy="true">
          <div class="h-5 w-40 rounded bg-white/[0.08]"></div>
          <div class="divide-y divide-white/[0.06]">
            {#each [0, 1, 2, 3, 4] as _}
              <div class="flex items-center gap-4 py-3">
                <div class="w-9 h-9 rounded-[6px] bg-white/[0.08]"></div>
                <div class="flex-1 space-y-2">
                  <div class="h-3.5 w-48 rounded bg-white/[0.09]"></div>
                  <div class="h-3 w-32 rounded bg-white/[0.05]"></div>
                </div>
                <div class="h-3 w-28 rounded bg-white/[0.05] hidden lg:block"></div>
                <div class="h-3 w-10 rounded bg-white/[0.06]"></div>
              </div>
            {/each}
          </div>
          <div class="pt-4 space-y-3">
            <div class="h-5 w-36 rounded bg-white/[0.08]"></div>
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3.5">
              {#each [0, 1, 2, 3, 4] as _}
                <div class="space-y-2.5">
                  <div class="aspect-square w-full rounded-xl bg-white/[0.08]"></div>
                  <div class="h-3.5 w-4/5 rounded bg-white/[0.08]"></div>
                  <div class="h-3 w-1/2 rounded bg-white/[0.05]"></div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      {:else if details}
        <ArtistPopularTracks
          artistName={details.name}
          tracks={details.tracks}
          onShowAll={openAllSongs}
        />
        <ArtistDiscography {details} />
      {/if}
    </div>
  </div>
{/if}
