<script lang="ts">
  import { LocateFixed, Music } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import CoverImage from "../CoverImage.svelte";

  let lyricRefs = $state<Record<number, HTMLButtonElement>>({});
  let autoScrollPaused = $state(false);
  let lastTrackId = $state<string | null>(null);

  $effect(() => {
    const tid = player.currentTrack?.id ?? null;
    if (tid !== lastTrackId) {
      lastTrackId = tid;
      autoScrollPaused = false;
    }
  });

  $effect(() => {
    const idx = player.activeLyricIndex;
    if (idx != null && !autoScrollPaused) {
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

  function lyricOpacity(idx: number): number {
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
  <div class="flex items-center justify-between gap-2.5 pb-3 border-b border-white/[0.08]">
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

  {#if player.lyrics.type === "Synced"}
    <div class="relative flex-1 min-h-0 flex flex-col">
      <div
        role="region"
        aria-label="Lyrics"
        class="lyrics-fade flex-1 min-h-0 overflow-y-auto overflow-x-hidden pt-10 pb-28 flex flex-col gap-3.5"
        onwheel={markUserScroll}
        ontouchmove={markUserScroll}
      >
        {#each player.lyrics.lines as line, i}
          {@const active = player.activeLyricIndex === i}
          {@const prevLine = i > 0 ? player.lyrics.lines[i - 1] : null}
          {@const stanzaBreak = prevLine != null && line.time_secs - prevLine.time_secs >= 5.5}
          <button
            type="button"
            bind:this={lyricRefs[i]}
            class="sb-lyric-line text-left origin-left {active ? 'active-line' : ''} {stanzaBreak ? 'mt-3' : ''}"
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
    <div class="lyrics-fade flex-1 min-h-0 overflow-y-auto py-6">
      <pre class="whitespace-pre-wrap font-sans text-[14.5px] leading-relaxed text-[var(--text)]/85">{player.lyrics.text}</pre>
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
    font-size: 16.5px;
    font-weight: 700;
    line-height: 1.4;
    letter-spacing: -0.015em;
    color: var(--text);
    padding: 2px 0;
    will-change: transform, opacity;
    transition:
      opacity 260ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 260ms cubic-bezier(0.22, 1, 0.36, 1),
      color 260ms ease;
  }

  .sb-lyric-line:hover {
    opacity: 0.9 !important;
  }

  .sb-lyric-line.active-line {
    transform: scale(1.045);
    color: #ffffff;
  }
</style>
