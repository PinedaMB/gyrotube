<script lang="ts">
  import { Play, Volume2 } from "lucide-svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  let { tracks }: { tracks: Track[] } = $props();
</script>

{#if tracks.length > 0}
  <section class="mt-6 flex flex-col gap-3">
    <h2 class="section-title">Listen again</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2.5">
      {#each tracks as track, i (track.id)}
        {@const active = player.currentTrack?.id === track.id}
        <button
          class="group flex items-center gap-3 p-2 pr-3 rounded-xl border text-left transition-all duration-150 {active
            ? 'bg-[var(--accent-soft)] border-[var(--accent-border)]'
            : 'bg-[var(--surface)] border-[var(--border)] hover:bg-[var(--surface-hover)]'}"
          onclick={() => player.replaceQueueAndPlay(tracks, i, "Listen again")}
        >
          <CoverImage class="tile-cover" src={track.cover_url} alt="" />
          <div class="track-meta">
            <span class="track-title {active ? '!text-[var(--accent-bright)] !font-bold' : ''}">
              {track.title}
            </span>
            <span class="track-sub">{track.artist}</span>
          </div>
          <span
            class="w-7 h-7 rounded-full inline-flex items-center justify-center shrink-0 transition-all duration-150 {active
              ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
              : 'text-[var(--text-muted)] group-hover:bg-[var(--accent)] group-hover:text-white group-hover:shadow-[0_4px_12px_rgba(33,144,164,0.42)]'}"
            aria-hidden="true"
          >
            {#if active}
              <Volume2 size={14} />
            {:else}
              <Play size={13} fill="currentColor" class="translate-x-[1px]" />
            {/if}
          </span>
        </button>
      {/each}
    </div>
  </section>
{/if}
