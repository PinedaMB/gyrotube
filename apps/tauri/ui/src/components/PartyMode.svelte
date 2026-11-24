<script lang="ts">
  import {
    Heart,
    LocateFixed,
    Music,
    PartyPopper,
    Pause,
    Play,
    Repeat,
    Shuffle,
    SkipBack,
    SkipForward,
    X,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { scale } from "svelte/transition";
  import { t } from "../lib/i18n.svelte";
  import {
    computeLyricOpacity,
    isWaitingForFirstLyric,
    parsePlainStanzas,
  } from "../lib/lyricsFormat";
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";
  import SyncedVideoPlayer from "./SyncedVideoPlayer.svelte";

  let lineRefs = $state<Record<number, HTMLButtonElement>>({});
  let autoScrollPaused = $state(false);
  let lastTrackId = $state<string | null>(null);

  let isFav = $derived(player.currentTrack ? player.isFavorite(player.currentTrack.id) : false);
  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (player.positionSecs / player.durationSecs) * 100))
      : 0,
  );

  $effect(() => {
    const tid = player.currentTrack?.id ?? null;
    if (tid !== lastTrackId) {
      lastTrackId = tid;
      autoScrollPaused = false;
    }
  });

  $effect(() => {
    const idx = player.activeLyricIndex;
    if (idx != null && player.partyMode && !autoScrollPaused) {
      const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
      lineRefs[idx]?.scrollIntoView({ behavior: reduceMotion ? "instant" : "smooth", block: "center" });
    }
  });

  function markUserScroll() {
    autoScrollPaused = true;
  }

  function resumeSync() {
    autoScrollPaused = false;
  }

  let waitingForFirstLyric = $derived(
    isWaitingForFirstLyric(player.lyrics, player.activeLyricIndex, player.positionSecs),
  );

  let plainStanzas = $derived(
    player.lyrics.type === "Plain" ? parsePlainStanzas(player.lyrics.text).stanzas : [],
  );

  function lineOpacity(idx: number): number {
    return computeLyricOpacity(idx, player.activeLyricIndex);
  }

  function handleProgressClick(e: MouseEvent) {
    if (!player.currentTrack || player.durationSecs <= 0) return;
    const rect = (e.currentTarget as HTMLDivElement).getBoundingClientRect();
    const ratio = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    player.seek(ratio * player.durationSecs);
  }
</script>

<div
  class="fixed inset-0 z-50 flex flex-col bg-[var(--bg)] text-[var(--text)] select-none overflow-hidden"
  style="background-image: radial-gradient(ellipse at 26% 36%, rgba(var(--ambient-rgb), var(--ambient-party-a)) 0%, rgba(var(--ambient-rgb), var(--ambient-party-b)) 42%, transparent 74%);"
