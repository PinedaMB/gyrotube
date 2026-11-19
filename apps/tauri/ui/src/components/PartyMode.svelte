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
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";

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
      lineRefs[idx]?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  });

  function markUserScroll() {
    autoScrollPaused = true;
  }

  function resumeSync() {
    autoScrollPaused = false;
    const idx = player.activeLyricIndex;
    if (idx != null) {
      lineRefs[idx]?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  }

  function lineOpacity(idx: number): number {
    if (player.activeLyricIndex == null) return 0.6;
    if (idx === player.activeLyricIndex) return 1.0;
    const diff = Math.abs(idx - player.activeLyricIndex);
    if (diff === 1) return 0.36;
    if (diff === 2) return 0.2;
    return 0.11;
  }

  function isFarLine(idx: number): boolean {
    if (player.activeLyricIndex == null) return false;
    return Math.abs(idx - player.activeLyricIndex) >= 2;
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
  style="background-image: radial-gradient(ellipse at 26% 36%, rgba(var(--ambient-rgb), 0.34) 0%, rgba(var(--ambient-rgb), 0.1) 42%, transparent 74%);"
>
  <!-- Top bar -->
  <header class="shrink-0 flex items-center justify-between px-7 py-5">
    <div class="flex items-center gap-3 min-w-0">
      <span
        class="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/50"
      >
        <PartyPopper size={14} />
        Party Mode
      </span>
      <span class="text-[13.5px] font-medium text-[var(--text-muted)] truncate">
        {player.sourceName}
      </span>
    </div>

    <button
      class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full text-[13px] font-semibold bg-white/8 hover:bg-white/14 border border-white/10 transition-colors"
      onclick={() => (player.partyMode = false)}
    >
      <X size={15} />
      Exit
    </button>
  </header>

  <!-- Main 2-column stage -->
  <div class="flex-1 min-h-0 grid grid-cols-1 lg:grid-cols-[440px_minmax(0,1fr)] gap-10 px-10 pb-8 max-w-[1440px] w-full mx-auto">
    <!-- Left column: Art + Info + Controls + Up Next -->
    <div class="flex flex-col items-center justify-center gap-5">
      {#key player.currentTrack?.id ?? "none"}
        <div in:scale={{ start: 0.94, duration: 280, easing: cubicOut }}>
          <CoverImage
            class="party-cover !w-[292px] !h-[292px] !rounded-[22px] border border-white/12 shadow-[0_26px_64px_rgba(0,0,0,0.58)]"
            src={player.currentTrack?.cover_large_url}
            fallbackSrc={player.currentTrack?.cover_url}
            alt={player.currentTrack?.title ?? ""}
          />
        </div>
      {/key}

      <div class="w-[340px] text-center space-y-1">
        <h1 class="text-[28px] font-black tracking-[-0.03em] leading-tight line-clamp-2">
          {player.currentTrack?.title ?? "Nothing playing"}
        </h1>
        <p class="text-[16px] font-medium text-[var(--text-muted)] truncate">
          {player.currentTrack?.artist ?? ""}
        </p>
      </div>

      <!-- Interactive progress bar -->
      <div class="w-[340px] space-y-2.5">
        <div
          class="h-1.5 w-full rounded-full bg-white/15 cursor-pointer overflow-hidden group"
          role="slider"
          tabindex="0"
          aria-label="Seek"
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
            class="icon-btn !w-9 !h-9 transition-transform active:scale-90 {isFav
              ? 'text-[var(--coral)]'
              : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
            disabled={!player.currentTrack}
            title={isFav ? "Remove from favorites" : "Add to favorites"}
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
          class="icon-btn !w-10 !h-10 relative {player.shuffle
            ? 'active'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          title="Shuffle"
          onclick={() => player.toggleShuffle()}
        >
          <Shuffle size={18} />
          {#if player.shuffle}
            <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-1"></span>
          {/if}
        </button>

        <button
          class="icon-btn !w-11 !h-11"
          disabled={!player.currentTrack}
          title="Previous"
          onclick={() => void player.prev()}
        >
          <SkipBack size={22} fill="currentColor" />
        </button>

        <button
          class="play-circle !w-[62px] !h-[62px]"
          disabled={!player.currentTrack}
          title="Play / Pause"
          onclick={() => void player.togglePlay()}
        >
          {#if player.isPlaying}
            <Pause size={24} fill="currentColor" />
          {:else}
            <Play size={24} fill="currentColor" class="translate-x-[1.5px]" />
          {/if}
        </button>

        <button
          class="icon-btn !w-11 !h-11"
          disabled={!player.currentTrack}
          title="Next"
          onclick={() => void player.next()}
        >
          <SkipForward size={22} fill="currentColor" />
        </button>

        <button
          class="icon-btn !w-10 !h-10 relative {player.repeat
            ? 'active'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          title="Repeat"
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
          class="w-[340px] flex items-center gap-3 p-3 rounded-2xl bg-white/[0.05] hover:bg-white/[0.08] border border-white/10 text-left transition-colors"
          onclick={() => void player.next()}
        >
          <CoverImage class="up-next-cover !w-11 !h-11 !rounded-lg" src={player.nextTrack.cover_url} alt="" />
          <div class="min-w-0 flex-1">
            <span class="block text-[10px] font-extrabold uppercase tracking-[0.1em] text-[var(--text-muted)]">
              Up Next
            </span>
            <span class="block text-[13.5px] font-bold truncate">{player.nextTrack.title}</span>
            <span class="block text-[12px] text-[var(--text-muted)] truncate">{player.nextTrack.artist}</span>
          </div>
        </button>
      {/if}
    </div>

    <!-- Right column: Cinema-grade lyrics -->
    <div class="min-h-0 flex flex-col">
      <div class="shrink-0 flex items-center gap-2.5 pb-2">
        <span class="text-[13px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
          Lyrics
        </span>
        {#if player.lyrics.type === "Synced"}
          <span
            class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"
          >
            <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span>
            Synced
          </span>
        {:else if player.lyrics.type === "Plain"}
          <span class="px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-white/8 text-[var(--text-muted)]">
            Static
          </span>
        {/if}
      </div>

      {#if player.lyrics.type === "Synced"}
        <div class="relative flex-1 min-h-0 flex flex-col">
          <div
            role="region"
            aria-label="Lyrics"
            class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto py-[36vh] pr-8 pl-4 flex flex-col gap-8"
            onwheel={markUserScroll}
            ontouchmove={markUserScroll}
          >
            {#each player.lyrics.lines as line, i}
              {@const active = player.activeLyricIndex === i}
              {@const prevLine = i > 0 ? player.lyrics.lines[i - 1] : null}
              {@const stanzaBreak = prevLine != null && line.time_secs - prevLine.time_secs >= 5.5}
              <button
                bind:this={lineRefs[i]}
                class="party-line text-left origin-left {active ? 'active-line' : ''} {isFarLine(i) ? 'far-line' : ''} {stanzaBreak ? 'mt-5' : ''}"
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
              transition:scale={{ start: 0.88, duration: 180, easing: cubicOut }}
              class="absolute bottom-6 left-1/2 -translate-x-1/2 inline-flex items-center gap-2 px-4 py-2 rounded-full text-[13px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_10px_28px_rgba(0,0,0,0.5)] transition-all active:scale-95"
              onclick={resumeSync}
            >
              <LocateFixed size={15} />
              Sync
            </button>
          {/if}
        </div>
      {:else if player.lyrics.type === "Plain"}
        <div class="party-lyrics-scroll flex-1 min-h-0 overflow-y-auto py-12 pr-6">
          <pre class="whitespace-pre-wrap font-sans text-[24px] font-bold leading-[1.6] tracking-[-0.02em] text-white/80">{player.lyrics.text}</pre>
        </div>
      {:else}
        <div class="flex-1 flex flex-col items-center justify-center text-center gap-3 text-[var(--text-muted)]">
          <Music size={40} class="opacity-30" />
          <p class="text-[18px] font-semibold">No lyrics available for this track</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .party-lyrics-scroll {
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent 0%, black 14%, black 86%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 14%, black 86%, transparent 100%);
  }

  .party-lyrics-scroll::-webkit-scrollbar {
    display: none;
  }

  .party-line {
    position: relative;
    font-size: clamp(22px, 2.1vw, 29px);
    font-weight: 700;
    line-height: 1.38;
    letter-spacing: -0.025em;
    color: var(--text);
    padding: 6px 0;
    transition:
      opacity 380ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 380ms cubic-bezier(0.22, 1, 0.36, 1),
      font-size 380ms cubic-bezier(0.22, 1, 0.36, 1),
      color 380ms cubic-bezier(0.22, 1, 0.36, 1),
      filter 380ms cubic-bezier(0.22, 1, 0.36, 1),
      text-shadow 380ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .party-line.far-line {
    filter: blur(0.6px);
  }

  .party-line:hover {
    opacity: 0.92 !important;
    filter: none;
  }

  .party-line.active-line {
    font-size: clamp(32px, 3.2vw, 44px);
    font-weight: 900;
    line-height: 1.24;
    color: #ffffff;
    transform: translateX(6px);
    filter: none;
    text-shadow:
      0 0 36px rgba(var(--ambient-rgb), 0.72),
      0 4px 24px rgba(0, 0, 0, 0.45);
  }
</style>
