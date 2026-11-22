<script lang="ts">
  import TrackRow from "../TrackRow.svelte";
  import { ArrowUpDown, Compass, Heart, Music } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track, UserPlaylist } from "../../lib/types";

  export type SortBy = "custom" | "title" | "artist" | "duration";

  interface Props {
    mode: "favorites" | "playlist";
    playlist?: UserPlaylist | null;
    accentColor: string;
    tracks: Track[];
    sortBy: SortBy;
    onCycleSort: () => void;
    onPlayTrack: (index: number) => void;
  }

  let {
    mode,
    playlist = null,
    accentColor,
    tracks,
    sortBy,
    onCycleSort,
    onPlayTrack,
  }: Props = $props();

  function sortLabel(s: SortBy): string {
    if (s === "title") return t("collection.sort_title");
    if (s === "artist") return t("collection.sort_artist");
    if (s === "duration") return t("collection.sort_duration");
    return t("collection.sort_custom");
  }
</script>

<section class="min-w-0 flex flex-col gap-3">
  {#if tracks.length === 0}
    <div
      class="min-h-[340px] flex flex-col items-center justify-center text-center px-6 py-12 rounded-2xl bg-[var(--surface)] border border-dashed border-[var(--border-strong)]"
    >
      <div
        class="w-14 h-14 rounded-2xl flex items-center justify-center text-white mb-4 shadow-lg border border-white/15"
        style="background: linear-gradient(135deg, {accentColor}cc, {accentColor}66);"
      >
        {#if mode === "favorites"}
          <Heart size={24} fill="currentColor" />
        {:else}
          <Music size={24} />
        {/if}
      </div>
      <h2 class="text-[17px] font-bold tracking-[-0.01em] mb-1.5">
        {mode === "favorites"
          ? t("collection.empty_favorites_title")
          : t("collection.empty_title")}
      </h2>
      <p class="text-[13.5px] text-[var(--text-muted)] max-w-[340px] leading-relaxed mb-5">
        {mode === "favorites"
          ? t("collection.empty_favorites")
          : t("collection.empty_playlist")}
      </p>
      <button
        class="pill-btn primary !px-4 !py-2"
        disabled={player.isOffline}
        onclick={() => player.navigateTab("explore")}
      >
        <Compass size={15} />
        {t("collection.explore_music")}
      </button>
    </div>
  {:else}
    {#if tracks.length > 1}
      <div class="flex items-center justify-end">
        <button
          class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[12px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-colors"
          onclick={onCycleSort}
        >
          <ArrowUpDown size={13} />
          {t("collection.sort", { mode: sortLabel(sortBy) })}
        </button>
      </div>
    {/if}

    <div>
      <div class="track-list">
        {#each tracks as track, i (track.id + ":" + i)}
          {@const unavailableOffline = player.isOffline && !player.isTrackDownloaded(track.id)}
          <div class={unavailableOffline ? "opacity-45" : ""}>
            <TrackRow
              {track}
              number={i + 1}
              onPlay={() => {
                if (unavailableOffline) {
                  player.showToast(t("offline.not_downloaded"));
                  return;
                }
                onPlayTrack(i);
              }}
              onRemove={mode === "playlist" && playlist ? () => player.removeTrackFromPlaylist(playlist.id, track.id) : undefined}
              removeLabel={t("collection.remove_from_playlist")}
            />
          </div>
        {/each}
      </div>
    </div>
  {/if}
</section>
