<script lang="ts">
  import {
    ChevronRight,
    Disc3,
    Film,
    Music,
    Pause,
    Play,
    Repeat,
    Shuffle,
    SkipBack,
    SkipForward,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";
  import SegmentedControl, { type SegmentedOption } from "../SegmentedControl.svelte";
  import SyncedVideoPlayer from "../SyncedVideoPlayer.svelte";
  import SidebarActionPopovers from "./SidebarActionPopovers.svelte";

  type MediaMode = "song" | "video";

  interface Props {
    sleepRemainingSecs: number | null;
    onSetSleepTimer: (minutes: number | null) => void;
  }

  let { sleepRemainingSecs, onSetSleepTimer }: Props = $props();

  let seeking = $state(false);
  let seekDraft = $state(0);
  let resolvingArtist = $state<string | null>(null);

  let artistNames = $derived.by(() => {
    const raw = player.currentTrack?.artist?.trim();
    if (!raw) return [];
    const parts = raw
      .split(/\s*(?:,|feat\.?|ft\.?|featuring)\s+/i)
      .map((s) => s.trim())
      .filter(Boolean);
    return parts.length > 0 ? parts : [raw];
  });

  async function goToArtist(name: string, index: number) {
    const tr = player.currentTrack;
    if (!tr || resolvingArtist) return;
    resolvingArtist = name;
    try {
      await player.openArtistByNameOrId(index === 0 ? tr.artist_id : null, name);
    } finally {
      resolvingArtist = null;
    }
  }

  // Horizontal swipe (next/prev) & double-tap (±10s seek) gestures on cover stage
  let swipeX = $state(0);
  let swipeActive = false;
  let swipeStartX = 0;
  let swipeStartY = 0;
  let swipeLockedHorizontal = false;

  let lastTapAt = 0;
  let lastTapSide: "left" | "right" | null = null;
  let seekFeedback = $state<{ side: "left" | "right"; delta: number } | null>(null);
  let seekFeedbackTimer: number | null = null;

  function triggerSeekFeedback(side: "left" | "right") {
    const step = side === "right" ? 10 : -10;
    const target = Math.max(
      0,
      Math.min(player.durationSecs || 0, player.getExactPositionSecs() + step),
    );
    player.seek(target);
    const prevDelta = seekFeedback && seekFeedback.side === side ? seekFeedback.delta : 0;
    seekFeedback = { side, delta: prevDelta + step };
    if (seekFeedbackTimer !== null) window.clearTimeout(seekFeedbackTimer);
    seekFeedbackTimer = window.setTimeout(() => {
      seekFeedback = null;
      seekFeedbackTimer = null;
    }, 650);
  }

  function handleCoverTouchStart(e: TouchEvent) {
    if (e.touches.length !== 1 || !player.currentTrack) return;
    const t0 = e.touches[0];
    swipeActive = true;
    swipeLockedHorizontal = false;
    swipeStartX = t0.clientX;
    swipeStartY = t0.clientY;
    swipeX = 0;
  }

  function handleCoverTouchMove(e: TouchEvent) {
    if (!swipeActive || e.touches.length !== 1) return;
    const t0 = e.touches[0];
    const dx = t0.clientX - swipeStartX;
    const dy = t0.clientY - swipeStartY;
    if (!swipeLockedHorizontal) {
      if (Math.abs(dx) < 12 && Math.abs(dy) < 12) return;
      if (Math.abs(dx) > Math.abs(dy) * 1.25) {
        swipeLockedHorizontal = true;
      } else {
        // Vertical gesture — let SidebarPlayer handle swipe-down to close
        swipeActive = false;
        swipeX = 0;
        return;
      }
    }
    e.stopPropagation();
    swipeX = Math.max(-84, Math.min(84, dx * 0.72));
  }

  function handleCoverTouchEnd(e: TouchEvent) {
    if (!swipeActive) return;
    const dx = swipeX;
    const wasHorizontal = swipeLockedHorizontal;
    swipeActive = false;
    swipeLockedHorizontal = false;
    swipeX = 0;

    if (wasHorizontal) {
      e.stopPropagation();
      if (dx < -44) {
        void player.next();
      } else if (dx > 44) {
        void player.prev();
      }
      return;
    }

    // Check for double-tap on left or right half of the cover stage
    const touch = e.changedTouches[0];
    const el = e.currentTarget as HTMLElement | null;
    if (!touch || !el) return;
    const rect = el.getBoundingClientRect();
    const side: "left" | "right" = touch.clientX < rect.left + rect.width / 2 ? "left" : "right";
    const now = performance.now();
    if (now - lastTapAt < 320 && lastTapSide === side) {
      lastTapAt = 0;
      lastTapSide = null;
      triggerSeekFeedback(side);
    } else {
      lastTapAt = now;
      lastTapSide = side;
    }
  }

  let modeOptions = $derived<SegmentedOption<MediaMode>[]>([
    {
      value: "song",
      label: t("player.mode_song"),
      icon: Music,
      disabled: player.videoMode && !player.canSwitchVideoMode,
    },
    {
      value: "video",
      label: t("player.mode_video"),
      icon: Film,
      disabled: player.isOffline || (!player.videoMode && !player.canSwitchVideoMode),
    },
  ]);

  let displayPos = $derived(seeking ? seekDraft : player.positionSecs);
  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (displayPos / player.durationSecs) * 100))
      : 0,
  );
