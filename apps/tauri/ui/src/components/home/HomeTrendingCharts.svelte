<script lang="ts">
  import { Heart, Plus, Volume2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  let chartTab = $state<"trending" | "top">("trending");

  let chartTracks = $derived<Track[]>(
    chartTab === "top"
      ? (player.homeFeed?.top ?? [])
      : (player.homeFeed?.trending ?? []),
  );
</script>

{#if chartTracks.length > 0}
  <section class="mt-6 flex flex-col gap-3" id="trending-hits">
    <div class="flex items-center justify-between gap-3">
      <h2 class="section-title">{t("home.trending_hits")}</h2>
      <div class="inline-flex gap-1 p-1 rounded-full bg-[var(--surface)] border border-[var(--border)]">
        <button
          class="px-3 py-1 rounded-full text-[12px] font-semibold transition-colors {chartTab === 'trending'
            ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          onclick={() => (chartTab = "trending")}
        >
          {t("home.tab_trending")}
        </button>
        <button
          class="px-3 py-1 rounded-full text-[12px] font-semibold transition-colors {chartTab === 'top'
            ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          onclick={() => (chartTab = "top")}
        >
          {t("home.tab_top")}
        </button>
      </div>
    </div>

    <div class="track-list">
      {#each chartTracks as track, i (track.id)}
        {@const active = player.currentTrack?.id === track.id}
        <div
          class="track-row cursor-pointer"
          class:active
          role="button"
          tabindex="0"
          onclick={() => player.replaceQueueAndPlay(chartTracks, i, "Trending hits")}
          onkeydown={(e) => e.key === "Enter" && player.replaceQueueAndPlay(chartTracks, i, "Trending hits")}
        >
          <span class="w-6 text-center text-[12px] font-bold text-[var(--text-muted)] numeric shrink-0">
            {#if active}
              <Volume2 size={15} class="text-[var(--accent-bright)] mx-auto" />
            {:else}
              {String(i + 1).padStart(2, "0")}
            {/if}
          </span>
          <CoverImage class="track-cover" src={track.cover_url} alt="" />
          <div class="track-meta">
            <span class="track-title">{track.title}</span>
            <span class="track-sub">{track.artist}</span>
          </div>
          <span class="text-[12px] text-[var(--text-muted)] numeric mr-1">
            {formatDuration(track.duration_secs)}
          </span>
          <button
            class="icon-btn"
            class:fav-active={player.isFavorite(track.id)}
            title="Favorite"
            onclick={(e) => {
              e.stopPropagation();
              player.toggleFavorite(track);
            }}
          >
            <Heart size={16} fill={player.isFavorite(track.id) ? "currentColor" : "none"} />
          </button>
          <button
            class="icon-btn"
            title="Add to queue"
            onclick={(e) => {
              e.stopPropagation();
              player.enqueueTrack(track);
            }}
          >
            <Plus size={16} />
          </button>
        </div>
      {/each}
    </div>
  </section>
{:else if !player.homeLoading}
  <div class="py-12 text-center">
    <p class="text-[var(--text-muted)]">Connect Tauri runtime (`cargo tauri dev`) to load live YouTube Music feeds.</p>
  </div>
{/if}
