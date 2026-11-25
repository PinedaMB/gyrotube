<script lang="ts">
  import CoverImage from "../CoverImage.svelte";
  import TrackRow from "../TrackRow.svelte";
  import {
    ArrowUpDown,
    Compass,
    GripVertical,
    Heart,
    Music,
    Pause,
    Play,
    Plus,
    RefreshCw,
    Search,
    X,
  } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { contextMenu } from "../../lib/stores/contextMenu.svelte";
  import { device } from "../../lib/stores/device.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track, type UserPlaylist } from "../../lib/types";

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
  let loadingSuggestions = $state(false);
  let suggestedTracks = $state<Track[]>([]);
  let lastSuggestedPlaylistId = $state<string | null>(null);
  let suggestionSeedOffset = 0;
  let suggestionGen = 0;

  // Automatically load suggestions (YouTube Music style) for both Playlists and Favorites
  $effect(() => {
    const sourceKey = mode === "favorites" ? "__favorites__" : (playlist?.id ?? null);
    const offline = player.isOffline;
    const hasTracks = tracks.length > 0;
    if (sourceKey !== lastSuggestedPlaylistId) {
      lastSuggestedPlaylistId = sourceKey;
      suggestedTracks = [];
      suggestionSeedOffset = 0;
      suggestionGen += 1;
      if (sourceKey && !offline && hasTracks) {
        void fetchSuggestions(false);
      }
    }
  });

  let supportsSuggestions = $derived(mode === "favorites" || (mode === "playlist" && playlist != null));

  // Remove already-added tracks from suggestions automatically
  let visibleSuggestions = $derived.by(() => {
    if (!supportsSuggestions || player.isOffline) return [];
    const existing = new Set(tracks.map((tr) => tr.id));
    return suggestedTracks.filter((tr) => !existing.has(tr.id) && !player.isTrackHidden(tr.id));
  });

  async function fetchSuggestions(rotateSeeds: boolean) {
    if (!supportsSuggestions || !isTauriRuntime() || player.isOffline || tracks.length === 0) {
      return;
    }
    if (loadingSuggestions) return;
    const gen = ++suggestionGen;
    loadingSuggestions = true;
    try {
      if (rotateSeeds) {
        suggestionSeedOffset = (suggestionSeedOffset + 1) % tracks.length;
      }
      const firstIdx = suggestionSeedOffset % tracks.length;
      const secondIdx = (firstIdx + Math.max(1, Math.floor(tracks.length / 2))) % tracks.length;
      const seeds =
        tracks.length > 1 && secondIdx !== firstIdx
          ? [tracks[firstIdx], tracks[secondIdx]]
          : [tracks[firstIdx]];
      const results = await Promise.all(
        seeds.map((s) => api.getRadioTracks(s.id).catch(() => [] as Track[])),
      );
      if (gen !== suggestionGen) return;
      const existing = new Set(tracks.map((tr) => tr.id));
      const previousIds = rotateSeeds
        ? new Set(suggestedTracks.map((tr) => tr.id))
        : new Set<string>();
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
        if (rotateSeeds) player.showToast(t("filter.no_matches"));
        return;
      }
      suggestedTracks = picked;
    } finally {
      if (gen === suggestionGen) {
        loadingSuggestions = false;
      }
    }
  }

  let canReorder = $derived(
    mode === "playlist" &&
      playlist != null &&
      sortBy === "custom" &&
      filterText.trim() === "" &&
      tracks.length > 1,
  );

  let filteredEntries = $derived.by(() => {
    const q = filterText.trim().toLowerCase();
    const out: { track: Track; originalIndex: number }[] = [];
    for (let i = 0; i < tracks.length; i++) {
      const tr = tracks[i];
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
    {#if tracks.length > 4 || tracks.length > 1}
      <div class="flex items-center gap-2">
        {#if tracks.length > 4}
          <div
            class="flex items-center gap-2 flex-1 min-w-0 sm:max-w-[300px] h-9 sm:h-8 px-3 rounded-full bg-[var(--surface)] border border-[var(--border)] focus-within:border-[var(--accent-border)] transition-colors"
          >
            <Search size={13} class="text-[var(--text-muted)] shrink-0" />
            <input
              type="text"
              bind:value={filterText}
              placeholder={t("filter.placeholder")}
              class="w-full min-w-0 bg-transparent outline-none focus-visible:!outline-none text-[13px] sm:text-[12.5px] placeholder:text-[var(--text-muted)]"
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

        {#if tracks.length > 1}
          <button
            type="button"
            class="ml-auto shrink-0 inline-flex items-center gap-1.5 h-9 sm:h-8 px-3 rounded-full text-[12px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-colors"
            onclick={onCycleSort}
          >
            <ArrowUpDown size={13} />
            {t("collection.sort", { mode: sortLabel(sortBy) })}
          </button>
        {/if}
      </div>
    {/if}

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

        {#if supportsSuggestions && !player.isOffline && (visibleSuggestions.length > 0 || loadingSuggestions)}
          <div class="mt-8 pt-6 border-t border-[var(--border)] space-y-2.5">
            <div class="flex items-center justify-between gap-2 px-2">
              <h3 class="text-[16px] font-bold tracking-[-0.015em] text-[var(--text)]">
                {t("header.suggestions")}
              </h3>
              <button
                type="button"
                disabled={loadingSuggestions}
                class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[12px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-[var(--text-muted)] hover:text-[var(--text)] transition-colors disabled:opacity-50 active:scale-95"
                onclick={() => void fetchSuggestions(true)}
              >
                <RefreshCw size={12} class={loadingSuggestions ? "animate-spin" : ""} />
                <span>{t("collection.enhance_refresh")}</span>
              </button>
            </div>

            {#if loadingSuggestions && visibleSuggestions.length === 0}
              <div class="flex flex-col gap-1">
                {#each [0, 1, 2, 3] as idx (idx)}
                  <div class="flex items-center gap-2.5 px-2 py-1.5 animate-pulse">
                    <div class="w-10 h-10 md:w-11 md:h-11 rounded-[8px] bg-[var(--surface)] shrink-0"></div>
                    <div class="flex-1 min-w-0 space-y-1.5">
                      <div class="h-3.5 w-40 max-w-[60%] rounded bg-[var(--surface)]"></div>
                      <div class="h-3 w-28 max-w-[40%] rounded bg-[var(--surface)]/70"></div>
                    </div>
                    <div class="w-8 h-8 rounded-full bg-[var(--surface)] shrink-0"></div>
                  </div>
                {/each}
              </div>
            {:else}
              <div class="flex flex-col gap-0.5 transition-opacity duration-200 {loadingSuggestions ? 'opacity-55 pointer-events-none' : ''}">
                {#each visibleSuggestions as sug (sug.id)}
                  {@const isCurrentSug = player.currentTrack?.id === sug.id}
                  {@const isPlayingSug = isCurrentSug && player.isPlaying}
                  {@const playSug = () =>
                    player.replaceQueueAndPlay(
                      [sug, ...tracks],
                      0,
                      mode === "favorites" ? t("collection.favorites") : (playlist?.name ?? "Playlist"),
                    )}
                  <div
                    role="group"
                    class="group flex items-center gap-2.5 rounded-xl px-2 py-1.5 transition-colors {isPlayingSug
                      ? 'bg-[var(--accent-soft)]'
                      : isCurrentSug
                        ? 'bg-[var(--surface-hover)]'
                        : 'hover:bg-[var(--surface-hover)]'}"
                    oncontextmenu={(e) => contextMenu.openTrackMenu(e, sug, { onPlay: playSug })}
                  >
                    <button
                      type="button"
                      class="flex items-center gap-2.5 flex-1 min-w-0 text-left"
                      onclick={playSug}
                    >
                      <div class="relative shrink-0">
                        <CoverImage
                          class="track-cover !w-10 !h-10 md:!w-11 md:!h-11 !rounded-[8px]"
                          src={player.trackCoverUrl(sug, false)}
                          fallbackSrc={player.offlineCoverUrl(sug.id)}
                          alt=""
                        />
                        {#if isCurrentSug}
                          <span
                            class="absolute inset-0 rounded-[8px] bg-black/55 flex items-center justify-center text-[var(--accent-bright)]"
                            aria-hidden="true"
                          >
                            {#if isPlayingSug}
                              <span class="eq-bars">
                                <span class="eq-bar"></span>
                                <span class="eq-bar"></span>
                                <span class="eq-bar"></span>
                                <span class="eq-bar"></span>
                              </span>
                            {:else}
                              <Pause size={15} fill="currentColor" />
                            {/if}
                          </span>
                        {:else}
                          <span
                            class="absolute inset-0 rounded-[8px] bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white"
                            aria-hidden="true"
                          >
                            <Play size={15} fill="currentColor" class="translate-x-[1px]" />
                          </span>
                        {/if}
                      </div>
                      <div class="min-w-0 flex-1">
                        <div
                          class="text-[14px] font-semibold truncate {isCurrentSug
                            ? 'text-[var(--accent-bright)]'
                            : 'text-[var(--text)]'}"
                        >
                          {sug.title}
                        </div>
                        <div class="text-[12px] text-[var(--text-muted)] truncate">
                          {sug.artist}{sug.album ? ` · ${sug.album}` : ""}
                        </div>
                      </div>
                    </button>

                    {#if sug.duration_secs}
                      <span class="hidden sm:inline text-[12px] text-[var(--text-muted)] tabular-nums shrink-0 pr-1">
                        {formatDuration(sug.duration_secs)}
                      </span>
                    {/if}

                    <button
                      type="button"
                      class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-[12px] font-semibold bg-[var(--surface)] hover:bg-[var(--accent)] border border-[var(--border)] hover:border-[var(--accent)] text-[var(--text-muted)] hover:text-white shrink-0 transition-all active:scale-95"
                      title={mode === "favorites" ? t("player.favorite_add") : t("collection.enhance_add")}
                      onclick={() => {
                        if (mode === "favorites") {
                          if (!player.isFavorite(sug.id)) player.toggleFavorite(sug);
                        } else if (playlist) {
                          player.addTrackToPlaylist(playlist.id, sug);
                        }
                      }}
                    >
                      <Plus size={13} />
                      <span>{t("collection.add_short")}</span>
                    </button>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</section>