</script>

<div in:fade={{ duration: 180 }} class="flex-1 flex flex-col items-center justify-between py-3 gap-4">
  <!-- Song / Video mode toggle -->
  {#if player.currentTrack}
    <SegmentedControl
      options={modeOptions}
      value={player.videoMode ? "video" : "song"}
      size="sm"
      ariaLabel={t("player.mode_song")}
      onchange={(mode) => player.setVideoMode(mode === "video")}
    />
  {/if}

  <!-- Centered large cover art or synced video -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="relative flex-1 flex items-center justify-center w-full py-1"
    ontouchstart={handleCoverTouchStart}
    ontouchmove={handleCoverTouchMove}
    ontouchend={handleCoverTouchEnd}
    ontouchcancel={() => {
      swipeActive = false;
      swipeLockedHorizontal = false;
      swipeX = 0;
    }}
  >
    {#if player.videoMode && player.currentTrack}
      <SyncedVideoPlayer class="w-full aspect-video rounded-2xl" />
    {:else}
      {#key player.currentTrack?.id ?? "none"}
        <div
          in:scale={{ start: 0.94, duration: 260, easing: cubicOut }}
          style={swipeX !== 0
            ? `transform: translateX(${swipeX}px); opacity: ${Math.max(0.55, 1 - Math.abs(swipeX) / 180)}; transition: none;`
            : "transition: transform 180ms cubic-bezier(0.22, 1, 0.36, 1), opacity 180ms ease;"}
        >
          <CoverImage
            class="cover-large !w-[264px] !h-[264px] !rounded-2xl border border-[var(--border)] shadow-[0_22px_50px_rgba(15,23,42,0.28)] transition-transform duration-300 hover:scale-[1.015]"
            src={player.trackCoverUrl(player.currentTrack, true)}
            fallbackSrc={player.currentTrack ? (player.offlineCoverUrl(player.currentTrack.id) ?? player.currentTrack.cover_url) : null}
            alt={player.currentTrack?.title ?? ""}
            eager={true}
          />
        </div>
      {/key}
    {/if}

    {#if seekFeedback}
      <div
        transition:fade={{ duration: 120 }}
        class="pointer-events-none absolute top-1/2 -translate-y-1/2 px-3.5 py-2 rounded-full bg-black/75 text-white text-[13px] font-extrabold numeric shadow-lg border border-white/15 {seekFeedback.side === 'left'
          ? 'left-4'
          : 'right-4'}"
      >
        {seekFeedback.delta > 0 ? `+${seekFeedback.delta}s` : `${seekFeedback.delta}s`}
      </div>
    {/if}
  </div>

  <!-- Lower controls stack -->
  <div class="w-full flex flex-col items-center gap-4">
    <!-- Track metadata (artist & album interactive) -->
    <div class="w-full flex flex-col items-center text-center px-1 gap-1.5">
      <h2 class="text-[19px] font-extrabold leading-tight tracking-tight line-clamp-2">
        {player.currentTrack?.title ?? t("player.nothing_playing")}
      </h2>
      {#if artistNames.length > 0}
        <div class="flex flex-wrap items-center justify-center gap-x-0.5 gap-y-0.5 max-w-full">
          {#each artistNames as name, i (`${name}:${i}`)}
            <button
              type="button"
              disabled={resolvingArtist !== null || player.isOffline}
              class="group inline-flex items-center justify-center gap-1 max-w-full px-2 py-0.5 rounded-full text-[14.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--accent-bright)] hover:bg-[var(--surface-hover)] active:bg-[var(--surface-hover)] transition-colors"
              title={t("ctx.go_artist")}
              onclick={() => void goToArtist(name, i)}
            >
              {#if resolvingArtist === name}
                <span class="w-3 h-3 rounded-full border-2 border-current/30 border-t-current animate-spin shrink-0"></span>
              {/if}
              <span class="truncate">{name}</span>
              {#if artistNames.length === 1}
                <ChevronRight
                  size={13}
                  class="shrink-0 opacity-0 -ml-0.5 group-hover:opacity-80 group-hover:ml-0 transition-all"
                />
              {/if}
            </button>{#if i < artistNames.length - 1}<span class="text-[14px] font-semibold text-[var(--text-muted)]/60 -ml-1 mr-0.5 select-none">,</span>{/if}
          {/each}
        </div>
      {:else}
        <p class="text-[14px] font-medium text-[var(--text-muted)] truncate">
          {t("player.select_song")}
        </p>
      {/if}
      {#if player.currentTrack?.album}
        {#if player.currentTrack.album_id}
          <button
            type="button"
            disabled={player.isOffline}
            class="group inline-flex items-center justify-center gap-1.5 max-w-full px-3 py-1 rounded-full text-[12px] font-medium bg-[var(--surface)] border border-[var(--border)] text-[var(--text-muted)] hover:bg-[var(--accent-soft)] hover:border-[var(--accent-border)] hover:text-[var(--accent-bright)] transition-colors"
            title={t("player.album_prefix", { album: player.currentTrack.album })}
            onclick={() => {
              const tr = player.currentTrack;
              if (!tr?.album_id || !tr.album) return;
              player.openAlbum(tr.album_id, tr.album, tr.cover_large_url ?? tr.cover_url);
            }}
          >
            <Disc3 size={12} class="shrink-0 opacity-75 group-hover:opacity-100" />
            <span class="truncate">{player.currentTrack.album}</span>
            <ChevronRight
              size={12}
              class="shrink-0 opacity-50 group-hover:opacity-100 group-hover:translate-x-0.5 transition-all"
            />
          </button>
        {:else}
          <span class="inline-flex items-center justify-center gap-1.5 max-w-full px-2.5 py-0.5 text-[12px] text-[var(--text-muted)]/80">
            <Disc3 size={12} class="shrink-0 opacity-70" />
            <span class="truncate">{player.currentTrack.album}</span>
          </span>
        {/if}
      {/if}
    </div>

    <!-- 5-icon action row + Volume/Timer popovers -->
    <SidebarActionPopovers {sleepRemainingSecs} {onSetSleepTimer} />

    <!-- Seek bar + Time / Quality -->
    <div class="w-full space-y-2.5">
      <input
        class="sidebar-seek w-full"
        type="range"
        min="0"
        max={Math.max(1, player.durationSecs)}
        step="0.5"
        value={displayPos}
        disabled={!player.currentTrack}
        aria-label="Seek"
        style="background: linear-gradient(to right, var(--accent-bright) {progressPct}%, var(--control-track) {progressPct}%);"
        onpointerdown={() => {
          seeking = true;
          seekDraft = player.positionSecs;
        }}
        oninput={(e) => {
          seekDraft = Number((e.currentTarget as HTMLInputElement).value);
        }}
        onchange={(e) => {
          const target = Number((e.currentTarget as HTMLInputElement).value);
          seeking = false;
          player.seek(target);
        }}
      />
      <div class="flex items-center justify-between text-[12px] text-[var(--text-muted)] numeric">
        <span>{formatDuration(displayPos)}</span>
        <span class="px-3 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]">
          {player.qualityLabel}
        </span>
        <span>{formatDuration(player.durationSecs)}</span>
      </div>
    </div>

    <!-- Transport controls -->
    <div class="w-full flex items-center justify-center gap-4 pt-1 pb-1">
      <button
        type="button"
        class="icon-btn relative {player.shuffle
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={t("player.shuffle")}
        onclick={() => player.toggleShuffle()}
      >
        <Shuffle size={17} />
        {#if player.shuffle}
          <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>
        {/if}
      </button>

      <button
        type="button"
        class="icon-btn !w-9 !h-9"
        disabled={!player.currentTrack}
        title={t("player.previous")}
        onclick={() => void player.prev()}
      >
        <SkipBack size={20} fill="currentColor" />
      </button>

      <button
        type="button"
        class="play-circle !w-[56px] !h-[56px] shrink-0"
        disabled={!player.currentTrack}
        title={t("player.play_pause")}
        onclick={() => void player.togglePlay()}
      >
        {#if player.isLoadingAudio}
          <span class="w-5 h-5 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>
        {:else if player.isPlaying}
          <Pause size={22} fill="currentColor" />
        {:else}
          <Play size={22} fill="currentColor" class="translate-x-[1.5px]" />
        {/if}
      </button>

      <button
        type="button"
        class="icon-btn !w-9 !h-9"
        disabled={!player.currentTrack}
        title={t("player.next")}
        onclick={() => void player.next()}
      >
        <SkipForward size={20} fill="currentColor" />
      </button>

      <button
        type="button"
        class="icon-btn relative {player.repeat
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={t("player.repeat")}
        onclick={() => player.toggleRepeat()}
      >
        <Repeat size={17} />
        {#if player.repeat}
          <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  :global(.sidebar-seek) {
    height: 4px;
  }

  :global(.sidebar-seek::-webkit-slider-thumb) {
    width: 17px;
    height: 17px;
    background: #ffffff;
    border: 2px solid var(--accent);
    box-shadow: 0 2px 8px rgba(15, 23, 42, 0.28);
  }

  :global(.sidebar-seek::-moz-range-thumb) {
    width: 17px;
    height: 17px;
    background: #ffffff;
    border: 2px solid var(--accent);
    box-shadow: 0 2px 8px rgba(15, 23, 42, 0.28);
  }
</style>
