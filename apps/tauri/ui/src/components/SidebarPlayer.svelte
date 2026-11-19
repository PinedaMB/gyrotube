<script lang="ts">
  import {
    ChevronLeft,
    GripVertical,
    Heart,
    ListMusic,
    LocateFixed,
    MessageSquareQuote,
    Music,
    PartyPopper,
    Pause,
    Play,
    Plus,
    Radio,
    Repeat,
    Shuffle,
    SkipBack,
    SkipForward,
    Timer,
    Volume1,
    Volume2,
    VolumeX,
    X,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly, scale } from "svelte/transition";
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

  let sleepRemainingSecs = $state<number | null>(null);
  let sleepInterval: number | undefined;
  let autoScrollPaused = $state(false);
  let lastTrackId = $state<string | null>(null);

  let displayPos = $derived(seeking ? seekDraft : player.positionSecs);
  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (displayPos / player.durationSecs) * 100))
      : 0,
  );
  let volumePct = $derived(Math.round(player.volume * 100));
  let isFav = $derived(player.currentTrack ? player.isFavorite(player.currentTrack.id) : false);

  $effect(() => {
    const tid = player.currentTrack?.id ?? null;
    if (tid !== lastTrackId) {
      lastTrackId = tid;
      autoScrollPaused = false;
    }
  });

  $effect(() => {
    const idx = player.activeLyricIndex;
    if (idx != null && player.sidebarTab === "lyrics" && !autoScrollPaused) {
      const el = lyricRefs[idx];
      el?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  });

  function markUserScroll() {
    autoScrollPaused = true;
  }

  function resumeSync() {
    autoScrollPaused = false;
    const idx = player.activeLyricIndex;
    if (idx != null) {
      lyricRefs[idx]?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  }

  function togglePopover(kind: Exclude<ActionPopover, null>) {
    openPopover = openPopover === kind ? null : kind;
  }

  function lyricOpacity(idx: number): number {
    if (player.activeLyricIndex == null) return 0.65;
    if (idx === player.activeLyricIndex) return 1.0;
    const diff = Math.abs(idx - player.activeLyricIndex);
    if (diff === 1) return 0.42;
    if (diff === 2) return 0.24;
    return 0.14;
  }

  function isFarLyric(idx: number): boolean {
    if (player.activeLyricIndex == null) return false;
    return Math.abs(idx - player.activeLyricIndex) >= 2;
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
      <span class="text-[11.5px] font-medium text-[var(--text-muted)]">Playing from</span>
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
      <ChevronLeft size={19} />
    </button>
  </div>

  <!-- Main scrollable area -->
  <div
    class="flex-1 min-h-0 px-7 pb-4 flex flex-col {player.sidebarTab === 'lyrics'
      ? 'overflow-hidden'
      : 'overflow-y-auto scroll-y'}"
  >
    {#if player.sidebarTab === "player"}
      <div in:fade={{ duration: 180 }} class="flex-1 flex flex-col items-center justify-between py-4 gap-5">
        <!-- Centered large cover art -->
        <div class="flex-1 flex items-center justify-center w-full py-2">
          {#key player.currentTrack?.id ?? "none"}
            <div in:scale={{ start: 0.94, duration: 260, easing: cubicOut }}>
              <CoverImage
                class="cover-large !w-[264px] !h-[264px] !rounded-2xl border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)] transition-transform duration-300 hover:scale-[1.015]"
                src={player.currentTrack?.cover_large_url}
                fallbackSrc={player.currentTrack?.cover_url}
                alt={player.currentTrack?.title ?? ""}
              />
            </div>
          {/key}
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
            <button
              class="icon-btn !w-9 !h-9 {openPopover === 'volume' || player.volume === 0
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title="Volume ({volumePct}%)"
              onclick={() => togglePopover("volume")}
            >
              {#if player.volume === 0}
                <VolumeX size={18} />
              {:else if player.volume < 0.5}
                <Volume1 size={18} />
              {:else}
                <Volume2 size={18} />
              {/if}
            </button>

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
                <Radio size={18} />
              {/if}
            </button>

            <button
              class="icon-btn !w-9 !h-9 relative {openPopover === 'timer' || sleepRemainingSecs != null
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title={sleepRemainingSecs != null
                ? `Sleep timer: ${formatDuration(sleepRemainingSecs)}`
                : "Sleep timer"}
              onclick={() => togglePopover("timer")}
            >
              <Timer size={18} />
              {#if sleepRemainingSecs != null}
                <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1"></span>
              {/if}
            </button>

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
              <PartyPopper size={18} />
            </button>

            {#if openPopover !== null}
              <button
                type="button"
                aria-label="Close popover"
                class="fixed inset-0 z-20 cursor-default bg-transparent"
                onclick={() => (openPopover = null)}
              ></button>
            {/if}

            {#if openPopover === "volume"}
              <div
                transition:fly={{ y: 8, duration: 170, easing: cubicOut }}
                class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"
              >
                <div class="flex items-center justify-between text-[12px]">
                  <span class="font-semibold text-[var(--text-muted)]">Volume</span>
                  <span class="font-bold text-[var(--accent-bright)] numeric">{volumePct}%</span>
                </div>
                <div class="flex items-center gap-3">
                  <button
                    class="icon-btn !w-8 !h-8 shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] {player.volume === 0
                      ? 'text-[var(--coral)]'
                      : 'text-[var(--text)]'}"
                    onclick={toggleMute}
                    title={player.volume === 0 ? "Unmute" : "Mute"}
                  >
                    {#if player.volume === 0}
                      <VolumeX size={16} />
                    {:else if player.volume < 0.5}
                      <Volume1 size={16} />
                    {:else}
                      <Volume2 size={16} />
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
                transition:fly={{ y: 8, duration: 170, easing: cubicOut }}
                class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"
              >
                <div class="flex items-center justify-between text-[12px]">
                  <span class="font-semibold text-[var(--text-muted)]">Sleep timer</span>
                  {#if sleepRemainingSecs != null}
                    <span class="font-bold text-[var(--accent-bright)] numeric">
                      {formatDuration(sleepRemainingSecs)}
                    </span>
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
              <span class="px-3 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]">
                {player.qualityLabel}
              </span>
              <span>{formatDuration(player.durationSecs)}</span>
            </div>
          </div>

          <!-- Transport controls -->
          <div class="w-full flex items-center justify-center gap-4 pt-1 pb-1">
            <button
              class="icon-btn relative {player.shuffle
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title="Shuffle"
              onclick={() => player.toggleShuffle()}
            >
              <Shuffle size={17} />
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
              <SkipBack size={20} fill="currentColor" />
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
                <Pause size={22} fill="currentColor" />
              {:else}
                <Play size={22} fill="currentColor" class="translate-x-[1.5px]" />
              {/if}
            </button>

            <button
              class="icon-btn !w-9 !h-9"
              disabled={!player.currentTrack}
              title="Next"
              onclick={() => void player.next()}
            >
              <SkipForward size={20} fill="currentColor" />
            </button>

            <button
              class="icon-btn relative {player.repeat
                ? 'active'
                : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
              title="Repeat"
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
          <div class="relative flex-1 min-h-0 flex flex-col">
            <div
              role="region"
              aria-label="Lyrics"
              class="lyrics-fade flex-1 min-h-0 overflow-y-auto overflow-x-hidden px-2 pt-28 pb-36 flex flex-col gap-4"
              onwheel={markUserScroll}
              ontouchmove={markUserScroll}
            >
              {#each player.lyrics.lines as line, i}
                {@const active = player.activeLyricIndex === i}
                {@const prevLine = i > 0 ? player.lyrics.lines[i - 1] : null}
                {@const stanzaBreak = prevLine != null && line.time_secs - prevLine.time_secs >= 5.5}
                <button
                  bind:this={lyricRefs[i]}
                  class="sb-lyric-line text-left origin-left {active ? 'active-line' : ''} {isFarLyric(i) ? 'far-line' : ''} {stanzaBreak ? 'mt-3' : ''}"
                  style="opacity: {lyricOpacity(i)};"
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
                transition:scale={{ start: 0.88, duration: 170, easing: cubicOut }}
                class="absolute bottom-3 left-1/2 -translate-x-1/2 inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_8px_24px_rgba(0,0,0,0.45)] transition-all active:scale-95"
                onclick={resumeSync}
              >
                <LocateFixed size={14} />
                Sync
              </button>
            {/if}
          </div>
        {:else if player.lyrics.type === "Plain"}
          <div class="lyrics-fade flex-1 min-h-0 overflow-y-auto py-8">
            <pre class="whitespace-pre-wrap font-sans text-[14.5px] leading-relaxed text-[var(--text)]/85">{player.lyrics.text}</pre>
          </div>
        {:else}
          <div class="flex-1 flex flex-col items-center justify-center text-center gap-2 py-12 text-[var(--text-muted)]">
            <Music size={28} class="opacity-40" />
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
            <ListMusic size={26} class="opacity-40" />
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
                <GripVertical size={13} class="text-[var(--text-muted)]/45 cursor-grab shrink-0" />
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
                  class="icon-btn !w-6 !h-6 opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
                  title="Remove from queue"
                  onclick={(e) => {
                    e.stopPropagation();
                    player.removeQueueIndex(i);
                  }}
                >
                  <X size={13} />
                </button>
              </div>
            {/each}
          </div>
        {/if}

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
                  <Plus size={15} />
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Bottom tab bar -->
  <nav class="shrink-0 grid grid-cols-3 gap-1.5 px-3 py-2.5 border-t border-[var(--border)] bg-[var(--bg-sidebar)]">
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'player'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "player")}
    >
      <Music size={15} />
      Player
    </button>
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'lyrics'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "lyrics")}
    >
      <MessageSquareQuote size={15} />
      Lyrics
    </button>
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'queue'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "queue")}
    >
      <ListMusic size={15} />
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
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent 0%, black 12%, black 88%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 12%, black 88%, transparent 100%);
  }

  .lyrics-fade::-webkit-scrollbar {
    display: none;
  }

  .sb-lyric-line {
    font-size: 15.5px;
    font-weight: 650;
    line-height: 1.42;
    letter-spacing: -0.015em;
    color: var(--text);
    padding: 2px 4px;
    transition:
      opacity 360ms cubic-bezier(0.22, 1, 0.36, 1),
      font-size 360ms cubic-bezier(0.22, 1, 0.36, 1),
      color 360ms cubic-bezier(0.22, 1, 0.36, 1),
      filter 360ms cubic-bezier(0.22, 1, 0.36, 1),
      text-shadow 360ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .sb-lyric-line.far-line {
    filter: blur(0.45px);
  }

  .sb-lyric-line:hover {
    opacity: 0.92 !important;
    filter: none;
  }

  .sb-lyric-line.active-line {
    font-size: 20px;
    font-weight: 850;
    line-height: 1.3;
    letter-spacing: -0.02em;
    color: #ffffff;
    filter: none;
    text-shadow:
      0 0 24px rgba(var(--ambient-rgb), 0.65),
      0 2px 14px rgba(0, 0, 0, 0.4);
  }

  .eq-bars {
    display: inline-flex;
    align-items: flex-end;
    gap: 2.5px;
    height: 14px;
    padding-bottom: 1px;
  }

  .eq-bars span {
    width: 3px;
    height: 100%;
    border-radius: 2px;
    background: var(--accent-bright);
    transform-origin: bottom center;
    will-change: transform;
  }

  .eq-bars span:nth-child(1) {
    animation: eq-bar-1 0.85s ease-in-out infinite;
  }

  .eq-bars span:nth-child(2) {
    animation: eq-bar-2 0.72s ease-in-out infinite;
  }

  .eq-bars span:nth-child(3) {
    animation: eq-bar-3 0.95s ease-in-out infinite;
  }

  @keyframes eq-bar-1 {
    0%, 100% { transform: scaleY(0.25); }
    45% { transform: scaleY(0.95); }
    75% { transform: scaleY(0.45); }
  }

  @keyframes eq-bar-2 {
    0%, 100% { transform: scaleY(0.9); }
    35% { transform: scaleY(0.3); }
    68% { transform: scaleY(1); }
  }

  @keyframes eq-bar-3 {
    0%, 100% { transform: scaleY(0.4); }
    50% { transform: scaleY(0.85); }
    80% { transform: scaleY(0.22); }
  }
</style>
