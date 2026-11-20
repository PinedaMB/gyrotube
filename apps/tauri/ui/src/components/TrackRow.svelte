<script lang="ts">
  import { AudioLines, Heart, ListPlus, Pause, Plus, Trash2 } from "lucide-svelte";
  import type { Snippet } from "svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";
  import PlaylistIcon from "./collection/PlaylistIcon.svelte";

  let { track, onPlay, number, active, leading, onRemove, removeLabel }: {
    track: Track;
    onPlay: () => void;
    number?: number;
    active?: boolean;
    leading?: Snippet;
    onRemove?: () => void;
    removeLabel?: string;
  } = $props();

  let current = $derived(active ?? player.currentTrack?.id === track.id);
  let playing = $derived(current && player.isPlaying);
  let playbackLabel = $derived(t(playing ? "player.playing" : "player.paused"));
  let favorite = $derived(player.isFavorite(track.id));
  let pickerOpen = $state(false);
  let pickerButton: HTMLButtonElement;
  let pickerId = $props.id();
</script>

<div>
  <div class="flex min-w-0 items-center gap-1 px-2 transition-colors {playing ? 'bg-[var(--accent-soft)]' : current ? 'bg-[var(--surface-hover)]' : 'hover:bg-[var(--surface-hover)]'}">
    {@render leading?.()}
    <button
      type="button"
      class="flex min-w-0 flex-1 items-center gap-2 py-2 text-left"
      aria-label={`${t("home.play")} ${track.title}`}
      aria-current={current ? "true" : undefined}
      aria-describedby={current ? `${pickerId}-playback` : undefined}
      onclick={onPlay}
    >
      {#if number != null}
        <span class="w-5 shrink-0 text-center text-[11px] font-semibold text-[var(--text-muted)] tabular-nums" aria-hidden="true">{number}</span>
      {/if}
      <div class="relative shrink-0">
        <CoverImage class="track-cover !w-9 !h-9 md:!w-10 md:!h-10" src={track.cover_url} alt="" />
        {#if current}
          <span
            class="absolute -bottom-0.5 -right-0.5 grid size-5 place-items-center rounded-full border-2 border-[var(--bg-elevated)] {playing ? 'bg-[var(--accent-bright)] text-black' : 'bg-[var(--bg-elevated)] text-[var(--text)]'}"
            aria-hidden="true"
            title={playbackLabel}
          >
            {#if playing}
              <AudioLines size={12} />
            {:else}
              <Pause size={11} fill="currentColor" />
            {/if}
          </span>
        {/if}
      </div>
      <div class="track-meta">
        <span class="track-title {current ? '!font-bold' : ''}" title={track.title}>{track.title}</span>
        <span class="flex min-w-0 items-center gap-1">
          <span class="track-sub min-w-0" title={track.artist}>{track.artist}{track.album ? ` · ${track.album}` : ""}</span>
          {#if track.duration_secs != null}
            <span class="text-[11px] text-[var(--text-muted)] tabular-nums shrink-0 sm:hidden">· {formatDuration(track.duration_secs)}</span>
          {/if}
        </span>
      </div>
      {#if track.duration_secs != null}
        <span class="hidden sm:inline text-[11px] text-[var(--text-muted)] tabular-nums shrink-0">{formatDuration(track.duration_secs)}</span>
      {/if}
    </button>
    {#if current}
      <span id={`${pickerId}-playback`} class="sr-only">{playbackLabel}</span>
    {/if}
    <button
      type="button"
      class="icon-btn !w-10 !h-11 md:!w-8 md:!h-8 shrink-0"
      class:fav-active={favorite}
      title={favorite ? t("player.favorite_remove") : t("player.favorite_add")}
      aria-pressed={favorite}
      onclick={() => player.toggleFavorite(track)}
    >
      <Heart size={16} fill={favorite ? "currentColor" : "none"} />
    </button>
    <button
      type="button"
      class="icon-btn !w-10 !h-11 md:!w-8 md:!h-8 shrink-0"
      title={t("queue.add")}
      onclick={() => player.enqueueTrack(track)}
    >
      <ListPlus size={16} />
    </button>
    <button
      type="button"
      bind:this={pickerButton}
      class="icon-btn !w-10 !h-11 md:!w-8 md:!h-8 shrink-0"
      class:active={pickerOpen}
      title={t("collection.add_to_playlist")}
      aria-expanded={pickerOpen}
      aria-controls={pickerOpen ? pickerId : undefined}
      onclick={() => (pickerOpen = !pickerOpen)}
    >
      <Plus size={16} />
    </button>
    {#if onRemove}
      <button type="button" class="icon-btn !w-10 !h-11 md:!w-8 md:!h-8 shrink-0 hover:text-[var(--fav-coral)]" title={removeLabel} onclick={onRemove}>
        <Trash2 size={15} />
      </button>
    {/if}
  </div>

  {#if pickerOpen}
    <div id={pickerId} class="flex flex-wrap items-center gap-2 border-t border-[var(--border)] px-3 py-2">
      <span class="w-full text-xs text-[var(--text-muted)]">{t("collection.add_to_playlist")}</span>
      {#each player.userPlaylists as playlist (playlist.id)}
        <button type="button" class="pill-btn min-w-0 max-w-full" onclick={() => {
          player.addTrackToPlaylist(playlist.id, track);
          pickerOpen = false;
          pickerButton.focus();
        }}>
          <PlaylistIcon name={playlist.icon_name} size={13} />
          <span class="truncate">{playlist.name}</span>
        </button>
      {/each}
      <button type="button" class="pill-btn primary" onclick={() => {
        pickerOpen = false;
        player.playlistDraftTrack = track;
      }}>
        <Plus size={13} />
        {t("collection.new_playlist")}
      </button>
    </div>
  {/if}
</div>
