<script lang="ts">
  import { extractDominantRgb } from "../../lib/audioEngine";
  import { rgbToHex } from "../../lib/colorUtils";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { shuffleItems } from "../../lib/stores/queueOps";
  import type { Track, UserPlaylist } from "../../lib/types";
  import PlaylistSidebarHero from "./PlaylistSidebarHero.svelte";
  import PlaylistTrackList, { type SortBy } from "./PlaylistTrackList.svelte";

  interface Props {
    mode: "favorites" | "playlist";
    playlist?: UserPlaylist | null;
    onBack: () => void;
    onEdit?: (pl: UserPlaylist) => void;
    onDelete?: (pl: UserPlaylist) => void;
  }

  let { mode, playlist = null, onBack, onEdit, onDelete }: Props = $props();

  let sortBy = $state<SortBy>("custom");
  let extractedCoverHex = $state<string | null>(null);

  $effect(() => {
    const cover = mode === "playlist" ? playlist?.cover_path ?? null : null;
    if (!cover) {
      extractedCoverHex = null;
      return;
    }
    let cancelled = false;
    void extractDominantRgb(cover).then((rgb) => {
      if (!cancelled) {
        extractedCoverHex = rgb ? rgbToHex(rgb) : null;
      }
    });
    return () => {
      cancelled = true;
    };
  });

  let rawTracks = $derived<Track[]>(
    mode === "favorites" ? player.favorites : (playlist?.tracks ?? []),
  );
  let title = $derived(
    mode === "favorites" ? t("collection.favorites") : (playlist?.name ?? "Playlist"),
  );
  let description = $derived(
    mode === "favorites" ? t("collection.favorites_desc") : (playlist?.description ?? ""),
  );
  let accentColor = $derived(
    mode === "favorites"
      ? "#e11d48"
      : (playlist?.color_hex || extractedCoverHex || "#10b981"),
  );

  let tracks = $derived.by(() => {
    const copy = [...rawTracks];
    if (sortBy === "title") copy.sort((a, b) => a.title.localeCompare(b.title));
    else if (sortBy === "artist") copy.sort((a, b) => a.artist.localeCompare(b.artist));
    else if (sortBy === "duration")
      copy.sort((a, b) => (a.duration_secs ?? 0) - (b.duration_secs ?? 0));
    return copy;
  });

  let totalMins = $derived(
    Math.round(rawTracks.reduce((sum, tr) => sum + (tr.duration_secs ?? 0), 0) / 60),
  );

  function cycleSort() {
    const order: SortBy[] = ["custom", "title", "artist", "duration"];
    sortBy = order[(order.indexOf(sortBy) + 1) % order.length];
  }

  function playableTracks(): Track[] {
    if (!player.isOffline) return tracks;
    return tracks.filter((tr) => player.isTrackDownloaded(tr.id));
  }

  function playAll() {
    const list = playableTracks();
    if (!list.length) {
      if (player.isOffline) player.showToast(t("offline.not_downloaded"));
      return;
    }
    player.replaceQueueAndPlay(list, 0, title);
  }

  function shufflePlay() {
    const list = playableTracks();
    if (!list.length) {
      if (player.isOffline) player.showToast(t("offline.not_downloaded"));
      return;
    }
    const shuffled = shuffleItems(list);
    player.replaceQueueAndPlay(shuffled, 0, title);
  }

  function playTrack(index: number) {
    if (player.isOffline) {
      const target = tracks[index];
      const list = playableTracks();
      const idx = target ? list.findIndex((tr) => tr.id === target.id) : -1;
      if (idx < 0) {
        player.showToast(t("offline.not_downloaded"));
        return;
      }
      player.replaceQueueAndPlay(list, idx, title);
      return;
    }
    player.replaceQueueAndPlay(tracks, index, title);
  }
</script>

<div
  class="w-full min-h-full"
  style="background-image: radial-gradient(ellipse 75% 48% at 24% 26%, {accentColor}30 0%, {accentColor}0d 48%, transparent 78%);"
>
  <div class="w-full max-w-[1280px] mx-auto px-4 sm:px-8 pt-[calc(0.75rem+var(--safe-top))] sm:pt-6 pb-36 md:pb-28">
    <div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-5 lg:gap-12 items-start">
      <PlaylistSidebarHero
        {mode}
        {playlist}
        {title}
        {description}
        {accentColor}
        {tracks}
        {totalMins}
        {onBack}
        onPlayAll={playAll}
        onShuffle={shufflePlay}
        {onEdit}
        {onDelete}
      />

      <PlaylistTrackList
        {mode}
        {playlist}
        {accentColor}
        {tracks}
        {sortBy}
        onCycleSort={cycleSort}
        onPlayTrack={playTrack}
      />
    </div>
  </div>
</div>
