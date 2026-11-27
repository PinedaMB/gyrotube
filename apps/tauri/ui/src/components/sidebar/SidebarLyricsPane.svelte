<script lang="ts">
  import {
    Check,
    LocateFixed,
    Music,
    Pause,
    Play,
    Share2,
    SkipBack,
    SkipForward,
    X,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import {
    MAX_SELECTED_LYRIC_LINES,
    computeLyricOpacity,
    isSelectableLyricText,
    isWaitingForFirstLyric,
    parsePlainStanzas,
    smoothScrollToLyric,
    toggleContiguousLyricSelection,
  } from "../../lib/lyricsFormat";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type LyricWord } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";
  import LyricsOptionsMenu from "./LyricsOptionsMenu.svelte";

  let scrollEl = $state<HTMLDivElement | null>(null);
  let lyricRefs = $state<Record<number, HTMLButtonElement>>({});
  let autoScrollPaused = $state(false);
  let lastTrackId = $state<string | null>(null);
  let liveKaraokePos = $state(0);
  let cancelScrollAnim: (() => void) | null = null;

  let selecting = $state(false);
  let selectedIndices = $state<number[]>([]);
  let cardModalOpen = $state(false);

  let plainParsed = $derived(
    player.lyrics.type === "Plain"
      ? parsePlainStanzas(player.lyrics.text)
      : { stanzas: [], selectable: [] },
  );

  let hasLyrics = $derived(
    (player.lyrics.type === "Synced" && player.lyrics.lines.length > 0) ||
      (player.lyrics.type === "Plain" && plainParsed.selectable.length > 0),
  );

  let waitingForFirstLyric = $derived(
    isWaitingForFirstLyric(
      player.lyrics,
      player.activeLyricIndex,
      player.positionSecs + player.currentLyricsOffset,
    ),
  );

  let offsetLabel = $derived(
    player.currentLyricsOffset === 0
      ? "0.0s"
      : `${player.currentLyricsOffset > 0 ? "+" : ""}${player.currentLyricsOffset.toFixed(1)}s`,
  );

  let hasWordLyrics = $derived(
    player.lyrics.type === "Synced" &&
      player.lyrics.lines.some((l) => (l.words?.length ?? 0) > 0),
  );

  let hasRomanization = $derived(
    player.lyrics.type === "Synced" && player.lyrics.lines.some((l) => Boolean(l.romanized)),
  );

  $effect(() => {
    liveKaraokePos = player.positionSecs + player.currentLyricsOffset;
    if (!hasWordLyrics || !player.isPlaying) return;
    let rafId = 0;
    const tick = () => {
      liveKaraokePos = player.getExactPositionSecs() + player.currentLyricsOffset;
      rafId = requestAnimationFrame(tick);
    };
    rafId = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafId);
  });

  function wordProgressPct(word: LyricWord, tSecs: number): number {
    if (tSecs <= word.start_secs) return 0;
    if (tSecs >= word.end_secs) return 100;
    return Math.round(
      ((tSecs - word.start_secs) / Math.max(0.05, word.end_secs - word.start_secs)) * 100,
    );
  }

  let selectedLineTexts = $derived.by(() => {
    const sorted = [...selectedIndices].sort((a, b) => a - b);
    if (player.lyrics.type === "Synced") {
      return sorted
        .map((idx) => (player.lyrics.type === "Synced" ? player.lyrics.lines[idx]?.text ?? "" : ""))
        .filter(Boolean);
    }
    if (player.lyrics.type === "Plain") {
      return sorted.map((idx) => plainParsed.selectable[idx] ?? "").filter(Boolean);
    }
    return [];
  });

  $effect(() => {
    const tid = player.currentTrack?.id ?? null;
    if (tid !== lastTrackId) {
      lastTrackId = tid;
      autoScrollPaused = false;
      selecting = false;
      selectedIndices = [];
      cardModalOpen = false;
    }
  });

  $effect(() => {
    const idx = player.activeLyricIndex;
    if (idx != null && !autoScrollPaused && !selecting) {
      cancelScrollAnim?.();
      cancelScrollAnim = smoothScrollToLyric(scrollEl, lyricRefs[idx] ?? null, 0.42, 520);
      return () => {
        cancelScrollAnim?.();
        cancelScrollAnim = null;
      };
    }
  });

  function markUserScroll() {
    cancelScrollAnim?.();
    cancelScrollAnim = null;
    autoScrollPaused = true;
  }

  function resumeSync() {
    autoScrollPaused = false;
    const idx = player.activeLyricIndex;
    if (idx != null) {
      cancelScrollAnim?.();
      cancelScrollAnim = smoothScrollToLyric(scrollEl, lyricRefs[idx] ?? null, 0.42, 460);
    }
  }

  function isSelectableLine(idx: number): boolean {
    if (player.lyrics.type === "Synced") {
      return isSelectableLyricText(player.lyrics.lines[idx]?.text ?? "");
    }
    if (player.lyrics.type === "Plain") {
      return isSelectableLyricText(plainParsed.selectable[idx] ?? "");
    }
    return false;
  }

  function startSelecting() {
    if (!hasLyrics) return;
    selecting = true;
    autoScrollPaused = true;
    const total =
      player.lyrics.type === "Synced"
        ? player.lyrics.lines.length
        : plainParsed.selectable.length;
    let base =
      player.lyrics.type === "Synced" && player.activeLyricIndex != null
        ? player.activeLyricIndex
        : 0;
    while (base < total && !isSelectableLine(base)) {
      base += 1;
    }
    if (base >= total) {
      selectedIndices = [];
      return;
    }
    const initial = [base];
    let next = base + 1;
    while (next < total && !isSelectableLine(next)) {
      next += 1;
    }
    if (next < total && next - base <= 2) {
      initial.push(next);
    }
    selectedIndices = initial;
  }

  function cancelSelecting() {
    selecting = false;
    selectedIndices = [];
    cardModalOpen = false;
  }

  function toggleLineSelection(idx: number) {
    selectedIndices = toggleContiguousLyricSelection(
      selectedIndices,
      idx,
      isSelectableLine,
      MAX_SELECTED_LYRIC_LINES,
    );
  }

  function lyricOpacity(idx: number): number {
    if (selecting) {
      return selectedIndices.includes(idx) ? 1.0 : 0.42;
    }
    return computeLyricOpacity(idx, player.activeLyricIndex);
  }

  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (player.positionSecs / player.durationSecs) * 100))
      : 0,
  );

  function handleProgressSeek(e: MouseEvent) {
    if (!player.currentTrack || player.durationSecs <= 0) return;
    const rect = (e.currentTarget as HTMLDivElement).getBoundingClientRect();
    const ratio = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    player.seek(ratio * player.durationSecs);
  }
