<script lang="ts">
  import { ChevronDown, ChevronUp, Heart, Play, Plus, Volume2 } from "lucide-svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  interface Props {
    artistName: string;
    tracks: Track[];
  }

  let { artistName, tracks }: Props = $props();
  let expanded = $state(false);

  $effect(() => {
    void artistName;
    expanded = false;
  });

  let visibleTracks = $derived(expanded ? tracks : tracks.slice(0, 5));
</script>

{#if visibleTracks.length > 0}
  <section class="space-y-3">
    <h2 class="section-title">Popular songs</h2>

    <div class="flex flex-col border-t border-white/[0.06]">
      {#each visibleTracks as track, i (track.id)}
        {@const active = player.currentTrack?.id === track.id}
        <div
          class="group grid grid-cols-[minmax(0,1fr)_auto] md:grid-cols-[minmax(0,1.5fr)_minmax(0,1fr)_auto] lg:grid-cols-[minmax(0,1.6fr)_minmax(0,0.9fr)_minmax(0,1fr)_auto] items-center gap-4 py-2.5 px-2.5 border-b border-white/[0.06] cursor-pointer transition-colors {active
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
              <CoverImage class="track-cover !w-10 !h-10 !rounded-lg" src={track.cover_url} alt="" />
              {#if active}
                <div class="absolute inset-0 rounded-lg bg-black/55 flex items-center justify-center text-[var(--accent-bright)]">
                  <Volume2 size={16} />
                </div>
              {:else}
                <div class="absolute inset-0 rounded-lg bg-black/55 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white">
                  <Play size={15} fill="currentColor" class="translate-x-[1px]" />
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
          <span class="hidden md:block text-[13px] text-[var(--text-muted)] truncate">
            {track.artist}
          </span>

          <!-- Album column (lg+) -->
          <span class="hidden lg:block text-[13px] text-[var(--text-muted)] truncate">
            {track.album ?? ""}
          </span>

          <!-- Duration + Actions -->
          <div class="flex items-center gap-1.5 shrink-0">
            <span class="text-[12.5px] text-[var(--text-muted)] numeric mr-2">
              {formatDuration(track.duration_secs)}
            </span>
            <button
              class="icon-btn !w-8 !h-8"
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
              class="icon-btn !w-8 !h-8"
              title="Add to queue"
              onclick={(e) => {
                e.stopPropagation();
                player.enqueueTrack(track);
              }}
            >
              <Plus size={16} />
            </button>
          </div>
        </div>
      {/each}
    </div>

    {#if tracks.length > 5}
      <button class="pill-btn mt-1" onclick={() => (expanded = !expanded)}>
        {#if expanded}
          <ChevronUp size={15} />
          Show less
        {:else}
          <ChevronDown size={15} />
          Show all ({tracks.length})
        {/if}
      </button>
    {/if}
  </section>
{/if}
