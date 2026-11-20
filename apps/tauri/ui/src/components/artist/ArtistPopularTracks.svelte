<script lang="ts">
  import TrackRow from "../TrackRow.svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";

  interface Props {
    artistName: string;
    tracks: Track[];
    onShowAll?: () => void;
  }

  let { artistName, tracks, onShowAll }: Props = $props();

  let visibleTracks = $derived(tracks.slice(0, 5));
</script>

{#if visibleTracks.length > 0}
  <section class="space-y-4">
    <h2 class="text-[22px] font-extrabold tracking-[-0.02em]">
      {t("artist.popular")}
    </h2>

    <div class="track-list">
      {#each visibleTracks as track, i (track.id)}
        <TrackRow {track} onPlay={() => player.replaceQueueAndPlay(tracks, i, artistName)} />
      {/each}
    </div>

    {#if tracks.length > 5 && onShowAll}
      <button
        type="button"
        class="inline-flex items-center justify-center px-4 py-1.5 rounded-full text-[13px] font-semibold border border-white/15 hover:bg-white/[0.08] transition-colors"
        onclick={onShowAll}
      >
        {t("artist.show_all", { count: tracks.length })}
      </button>
    {/if}
  </section>
{/if}
