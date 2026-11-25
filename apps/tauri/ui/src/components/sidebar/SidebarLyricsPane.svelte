<script lang="ts">
  import { Check, LocateFixed, Music, Pause, Play, Share2, SkipBack, SkipForward, X } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import {
    computeLyricOpacity,
    isWaitingForFirstLyric,
    parsePlainStanzas,
  } from "../../lib/lyricsFormat";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";
  import LyricsCardModal from "./LyricsCardModal.svelte";

  const MAX_SELECTED_LINES = 5;

  let lyricRefs = $state<Record<number, HTMLButtonElement>>({});
  let autoScrollPaused = $state(false);
  let lastTrackId = $state<string | null>(null);

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
    isWaitingForFirstLyric(player.lyrics, player.activeLyricIndex, player.positionSecs),
  );

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
      lyricRefs[idx]?.scrollIntoView({ behavior: "smooth", block: "center" });
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

  const INSTRUMENTAL_ONLY_RE = /^[\s♪♫♬♩•·…\-—]+$/;

  function isSelectableLine(idx: number): boolean {
    if (player.lyrics.type === "Synced") {
      const text = player.lyrics.lines[idx]?.text ?? "";
      return text.trim().length > 0 && !INSTRUMENTAL_ONLY_RE.test(text);
    }
    if (player.lyrics.type === "Plain") {
      const text = plainParsed.selectable[idx] ?? "";
      return text.trim().length > 0 && !INSTRUMENTAL_ONLY_RE.test(text);
    }
    return false;
  }

  function buildContiguousRange(from: number, to: number): number[] {
    const start = Math.min(from, to);
    const end = Math.max(from, to);
    const out: number[] = [];
    for (let i = start; i <= end; i++) {
      if (isSelectableLine(i)) out.push(i);
    }
    return out;
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
    if (!isSelectableLine(idx)) return;
    if (selectedIndices.length === 0) {
      selectedIndices = [idx];
      return;
    }

    const sorted = [...selectedIndices].sort((a, b) => a - b);
    const min = sorted[0];
    const max = sorted[sorted.length - 1];

    if (sorted.includes(idx)) {
      if (sorted.length === 1) {
        selectedIndices = [];
      } else if (idx === min) {
        selectedIndices = sorted.slice(1);
      } else if (idx === max) {
        selectedIndices = sorted.slice(0, -1);
      } else {
        // Trim from tapped interior line onward to keep a single contiguous block
        selectedIndices = sorted.slice(0, sorted.indexOf(idx) + 1);
      }
      return;
    }

    const candidate = buildContiguousRange(Math.min(min, idx), Math.max(max, idx));
    if (candidate.length <= MAX_SELECTED_LINES) {
      // Fill contiguously between existing selection and tapped line
      selectedIndices = candidate;
    } else {
      // Tapped far away (> MAX_SELECTED_LINES span): move selection to the new line
      selectedIndices = [idx];
    }
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
      {#if hasLyrics}
        <button
          type="button"
          class="w-7 h-7 rounded-full inline-flex items-center justify-center transition-all active:scale-95 {selecting
            ? 'bg-[var(--accent)] text-white'
            : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] border border-[var(--border)]'}"
          title={t("lyrics.share_card")}
          aria-label={t("lyrics.share_card")}
          onclick={() => (selecting ? cancelSelecting() : startSelecting())}
        >
          {#if selecting}
            <X size={13} />
          {:else}
            <Share2 size={13} />
          {/if}
        </button>
      {/if}

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
    </div>
  </div>

  {#if selecting}
    <div class="flex items-center justify-between gap-2 px-3 py-1.5 rounded-xl bg-[var(--accent-soft)]/70 border border-[var(--accent-border)]/40 text-[11.5px] font-semibold text-[var(--accent-bright)]">
      <span>{t("lyrics.select_hint", { max: MAX_SELECTED_LINES })}</span>
      <span class="numeric opacity-85 shrink-0">{selectedIndices.length}/{MAX_SELECTED_LINES}</span>
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
                player.seek(line.time_secs);
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
            <span class="flex-1">{line.text}</span>
          </button>
        {/each}
      </div>

      {#if selecting}
        <div
          transition:scale={{ start: 0.94, duration: 150, easing: cubicOut }}
          class="absolute bottom-3 inset-x-0 flex items-center justify-between gap-2 px-3 py-2 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_12px_32px_rgba(0,0,0,0.45)]"
        >
          <span class="text-[12px] font-bold numeric text-[var(--text-muted)] pl-1">
            {selectedIndices.length}/{MAX_SELECTED_LINES}
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
        <div
          transition:scale={{ start: 0.94, duration: 150, easing: cubicOut }}
          class="absolute bottom-3 inset-x-0 flex items-center justify-between gap-2 px-3 py-2 rounded-2xl bg-[var(--bg-elevated)]/95 backdrop-blur-md border border-[var(--border-strong)] shadow-[0_12px_32px_rgba(0,0,0,0.45)]"
        >
          <span class="text-[12px] font-bold numeric text-[var(--text-muted)] pl-1">
            {selectedIndices.length}/{MAX_SELECTED_LINES}
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

<LyricsCardModal
  open={cardModalOpen}
  lines={selectedLineTexts}
  onClose={() => (cardModalOpen = false)}
/>

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
    will-change: transform, opacity;
    transition:
      opacity 240ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 240ms cubic-bezier(0.22, 1, 0.36, 1),
      background-color 180ms ease,
      border-color 180ms ease,
      color 220ms ease;
  }

  .sb-lyric-line:hover {
    opacity: 0.94 !important;
    background: color-mix(in srgb, var(--surface-hover) 60%, transparent);
  }

  .sb-lyric-line.active-line {
    transform: scale(1.03);
    color: var(--text);
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
