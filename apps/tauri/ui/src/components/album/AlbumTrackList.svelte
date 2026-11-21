<script lang="ts">
  import TrackRow from "../TrackRow.svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";

  interface Props {
    title: string;
    tracks: Track[];
    loading: boolean;
  }

  let { title, tracks, loading }: Props = $props();
</script>

<section class="min-w-0">
  {#if loading}
    <div class="divide-y divide-[var(--border)] animate-pulse" aria-busy="true">
      {#each [0, 1, 2, 3, 4, 5, 6, 7] as _}
        <div class="flex items-center gap-4 py-3.5 px-3">
          <div class="w-6 h-4 rounded bg-[var(--skeleton-soft)]"></div>
          <div class="flex-1 space-y-2">
            <div class="h-3.5 w-48 rounded bg-[var(--skeleton)]"></div>
            <div class="h-3 w-28 rounded bg-[var(--skeleton-soft)]"></div>
          </div>
          <div class="h-3 w-10 rounded bg-[var(--skeleton-soft)]"></div>
        </div>
      {/each}
    </div>
  {:else if tracks.length > 0}
    <div class="track-list">
      {#each tracks as track, i (track.id + ":" + i)}
        <TrackRow {track} number={i + 1} onPlay={() => player.replaceQueueAndPlay(tracks, i, title)} />
      {/each}
    </div>
  {:else}
    <p class="text-[var(--text-muted)] py-10 text-center">{t("album.no_tracks")}</p>
  {/if}
</section>
