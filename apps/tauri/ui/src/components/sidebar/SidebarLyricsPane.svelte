<script lang="ts">
  import { Check, LocateFixed, Music, Share2, X } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import CoverImage from "../CoverImage.svelte";
  import LyricsCardModal from "./LyricsCardModal.svelte";

  const MAX_SELECTED_LINES = 5;

  let lyricRefs = $state<Record<number, HTMLButtonElement>>({});
  let autoScrollPaused = $state(false);
  let lastTrackId = $state<string | null>(null);

  let selecting = $state(false);
  let selectedIndices = $state<number[]>([]);
  let cardModalOpen = $state(false);

  let plainLines = $derived.by(() => {
    if (player.lyrics.type !== "Plain") return [];
    return player.lyrics.text
      .split(/\r?\n/)
      .map((l) => l.trim())
      .filter((l) => l.length > 0);
  });

  let hasLyrics = $derived(
    (player.lyrics.type === "Synced" && player.lyrics.lines.length > 0) ||
      (player.lyrics.type === "Plain" && plainLines.length > 0),
  );

  let selectedLineTexts = $derived.by(() => {
    const sorted = [...selectedIndices].sort((a, b) => a - b);
    if (player.lyrics.type === "Synced") {
      return sorted
        .map((idx) => player.lyrics.type === "Synced" ? player.lyrics.lines[idx]?.text ?? "" : "")
        .filter(Boolean);
    }
    if (player.lyrics.type === "Plain") {
      return sorted.map((idx) => plainLines[idx] ?? "").filter(Boolean);
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

  function startSelecting() {
    if (!hasLyrics) return;
    selecting = true;
    autoScrollPaused = true;
    const total =
      player.lyrics.type === "Synced" ? player.lyrics.lines.length : plainLines.length;
    const base =
      player.lyrics.type === "Synced" && player.activeLyricIndex != null
        ? player.activeLyricIndex
        : 0;
    const initial = [base];
    if (base + 1 < total) initial.push(base + 1);
    selectedIndices = initial;
  }

  function cancelSelecting() {
    selecting = false;
    selectedIndices = [];
    cardModalOpen = false;
  }

  function toggleLineSelection(idx: number) {
    if (selectedIndices.includes(idx)) {
      selectedIndices = selectedIndices.filter((i) => i !== idx);
      return;
    }
    if (selectedIndices.length >= MAX_SELECTED_LINES) {
      player.showToast(t("lyrics.select_max", { max: MAX_SELECTED_LINES }));
      return;
    }
    selectedIndices = [...selectedIndices, idx].sort((a, b) => a - b);
  }

  function lyricOpacity(idx: number): number {
    if (selecting) {
      return selectedIndices.includes(idx) ? 1.0 : 0.42;
    }
    if (player.activeLyricIndex == null) return 0.7;
    if (idx === player.activeLyricIndex) return 1.0;
    const diff = Math.abs(idx - player.activeLyricIndex);
    if (diff === 1) return 0.48;
    if (diff === 2) return 0.3;
    return 0.18;
  }
</script>

<div in:fade={{ duration: 150 }} class="flex flex-col gap-2 pt-1 flex-1 min-h-0">
  <!-- Compact track header -->
  <div class="flex items-center justify-between gap-2 pb-3 border-b border-[var(--border)]">
    <div class="flex items-center gap-2.5 min-w-0">
      {#if player.currentTrack}
        <CoverImage
          class="track-cover !w-9 !h-9 !rounded-[8px] shrink-0"
          src={player.currentTrack.cover_url}
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
        <span class="shrink-0 px-2.5 py-0.5 rounded-full text-[10.5px] font-semibold bg-[var(--surface)] text-[var(--text-muted)]">
          {t("lyrics.static")}
        </span>
      {/if}
    </div>
  </div>

  {#if selecting}
    <p class="text-[11.5px] font-medium text-[var(--accent-bright)] px-0.5">
      {t("lyrics.select_hint", { max: MAX_SELECTED_LINES })}
    </p>
  {/if}

  {#if player.lyricsLoading && player.currentTrack}
    <div class="flex-1 flex flex-col gap-3.5 pt-8 px-1 animate-pulse" aria-busy="true">
      {#each [72, 58, 84, 66, 48, 78, 62, 52] as w}
        <div class="h-4 rounded-md bg-[var(--skeleton)]" style="width: {w}%;"></div>
      {/each}
    </div>
  {:else if player.lyrics.type === "Synced"}
    <div class="relative flex-1 min-h-0 flex flex-col">
      <div
        role="region"
        aria-label="Lyrics"
        class="lyrics-fade flex-1 min-h-0 overflow-y-auto overflow-x-hidden pt-8 pb-28 flex flex-col gap-3"
        onwheel={markUserScroll}
        ontouchmove={markUserScroll}
      >
        {#each player.lyrics.lines as line, i}
          {@const active = !selecting && player.activeLyricIndex === i}
          {@const picked = selecting && selectedIndices.includes(i)}
          {@const prevLine = i > 0 ? player.lyrics.lines[i - 1] : null}
          {@const stanzaBreak = prevLine != null && line.time_secs - prevLine.time_secs >= 5.5}
          <button
            type="button"
            bind:this={lyricRefs[i]}
            class="sb-lyric-line text-left origin-left flex items-start gap-2 rounded-xl {active
              ? 'active-line'
              : ''} {picked ? 'selected-line' : ''} {stanzaBreak ? 'mt-3' : ''}"
            style="opacity: {lyricOpacity(i)};"
            onclick={() => {
              if (selecting) {
                toggleLineSelection(i);
              } else {
                autoScrollPaused = false;
                player.seek(line.time_secs);
              }
            }}
          >
            {#if selecting}
              <span
                class="mt-1 w-4 h-4 rounded-full border inline-flex items-center justify-center shrink-0 transition-colors {picked
                  ? 'bg-[var(--accent)] border-[var(--accent)] text-white'
                  : 'border-[var(--border-strong)]'}"
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
      {#if selecting}
        <div class="lyrics-fade flex-1 min-h-0 overflow-y-auto pt-4 pb-24 flex flex-col gap-2">
          {#each plainLines as line, i}
            {@const picked = selectedIndices.includes(i)}
            <button
              type="button"
              class="sb-lyric-line text-left flex items-start gap-2 rounded-xl {picked
                ? 'selected-line'
                : ''}"
              style="opacity: {picked ? 1 : 0.45};"
              onclick={() => toggleLineSelection(i)}
            >
              <span
                class="mt-1 w-4 h-4 rounded-full border inline-flex items-center justify-center shrink-0 transition-colors {picked
                  ? 'bg-[var(--accent)] border-[var(--accent)] text-white'
                  : 'border-[var(--border-strong)]'}"
              >
                {#if picked}
                  <Check size={10} strokeWidth={3} />
                {/if}
              </span>
              <span class="flex-1">{line}</span>
            </button>
          {/each}
        </div>

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
      {:else}
        <div class="lyrics-fade flex-1 min-h-0 overflow-y-auto py-6">
          <pre class="whitespace-pre-wrap text-[15px] font-semibold leading-relaxed text-[var(--text)]/85" style="font-family: var(--lyrics-font);">{player.lyrics.text}</pre>
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
</div>

<LyricsCardModal
  open={cardModalOpen}
  lines={selectedLineTexts}
  onClose={() => (cardModalOpen = false)}
/>

<style>
  .lyrics-fade {
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent 0%, black 9%, black 88%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 9%, black 88%, transparent 100%);
  }

  .lyrics-fade::-webkit-scrollbar {
    display: none;
  }

  .sb-lyric-line {
    font-family: var(--lyrics-font);
    font-size: 16.5px;
    font-weight: 700;
    line-height: 1.4;
    letter-spacing: -0.015em;
    color: var(--text);
    padding: 3px 6px;
    will-change: transform, opacity;
    transition:
      opacity 260ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 260ms cubic-bezier(0.22, 1, 0.36, 1),
      background-color 180ms ease,
      color 260ms ease;
  }

  .sb-lyric-line:hover {
    opacity: 0.9 !important;
  }

  .sb-lyric-line.active-line {
    transform: scale(1.045);
    color: var(--text);
  }

  .sb-lyric-line.selected-line {
    background: var(--accent-soft);
    color: var(--text);
  }
</style>
