<script lang="ts">
  import {
    ArrowLeft,
    ArrowUpDown,
    Clock3,
    Heart,
    Pencil,
    Play,
    Plus,
    Shuffle,
    Trash2,
    Volume2,
    X,
  } from "lucide-svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track, type UserPlaylist } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";
  import PlaylistIcon from "./PlaylistIcon.svelte";

  interface Props {
    mode: "favorites" | "playlist";
    playlist?: UserPlaylist | null;
    onBack: () => void;
    onEdit?: (pl: UserPlaylist) => void;
    onDelete?: (pl: UserPlaylist) => void;
  }

  let { mode, playlist = null, onBack, onEdit, onDelete }: Props = $props();

  type SortBy = "custom" | "title" | "artist" | "duration";
  let sortBy = $state<SortBy>("custom");

  let rawTracks = $derived<Track[]>(mode === "favorites" ? player.favorites : (playlist?.tracks ?? []));
  let title = $derived(mode === "favorites" ? "Favorites" : (playlist?.name ?? "Playlist"));
  let description = $derived(
    mode === "favorites" ? "Songs you marked with a heart" : (playlist?.description ?? ""),
  );
  let accentColor = $derived(
    mode === "favorites" ? "#e01b24" : (playlist?.color_hex || "#2190a4"),
  );

  let tracks = $derived.by(() => {
    const copy = [...rawTracks];
    if (sortBy === "title") copy.sort((a, b) => a.title.localeCompare(b.title));
    else if (sortBy === "artist") copy.sort((a, b) => a.artist.localeCompare(b.artist));
    else if (sortBy === "duration") copy.sort((a, b) => (a.duration_secs ?? 0) - (b.duration_secs ?? 0));
    return copy;
  });

  let totalMins = $derived(
    Math.round(rawTracks.reduce((sum, t) => sum + (t.duration_secs ?? 0), 0) / 60),
  );

  function cycleSort() {
    const order: SortBy[] = ["custom", "title", "artist", "duration"];
    sortBy = order[(order.indexOf(sortBy) + 1) % order.length];
  }

  function sortLabel(s: SortBy): string {
    if (s === "title") return "Title";
    if (s === "artist") return "Artist";
    if (s === "duration") return "Duration";
    return "Custom";
  }

  function shufflePlay() {
    if (!tracks.length) return;
    const shuffled = [...tracks].sort(() => Math.random() - 0.5);
    player.replaceQueueAndPlay(shuffled, 0, title);
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex flex-col sm:flex-row items-start sm:items-center gap-6">
    <div class="flex items-start gap-4">
      <button
        class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 shrink-0"
        title="Back"
        onclick={onBack}
      >
        <ArrowLeft size={17} />
      </button>

      <div
        class="w-28 h-28 rounded-2xl flex items-center justify-center text-white shadow-[0_16px_38px_rgba(0,0,0,0.42)] shrink-0"
        style={mode === "favorites"
          ? "background: linear-gradient(135deg, #e01b24, #ff6b81);"
          : `background: linear-gradient(135deg, ${accentColor}, ${accentColor}aa);`}
      >
        {#if mode === "favorites"}
          <Heart size={42} fill="currentColor" />
        {:else}
          <PlaylistIcon name={playlist?.icon_name} size={42} />
        {/if}
      </div>
    </div>

    <div class="min-w-0 flex-1 space-y-1.5">
      <span
        class="inline-flex px-2.5 py-0.5 rounded-full text-[10.5px] font-extrabold uppercase tracking-[0.08em] bg-[var(--accent-soft)] text-[var(--accent-bright)]"
      >
        {mode === "favorites" ? "Favorites" : "Local playlist"}
      </span>
      <h1 class="text-[30px] font-black tracking-[-0.03em] leading-tight truncate">{title}</h1>
      {#if description}
        <p class="text-[13.5px] text-[var(--text-muted)]">{description}</p>
      {/if}
      <p class="text-[12.5px] text-[var(--text-muted)] numeric">
        {rawTracks.length} {rawTracks.length === 1 ? "song" : "songs"}{totalMins ? ` · ${totalMins} min` : ""}
      </p>

      <div class="flex flex-wrap items-center gap-2 pt-2">
        <button
          class="pill-btn primary !px-5"
          disabled={tracks.length === 0}
          onclick={() => player.replaceQueueAndPlay(tracks, 0, title)}
        >
          <Play size={14} fill="currentColor" />
          Play all
        </button>

        <button
          class="pill-btn"
          disabled={tracks.length === 0}
          onclick={shufflePlay}
        >
          <Shuffle size={14} />
          Shuffle
        </button>

        {#if rawTracks.length > 1}
          <button class="pill-btn" onclick={cycleSort} title="Change sort order">
            <ArrowUpDown size={14} />
            Sort: {sortLabel(sortBy)}
          </button>
        {/if}

        {#if mode === "playlist" && playlist}
          <button class="pill-btn" onclick={() => onEdit?.(playlist)}>
            <Pencil size={14} />
            Customize
          </button>
          <button
            class="pill-btn hover:!bg-red-500/20 hover:!border-red-500/40 hover:!text-red-400"
            onclick={() => onDelete?.(playlist)}
          >
            <Trash2 size={14} />
            Delete
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Track list -->
  {#if tracks.length === 0}
    <div class="py-12 text-center rounded-2xl bg-[var(--surface)] border border-[var(--border)]">
      <p class="text-[var(--text-muted)] text-[14px]">
        {mode === "favorites"
          ? "Tap the heart on any song to save it here."
          : "This playlist is empty. Add songs from Recently played or search results."}
      </p>
    </div>
  {:else}
    <div>
      <div class="flex items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]">
        <span class="w-6 text-center">#</span>
        <span class="flex-1">Title</span>
        <span class="w-11 flex justify-end"><Clock3 size={13} /></span>
      </div>

      <div class="flex flex-col">
        {#each tracks as track, i (track.id + ":" + i)}
          {@const active = player.currentTrack?.id === track.id}
          <div
            class="group flex items-center gap-3.5 py-2.5 px-3 border-b border-white/[0.06] last:border-b-0 cursor-pointer transition-colors {active
              ? 'bg-[var(--accent-soft)]'
              : 'hover:bg-white/[0.05]'}"
            role="button"
            tabindex="0"
            onclick={() => player.replaceQueueAndPlay(tracks, i, title)}
            onkeydown={(e) => e.key === "Enter" && player.replaceQueueAndPlay(tracks, i, title)}
          >
            <div class="w-6 flex items-center justify-center shrink-0">
              {#if active}
                <Volume2 size={15} class="text-[var(--accent-bright)]" />
              {:else}
                <span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric group-hover:hidden">
                  {i + 1}
                </span>
                <Play size={13} fill="currentColor" class="hidden group-hover:block translate-x-[1px]" />
              {/if}
            </div>

            <CoverImage class="track-cover !w-10 !h-10 !rounded-lg shrink-0" src={track.cover_url} alt="" />

            <div class="min-w-0 flex-1">
              <div class="text-[14px] font-semibold truncate {active ? 'text-[var(--accent-bright)] font-bold' : ''}">
                {track.title}
              </div>
              <div class="text-[12px] text-[var(--text-muted)] truncate">
                {track.artist}{track.album ? ` • ${track.album}` : ""}
              </div>
            </div>

            <div class="flex items-center gap-1 shrink-0">
              <button
                class="icon-btn !w-8 !h-8"
                class:fav-active={player.isFavorite(track.id)}
                title="Favorite"
                onclick={(e) => {
                  e.stopPropagation();
                  player.toggleFavorite(track);
                }}
              >
                <Heart size={16} fill={player.isFavorite(track.id) ? "currentColor" : "none"} />
              </button>
              <button
                class="icon-btn !w-8 !h-8"
                title="Add to queue"
                onclick={(e) => {
                  e.stopPropagation();
                  player.enqueueTrack(track);
                }}
              >
                <Plus size={16} />
              </button>
              {#if mode === "playlist" && playlist}
                <button
                  class="icon-btn !w-8 !h-8 opacity-0 group-hover:opacity-100 transition-opacity hover:!text-red-400"
                  title="Remove from playlist"
                  onclick={(e) => {
                    e.stopPropagation();
                    player.removeTrackFromPlaylist(playlist.id, track.id);
                  }}
                >
                  <X size={15} />
                </button>
              {/if}
              <span class="text-[12.5px] text-[var(--text-muted)] numeric w-11 text-right">
                {formatDuration(track.duration_secs)}
              </span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
