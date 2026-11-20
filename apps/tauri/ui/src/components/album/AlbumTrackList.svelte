<script lang="ts">
  import { Clock3, Heart, Play, Plus, Volume2 } from "lucide-svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  interface Props {
    title: string;
    isPlaylist?: boolean;
    tracks: Track[];
    loading: boolean;
  }

  let { title, isPlaylist = false, tracks, loading }: Props = $props();
</script>

<section class="min-w-0">
  {#if loading}
    <div class="divide-y divide-white/[0.06] animate-pulse" aria-busy="true">
      {#each [0, 1, 2, 3, 4, 5, 6, 7] as _}
        <div class="flex items-center gap-4 py-3.5 px-3">
          <div class="w-6 h-4 rounded bg-white/[0.07]"></div>
          <div class="flex-1 space-y-2">
            <div class="h-3.5 w-48 rounded bg-white/[0.09]"></div>
            <div class="h-3 w-28 rounded bg-white/[0.05]"></div>
          </div>
          <div class="h-3 w-10 rounded bg-white/[0.06]"></div>
        </div>
      {/each}
    </div>
  {:else if tracks.length > 0}
    <div class="flex items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]">
      <span class="w-7 text-center">#</span>
      <span class="flex-1">Title</span>
      <span class="w-11 flex justify-end"><Clock3 size={13} /></span>
    </div>

    <div class="flex flex-col">
      {#each tracks as track, i (track.id + ":" + i)}
        {@const active = player.currentTrack?.id === track.id}
        <div
          class="group flex items-center gap-3.5 py-2.5 px-3 border-b border-white/[0.06] last:border-b-0 cursor-pointer transition-colors {active
            ? 'bg-[var(--accent-soft)]'
            : 'hover:bg-white/[0.05]'}"
          role="button"
          tabindex="0"
          onclick={() => player.replaceQueueAndPlay(tracks, i, title)}
          onkeydown={(e) => e.key === "Enter" && player.replaceQueueAndPlay(tracks, i, title)}
        >
          <!-- Track number / Hover play / Active speaker -->
          <div class="w-7 flex items-center justify-center shrink-0">
            {#if active}
              <Volume2 size={16} class="text-[var(--accent-bright)]" />
            {:else}
              <span class="text-[13px] font-semibold text-[var(--text-muted)] numeric group-hover:hidden">
                {i + 1}
              </span>
              <Play
                size={14}
                fill="currentColor"
                class="hidden group-hover:block text-[var(--text)] translate-x-[1px]"
              />
            {/if}
          </div>

          {#if isPlaylist}
            <CoverImage class="track-cover !w-9 !h-9 !rounded-lg shrink-0" src={track.cover_url} alt="" />
          {/if}

          <!-- Title & artist -->
          <div class="min-w-0 flex-1">
            <div
              class="text-[14.5px] font-semibold truncate {active
                ? 'text-[var(--accent-bright)] font-bold'
                : ''}"
            >
              {track.title}
            </div>
            <div class="text-[12.5px] text-[var(--text-muted)] truncate">
              {track.artist}
            </div>
          </div>

          <!-- Actions + duration -->
          <div class="flex items-center gap-1.5 shrink-0">
            <button
              class="icon-btn !w-8 !h-8 {player.isFavorite(track.id)
                ? 'fav-active'
                : 'opacity-65 group-hover:opacity-100'}"
              title="Favorite"
              onclick={(e) => {
                e.stopPropagation();
                player.toggleFavorite(track);
              }}
            >
              <Heart size={16} fill={player.isFavorite(track.id) ? "currentColor" : "none"} />
            </button>
            <button
              class="icon-btn !w-8 !h-8 opacity-65 group-hover:opacity-100"
              title="Add to queue"
              onclick={(e) => {
                e.stopPropagation();
                player.enqueueTrack(track);
              }}
            >
              <Plus size={16} />
            </button>
            <span class="text-[13px] text-[var(--text-muted)] numeric w-11 text-right">
              {formatDuration(track.duration_secs)}
            </span>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <p class="text-[var(--text-muted)] py-10 text-center">No tracks found.</p>
  {/if}
</section>
