<script lang="ts">
  import { LocateFixed, Music } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";

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
</script>

<div in:fade={{ duration: 180 }} class="flex flex-col gap-3 pt-1 flex-1 min-h-0">
  <div class="flex items-start justify-between gap-2 pb-2 border-b border-[var(--border)]">
    <div class="min-w-0">
      <h3 class="text-[14.5px] font-bold truncate">
        {player.currentTrack?.title ?? t("lyrics.title")}
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
          {t("lyrics.sync_btn")}
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
        {player.currentTrack ? t("lyrics.none_track") : t("lyrics.none_idle")}
      </p>
    </div>
  {/if}
</div>

<style>
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
</style>
