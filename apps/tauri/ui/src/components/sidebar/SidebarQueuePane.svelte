<script lang="ts">
  import { BookmarkPlus, GripVertical, ListMusic } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { device } from "../../lib/stores/device.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import TrackRow from "../TrackRow.svelte";

  function saveQueueAsPlaylist() {
    if (player.queue.length === 0) return;
    const baseName = player.sourceName?.trim() || `${t("queue.title")} · ${new Date().toLocaleDateString()}`;
    player.createUserPlaylist(baseName, "", null, "list-music", [...player.queue]);
    player.showToast(t("queue.saved_as_playlist"));
  }

  let dragFrom = $state<number | null>(null);
  let dragOverIdx = $state<number | null>(null);

  // Touch / pointer-based reordering via the drag handle (HTML5 DnD does not work on Android touch)
  let pointerDragFrom = $state<number | null>(null);
  let pointerDragOver = $state<number | null>(null);
  let pointerStartY = 0;
  let pointerOffsetY = $state(0);
  let rowEls = $state<Record<number, HTMLDivElement>>({});

  function findRowIndexAtY(clientY: number): number | null {
    const count = player.queue.length;
    if (count === 0) return null;
    for (let idx = 0; idx < count; idx++) {
      const el = rowEls[idx];
      if (!el) continue;
      const rect = el.getBoundingClientRect();
      if (clientY < rect.top + rect.height / 2) {
        return idx;
      }
    }
    return count - 1;
  }

  function startPointerDrag(e: PointerEvent, index: number) {
    e.preventDefault();
    e.stopPropagation();
    const target = e.currentTarget as HTMLElement;
    try {
      target.setPointerCapture(e.pointerId);
    } catch {
      // ignore if capture unavailable
    }
    pointerDragFrom = index;
    pointerDragOver = index;
    pointerStartY = e.clientY;
    pointerOffsetY = 0;
  }

  function movePointerDrag(e: PointerEvent) {
    if (pointerDragFrom == null) return;
    e.preventDefault();
    pointerOffsetY = e.clientY - pointerStartY;
    const targetIdx = findRowIndexAtY(e.clientY);
    if (targetIdx != null) {
      pointerDragOver = targetIdx;
    }
  }

  function endPointerDrag(e: PointerEvent, commit: boolean) {
    if (pointerDragFrom == null) return;
    e.preventDefault();
    const from = pointerDragFrom;
    const to = pointerDragOver;
    pointerDragFrom = null;
    pointerDragOver = null;
    pointerOffsetY = 0;
    if (commit && to != null && from !== to) {
      player.moveQueueItem(from, to);
    }
  }
</script>

<div in:fade={{ duration: 180 }} class="flex flex-col gap-3 pt-1">
  <div class="flex items-center justify-between gap-2">
    <h3 class="text-[14px] font-bold">
      {t("queue.title")} <span class="text-[var(--text-muted)] font-medium">· {player.queue.length}</span>
    </h3>
    <div class="flex items-center gap-1.5">
      {#if player.queue.length > 0}
        <button
          type="button"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] transition-colors"
          title={t("queue.save_as_playlist")}
          onclick={saveQueueAsPlaylist}
        >
          <BookmarkPlus size={13} />
          <span>{t("queue.save_short")}</span>
        </button>
      {/if}
      {#if player.queue.length > 1}
        <button
          type="button"
          class="px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-red-500/20 hover:text-red-400 transition-colors"
          onclick={() => player.clearQueue()}
        >
          {t("queue.clear")}
        </button>
      {/if}
    </div>
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
        {@const isPointerDragging = pointerDragFrom === i}
        {@const activeDropTarget =
          (dragOverIdx === i && dragFrom !== i) ||
          (pointerDragOver === i && pointerDragFrom != null && pointerDragFrom !== i)}
        <div
          bind:this={rowEls[i]}
          class="relative rounded-xl transition-shadow {activeDropTarget
            ? 'outline -outline-offset-1 outline-[var(--accent-bright)]'
            : ''} {isPointerDragging ? 'z-20 shadow-lg bg-[var(--bg-elevated)] opacity-95' : ''}"
          style={isPointerDragging ? `transform: translateY(${pointerOffsetY}px);` : undefined}
          role="listitem"
          draggable={!device.isTouch}
          oncontextmenu={(e) => {
            if (pointerDragFrom != null) {
              e.preventDefault();
              e.stopPropagation();
            }
          }}
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
              <span
                role="presentation"
                data-drag-handle
                class="inline-flex items-center justify-center p-1.5 -ml-1.5 rounded-md cursor-grab active:cursor-grabbing text-[var(--text-muted)] opacity-55 md:opacity-35 group-hover:opacity-90 touch-none select-none shrink-0"
                style="touch-action: none;"
                onpointerdown={(e) => startPointerDrag(e, i)}
                onpointermove={movePointerDrag}
                onpointerup={(e) => endPointerDrag(e, true)}
                onpointercancel={(e) => endPointerDrag(e, false)}
                onclick={(e) => e.stopPropagation()}
                oncontextmenu={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                }}
              >
                <GripVertical size={15} aria-hidden="true" />
              </span>
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
      aria-label={t("queue.autoplay")}
      class="w-10 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.autoplayRelated
        ? 'bg-[var(--accent)]'
        : 'bg-[var(--control-track)]'}"
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
