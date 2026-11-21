<script lang="ts">
  import SegmentedControl, { type SegmentedOption } from "../SegmentedControl.svelte";
  import TrackRow from "../TrackRow.svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";

  type ChartTab = "trending" | "top";
  let chartTab = $state<ChartTab>("trending");

  let chartOptions = $derived<SegmentedOption<ChartTab>[]>([
    { value: "trending", label: t("home.tab_trending") },
    { value: "top", label: t("home.tab_top") },
  ]);

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
      <SegmentedControl
        options={chartOptions}
        value={chartTab}
        size="sm"
        ariaLabel={t("home.trending_hits")}
        onchange={(v) => (chartTab = v)}
      />
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
