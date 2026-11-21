<script lang="ts">
  import { AudioLines, Ellipsis, Heart, ListPlus, Pause, Play, Plus, Trash2 } from "lucide-svelte";
  import type { Snippet } from "svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration, formatPlays, type Track } from "../lib/types";
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
  let mobileActionsOpen = $state(false);
  let pickerId = $props.id();
</script>

<div>
  <div
    class="group relative flex min-w-0 items-center gap-2 rounded-xl px-2 py-1.5 transition-colors {playing
      ? 'bg-[var(--accent-soft)]'
      : current
        ? 'bg-white/[0.07]'
        : 'hover:bg-white/[0.06]'}"
  >
    {@render leading?.()}

    <button
      type="button"
      class="flex min-w-0 flex-1 items-center gap-2.5 text-left"
      aria-label={`${t("home.play")} ${track.title}`}
      aria-current={current ? "true" : undefined}
      aria-describedby={current ? `${pickerId}-playback` : undefined}
      onclick={onPlay}
    >
      {#if number != null}
        <span class="w-5 shrink-0 text-center text-[12px] font-semibold text-[var(--text-muted)] tabular-nums" aria-hidden="true">
          {number}
        </span>
      {/if}

      <div class="relative shrink-0">
        <CoverImage class="track-cover !w-10 !h-10 !rounded-[8px]" src={track.cover_url} alt="" />
        {#if current}
          <span
            class="absolute inset-0 rounded-[8px] bg-black/55 flex items-center justify-center text-[var(--accent-bright)]"
            aria-hidden="true"
            title={playbackLabel}
          >
            {#if playing}
              <AudioLines size={16} />
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
          class="text-[13.5px] font-semibold truncate {current
            ? 'text-[var(--accent-bright)] font-bold'
            : ''}"
          title={track.title}
        >
          {track.title}
        </div>
        <div class="text-[12px] text-[var(--text-muted)] truncate" title={track.artist}>
          {track.artist}{track.album ? ` · ${track.album}` : ""}
        </div>
      </div>

      {#if track.view_count}
        <span class="hidden lg:inline text-[11.5px] text-[var(--text-muted)] tabular-nums shrink-0 mr-1 md:group-hover:hidden md:group-focus-within:hidden">
          {t("artist.plays", { count: formatPlays(track.view_count) })}
        </span>
      {/if}

      {#if favorite}
        <span class="text-[var(--fav-coral)] shrink-0 {pickerOpen ? 'hidden' : 'md:group-hover:hidden md:group-focus-within:hidden'}" aria-hidden="true">
          <Heart size={14} fill="currentColor" />
        </span>
      {/if}

      {#if track.duration_secs != null}
        <span class="text-[12px] text-[var(--text-muted)] tabular-nums shrink-0 {pickerOpen ? 'hidden' : 'md:group-hover:hidden md:group-focus-within:hidden'}">
          {formatDuration(track.duration_secs)}
        </span>
      {/if}
    </button>

    {#if current}
      <span id={`${pickerId}-playback`} class="sr-only">{playbackLabel}</span>
    {/if}

    <!-- Desktop hover-revealed inline action cluster (no overlay box; text truncates cleanly) -->
    <div
      class="items-center gap-0.5 shrink-0 {pickerOpen
        ? 'hidden md:flex'
        : 'hidden md:group-hover:flex md:group-focus-within:flex'}"
    >
      <button
        type="button"
        class="icon-btn !w-7 !h-7 shrink-0"
        class:fav-active={favorite}
        title={favorite ? t("player.favorite_remove") : t("player.favorite_add")}
        aria-pressed={favorite}
        onclick={() => player.toggleFavorite(track)}
      >
        <Heart size={15} fill={favorite ? "currentColor" : "none"} />
      </button>
      <button
        type="button"
        class="icon-btn !w-7 !h-7 shrink-0"
        title={t("queue.add")}
        onclick={() => player.enqueueTrack(track)}
      >
        <ListPlus size={15} />
      </button>
      <button
        type="button"
        class="icon-btn !w-7 !h-7 shrink-0"
        class:active={pickerOpen}
        title={t("collection.add_to_playlist")}
        aria-expanded={pickerOpen}
        aria-controls={pickerOpen ? pickerId : undefined}
        onclick={() => (pickerOpen = !pickerOpen)}
      >
        <Plus size={15} />
      </button>
      {#if onRemove}
        <button
          type="button"
          class="icon-btn !w-7 !h-7 shrink-0 hover:!text-red-400"
          title={removeLabel}
          onclick={onRemove}
        >
          <Trash2 size={14} />
        </button>
      {/if}
    </div>

    <!-- Mobile compact menu trigger (keeps full width for titles on phones) -->
    <button
      type="button"
      class="md:hidden icon-btn !w-8 !h-8 shrink-0 text-[var(--text-muted)]"
      class:active={mobileActionsOpen || pickerOpen}
      aria-label="Track actions"
      onclick={() => {
        mobileActionsOpen = !mobileActionsOpen;
        if (!mobileActionsOpen) pickerOpen = false;
      }}
    >
      <Ellipsis size={16} />
    </button>
  </div>

  {#if mobileActionsOpen}
    <div class="md:hidden flex items-center gap-1.5 px-3 py-1.5">
      <button
        type="button"
        class="pill-btn !py-1 !text-[12px]"
        class:text-[var(--fav-coral)]={favorite}
        onclick={() => player.toggleFavorite(track)}
      >
        <Heart size={13} fill={favorite ? "currentColor" : "none"} />
        {favorite ? t("player.favorite_remove") : t("player.favorite_add")}
      </button>
      <button
        type="button"
        class="pill-btn !py-1 !text-[12px]"
        onclick={() => {
          player.enqueueTrack(track);
          mobileActionsOpen = false;
        }}
      >
        <ListPlus size={13} />
        {t("queue.add")}
      </button>
      <button
        type="button"
        class="pill-btn !py-1 !text-[12px]"
        onclick={() => (pickerOpen = !pickerOpen)}
      >
        <Plus size={13} />
      </button>
      {#if onRemove}
        <button
          type="button"
          class="pill-btn !py-1 !text-[12px] hover:!text-red-400"
          onclick={onRemove}
        >
          <Trash2 size={13} />
        </button>
      {/if}
    </div>
  {/if}

  {#if pickerOpen}
    <div id={pickerId} class="flex flex-wrap items-center gap-2 rounded-xl bg-[var(--surface)] border border-[var(--border)] mx-2 my-1 px-3 py-2">
      <span class="w-full text-xs text-[var(--text-muted)]">{t("collection.add_to_playlist")}</span>
      {#each player.userPlaylists as playlist (playlist.id)}
        <button
          type="button"
          class="pill-btn min-w-0 max-w-full"
          onclick={() => {
            player.addTrackToPlaylist(playlist.id, track);
            pickerOpen = false;
            mobileActionsOpen = false;
          }}
        >
          <PlaylistIcon name={playlist.icon_name} size={13} />
          <span class="truncate">{playlist.name}</span>
        </button>
      {/each}
      <button
        type="button"
        class="pill-btn primary"
        onclick={() => {
          pickerOpen = false;
          mobileActionsOpen = false;
          player.playlistDraftTrack = track;
        }}
      >
        <Plus size={13} />
        {t("collection.new_playlist")}
      </button>
    </div>
  {/if}
</div>