</script>

{#snippet selectionBar()}
  <div
    transition:scale={{ start: 0.94, duration: 150, easing: cubicOut }}
    class="absolute bottom-3 inset-x-0 flex items-center justify-between gap-2 px-3 py-2 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_12px_32px_rgba(0,0,0,0.45)]"
  >
    <span class="text-[12px] font-bold numeric text-[var(--text-muted)] pl-1">
      {selectedIndices.length}/{MAX_SELECTED_LYRIC_LINES}
    </span>
    <div class="flex items-center gap-1.5">
      <button
        type="button"
        class="pill-btn !px-3 !py-1.5 !text-[12px]"
        onclick={cancelSelecting}
      >
        {t("lyrics.cancel")}
      </button>
      <button
        type="button"
        class="pill-btn primary !px-3.5 !py-1.5 !text-[12px]"
        disabled={selectedIndices.length === 0}
        onclick={() => (cardModalOpen = true)}
      >
        <Share2 size={13} />
        {t("lyrics.create_card")}
      </button>
    </div>
  </div>
{/snippet}

<div in:fade={{ duration: 150 }} class="flex flex-col gap-2 pt-1 flex-1 min-h-0">
  <!-- Compact track header -->
  <div class="flex items-center justify-between gap-2 pb-3 border-b border-[var(--border)]">
    <div class="flex items-center gap-2.5 min-w-0">
      {#if player.currentTrack}
        <CoverImage
          class="track-cover !w-9 !h-9 !rounded-[8px] shrink-0"
          src={player.trackCoverUrl(player.currentTrack, false)}
          fallbackSrc={player.offlineCoverUrl(player.currentTrack.id)}
          alt=""
        />
      {/if}
      <div class="min-w-0">
        <h3 class="text-[14px] font-bold truncate leading-tight">
          {player.currentTrack?.title ?? t("lyrics.title")}
        </h3>
        {#if player.currentTrack}
          <p class="text-[12px] text-[var(--text-muted)] truncate mt-0.5">
            {player.currentTrack.artist}
          </p>
        {/if}
      </div>
    </div>

    <div class="flex items-center gap-1.5 shrink-0">
      {#if player.lyrics.type === "Synced"}
        <span class="shrink-0 inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10.5px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]">
          <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span>
          {t("lyrics.synced")}
        </span>
      {:else if player.lyrics.type === "Plain"}
        <span class="shrink-0 px-2.5 py-0.5 rounded-full text-[10.5px] font-semibold bg-[var(--surface)] border border-[var(--border)] text-[var(--text-muted)]">
          {t("lyrics.static")}
        </span>
      {/if}

      {#if selecting}
        <button
          type="button"
          class="w-7 h-7 rounded-full inline-flex items-center justify-center bg-[var(--accent)] text-white transition-all active:scale-95"
          title={t("lyrics.cancel")}
          aria-label={t("lyrics.cancel")}
          onclick={cancelSelecting}
        >
          <X size={13} />
        </button>
      {:else}
        <LyricsOptionsMenu
          {hasLyrics}
          {hasRomanization}
          showSidebarActions={true}
          compact={true}
          onStartSelecting={startSelecting}
        />
      {/if}
    </div>
  </div>

  {#if selecting}
    <div class="flex items-center justify-between gap-2 px-3 py-1.5 rounded-xl bg-[var(--accent-soft)]/70 border border-[var(--accent-border)]/40 text-[11.5px] font-semibold text-[var(--accent-bright)]">
      <span>{t("lyrics.select_hint", { max: MAX_SELECTED_LYRIC_LINES })}</span>
      <span class="numeric opacity-85 shrink-0">{selectedIndices.length}/{MAX_SELECTED_LYRIC_LINES}</span>
    </div>
  {/if}

  {#if player.lyricsLoading && player.currentTrack}
    <div class="flex-1 flex flex-col gap-3.5 pt-8 px-1.5 animate-pulse" aria-busy="true">
      {#each [72, 58, 84, 66, 48, 78, 62, 52] as w}
        <div class="h-4 rounded-md bg-[var(--skeleton)]" style="width: {w}%;"></div>
      {/each}
    </div>
  {:else if player.lyrics.type === "Synced"}
    <div class="relative flex-1 min-h-0 flex flex-col">
      <div
        bind:this={scrollEl}
        data-lyrics-size={player.lyricsFontSize}
        data-lyrics-align={player.lyricsAlign}
        role="region"
        aria-label={t("lyrics.title")}
        class="lyrics-fade flex-1 min-h-0 overflow-y-auto overflow-x-hidden pt-8 pb-32 px-1 pr-3 flex flex-col gap-2.5"
        onwheel={markUserScroll}
        ontouchmove={markUserScroll}
      >
        {#if waitingForFirstLyric && !selecting}
          <div class="flex items-center gap-1.5 px-2 pb-2" aria-hidden="true">
            <span class="intro-dot"></span>
            <span class="intro-dot" style="animation-delay: 180ms;"></span>
            <span class="intro-dot" style="animation-delay: 360ms;"></span>
          </div>
        {/if}

        {#each player.lyrics.lines as line, i}
          {@const active = !selecting && player.activeLyricIndex === i}
          {@const selectable = isSelectableLine(i)}
          {@const picked = selecting && selectable && selectedIndices.includes(i)}
          {@const prevLine = i > 0 ? player.lyrics.lines[i - 1] : null}
          {@const stanzaBreak = prevLine != null && line.time_secs - prevLine.time_secs >= 5.5}
          <button
            type="button"
            bind:this={lyricRefs[i]}
            disabled={selecting && !selectable}
            class="sb-lyric-line text-left origin-left flex items-start gap-2.5 rounded-xl {active
              ? 'active-line'
              : ''} {picked ? 'selected-line' : ''} {stanzaBreak ? 'mt-4' : ''} {selecting && !selectable
              ? 'cursor-default pointer-events-none'
              : ''}"
            style="opacity: {selecting && !selectable ? 0.22 : lyricOpacity(i)};"
            onclick={() => {
              if (selecting) {
                toggleLineSelection(i);
              } else {
                autoScrollPaused = false;
                player.seek(Math.max(0, line.time_secs - player.currentLyricsOffset));
              }
            }}
          >
            {#if selecting && selectable}
              <span
                class="mt-1.5 w-4 h-4 rounded-full border inline-flex items-center justify-center shrink-0 transition-all duration-150 {picked
                  ? 'bg-[var(--accent)] border-[var(--accent)] text-white scale-100'
                  : 'border-white/20 scale-95'}"
              >
                {#if picked}
                  <Check size={10} strokeWidth={3} />
                {/if}
              </span>
            {/if}
            <span class="flex-1">
              {#if active && line.words && line.words.length > 0}
                {#each line.words as word}
                  {@const pct = wordProgressPct(word, liveKaraokePos)}
                  {#if pct >= 100}
                    <span class="karaoke-word-sung">{word.text}</span>
                  {:else if pct <= 0}
                    <span class="karaoke-word-pending">{word.text}</span>
                  {:else}
                    <span
                      class="karaoke-word-active"
                      style="background-image: linear-gradient(90deg, var(--text) 0%, var(--text) {pct}%, color-mix(in srgb, var(--text) 34%, transparent) {pct}%, color-mix(in srgb, var(--text) 34%, transparent) 100%);"
                    >{word.text}</span>
                  {/if}
                {/each}
              {:else}
                {line.text}
              {/if}
              {#if player.showRomanization && line.romanized}
                <span class="block text-[12px] font-medium leading-snug text-[var(--text-muted)] mt-0.5">
                  {line.romanized}
                </span>
              {/if}
              {#if player.showTranslation && player.translatedLines?.[i]}
                <span class="block text-[12.5px] font-medium leading-snug text-[var(--accent-bright)]/85 mt-0.5">
                  {player.translatedLines[i]}
                </span>
              {/if}
            </span>
          </button>
        {/each}
      </div>

      {#if selecting}
        {@render selectionBar()}
      {:else if autoScrollPaused && player.activeLyricIndex != null}
        <button
          type="button"
          transition:scale={{ start: 0.9, duration: 150, easing: cubicOut }}
          class="absolute bottom-3 left-1/2 -translate-x-1/2 inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_8px_24px_rgba(0,0,0,0.45)] transition-colors active:scale-95"
          onclick={resumeSync}
        >
          <LocateFixed size={14} />
          {t("lyrics.sync_btn")}
        </button>
      {/if}
    </div>
  {:else if player.lyrics.type === "Plain"}
    <div class="relative flex-1 min-h-0 flex flex-col">
      <div
        data-lyrics-size={player.lyricsFontSize}
        data-lyrics-align={player.lyricsAlign}
        role="region"
        aria-label={t("lyrics.title")}
        class="lyrics-fade flex-1 min-h-0 overflow-y-auto overflow-x-hidden pt-6 pb-28 px-1.5 space-y-5"
      >
        {#each plainParsed.stanzas as stanza, sIdx (sIdx)}
          <div class="flex flex-col gap-1.5">
            {#each stanza as item, lIdx (`${sIdx}:${lIdx}`)}
              {#if item.isHeader}
                <div class="text-[10.5px] font-extrabold uppercase tracking-[0.11em] text-[var(--accent-bright)]/80 pt-1 pb-0.5 px-1.5 select-none">
                  {item.text}
                </div>
              {:else if selecting && item.selectableIdx != null}
                {@const sIndex = item.selectableIdx}
                {@const picked = selectedIndices.includes(sIndex)}
                <button
                  type="button"
                  class="sb-lyric-line sb-static-line text-left flex items-start gap-2.5 rounded-xl {picked
                    ? 'selected-line'
                    : ''}"
                  style="opacity: {picked ? 1 : 0.44};"
                  onclick={() => toggleLineSelection(sIndex)}
                >
                  <span
                    class="mt-1.5 w-4 h-4 rounded-full border inline-flex items-center justify-center shrink-0 transition-all duration-150 {picked
                      ? 'bg-[var(--accent)] border-[var(--accent)] text-white scale-100'
                      : 'border-white/20 scale-95'}"
                  >
                    {#if picked}
                      <Check size={10} strokeWidth={3} />
                    {/if}
                  </span>
                  <span class="flex-1">{item.text}</span>
                </button>
              {:else}
                <p class="sb-static-line px-1.5">
                  {item.text}
                </p>
              {/if}
            {/each}
          </div>
        {/each}
      </div>

      {#if selecting}
        {@render selectionBar()}
      {/if}
    </div>
  {:else}
    <div class="flex-1 flex flex-col items-center justify-center text-center gap-2 py-12 text-[var(--text-muted)]">
      <Music size={28} class="opacity-40" />
      <p class="text-[13px]">
        {player.currentTrack ? t("lyrics.none_track") : t("lyrics.none_idle")}
      </p>
    </div>
  {/if}

  {#if player.currentTrack && !selecting}
    <div class="shrink-0 pt-2.5 pb-0.5 border-t border-[var(--border)] space-y-2">
      <!-- Compact interactive seek bar -->
      <div class="flex items-center gap-2">
        <span class="text-[10.5px] font-semibold text-[var(--text-muted)] numeric w-8 text-right shrink-0">
          {formatDuration(player.positionSecs)}
        </span>
        <div
          class="flex-1 h-1.5 rounded-full bg-[var(--control-track)] cursor-pointer overflow-hidden"
          role="slider"
          tabindex="0"
          aria-label={t("a11y.seek")}
          aria-valuemin={0}
          aria-valuemax={Math.round(player.durationSecs)}
          aria-valuenow={Math.round(player.positionSecs)}
          onclick={handleProgressSeek}
          onkeydown={(e) => {
            if (e.key === "ArrowRight") player.seek(player.positionSecs + 5);
            else if (e.key === "ArrowLeft") player.seek(Math.max(0, player.positionSecs - 5));
          }}
        >
          <div
            class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-150 ease-linear"
            style="width: {progressPct}%;"
          ></div>
        </div>
        <span class="text-[10.5px] font-semibold text-[var(--text-muted)] numeric w-8 shrink-0">
          {formatDuration(player.durationSecs)}
        </span>
      </div>

      <!-- Prev / Play-Pause / Next -->
      <div class="flex items-center justify-center gap-4">
        <button
          type="button"
          class="icon-btn !w-9 !h-9 text-[var(--text-muted)] hover:text-[var(--text)]"
          title={t("player.previous")}
          aria-label={t("player.previous")}
          onclick={() => void player.prev()}
        >
          <SkipBack size={18} fill="currentColor" />
        </button>

        <button
          type="button"
          class="w-10 h-10 rounded-full inline-flex items-center justify-center bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white shadow-[0_6px_18px_rgba(16,185,129,0.38)] transition-all active:scale-95"
          title={player.isPlaying ? t("player.paused") : t("home.play")}
          aria-label={player.isPlaying ? t("player.paused") : t("home.play")}
          onclick={() => void player.togglePlay()}
        >
          {#if player.isPlaying}
            <Pause size={18} fill="currentColor" />
          {:else}
            <Play size={18} fill="currentColor" class="translate-x-[1px]" />
          {/if}
        </button>

        <button
          type="button"
          class="icon-btn !w-9 !h-9 text-[var(--text-muted)] hover:text-[var(--text)]"
          title={t("player.next")}
          aria-label={t("player.next")}
          onclick={() => void player.next()}
        >
          <SkipForward size={18} fill="currentColor" />
        </button>
      </div>
    </div>
  {/if}
</div>

{#if cardModalOpen}
  {#await import("./LyricsCardModal.svelte") then { default: LyricsCardModal }}
    <LyricsCardModal
      open={cardModalOpen}
      lines={selectedLineTexts}
      onClose={() => (cardModalOpen = false)}
    />
  {/await}
{/if}

<style>
  .lyrics-fade {
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent 0%, black 7%, black 88%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 7%, black 88%, transparent 100%);
  }

  .lyrics-fade::-webkit-scrollbar {
    display: none;
  }

  .sb-lyric-line {
    font-family: var(--lyrics-font);
    font-size: 17px;
    font-weight: 700;
    line-height: 1.44;
    letter-spacing: -0.015em;
    color: var(--text);
    padding: 5px 10px;
    border: 1px solid transparent;
    transition:
      opacity 380ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 380ms cubic-bezier(0.22, 1, 0.36, 1),
      background-color 200ms ease,
      border-color 200ms ease,
      color 260ms ease;
  }

  [data-lyrics-size="sm"] .sb-lyric-line {
    font-size: 14.5px;
  }

  [data-lyrics-size="lg"] .sb-lyric-line {
    font-size: 20px;
  }

  [data-lyrics-align="center"] .sb-lyric-line,
  [data-lyrics-align="center"] .sb-static-line {
    text-align: center;
    transform-origin: center;
  }

  [data-lyrics-align="right"] .sb-lyric-line,
  [data-lyrics-align="right"] .sb-static-line {
    text-align: right;
    transform-origin: right;
  }

  .sb-lyric-line:hover {
    opacity: 0.94 !important;
    background: color-mix(in srgb, var(--surface-hover) 60%, transparent);
  }

  .sb-lyric-line.active-line {
    transform: scale(1.03);
    color: var(--text);
  }

  .karaoke-word-sung {
    color: var(--text);
  }

  .karaoke-word-pending {
    color: var(--text);
    opacity: 0.34;
  }

  .karaoke-word-active {
    display: inline;
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
    color: transparent;
  }

  .sb-lyric-line.selected-line {
    background: color-mix(in srgb, var(--accent-soft) 92%, transparent);
    border-color: color-mix(in srgb, var(--accent-border) 65%, transparent);
    color: var(--text);
  }

  .sb-static-line {
    font-family: var(--lyrics-font);
    font-size: 16.5px;
    font-weight: 700;
    line-height: 1.5;
    letter-spacing: -0.015em;
    color: color-mix(in srgb, var(--text) 88%, transparent);
    margin: 0;
  }

  .intro-dot {
    width: 7px;
    height: 7px;
    border-radius: 9999px;
    background: var(--accent-bright);
    opacity: 0.35;
    animation: introPulse 1.2s ease-in-out infinite;
  }

  @keyframes introPulse {
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
