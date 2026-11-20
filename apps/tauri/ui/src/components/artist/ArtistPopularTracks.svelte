<script lang="ts">
  import { Heart, ListPlus, Play, Volume2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

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

    <div class="flex flex-col">
      {#each visibleTracks as track, i (track.id)}
        {@const active = player.currentTrack?.id === track.id}
        {@const favorited = player.isFavorite(track.id)}
        <div
          class="group grid grid-cols-[minmax(0,1fr)_auto] md:grid-cols-[minmax(0,1.6fr)_minmax(0,0.85fr)_auto] lg:grid-cols-[minmax(0,1.65fr)_minmax(0,0.85fr)_minmax(0,1fr)_auto] items-center gap-6 py-2.5 px-2.5 border-b border-white/[0.08] last:border-b-0 cursor-pointer transition-colors {active
            ? 'bg-[var(--accent-soft)]'
            : 'hover:bg-white/[0.05]'}"
          role="button"
          tabindex="0"
          onclick={() => player.replaceQueueAndPlay(tracks, i, artistName)}
          onkeydown={(e) => e.key === "Enter" && player.replaceQueueAndPlay(tracks, i, artistName)}
        >
          <!-- Cover + Title -->
          <div class="flex items-center gap-3.5 min-w-0">
            <div class="relative shrink-0">
              <CoverImage class="track-cover !w-9 !h-9 !rounded-[6px]" src={track.cover_url} alt="" />
              {#if active}
                <div class="absolute inset-0 rounded-[6px] bg-black/55 flex items-center justify-center text-[var(--accent-bright)]">
                  <Volume2 size={15} />
                </div>
              {:else}
                <div class="absolute inset-0 rounded-[6px] bg-black/55 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white">
                  <Play size={14} fill="currentColor" class="translate-x-[1px]" />
                </div>
              {/if}
            </div>
            <div class="min-w-0">
              <div
                class="text-[14px] font-semibold truncate {active
                  ? 'text-[var(--accent-bright)] font-bold'
                  : ''}"
              >
                {track.title}
              </div>
              <div class="text-[12px] text-[var(--text-muted)] truncate md:hidden">
                {track.artist}{track.album ? ` • ${track.album}` : ""}
              </div>
            </div>
          </div>

          <!-- Artist column (md+) -->
          <span class="hidden md:block text-[13.5px] text-[var(--text-muted)] truncate">
            {track.artist}
          </span>

          <!-- Album column (lg+) -->
          <span class="hidden lg:block text-[13.5px] text-[var(--text-muted)] truncate">
            {track.album ?? ""}
          </span>

          <!-- Hover-revealed actions + clean right-aligned duration -->
          <div class="flex items-center gap-1 shrink-0">
            <button
              type="button"
              class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all"
              title={t("queue.add")}
              onclick={(e) => {
                e.stopPropagation();
                player.enqueueTrack(track);
              }}
            >
              <ListPlus size={16} />
            </button>
            <button
              type="button"
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
            <span class="text-[13px] text-[var(--text-muted)] numeric w-11 text-right">
              {formatDuration(track.duration_secs)}
            </span>
          </div>
        </div>
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
