<script lang="ts">
  import { Play, Volume2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  let { tracks }: { tracks: Track[] } = $props();
</script>

{#if tracks.length > 0}
  <section class="mt-5 flex flex-col gap-2 md:mt-6 md:gap-3">
    <h2 class="section-title">{t("home.listen_again")}</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 md:gap-2.5 max-md:divide-y max-md:divide-[var(--border)]">
      {#each tracks as track, i (track.id)}
        {@const active = player.currentTrack?.id === track.id}
        <button
          class="group flex min-w-0 items-center gap-2.5 px-2 py-1.5 text-left transition-colors duration-150 md:gap-3 md:p-2 md:pr-3 md:rounded-xl md:border {active
            ? 'bg-[var(--accent-soft)] border-[var(--accent-border)]'
            : 'md:bg-[var(--surface)] border-[var(--border)] hover:bg-[var(--surface-hover)]'}"
          onclick={() => player.replaceQueueAndPlay(tracks, i, "Listen again")}
        >
          <CoverImage class="tile-cover max-md:!w-10 max-md:!h-10" src={track.cover_url} alt="" />
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
