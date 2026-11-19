<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";

  type ActionPopover = "volume" | "timer" | null;

  let lyricRefs = $state<Record<number, HTMLButtonElement>>({});
  let dragFrom = $state<number | null>(null);
  let dragOverIdx = $state<number | null>(null);
  let seeking = $state(false);
  let seekDraft = $state(0);
  let openPopover = $state<ActionPopover>(null);
  let prevVolume = $state(1);
  let startingRadio = $state(false);

  // Sleep timer state
  let sleepRemainingSecs = $state<number | null>(null);
  let sleepInterval: number | undefined;

  let displayPos = $derived(seeking ? seekDraft : player.positionSecs);
  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (displayPos / player.durationSecs) * 100))
      : 0,
  );
  let volumePct = $derived(Math.round(player.volume * 100));
  let isFav = $derived(player.currentTrack ? player.isFavorite(player.currentTrack.id) : false);

  $effect(() => {
    const idx = player.activeLyricIndex;
    if (idx != null && player.sidebarTab === "lyrics") {
      const el = lyricRefs[idx];
      el?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  });

  function togglePopover(kind: Exclude<ActionPopover, null>) {
    openPopover = openPopover === kind ? null : kind;
  }

  function toggleMute() {
    if (player.volume > 0.01) {
      prevVolume = player.volume;
      player.setVolume(0);
    } else {
      player.setVolume(prevVolume > 0.05 ? prevVolume : 0.8);
    }
  }

  function setSleepTimer(minutes: number | null) {
    window.clearInterval(sleepInterval);
    sleepInterval = undefined;
    if (!minutes) {
      sleepRemainingSecs = null;
      openPopover = null;
      return;
    }
    sleepRemainingSecs = minutes * 60;
    openPopover = null;
    player.showToast(`Sleep timer set for ${minutes} min`);
    sleepInterval = window.setInterval(() => {
      if (sleepRemainingSecs == null) return;
      sleepRemainingSecs -= 1;
      if (sleepRemainingSecs <= 0) {
        window.clearInterval(sleepInterval);
        sleepInterval = undefined;
        sleepRemainingSecs = null;
        if (player.isPlaying) void player.togglePlay();
        player.showToast("Sleep timer ended — playback paused");
      }
    }, 1000);
  }
</script>

<aside
  class="w-[388px] shrink-0 h-full flex flex-col border-r border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden relative select-none"
  style="background-image: linear-gradient(180deg, rgba(var(--ambient-rgb), 0.22) 0%, rgba(var(--ambient-rgb), 0.08) 52%, transparent 100%);"
>
  <!-- Top context bar -->
  <div class="shrink-0 flex items-center justify-between gap-2 px-6 pt-4 pb-2">
    <div class="min-w-0 flex flex-col leading-tight">
      <span class="text-[11.5px] font-medium text-[var(--text-muted)]">
        Playing from
      </span>
      <span class="text-[13px] font-semibold truncate">{player.sourceName}</span>
    </div>
    <button
      class="icon-btn shrink-0 text-[var(--text-muted)] hover:text-[var(--text)]"
      title="Collapse player"
      onclick={() => {
        player.sidebarOpen = false;
        void player.persistSession();
      }}
    >
      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="15 18 9 12 15 6" />
      </svg>
    </button>
  </div>

  <!-- Main scrollable area -->
  <div class="flex-1 min-h-0 overflow-y-auto scroll-y px-7 pb-4 flex flex-col">
    {#if player.sidebarTab === "player"}
      <div class="flex-1 flex flex-col items-center justify-between py-4 gap-5">
        <!-- Centered large cover art -->
        <div class="flex-1 flex items-center justify-center w-full py-2">
          <CoverImage
            class="cover-large !w-[264px] !h-[264px] !rounded-2xl border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)]"
            src={player.currentTrack?.cover_large_url}
            fallbackSrc={player.currentTrack?.cover_url}
            alt={player.currentTrack?.title ?? ""}
          />
        </div>

        <!-- Lower controls stack -->
        <div class="w-full flex flex-col items-center gap-4">
          <!-- Track metadata -->
          <div class="w-full text-center px-1 space-y-1">
            <h2 class="text-[19px] font-extrabold leading-tight tracking-tight line-clamp-2">
              {player.currentTrack?.title ?? "Nothing playing"}
            </h2>
            <p class="text-[14px] font-medium text-[var(--text-muted)] truncate">
              {player.currentTrack?.artist ?? "Select a song"}
            </p>
            {#if player.currentTrack?.album}
              <p class="text-[12px] text-[var(--text-muted)]/75 truncate">
                Album: {player.currentTrack.album}
              </p>
            {/if}
          </div>

          <!-- 5-icon minimalist action row (Volume, Favorite, Start Radio, Sleep Timer, Party Mode) -->
          <div class="relative w-full flex items-center justify-center gap-5 py-0.5">
            <!-- Volume popover trigger -->
            <button
              class="icon-btn !w-9 !h-9 {openPopover === 'volume' || player.volume === 0
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title="Volume ({volumePct}%)"
              onclick={() => togglePopover("volume")}
            >
              {#if player.volume === 0}
                <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">
                  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                  <line x1="23" y1="9" x2="17" y2="15" />
                  <line x1="17" y1="9" x2="23" y2="15" />
                </svg>
              {:else if player.volume < 0.5}
                <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">
                  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                  <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
                </svg>
              {:else}
                <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">
                  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                  <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
                  <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
                </svg>
              {/if}
            </button>

            <!-- Favorite toggle -->
            <button
              class="icon-btn !w-9 !h-9 transition-transform active:scale-90 {isFav
                ? 'text-[var(--coral)]'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              disabled={!player.currentTrack}
              title={isFav ? "Remove from favorites" : "Add to favorites"}
              onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
            >
              <svg
                viewBox="0 0 24 24"
                width="18"
                height="18"
                fill={isFav ? "currentColor" : "none"}
                stroke="currentColor"
                stroke-width="1.9"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
              </svg>
            </button>

            <!-- Start radio from this song -->
            <button
              class="icon-btn !w-9 !h-9 {startingRadio
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              disabled={!player.currentTrack || startingRadio}
              title="Start radio from this song"
              onclick={async () => {
                openPopover = null;
                startingRadio = true;
                try {
                  await player.startRadioFromCurrent();
                } finally {
                  startingRadio = false;
                }
              }}
            >
              {#if startingRadio}
                <span class="w-4 h-4 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
              {:else}
                <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round">
                  <circle cx="12" cy="12" r="9" />
                  <line x1="8" y1="10" x2="8" y2="14" />
                  <line x1="12" y1="8" x2="12" y2="16" />
                  <line x1="16" y1="10" x2="16" y2="14" />
                </svg>
              {/if}
            </button>

            <!-- Sleep timer popover trigger -->
            <button
              class="icon-btn !w-9 !h-9 relative {openPopover === 'timer' || sleepRemainingSecs != null
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title={sleepRemainingSecs != null ? `Sleep timer: ${formatDuration(sleepRemainingSecs)}` : "Sleep timer"}
              onclick={() => togglePopover("timer")}
            >
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="13" r="8" />
                <path d="M12 9v4l2 2" />
                <path d="M9 2h6" />
                <path d="M12 2v3" />
              </svg>
              {#if sleepRemainingSecs != null}
                <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1"></span>
              {/if}
            </button>

            <!-- Party Mode trigger -->
            <button
              class="icon-btn !w-9 !h-9 {player.partyMode
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title="Party Mode"
              onclick={() => {
                openPopover = null;
                player.partyMode = true;
              }}
            >
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 3l1.9 5.3L19.2 10l-5.3 1.9L12 17.2l-1.9-5.3L4.8 10l5.3-1.7L12 3z" />
                <path d="M18 16l.8 2.2L21 19l-2.2.8L18 22l-.8-2.2L15 19l2.2-.8L18 16z" />
              </svg>
            </button>

            <!-- Click-away backdrop for popovers -->
            {#if openPopover !== null}
              <button
                type="button"
                aria-label="Close popover"
                class="fixed inset-0 z-20 cursor-default bg-transparent"
                onclick={() => (openPopover = null)}
              ></button>
            {/if}

            <!-- Floating popover for Volume / Sleep Timer -->
            {#if openPopover === "volume"}
              <div
                class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"
              >
                <div class="flex items-center justify-between text-[12px]">
                  <span class="font-semibold text-[var(--text-muted)]">Volume</span>
                  <span class="font-bold text-[var(--accent-bright)] numeric">{volumePct}%</span>
                </div>
                <div class="flex items-center gap-3">
                  <button
                    class="icon-btn !w-8 !h-8 shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] {player.volume === 0 ? 'text-[var(--coral)]' : 'text-[var(--text)]'}"
                    onclick={toggleMute}
                    title={player.volume === 0 ? "Unmute" : "Mute"}
                  >
                    {#if player.volume === 0}
                      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                        <line x1="23" y1="9" x2="17" y2="15" />
                        <line x1="17" y1="9" x2="23" y2="15" />
                      </svg>
                    {:else if player.volume < 0.5}
                      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                        <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
                      </svg>
                    {:else}
                      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                        <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
                        <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
                      </svg>
                    {/if}
                  </button>
                  <input
                    class="sidebar-seek flex-1 min-w-0"
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value={player.volume}
                    aria-label="Volume"
                    style="background: linear-gradient(to right, var(--accent-bright) {volumePct}%, rgba(255,255,255,0.16) {volumePct}%);"
                    oninput={(e) => player.setVolume(Number((e.currentTarget as HTMLInputElement).value))}
                  />
                </div>
              </div>
            {:else if openPopover === "timer"}
              <div
                class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"
              >
                <div class="flex items-center justify-between text-[12px]">
                  <span class="font-semibold text-[var(--text-muted)]">Sleep timer</span>
                  {#if sleepRemainingSecs != null}
                    <span class="font-bold text-[var(--accent-bright)] numeric">{formatDuration(sleepRemainingSecs)}</span>
                  {/if}
                </div>
                <div class="grid grid-cols-5 gap-1.5">
                  {#each [null, 15, 30, 45, 60] as mins}
                    <button
                      class="py-1.5 rounded-xl text-[12px] font-semibold bg-[var(--surface)] text-[var(--text-muted)] hover:bg-[var(--accent-soft)] hover:text-[var(--accent-bright)] transition-colors"
                      onclick={() => setSleepTimer(mins)}
                    >
                      {mins == null ? "Off" : `${mins}m`}
                    </button>
                  {/each}
                </div>
              </div>
            {/if}
          </div>

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
              style="background: linear-gradient(to right, var(--accent-bright) {progressPct}%, rgba(255,255,255,0.16) {progressPct}%);"
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
              <span
                class="px-3 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"
              >
                {player.qualityLabel}
              </span>
              <span>{formatDuration(player.durationSecs)}</span>
            </div>
          </div>

          <!-- Transport controls -->
          <div class="w-full flex items-center justify-center gap-4 pt-1 pb-1">
            <button
              class="icon-btn relative {player.shuffle ? 'active' : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title="Shuffle"
              onclick={() => player.toggleShuffle()}
            >
              <svg viewBox="0 0 24 24" width="17" height="17" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="16 3 21 3 21 8" />
                <line x1="4" y1="20" x2="21" y2="3" />
                <polyline points="21 16 21 21 16 21" />
                <line x1="15" y1="15" x2="21" y2="21" />
                <line x1="4" y1="4" x2="9" y2="9" />
              </svg>
              {#if player.shuffle}
                <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>
              {/if}
            </button>

            <button
              class="icon-btn !w-9 !h-9"
              disabled={!player.currentTrack}
              title="Previous"
              onclick={() => void player.prev()}
            >
              <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
                <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
              </svg>
            </button>

            <button
              class="play-circle !w-[56px] !h-[56px] shrink-0"
              disabled={!player.currentTrack}
              title="Play / Pause"
              onclick={() => void player.togglePlay()}
            >
              {#if player.isLoadingAudio}
                <span class="w-5 h-5 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>
              {:else if player.isPlaying}
                <svg viewBox="0 0 24 24" width="22" height="22" fill="currentColor">
                  <rect x="6" y="5" width="4" height="14" rx="1.2" />
                  <rect x="14" y="5" width="4" height="14" rx="1.2" />
                </svg>
              {:else}
                <svg viewBox="0 0 24 24" width="22" height="22" fill="currentColor" class="translate-x-[1.5px]">
                  <polygon points="6 4 20 12 6 20 6 4" />
                </svg>
              {/if}
            </button>

            <button
              class="icon-btn !w-9 !h-9"
              disabled={!player.currentTrack}
              title="Next"
              onclick={() => void player.next()}
            >
              <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
                <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
              </svg>
            </button>

            <button
              class="icon-btn relative {player.repeat ? 'active' : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title="Repeat"
              onclick={() => player.toggleRepeat()}
            >
              <svg viewBox="0 0 24 24" width="17" height="17" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="17 1 21 5 17 9" />
                <path d="M3 11V9a4 4 0 0 1 4-4h14" />
                <polyline points="7 23 3 19 7 15" />
                <path d="M21 13v2a4 4 0 0 1-4 4H3" />
              </svg>
              {#if player.repeat}
                <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>
              {/if}
            </button>
          </div>
        </div>
      </div>

    {:else if player.sidebarTab === "lyrics"}
      <div class="flex flex-col gap-3 pt-1 flex-1 min-h-0">
        <div class="flex items-start justify-between gap-2 pb-2 border-b border-[var(--border)]">
          <div class="min-w-0">
            <h3 class="text-[14.5px] font-bold truncate">
              {player.currentTrack?.title ?? "Lyrics"}
            </h3>
            {#if player.currentTrack}
              <p class="text-[12px] text-[var(--text-muted)] truncate">
                {player.currentTrack.artist}
              </p>
            {/if}
          </div>
          {#if player.lyrics.type === "Synced"}
            <span class="shrink-0 inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[10.5px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]">
              <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span>
              Synced
            </span>
          {:else if player.lyrics.type === "Plain"}
            <span class="shrink-0 px-2.5 py-0.5 rounded-full text-[10.5px] font-semibold bg-[var(--surface)] text-[var(--text-muted)]">
              Static
            </span>
          {/if}
        </div>

        {#if player.lyrics.type === "Synced"}
          <div class="lyrics-fade flex flex-col gap-1 py-4">
            {#each player.lyrics.lines as line, i}
              <button
                bind:this={lyricRefs[i]}
                class="lyric-line"
                class:active={player.activeLyricIndex === i}
                onclick={() => player.seek(line.time_secs)}
              >
                {line.text}
              </button>
            {/each}
          </div>
        {:else if player.lyrics.type === "Plain"}
          <pre class="whitespace-pre-wrap font-sans text-[14px] leading-relaxed text-[var(--text)]/85 py-2">{player.lyrics.text}</pre>
        {:else}
          <div class="flex-1 flex flex-col items-center justify-center text-center gap-2 py-12 text-[var(--text-muted)]">
            <span class="text-3xl opacity-40">♪</span>
            <p class="text-[13px]">
              {player.currentTrack ? "No lyrics available for this track." : "Play a song to view its lyrics."}
            </p>
          </div>
        {/if}
      </div>

    {:else}
      <div class="flex flex-col gap-3 pt-1">
        <div class="flex items-center justify-between gap-2">
          <h3 class="text-[14px] font-bold">
            Queue <span class="text-[var(--text-muted)] font-medium">· {player.queue.length}</span>
          </h3>
          {#if player.queue.length > 0}
            <button
              class="px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--surface)] hover:bg-red-500/20 hover:text-red-400 transition-colors"
              onclick={() => player.clearQueue()}
            >
              Clear
            </button>
          {/if}
        </div>

        {#if player.queue.length === 0}
          <div class="flex flex-col items-center justify-center text-center gap-2 py-10 text-[var(--text-muted)]">
            <span class="text-2xl opacity-40">☰</span>
            <p class="text-[13px]">The queue is empty.</p>
          </div>
        {:else}
          <div class="track-list">
            {#each player.queue as track, i (track.id + ":" + i)}
              {@const isCurrent = player.currentIndex === i}
              <div
                class="group flex items-center gap-2 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 cursor-pointer transition-colors {isCurrent
                  ? 'bg-[var(--accent-soft)]'
                  : 'hover:bg-[var(--surface-hover)]'} {dragOverIdx === i ? 'border-t-2 border-t-[var(--accent-bright)]' : ''}"
                draggable="true"
                role="button"
                tabindex="0"
                ondragstart={() => (dragFrom = i)}
                ondragover={(e) => {
                  e.preventDefault();
                  dragOverIdx = i;
                }}
                ondragleave={() => {
                  if (dragOverIdx === i) dragOverIdx = null;
                }}
                ondrop={() => {
                  if (dragFrom != null) player.moveQueueItem(dragFrom, i);
                  dragFrom = null;
                  dragOverIdx = null;
                }}
                ondragend={() => {
                  dragFrom = null;
                  dragOverIdx = null;
                }}
                onclick={() => void player.playIndex(i)}
                onkeydown={(e) => e.key === "Enter" && void player.playIndex(i)}
              >
                <span class="text-[11px] text-[var(--text-muted)]/50 cursor-grab select-none shrink-0" aria-hidden="true">
                  ⋮⋮
                </span>
                <CoverImage class="track-cover !w-9 !h-9 !rounded-md" src={track.cover_url} alt="" />
                <div class="track-meta">
                  <span class="track-title !text-[12.5px] {isCurrent ? '!text-[var(--accent-bright)] !font-bold' : ''}">
                    {track.title}
                  </span>
                  <span class="track-sub !text-[11px]">{track.artist}</span>
                </div>
                {#if isCurrent && player.isPlaying}
                  <span class="eq-bars shrink-0" aria-label="Playing">
                    <span></span><span></span><span></span>
                  </span>
                {:else}
                  <span class="text-[11px] text-[var(--text-muted)] numeric shrink-0">
                    {formatDuration(track.duration_secs)}
                  </span>
                {/if}
                <button
                  class="icon-btn !w-6 !h-6 text-[12px] opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
                  title="Remove from queue"
                  onclick={(e) => {
                    e.stopPropagation();
                    player.removeQueueIndex(i);
                  }}
                >
                  ✕
                </button>
              </div>
            {/each}
          </div>
        {/if}

        <!-- Autoplay switch card -->
        <div class="flex items-center justify-between gap-3 p-3 rounded-xl bg-[var(--surface)] border border-[var(--border)]">
          <div class="min-w-0">
            <div class="text-[13px] font-bold">Autoplay</div>
            <p class="text-[11.5px] text-[var(--text-muted)] leading-snug">
              Queue similar songs when queue ends
            </p>
          </div>
          <button
            type="button"
            role="switch"
            aria-checked={player.autoplayRelated}
            aria-label="Toggle autoplay"
            class="w-10 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.autoplayRelated
              ? 'bg-[var(--accent)]'
              : 'bg-white/15'}"
            onclick={() => player.setAutoplay(!player.autoplayRelated)}
          >
            <span
              class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.autoplayRelated
                ? 'translate-x-4'
                : 'translate-x-0'}"
            ></span>
          </button>
        </div>

        {#if player.suggestedTracks.length > 0}
          <h4 class="text-[11.5px] font-bold uppercase tracking-wider text-[var(--text-muted)] mt-1">
            Up next suggestions
          </h4>
          <div class="track-list">
            {#each player.suggestedTracks as track (track.id)}
              <div class="flex items-center gap-2.5 px-2.5 py-2 border-b border-[var(--border)] last:border-b-0 hover:bg-[var(--surface-hover)] transition-colors">
                <CoverImage class="track-cover !w-9 !h-9 !rounded-md" src={track.cover_url} alt="" />
                <div class="track-meta">
                  <span class="track-title !text-[12.5px]">{track.title}</span>
                  <span class="track-sub !text-[11px]">{track.artist}</span>
                </div>
                <button
                  class="icon-btn !w-7 !h-7 shrink-0"
                  title="Add to queue"
                  onclick={() => player.enqueueTrack(track)}
                >
                  <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="12" y1="5" x2="12" y2="19" />
                    <line x1="5" y1="12" x2="19" y2="12" />
                  </svg>
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Bottom tab bar matching GTK4 pill-outline style -->
  <nav class="shrink-0 grid grid-cols-3 gap-1.5 px-3 py-2.5 border-t border-[var(--border)] bg-[var(--bg-sidebar)]">
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'player'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "player")}
    >
      <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M9 18V5l12-2v13" />
        <circle cx="6" cy="18" r="3" />
        <circle cx="18" cy="16" r="3" />
      </svg>
      Player
    </button>
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'lyrics'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "lyrics")}
    >
      <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
      </svg>
      Lyrics
    </button>
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'queue'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "queue")}
    >
      <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="8" y1="6" x2="21" y2="6" />
        <line x1="8" y1="12" x2="21" y2="12" />
        <line x1="8" y1="18" x2="21" y2="18" />
        <line x1="3" y1="6" x2="3.01" y2="6" />
        <line x1="3" y1="12" x2="3.01" y2="12" />
        <line x1="3" y1="18" x2="3.01" y2="18" />
      </svg>
      Queue
    </button>
  </nav>
</aside>

<style>
  .sidebar-seek {
    height: 4px;
  }

  .sidebar-seek::-webkit-slider-thumb {
    width: 17px;
    height: 17px;
    background: #e5e7eb;
    border: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.45);
  }

  .sidebar-seek::-moz-range-thumb {
    width: 17px;
    height: 17px;
    background: #e5e7eb;
    border: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.45);
  }

  .lyrics-fade {
    mask-image: linear-gradient(to bottom, transparent 0%, black 7%, black 93%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 7%, black 93%, transparent 100%);
  }

  .eq-bars {
    display: inline-flex;
    align-items: flex-end;
    gap: 2px;
    height: 12px;
  }

  .eq-bars span {
    width: 2.5px;
    border-radius: 2px;
    background: var(--accent-bright);
    animation: eq-bounce 0.9s ease-in-out infinite;
  }

  .eq-bars span:nth-child(1) {
    height: 55%;
    animation-delay: 0s;
  }

  .eq-bars span:nth-child(2) {
    height: 100%;
    animation-delay: 0.2s;
  }

  .eq-bars span:nth-child(3) {
    height: 70%;
    animation-delay: 0.4s;
  }

  @keyframes eq-bounce {
    0%,
    100% {
      transform: scaleY(0.35);
    }
    50% {
      transform: scaleY(1);
    }
  }
</style>