>
  <!-- Top bar -->
  <header
    class="shrink-0 flex items-center justify-between gap-2 px-4 sm:px-7 pt-[max(0.75rem,env(safe-area-inset-top))] pb-3 lg:py-5"
  >
    <div class="flex items-center gap-2.5 sm:gap-3 min-w-0">
      <span
        class="inline-flex items-center gap-1.5 px-3 sm:px-3.5 py-1.5 rounded-full text-[11.5px] sm:text-[12px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/50 shrink-0"
      >
        <PartyPopper size={14} />
        {t("party.badge")}
      </span>
      <span class="hidden sm:inline text-[13.5px] font-medium text-[var(--text-muted)] truncate">
        {player.sourceName === "Not playing" ? t("player.not_playing") : player.sourceName}
      </span>
    </div>

    <button
      type="button"
      class="inline-flex items-center gap-1.5 px-3.5 sm:px-4 py-1.5 rounded-full text-[12.5px] sm:text-[13px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-colors shrink-0"
      onclick={() => (player.partyMode = false)}
    >
      <X size={15} />
      {t("party.exit")}
    </button>
  </header>

  <!-- Mobile compact now-playing header -->
  {#if player.currentTrack}
    <div class="flex lg:hidden shrink-0 items-center gap-3 px-4 pb-2">
      <CoverImage
        class="track-cover !w-12 !h-12 !rounded-[10px] shrink-0 shadow-md"
        src={player.currentTrack.cover_url}
        fallbackSrc={player.currentTrack.cover_large_url}
        alt={player.currentTrack.title}
        eager={true}
      />
      <div class="min-w-0 flex-1">
        <h1 class="text-[16px] font-extrabold tracking-tight truncate leading-snug">
          {player.currentTrack.title}
        </h1>
        <p class="text-[13px] font-medium text-[var(--text-muted)] truncate">
          {player.currentTrack.artist}
        </p>
      </div>
      <button
        type="button"
        class="icon-btn !w-10 !h-10 shrink-0 transition-transform active:scale-90 {isFav
          ? 'text-[var(--coral)]'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={isFav ? t("player.favorite_remove") : t("player.favorite_add")}
        onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
      >
        <Heart size={19} fill={isFav ? "currentColor" : "none"} />
      </button>
    </div>
  {/if}

  <!-- Main stage: full-height lyrics on mobile, 2-column stage on desktop -->
  <div class="flex-1 min-h-0 overflow-hidden flex flex-col lg:grid lg:grid-cols-[440px_minmax(0,1fr)] lg:gap-10 px-4 sm:px-8 lg:px-10 lg:pb-8 max-w-[1440px] w-full mx-auto">
    <!-- Desktop left column: Art / Synced Video + Info + Controls + Up Next -->
    <div class="hidden lg:flex flex-col items-center justify-center gap-5">
      {#if player.videoMode && player.currentTrack}
        <SyncedVideoPlayer class="w-full max-w-[420px] aspect-video rounded-[20px]" />
      {:else}
        {#key player.currentTrack?.id ?? "none"}
          <div in:scale={{ start: 0.94, duration: 280, easing: cubicOut }}>
            <CoverImage
              class="party-cover !w-[292px] !h-[292px] !rounded-[22px] border border-white/12 shadow-[0_26px_64px_rgba(0,0,0,0.58)]"
              src={player.currentTrack?.cover_large_url}
              fallbackSrc={player.currentTrack?.cover_url}
              alt={player.currentTrack?.title ?? ""}
              eager={true}
            />
          </div>
        {/key}
      {/if}

      <div class="w-full max-w-[340px] text-center space-y-1">
        <h1 class="text-[28px] font-black tracking-[-0.03em] leading-tight line-clamp-2">
          {player.currentTrack?.title ?? t("player.nothing_playing")}
        </h1>
        <p class="text-[16px] font-medium text-[var(--text-muted)] truncate">
          {player.currentTrack?.artist ?? ""}
        </p>
      </div>

      <!-- Interactive progress bar -->
      <div class="w-full max-w-[340px] space-y-2.5">
        <div
          class="h-1.5 w-full rounded-full bg-[var(--control-track)] cursor-pointer overflow-hidden group"
          role="slider"
          tabindex="0"
          aria-label={t("a11y.seek")}
          aria-valuemin={0}
          aria-valuemax={Math.round(player.durationSecs)}
          aria-valuenow={Math.round(player.positionSecs)}
          onclick={handleProgressClick}
          onkeydown={(e) => {
            if (e.key === "ArrowRight") player.seek(player.positionSecs + 5);
            else if (e.key === "ArrowLeft") player.seek(Math.max(0, player.positionSecs - 5));
          }}
        >
          <div
            class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-150 ease-linear"
            style="width: {progressPct}%"
          ></div>
        </div>

        <div class="flex items-center justify-between text-[12.5px] font-medium text-[var(--text-muted)] numeric">
          <span>{formatDuration(player.positionSecs)}</span>
          <button
            type="button"
            class="icon-btn !w-9 !h-9 transition-transform active:scale-90 {isFav
              ? 'text-[var(--coral)]'
              : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
            disabled={!player.currentTrack}
            title={isFav ? t("player.favorite_remove") : t("player.favorite_add")}
            onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
          >
            <Heart size={18} fill={isFav ? "currentColor" : "none"} />
          </button>
          <span>{formatDuration(player.durationSecs)}</span>
        </div>
      </div>

      <!-- Transport controls -->
      <div class="flex items-center justify-center gap-4">
        <button
          type="button"
          class="icon-btn !w-10 !h-10 relative {player.shuffle
            ? 'active'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          title={t("player.shuffle")}
          onclick={() => player.toggleShuffle()}
        >
          <Shuffle size={18} />
          {#if player.shuffle}
            <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-1"></span>
          {/if}
        </button>

        <button
          type="button"
          class="icon-btn !w-11 !h-11"
          disabled={!player.currentTrack}
          title={t("player.previous")}
          onclick={() => void player.prev()}
        >
          <SkipBack size={22} fill="currentColor" />
        </button>

        <button
          type="button"
          class="play-circle !w-[62px] !h-[62px]"
          disabled={!player.currentTrack}
          title={t("player.play_pause")}
          onclick={() => void player.togglePlay()}
        >
          {#if player.isPlaying}
            <Pause size={24} fill="currentColor" />
          {:else}
            <Play size={24} fill="currentColor" class="translate-x-[1.5px]" />
          {/if}
        </button>

        <button
          type="button"
          class="icon-btn !w-11 !h-11"
          disabled={!player.currentTrack}
          title={t("player.next")}
          onclick={() => void player.next()}
        >
          <SkipForward size={22} fill="currentColor" />
        </button>

        <button
          type="button"
          class="icon-btn !w-10 !h-10 relative {player.repeat
            ? 'active'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          title={t("player.repeat")}
          onclick={() => player.toggleRepeat()}
        >
          <Repeat size={18} />
          {#if player.repeat}
            <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-1"></span>
          {/if}
        </button>
      </div>

      <!-- Up Next card -->
      {#if player.nextTrack}
        <button
          type="button"
          class="w-full max-w-[340px] flex items-center gap-3 p-3 rounded-2xl bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-left transition-colors"
          onclick={() => void player.next()}
        >
          <CoverImage class="up-next-cover !w-11 !h-11 !rounded-[8px]" src={player.nextTrack.cover_url} alt="" />
          <div class="min-w-0 flex-1">
            <span class="block text-[10px] font-extrabold uppercase tracking-[0.1em] text-[var(--text-muted)]">
              {t("party.up_next")}
            </span>
            <span class="block text-[13.5px] font-bold truncate">{player.nextTrack.title}</span>
            <span class="block text-[12px] text-[var(--text-muted)] truncate">{player.nextTrack.artist}</span>
          </div>
        </button>
      {/if}
    </div>

    <!-- Cinema-grade lyrics (full-height on mobile, right column on desktop) -->
    <div class="flex-1 min-h-0 flex flex-col pt-2 lg:pt-0">
      <div class="shrink-0 flex items-center gap-2.5 pb-1.5 lg:pb-2">
        <span class="text-[12px] lg:text-[13px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
          {t("lyrics.title")}
        </span>
        {#if player.lyrics.type === "Synced"}
          <span
            class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10.5px] lg:text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"
          >
            <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span>
            {t("lyrics.synced")}
          </span>
        {:else if player.lyrics.type === "Plain"}
          <span class="px-2.5 py-0.5 rounded-full text-[10.5px] lg:text-[11px] font-semibold bg-[var(--surface)] text-[var(--text-muted)]">
            {t("lyrics.static")}
          </span>
        {/if}
      </div>

      {#if player.lyricsLoading && player.currentTrack}
        <div class="flex-1 flex flex-col justify-center gap-5 lg:gap-6 py-8 lg:py-10 lg:pr-8 animate-pulse" aria-busy="true">
          {#each [68, 52, 80, 60, 44, 74] as w}
            <div class="h-6 lg:h-7 rounded-lg bg-[var(--skeleton)]" style="width: {w}%;"></div>
          {/each}
        </div>
      {:else if player.lyrics.type === "Synced"}
        <div class="relative flex-1 min-h-0 flex flex-col">
          <div
            role="region"
            aria-label={t("lyrics.title")}
            class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto overflow-x-hidden pt-10 pb-24 lg:pt-16 lg:pb-36 px-1 pr-4 lg:pr-10 flex flex-col gap-5 lg:gap-6"
            onwheel={markUserScroll}
            ontouchmove={markUserScroll}
          >
            {#if waitingForFirstLyric}
              <div class="flex items-center gap-2 pb-2" aria-hidden="true">
                <span class="party-intro-dot"></span>
                <span class="party-intro-dot" style="animation-delay: 180ms;"></span>
                <span class="party-intro-dot" style="animation-delay: 360ms;"></span>
              </div>
            {/if}

            {#each player.lyrics.lines as line, i}
              {@const active = player.activeLyricIndex === i}
              {@const prevLine = i > 0 ? player.lyrics.lines[i - 1] : null}
              {@const stanzaBreak = prevLine != null && line.time_secs - prevLine.time_secs >= 5.5}
              <button
                type="button"
                bind:this={lineRefs[i]}
                class="party-line text-left origin-left rounded-2xl {active ? 'active-line' : ''} {stanzaBreak ? 'mt-4 lg:mt-5' : ''}"
                style="opacity: {lineOpacity(i)};"
                onclick={() => {
                  autoScrollPaused = false;
                  player.seek(line.time_secs);
                }}
              >
                {line.text}
              </button>
            {/each}
          </div>

          {#if autoScrollPaused && player.activeLyricIndex != null}
            <button
              type="button"
              transition:scale={{ start: 0.88, duration: 180, easing: cubicOut }}
              class="absolute bottom-4 lg:bottom-6 left-1/2 -translate-x-1/2 inline-flex items-center gap-2 px-4 py-2 rounded-full text-[13px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_10px_28px_rgba(0,0,0,0.5)] transition-all active:scale-95"
              onclick={resumeSync}
            >
              <LocateFixed size={15} />
              {t("lyrics.sync_btn")}
            </button>
          {/if}
        </div>
      {:else if player.lyrics.type === "Plain"}
        <div
          role="region"
          aria-label={t("lyrics.title")}
          class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto overflow-x-hidden pt-8 pb-28 lg:pt-12 lg:pb-36 px-1 pr-3 lg:pr-8 space-y-6 lg:space-y-7"
        >
          {#each plainStanzas as stanza, sIdx (sIdx)}
            <div class="flex flex-col gap-2">
              {#each stanza as item, lIdx (`${sIdx}:${lIdx}`)}
                {#if item.isHeader}
                  <div class="text-[11.5px] lg:text-[12.5px] font-extrabold uppercase tracking-[0.12em] text-[var(--accent-bright)]/85 pt-1 pb-0.5 select-none">
                    {item.text}
                  </div>
                {:else}
                  <p class="party-static-line">
                    {item.text}
                  </p>
                {/if}
              {/each}
            </div>
          {/each}
        </div>
      {:else}
        <div class="flex-1 flex flex-col items-center justify-center text-center gap-3 text-[var(--text-muted)]">
          <Music size={36} class="opacity-30" />
          <p class="text-[16px] lg:text-[18px] font-semibold">{t("lyrics.none_track")}</p>
        </div>
      {/if}
    </div>
  </div>

  <!-- Mobile pinned bottom controls dock -->
  <div
    class="flex lg:hidden shrink-0 flex-col gap-3 px-5 pt-2"
    style="padding-bottom: max(1rem, env(safe-area-inset-bottom));"
  >
    <div class="space-y-1.5">
      <div
        class="h-2 w-full rounded-full bg-[var(--control-track)] cursor-pointer overflow-hidden"
        role="slider"
        tabindex="0"
        aria-label={t("a11y.seek")}
        aria-valuemin={0}
        aria-valuemax={Math.round(player.durationSecs)}
        aria-valuenow={Math.round(player.positionSecs)}
        onclick={handleProgressClick}
        onkeydown={(e) => {
          if (e.key === "ArrowRight") player.seek(player.positionSecs + 5);
          else if (e.key === "ArrowLeft") player.seek(Math.max(0, player.positionSecs - 5));
        }}
      >
        <div
          class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-150 ease-linear"
          style="width: {progressPct}%"
        ></div>
      </div>
      <div class="flex items-center justify-between text-[11.5px] font-medium text-[var(--text-muted)] numeric">
        <span>{formatDuration(player.positionSecs)}</span>
        <span>{formatDuration(player.durationSecs)}</span>
      </div>
    </div>

    <div class="flex items-center justify-between max-w-[340px] w-full mx-auto">
      <button
        type="button"
        class="icon-btn !w-10 !h-10 relative {player.shuffle
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={t("player.shuffle")}
        onclick={() => player.toggleShuffle()}
      >
        <Shuffle size={18} />
        {#if player.shuffle}
          <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-1"></span>
        {/if}
      </button>

      <button
        type="button"
        class="icon-btn !w-11 !h-11"
        disabled={!player.currentTrack}
        title={t("player.previous")}
        onclick={() => void player.prev()}
      >
        <SkipBack size={22} fill="currentColor" />
      </button>

      <button
        type="button"
        class="play-circle !w-14 !h-14"
        disabled={!player.currentTrack}
        title={t("player.play_pause")}
        onclick={() => void player.togglePlay()}
      >
        {#if player.isPlaying}
          <Pause size={22} fill="currentColor" />
        {:else}
          <Play size={22} fill="currentColor" class="translate-x-[1.5px]" />
        {/if}
      </button>

      <button
        type="button"
        class="icon-btn !w-11 !h-11"
        disabled={!player.currentTrack}
        title={t("player.next")}
        onclick={() => void player.next()}
      >
        <SkipForward size={22} fill="currentColor" />
      </button>

      <button
        type="button"
        class="icon-btn !w-10 !h-10 relative {player.repeat
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={t("player.repeat")}
        onclick={() => player.toggleRepeat()}
      >
        <Repeat size={18} />
        {#if player.repeat}
          <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-1"></span>
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  .party-lyrics-scroll {
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent 0%, black 9%, black 88%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 9%, black 88%, transparent 100%);
  }

  .party-lyrics-scroll::-webkit-scrollbar {
    display: none;
  }

  .party-line {
    position: relative;
    font-family: var(--lyrics-font);
    font-size: clamp(20px, 5.2vw, 34px);
    font-weight: 700;
    line-height: 1.38;
    letter-spacing: -0.02em;
    color: var(--text);
    padding: 5px 10px;
    will-change: transform, opacity;
    transition:
      opacity 280ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 280ms cubic-bezier(0.22, 1, 0.36, 1),
      background-color 180ms ease,
      color 260ms ease;
  }

  .party-line:hover {
    opacity: 0.94 !important;
    background: color-mix(in srgb, var(--surface-hover) 50%, transparent);
  }

  .party-line.active-line {
    transform: scale(1.035);
    color: var(--text);
  }

  .party-static-line {
    font-family: var(--lyrics-font);
    font-size: clamp(19px, 4.4vw, 25px);
    font-weight: 700;
    line-height: 1.48;
    letter-spacing: -0.02em;
    color: color-mix(in srgb, var(--text) 88%, transparent);
    margin: 0;
  }

  .party-intro-dot {
    width: 9px;
    height: 9px;
    border-radius: 9999px;
    background: var(--accent-bright);
    opacity: 0.35;
    animation: partyIntroPulse 1.2s ease-in-out infinite;
  }

  @keyframes partyIntroPulse {
    0%, 100% {
      opacity: 0.25;
      transform: scale(0.85);
    }
    50% {
      opacity: 1;
      transform: scale(1.15);
    }
  }
</style>
