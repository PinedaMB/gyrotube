<script lang="ts">
  import { ArrowDownCircle, Check, Ellipsis, Heart, ListPlus, Pause, Play, Plus, Trash2, X } from "lucide-svelte";
  import type { Snippet } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { t } from "../lib/i18n.svelte";
  import { contextMenu } from "../lib/stores/contextMenu.svelte";
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
  let displayDuration = $derived(
    track.duration_secs ?? (current && player.durationSecs > 0 ? Math.round(player.durationSecs) : null),
  );
  let pickerOpen = $state(false);
  let pickerId = $props.id();
</script>

<div class="relative">
  <div
    role="group"
    class="group relative flex min-w-0 items-center gap-2 rounded-xl px-2 py-1.5 transition-colors {playing
      ? 'bg-[var(--accent-soft)]'
      : current
        ? 'bg-[var(--surface-hover)]'
        : 'hover:bg-[var(--surface-hover)]'}"
    oncontextmenu={(e) => contextMenu.openTrackMenu(e, track, { onPlay, onRemove, removeLabel })}
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
        <CoverImage
          class="track-cover !w-10 !h-10 md:!w-11 md:!h-11 !rounded-[8px]"
          src={player.trackCoverUrl(track, false)}
          fallbackSrc={player.offlineCoverUrl(track.id)}
          alt=""
        />
        {#if current}
          <span
            class="absolute inset-0 rounded-[8px] bg-black/55 flex items-center justify-center text-[var(--accent-bright)]"
            aria-hidden="true"
            title={playbackLabel}
          >
            {#if playing}
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
          class="text-[14px] md:text-[15px] font-semibold truncate {current
            ? 'text-[var(--accent-bright)] font-bold'
            : ''}"
          title={track.title}
        >
          {track.title}
        </div>
        <div class="flex items-center gap-1.5 text-[12.5px] md:text-[13px] text-[var(--text-muted)]" title={track.artist}>
          {#if player.isTrackDownloading(track.id)}
            <span
              class="w-3 h-3 rounded-full border-[1.5px] border-[var(--accent-bright)]/35 border-t-[var(--accent-bright)] animate-spin shrink-0"
              aria-hidden="true"
            ></span>
          {:else if player.isTrackDownloaded(track.id)}
            <ArrowDownCircle size={13} class="text-[var(--accent-bright)] shrink-0" aria-hidden="true" />
          {/if}
          <span class="truncate">{track.artist}{track.album ? ` · ${track.album}` : ""}</span>
        </div>
      </div>

      {#if track.view_count}
        <span class="hidden lg:inline text-[11.5px] text-[var(--text-muted)] tabular-nums shrink-0 mr-1 md:group-hover:hidden md:group-has-[:focus-visible]:hidden">
          {t("artist.plays", { count: formatPlays(track.view_count) })}
        </span>
      {/if}

      {#if favorite}
        <span class="text-[var(--fav-coral)] shrink-0 {pickerOpen ? 'hidden' : 'md:group-hover:hidden md:group-has-[:focus-visible]:hidden'}" aria-hidden="true">
          <Heart size={14} fill="currentColor" />
        </span>
      {/if}

      {#if displayDuration != null}
        <span class="text-[12px] text-[var(--text-muted)] tabular-nums shrink-0 min-w-[34px] text-right {pickerOpen ? 'hidden' : 'md:group-hover:hidden md:group-has-[:focus-visible]:hidden'}">
          {formatDuration(displayDuration)}
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
        : 'hidden md:group-hover:flex md:group-has-[:focus-visible]:flex'}"
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

    <!-- Mobile compact menu trigger (opens native Bottom Sheet context menu) -->
    <button
      type="button"
      class="md:hidden icon-btn !w-8 !h-8 shrink-0 text-[var(--text-muted)]"
      aria-label="Track actions"
      onclick={() => contextMenu.openTrackSheet(track, { onPlay, onRemove, removeLabel })}
    >
      <Ellipsis size={16} />
    </button>
  </div>

  {#if pickerOpen}
    <button
      type="button"
      aria-label="Close playlist picker"
      class="fixed inset-0 z-30 cursor-default bg-transparent"
      onclick={() => (pickerOpen = false)}
    ></button>
    <div
      id={pickerId}
      transition:fly={{ y: -6, duration: 150, easing: cubicOut }}
      class="absolute right-2 top-full mt-1 z-40 w-[236px] rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_18px_44px_rgba(0,0,0,0.6)] p-1.5 select-none"
    >
      <div class="flex items-center justify-between px-2.5 pt-1 pb-1.5 mb-1 border-b border-white/[0.08]">
        <span class="text-[11px] font-bold uppercase tracking-[0.06em] text-[var(--text-muted)]">
          {t("collection.add_to_playlist").replace(/:$/, "")}
        </span>
        <button
          type="button"
          class="w-5 h-5 rounded-full flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-white/10 transition-colors"
          aria-label="Close"
          onclick={() => (pickerOpen = false)}
        >
          <X size={12} />
        </button>
      </div>

      {#if player.userPlaylists.length > 0}
        <div class="max-h-44 overflow-y-auto scroll-y space-y-0.5 mb-1">
          {#each player.userPlaylists as playlist (playlist.id)}
            {@const alreadyIn = playlist.tracks.some((tr) => tr.id === track.id)}
            <button
              type="button"
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-xl text-[13px] font-medium text-left hover:bg-white/[0.08] transition-colors"
              onclick={() => {
                player.addTrackToPlaylist(playlist.id, track);
                pickerOpen = false;
              }}
            >
              <span
                class="w-6 h-6 rounded-lg flex items-center justify-center shrink-0 text-white"
                style="background: {playlist.color_hex || '#10b981'};"
              >
                <PlaylistIcon name={playlist.icon_name} size={13} />
              </span>
              <span class="truncate flex-1">{playlist.name}</span>
              {#if alreadyIn}
                <Check size={14} class="text-[var(--accent-bright)] shrink-0" />
              {/if}
            </button>
          {/each}
        </div>
      {/if}

      <button
        type="button"
        class="w-full flex items-center gap-2 px-2.5 py-2 rounded-xl text-[13px] font-semibold text-[var(--accent-bright)] hover:bg-[var(--accent-soft)] transition-colors"
        onclick={() => {
          pickerOpen = false;
          player.playlistDraftTrack = track;
        }}
      >
        <Plus size={14} />
        <span>{t("collection.new_playlist")}</span>
      </button>
    </div>
  {/if}
</div>

<style>
  .eq-bars {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 2.5px;
    height: 16px;
  }

  .eq-bar {
    width: 2.5px;
    height: 15px;
    border-radius: 999px;
    background: currentColor;
    transform-origin: center;
    will-change: transform;
    animation: eq-bounce 0.72s ease-in-out infinite alternate;
  }

  .eq-bar:nth-child(1) {
    animation-duration: 0.74s;
    animation-delay: -0.15s;
  }

  .eq-bar:nth-child(2) {
    animation-duration: 0.56s;
    animation-delay: -0.42s;
  }

  .eq-bar:nth-child(3) {
    animation-duration: 0.82s;
    animation-delay: -0.28s;
  }

  .eq-bar:nth-child(4) {
    animation-duration: 0.64s;
    animation-delay: -0.52s;
  }

  @keyframes eq-bounce {
    0% {
      transform: scaleY(0.24);
    }
    50% {
      transform: scaleY(0.95);
    }
    100% {
      transform: scaleY(0.38);
    }
  }
</style>
