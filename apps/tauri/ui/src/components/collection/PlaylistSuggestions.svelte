<script lang="ts">
  import { Pause, Play, Plus, RefreshCw } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { contextMenu } from "../../lib/stores/contextMenu.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track, type UserPlaylist } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  interface Props {
    mode: "favorites" | "playlist";
    playlist?: UserPlaylist | null;
    tracks: Track[];
  }

  let { mode, playlist = null, tracks }: Props = $props();

  let loadingSuggestions = $state(false);
  let suggestedTracks = $state<Track[]>([]);
  let lastSuggestedPlaylistId = $state<string | null>(null);
  let suggestionSeedOffset = 0;
  let suggestionGen = 0;

  let supportsSuggestions = $derived(
    mode === "favorites" || (mode === "playlist" && playlist != null),
  );

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
</script>

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
      <div
        class="flex flex-col gap-0.5 transition-opacity duration-200 {loadingSuggestions
          ? 'opacity-55 pointer-events-none'
          : ''}"
      >
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
