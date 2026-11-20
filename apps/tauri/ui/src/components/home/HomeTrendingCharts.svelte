<script lang="ts">
  import TrackRow from "../TrackRow.svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";

  let chartTab = $state<"trending" | "top">("trending");

  let chartTracks = $derived<Track[]>(
    chartTab === "top"
      ? (player.homeFeed?.top ?? [])
      : (player.homeFeed?.trending ?? []),
  );
</script>

{#if chartTracks.length > 0}
  <section class="mt-6 flex flex-col gap-3" id="trending-hits">
    <div class="flex items-center justify-between gap-3 max-md:flex-wrap max-md:gap-y-2">
      <h2 class="section-title">{t("home.trending_hits")}</h2>
      <div class="inline-flex gap-1 p-1 rounded-full bg-[var(--surface)] border border-[var(--border)]">
        <button
          class="px-3 py-1 max-md:min-h-9 rounded-full text-[12px] font-semibold transition-colors {chartTab === 'trending'
            ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          onclick={() => (chartTab = "trending")}
          aria-pressed={chartTab === "trending"}
        >
          {t("home.tab_trending")}
        </button>
        <button
          class="px-3 py-1 max-md:min-h-9 rounded-full text-[12px] font-semibold transition-colors {chartTab === 'top'
            ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          onclick={() => (chartTab = "top")}
          aria-pressed={chartTab === "top"}
        >
          {t("home.tab_top")}
        </button>
      </div>
    </div>

    <div class="track-list">
      {#each chartTracks as track, i (track.id)}
        <TrackRow {track} number={i + 1} onPlay={() => player.replaceQueueAndPlay(chartTracks, i, t("home.trending_hits"))} />
      {/each}
    </div>
  </section>
{:else if !player.homeLoading}
  <div class="py-12 text-center">
    <p class="text-[var(--text-muted)]">Connect Tauri runtime (`cargo tauri dev`) to load live YouTube Music feeds.</p>
  </div>
{/if}
