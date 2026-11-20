<script lang="ts">
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
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
    mode === "favorites" ? "#e01b24" : (playlist?.color_hex || "#2190a4"),
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

  function playAll() {
    if (!tracks.length) return;
    player.replaceQueueAndPlay(tracks, 0, title);
  }

  function shufflePlay() {
    if (!tracks.length) return;
    const shuffled = [...tracks].sort(() => Math.random() - 0.5);
    player.replaceQueueAndPlay(shuffled, 0, title);
  }

  function playTrack(index: number) {
    player.replaceQueueAndPlay(tracks, index, title);
  }
</script>

<div
  class="w-full min-h-full transition-all duration-500"
  style="background-image: radial-gradient(ellipse 75% 48% at 24% 26%, {accentColor}30 0%, {accentColor}0d 48%, transparent 78%);"
>
  <div class="w-full max-w-[1280px] mx-auto px-8 pt-6 pb-28">
    <div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-12 items-start">
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
        {title}
        {accentColor}
        {tracks}
        {sortBy}
        onCycleSort={cycleSort}
        onPlayTrack={playTrack}
      />
    </div>
  </div>
</div>
