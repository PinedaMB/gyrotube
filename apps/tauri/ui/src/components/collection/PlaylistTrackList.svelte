<script lang="ts">
  import TrackRow from "../TrackRow.svelte";
  import {
    ArrowDownCircle,
    ArrowUpDown,
    Compass,
    GripVertical,
    Heart,
    Music,
    Plus,
    RefreshCw,
    Search,
    Sparkles,
    X,
  } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { device } from "../../lib/stores/device.svelte";
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

  let filterText = $state("");
  let onlyDownloaded = $state(false);
  let enhanced = $state(false);
  let enhancing = $state(false);
  let enhancedTracks = $state<Track[]>([]);
  let lastEnhancedPlaylistId = $state<string | null>(null);
  let enhanceSeedOffset = 0;

  $effect(() => {
    const pid = playlist?.id ?? null;
    if (pid !== lastEnhancedPlaylistId) {
      lastEnhancedPlaylistId = pid;
      enhanced = false;
      enhancedTracks = [];
      enhanceSeedOffset = 0;
    }
  });

  // Remove already-added tracks from enhanced suggestions automatically
  let visibleEnhanced = $derived.by(() => {
    if (!enhanced) return [];
    const existing = new Set(tracks.map((tr) => tr.id));
    return enhancedTracks.filter((tr) => !existing.has(tr.id) && !player.isTrackHidden(tr.id));
  });

  async function fetchEnhancedSuggestions(rotateSeeds: boolean) {
    if (!isTauriRuntime() || player.isOffline || tracks.length === 0 || enhancing) return;
    enhancing = true;
    try {
      if (rotateSeeds) {
        enhanceSeedOffset = (enhanceSeedOffset + 1) % tracks.length;
      }
      const firstIdx = enhanceSeedOffset % tracks.length;
      const secondIdx = (firstIdx + Math.max(1, Math.floor(tracks.length / 2))) % tracks.length;
      const seeds = tracks.length > 1 && secondIdx !== firstIdx
        ? [tracks[firstIdx], tracks[secondIdx]]
        : [tracks[firstIdx]];
      const results = await Promise.all(
        seeds.map((s) => api.getRadioTracks(s.id).catch(() => [] as Track[])),
      );
      const existing = new Set(tracks.map((tr) => tr.id));
      const previousIds = rotateSeeds ? new Set(enhancedTracks.map((tr) => tr.id)) : new Set<string>();
      const seen = new Set<string>();
      const fresh: Track[] = [];
      const fallback: Track[] = [];
      for (const list of results) {
        for (const tr of list) {
          if (existing.has(tr.id) || seen.has(tr.id) || player.isTrackHidden(tr.id)) continue;
          seen.add(tr.id);
          if (previousIds.has(tr.id)) {
            fallback.push(tr);
          } else {
            fresh.push(tr);
          }
        }
      }
      const picked = [...fresh, ...fallback].slice(0, 6);
      if (picked.length === 0) {
        player.showToast(t("filter.no_matches"));
        return;
      }
      enhancedTracks = picked;
      enhanced = true;
    } finally {
      enhancing = false;
    }
  }

  async function toggleEnhance() {
    if (enhanced) {
      enhanced = false;
      enhancedTracks = [];
      return;
    }
    await fetchEnhancedSuggestions(false);
  }

  let hasAnyDownloaded = $derived(tracks.some((tr) => player.isTrackDownloaded(tr.id)));
  let canReorder = $derived(
    mode === "playlist" &&
      playlist != null &&
      sortBy === "custom" &&
      filterText.trim() === "" &&
      !onlyDownloaded &&
      tracks.length > 1,
  );

  let filteredEntries = $derived.by(() => {
    const q = filterText.trim().toLowerCase();
    const out: { track: Track; originalIndex: number }[] = [];
    for (let i = 0; i < tracks.length; i++) {
      const tr = tracks[i];
      if (onlyDownloaded && !player.isTrackDownloaded(tr.id)) continue;
      if (
        q &&
        !tr.title.toLowerCase().includes(q) &&
        !tr.artist.toLowerCase().includes(q) &&
        !(tr.album && tr.album.toLowerCase().includes(q))
      ) {
        continue;
      }
      out.push({ track: tr, originalIndex: i });
    }
    return out;
  });

  // Drag-and-drop reordering for custom-sorted local playlists
  let dragFrom = $state<number | null>(null);
  let dragOverIdx = $state<number | null>(null);
  let pointerDragFrom = $state<number | null>(null);
  let pointerDragOver = $state<number | null>(null);
  let pointerStartY = 0;
  let pointerOffsetY = $state(0);
  let rowEls = $state<Record<number, HTMLDivElement>>({});

  function findRowIndexAtY(clientY: number): number | null {
    const count = tracks.length;
    if (count === 0) return null;
    for (let idx = 0; idx < count; idx++) {
      const el = rowEls[idx];
      if (!el) continue;
      const rect = el.getBoundingClientRect();
      if (clientY < rect.top + rect.height / 2) return idx;
    }
    return count - 1;
  }

  function startPointerDrag(e: PointerEvent, index: number) {
    if (!canReorder) return;
    e.preventDefault();
    e.stopPropagation();
    try {
      (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    } catch {
      // ignore
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
    if (targetIdx != null) pointerDragOver = targetIdx;
  }

  function endPointerDrag(e: PointerEvent, commit: boolean) {
    if (pointerDragFrom == null) return;
    e.preventDefault();
    const from = pointerDragFrom;
    const to = pointerDragOver;
    pointerDragFrom = null;
    pointerDragOver = null;
    pointerOffsetY = 0;
    if (commit && playlist && to != null && from !== to) {
      player.movePlaylistTrack(playlist.id, from, to);
    }
  }

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
    <div class="flex flex-wrap items-center justify-between gap-2">
      {#if tracks.length > 4}
        <div
          class="flex items-center gap-2 flex-1 min-w-[180px] max-w-[300px] h-8 px-3 rounded-full bg-[var(--surface)] border border-[var(--border)] focus-within:border-[var(--accent-border)] transition-colors"
        >
          <Search size={13} class="text-[var(--text-muted)] shrink-0" />
          <input
            type="text"
            bind:value={filterText}
            placeholder={t("filter.placeholder")}
            class="w-full bg-transparent outline-none focus-visible:!outline-none text-[12.5px] placeholder:text-[var(--text-muted)]"
          />
          {#if filterText}
            <button
              type="button"
              class="text-[var(--text-muted)] hover:text-[var(--text)]"
              aria-label={t("header.clear")}
              onclick={() => (filterText = "")}
            >
              <X size={13} />
            </button>
          {/if}
        </div>
      {/if}

      <div class="flex flex-wrap items-center gap-1.5 {tracks.length > 4 ? 'ml-auto' : 'w-full justify-between'}">
        {#if mode === "playlist" && playlist && !player.isOffline && tracks.length > 0}
          <button
            type="button"
            aria-pressed={enhanced}
            disabled={enhancing}
            class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[12px] font-semibold border transition-colors disabled:opacity-50 {enhanced
              ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] border-[var(--accent-border)]'
              : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] border-[var(--border)] text-[var(--text-muted)] hover:text-[var(--text)]'}"
            onclick={() => void toggleEnhance()}
          >
            <Sparkles size={13} class={enhancing ? "animate-spin" : ""} />
            {enhancing
              ? t("collection.enhance_loading")
              : enhanced
                ? t("collection.enhance_active")
                : t("collection.enhance")}
          </button>
        {/if}

        {#if hasAnyDownloaded}
          <button
            type="button"
            aria-pressed={onlyDownloaded}
            class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[12px] font-semibold border transition-colors {onlyDownloaded
              ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] border-[var(--accent-border)]'
              : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] border-[var(--border)] text-[var(--text-muted)]'}"
            onclick={() => (onlyDownloaded = !onlyDownloaded)}
          >
            <ArrowDownCircle size={13} />
            {t("filter.downloaded_only")}
          </button>
        {/if}

        {#if tracks.length > 1}
          <button
            type="button"
            class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[12px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-colors"
            onclick={onCycleSort}
          >
            <ArrowUpDown size={13} />
            {t("collection.sort", { mode: sortLabel(sortBy) })}
          </button>
        {/if}
      </div>
    </div>

    {#if filteredEntries.length === 0}
      <p class="text-[13px] text-[var(--text-muted)] text-center py-10">
        {t("filter.no_matches")}
      </p>
    {:else}
      <div>
        <div class="track-list" role="list">
          {#each filteredEntries as { track, originalIndex } (track.id + ":" + originalIndex)}
            {@const unavailableOffline = player.isOffline && !player.isTrackDownloaded(track.id)}
            {@const isPointerDragging = pointerDragFrom === originalIndex}
            {@const activeDropTarget =
              canReorder &&
              ((dragOverIdx === originalIndex && dragFrom !== originalIndex) ||
                (pointerDragOver === originalIndex &&
                  pointerDragFrom != null &&
                  pointerDragFrom !== originalIndex))}
            <div
              bind:this={rowEls[originalIndex]}
              role="listitem"
              class="relative rounded-xl transition-shadow {unavailableOffline ? 'opacity-45' : ''} {activeDropTarget
                ? 'outline -outline-offset-1 outline-[var(--accent-bright)]'
                : ''} {isPointerDragging ? 'z-20 shadow-lg bg-[var(--bg-elevated)] opacity-95' : ''}"
              style={isPointerDragging ? `transform: translateY(${pointerOffsetY}px);` : undefined}
              draggable={canReorder && !device.isTouch}
              ondragstart={() => {
                if (canReorder) dragFrom = originalIndex;
              }}
              ondragover={(e) => {
                if (!canReorder) return;
                e.preventDefault();
                dragOverIdx = originalIndex;
              }}
              ondragleave={() => {
                if (dragOverIdx === originalIndex) dragOverIdx = null;
              }}
              ondrop={() => {
                if (canReorder && playlist && dragFrom != null && dragFrom !== originalIndex) {
                  player.movePlaylistTrack(playlist.id, dragFrom, originalIndex);
                }
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
                number={originalIndex + 1}
                onPlay={() => {
                  if (unavailableOffline) {
                    player.showToast(t("offline.not_downloaded"));
                    return;
                  }
                  onPlayTrack(originalIndex);
                }}
                onRemove={mode === "playlist" && playlist ? () => player.removeTrackFromPlaylist(playlist.id, track.id) : undefined}
                removeLabel={t("collection.remove_from_playlist")}
              >
                {#snippet leading()}
                  {#if canReorder}
                    <span
                      role="presentation"
                      data-drag-handle
                      class="inline-flex items-center justify-center p-1 -ml-1 rounded-md cursor-grab active:cursor-grabbing text-[var(--text-muted)] opacity-45 md:opacity-30 group-hover:opacity-90 touch-none select-none shrink-0"
                      style="touch-action: none;"
                      onpointerdown={(e) => startPointerDrag(e, originalIndex)}
                      onpointermove={movePointerDrag}
                      onpointerup={(e) => endPointerDrag(e, true)}
                      onpointercancel={(e) => endPointerDrag(e, false)}
                      onclick={(e) => e.stopPropagation()}
                    >
                      <GripVertical size={14} aria-hidden="true" />
                    </span>
                  {/if}
                {/snippet}
              </TrackRow>
            </div>
          {/each}
        </div>

        {#if visibleEnhanced.length > 0 && playlist}
          <div class="mt-5 pt-4 border-t border-[var(--border)] space-y-2">
            <div class="flex items-center justify-between gap-2 px-1">
              <span class="inline-flex items-center gap-1.5 text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--accent-bright)]">
                <Sparkles size={12} />
                {t("collection.enhance_active")} · {visibleEnhanced.length}
              </span>
              <button
                type="button"
                disabled={enhancing || player.isOffline}
                class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-[var(--text-muted)] hover:text-[var(--text)] transition-colors disabled:opacity-50 active:scale-95"
                onclick={() => void fetchEnhancedSuggestions(true)}
              >
                <RefreshCw size={12} class={enhancing ? "animate-spin" : ""} />
                <span>{t("collection.enhance_refresh")}</span>
              </button>
            </div>
            <div class="track-list">
              {#each visibleEnhanced as sug (sug.id)}
                <div class="flex items-center gap-1.5 rounded-xl bg-[var(--accent-soft)]/35 border border-[var(--accent-border)]/30 pr-2">
                  <div class="flex-1 min-w-0">
                    <TrackRow
                      track={sug}
                      onPlay={() =>
                        player.replaceQueueAndPlay([sug, ...tracks], 0, playlist?.name ?? "Playlist")}
                    />
                  </div>
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[11.5px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shrink-0 transition-all active:scale-95"
                    title={t("collection.enhance_add")}
                    onclick={() => playlist && player.addTrackToPlaylist(playlist.id, sug)}
                  >
                    <Plus size={12} />
                    {t("queue.add").split(" ")[0]}
                  </button>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</section>
