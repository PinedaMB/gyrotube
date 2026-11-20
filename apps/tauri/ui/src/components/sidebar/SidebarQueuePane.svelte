<script lang="ts">
  import { GripVertical, ListMusic, Plus, X } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  let dragFrom = $state<number | null>(null);
  let dragOverIdx = $state<number | null>(null);
</script>

<div in:fade={{ duration: 180 }} class="flex flex-col gap-3 pt-1">
  <div class="flex items-center justify-between gap-2">
    <h3 class="text-[14px] font-bold">
      {t("queue.title")} <span class="text-[var(--text-muted)] font-medium">· {player.queue.length}</span>
    </h3>
    {#if player.queue.length > 1}
      <button
        class="px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-red-500/20 hover:text-red-400 transition-colors"
        onclick={() => player.clearQueue()}
      >
        {t("queue.clear")}
      </button>
    {/if}
  </div>

  {#if player.queue.length === 0}
    <div class="flex flex-col items-center justify-center text-center gap-2 py-10 text-[var(--text-muted)]">
      <ListMusic size={26} class="opacity-40" />
      <p class="text-[13px]">{t("queue.empty")}</p>
    </div>
  {:else}
    <div class="track-list">
      {#each player.queue as track, i (track.id + ":" + i)}
        {@const isCurrent = player.currentIndex === i}
        <div
          class="group flex items-center gap-2 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 cursor-pointer transition-colors {isCurrent
            ? 'bg-[var(--accent-soft)]'
            : 'hover:bg-[var(--surface-hover)]'} {dragOverIdx === i ? 'border-t-2 border-t-[var(--accent-bright)]' : ''}"
          draggable="true"
          role="button"
          tabindex="0"
          ondragstart={() => (dragFrom = i)}
          ondragover={(e) => {
            e.preventDefault();
            dragOverIdx = i;
          }}
          ondragleave={() => {
            if (dragOverIdx === i) dragOverIdx = null;
          }}
          ondrop={() => {
            if (dragFrom != null) player.moveQueueItem(dragFrom, i);
            dragFrom = null;
            dragOverIdx = null;
          }}
          ondragend={() => {
            dragFrom = null;
            dragOverIdx = null;
          }}
          onclick={() => void player.playIndex(i)}
          onkeydown={(e) => e.key === "Enter" && void player.playIndex(i)}
        >
          <GripVertical size={13} class="text-[var(--text-muted)]/45 cursor-grab shrink-0" />
          <CoverImage class="track-cover !w-9 !h-9 !rounded-md" src={track.cover_url} alt="" />
          <div class="track-meta">
            <span class="track-title !text-[12.5px] {isCurrent ? '!text-[var(--accent-bright)] !font-bold' : ''}">
              {track.title}
            </span>
            <span class="track-sub !text-[11px]">{track.artist}</span>
          </div>
          {#if isCurrent && player.isPlaying}
            <span class="eq-bars shrink-0" aria-label="Playing">
              <span></span><span></span><span></span>
            </span>
          {:else}
            <span class="text-[11px] text-[var(--text-muted)] numeric shrink-0">
              {formatDuration(track.duration_secs)}
            </span>
          {/if}
          <button
            class="icon-btn !w-6 !h-6 opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
            title="Remove from queue"
            onclick={(e) => {
              e.stopPropagation();
              player.removeQueueIndex(i);
            }}
          >
            <X size={13} />
          </button>
        </div>
      {/each}
    </div>
  {/if}

  <div class="flex items-center justify-between gap-3 p-3 rounded-xl bg-[var(--surface)] border border-[var(--border)]">
    <div class="min-w-0">
      <div class="text-[13px] font-bold">{t("queue.autoplay")}</div>
      <p class="text-[11.5px] text-[var(--text-muted)] leading-snug">
        {t("queue.autoplay_desc")}
      </p>
    </div>
    <button
      type="button"
      role="switch"
      aria-checked={player.autoplayRelated}
      aria-label="Toggle autoplay"
      class="w-10 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.autoplayRelated
        ? 'bg-[var(--accent)]'
        : 'bg-white/15'}"
      onclick={() => player.setAutoplay(!player.autoplayRelated)}
    >
      <span
        class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.autoplayRelated
          ? 'translate-x-4'
          : 'translate-x-0'}"
      ></span>
    </button>
  </div>

  {#if player.suggestedTracks.length > 0}
    <h4 class="text-[11.5px] font-bold uppercase tracking-wider text-[var(--text-muted)] mt-1">
      {t("queue.suggestions")}
    </h4>
    <div class="track-list">
      {#each player.suggestedTracks as track (track.id)}
        <div class="flex items-center gap-2.5 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 hover:bg-[var(--surface-hover)] transition-colors">
          <CoverImage class="track-cover !w-9 !h-9 !rounded-md" src={track.cover_url} alt="" />
          <div class="track-meta">
            <span class="track-title !text-[12.5px]">{track.title}</span>
            <span class="track-sub !text-[11px]">{track.artist}</span>
          </div>
          <button
            class="icon-btn !w-7 !h-7 shrink-0"
            title="Add to queue"
            onclick={() => player.enqueueTrack(track)}
          >
            <Plus size={15} />
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .eq-bars {
    display: inline-flex;
    align-items: flex-end;
    gap: 2.5px;
    height: 14px;
    padding-bottom: 1px;
  }

  .eq-bars span {
    width: 3px;
    height: 100%;
    border-radius: 2px;
    background: var(--accent-bright);
    transform-origin: bottom center;
    will-change: transform;
  }

  .eq-bars span:nth-child(1) {
    animation: eq-bar-1 0.85s ease-in-out infinite;
  }

  .eq-bars span:nth-child(2) {
    animation: eq-bar-2 0.72s ease-in-out infinite;
  }

  .eq-bars span:nth-child(3) {
    animation: eq-bar-3 0.95s ease-in-out infinite;
  }

  @keyframes eq-bar-1 {
    0%, 100% { transform: scaleY(0.25); }
    45% { transform: scaleY(0.95); }
    75% { transform: scaleY(0.45); }
  }

  @keyframes eq-bar-2 {
    0%, 100% { transform: scaleY(0.9); }
    35% { transform: scaleY(0.3); }
    68% { transform: scaleY(1); }
  }

  @keyframes eq-bar-3 {
    0%, 100% { transform: scaleY(0.4); }
    50% { transform: scaleY(0.85); }
    80% { transform: scaleY(0.22); }
  }
</style>
