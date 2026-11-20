<script lang="ts">
  import TrackRow from "../TrackRow.svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";

  let { tracks }: { tracks: Track[] } = $props();
</script>

{#if tracks.length > 0}
  <section class="mt-5 flex flex-col gap-2 md:mt-6 md:gap-3">
    <h2 class="section-title">{t("home.listen_again")}</h2>
    <div class="track-list">
      {#each tracks as track, i (track.id)}
        <TrackRow {track} onPlay={() => player.replaceQueueAndPlay(tracks, i, t("home.listen_again"))} />
      {/each}
    </div>
  </section>
{/if}
