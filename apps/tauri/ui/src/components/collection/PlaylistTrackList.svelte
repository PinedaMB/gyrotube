<script lang="ts">
  import {
    ArrowUpDown,
    Clock3,
    Compass,
    Heart,
    ListPlus,
    Music,
    Play,
    Trash2,
    Volume2,
  } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track, type UserPlaylist } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  export type SortBy = "custom" | "title" | "artist" | "duration";

  interface Props {
    mode: "favorites" | "playlist";
    playlist?: UserPlaylist | null;
    title: string;
    accentColor: string;
    tracks: Track[];
    sortBy: SortBy;
    onCycleSort: () => void;
    onPlayTrack: (index: number) => void;
  }

  let {
    mode,
    playlist = null,
    title,
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
      class="min-h-[340px] flex flex-col items-center justify-center text-center px-6 py-12 rounded-2xl bg-white/[0.025] border border-dashed border-white/[0.1]"
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
          class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[12px] font-semibold bg-white/[0.05] hover:bg-white/[0.1] border border-white/10 transition-colors"
          onclick={onCycleSort}
        >
          <ArrowUpDown size={13} />
          {t("collection.sort", { mode: sortLabel(sortBy) })}
        </button>
      </div>
    {/if}

    <div>
      <div
        class="flex items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]"
      >
        <span class="w-6 text-center">#</span>
        <span class="flex-1">{t("collection.col_title")}</span>
        <span class="w-11 flex justify-end"><Clock3 size={13} /></span>
      </div>

      <div class="flex flex-col">
        {#each tracks as track, i (track.id + ":" + i)}
          {@const active = player.currentTrack?.id === track.id}
          {@const favorited = player.isFavorite(track.id)}
          <div
            class="group flex items-center gap-3.5 py-2.5 px-3 border-b border-white/[0.06] last:border-b-0 cursor-pointer transition-colors {active
              ? 'bg-[var(--accent-soft)]'
              : 'hover:bg-white/[0.05]'}"
            role="button"
            tabindex="0"
            onclick={() => onPlayTrack(i)}
            onkeydown={(e) => e.key === "Enter" && onPlayTrack(i)}
          >
            <div class="w-6 flex items-center justify-center shrink-0">
              {#if active}
                <Volume2 size={15} class="text-[var(--accent-bright)]" />
              {:else}
                <span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric group-hover:hidden">
                  {i + 1}
                </span>
                <Play size={13} fill="currentColor" class="hidden group-hover:block translate-x-[1px]" />
              {/if}
            </div>

            <CoverImage
              class="track-cover !w-10 !h-10 !rounded-[8px] shrink-0"
              src={track.cover_url}
              alt={title}
            />

            <div class="min-w-0 flex-1">
              <div
                class="text-[14px] font-semibold truncate {active
                  ? 'text-[var(--accent-bright)] font-bold'
                  : ''}"
              >
                {track.title}
              </div>
              <div class="text-[12px] text-[var(--text-muted)] truncate">
                {track.artist}{track.album ? ` • ${track.album}` : ""}
              </div>
            </div>

            <!-- Hover-revealed action cluster + always-visible favorite state -->
            <div class="flex items-center gap-1 shrink-0">
              <div
                class="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition-opacity duration-150"
              >
                <button
                  class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-white/[0.1] transition-colors"
                  title={t("queue.add")}
                  onclick={(e) => {
                    e.stopPropagation();
                    player.enqueueTrack(track);
                  }}
                >
                  <ListPlus size={16} />
                </button>
                {#if mode === "playlist" && playlist}
                  <button
                    class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-red-400 hover:bg-red-500/15 transition-colors"
                    title={t("collection.remove_from_playlist")}
                    onclick={(e) => {
                      e.stopPropagation();
                      player.removeTrackFromPlaylist(playlist.id, track.id);
                    }}
                  >
                    <Trash2 size={15} />
                  </button>
                {/if}
              </div>

              <button
                class="w-8 h-8 rounded-full inline-flex items-center justify-center transition-all {favorited
                  ? 'text-[var(--fav-coral)] opacity-100 hover:bg-white/[0.08]'
                  : 'text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1]'}"
                title={favorited ? t("player.favorite_remove") : t("player.favorite_add")}
                onclick={(e) => {
                  e.stopPropagation();
                  player.toggleFavorite(track);
                }}
              >
                <Heart size={16} fill={favorited ? "currentColor" : "none"} />
              </button>

              <span class="text-[12.5px] text-[var(--text-muted)] numeric w-11 text-right">
                {formatDuration(track.duration_secs)}
              </span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</section>
