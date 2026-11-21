<script lang="ts">
  import { GripVertical, ListMusic } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import TrackRow from "../TrackRow.svelte";

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
    <div class="track-list" role="list">
      {#each player.queue as track, i (track.id + ":" + i)}
        {@const isCurrent = player.currentIndex === i}
        <div
          class={dragOverIdx === i ? "outline -outline-offset-1 outline-[var(--accent-bright)]" : ""}
          role="listitem"
          draggable="true"
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
        >
          <TrackRow
            {track}
            active={isCurrent}
            onPlay={() => void player.playIndex(i)}
            onRemove={() => player.removeQueueIndex(i)}
            removeLabel={t("queue.remove")}
          >
            {#snippet leading()}
              <GripVertical
                size={13}
                class="shrink-0 cursor-grab text-[var(--text-muted)] opacity-35 group-hover:opacity-90 transition-opacity"
                aria-hidden="true"
              />
            {/snippet}
          </TrackRow>
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
        <TrackRow {track} onPlay={() => player.replaceQueueAndPlay([track], 0, t("queue.suggestions"))} />
      {/each}
    </div>
  {/if}
</div>
