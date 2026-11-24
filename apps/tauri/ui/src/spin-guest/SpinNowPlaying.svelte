<script lang="ts">
  import {
    Disc3,
    Headphones,
    Music,
    Pause,
    Play,
    SkipBack,
    SkipForward,
    Volume1,
    Volume2,
    VolumeX,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { sg } from "./i18n";
  import { fmtTime, type JamTrackInfo } from "./types";

  interface Props {
    nowPlaying: JamTrackInfo | null;
    isPlaying: boolean;
    allowControls: boolean;
    smoothPos: number;
    durationSecs: number;
    listenHere: boolean;
    localVolume: number;
    onControl: (action: "play_pause" | "next" | "prev") => void;
    onToggleListen: () => void;
    onVolumeChange: (v: number) => void;
  }

  let {
    nowPlaying,
    isPlaying,
    allowControls,
    smoothPos,
    durationSecs,
    listenHere,
    localVolume,
    onControl,
    onToggleListen,
    onVolumeChange,
  }: Props = $props();

  let progressRatio = $derived(
    durationSecs > 0 ? Math.max(0, Math.min(1, smoothPos / durationSecs)) : 0,
  );
</script>

<section
  class="rounded-2xl border border-[var(--border)] p-4 sm:p-5 shadow-[0_14px_38px_rgba(0,0,0,0.34)] relative overflow-hidden"
  style="background: linear-gradient(160deg, rgba(16, 185, 129, 0.14) 0%, var(--bg-elevated) 58%);"
  aria-label={sg("now.label")}
>
  <div class="flex items-center justify-between gap-2 mb-3.5">
    <span class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)]">
      {sg("now.label")}
    </span>
    {#if nowPlaying}
      <span
        class="inline-flex items-center gap-1.5 text-[11px] font-bold uppercase tracking-[0.06em] px-2 py-0.5 rounded-full {isPlaying
          ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
          : 'bg-white/[0.06] text-[var(--text-muted)]'}"
      >
        {#if isPlaying}
          <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] animate-pulse"></span>
          {sg("now.playing")}
        {:else}
          {sg("now.paused")}
        {/if}
      </span>
    {/if}
  </div>

  <!-- Cover + Metadata -->
  <div class="flex lg:flex-col items-center lg:text-center gap-3.5 lg:gap-4">
    <div class="relative w-[68px] h-[68px] lg:w-[208px] lg:h-[208px] rounded-xl lg:rounded-2xl overflow-hidden bg-[#16141c] border border-white/10 shadow-[0_14px_34px_rgba(0,0,0,0.48)] shrink-0">
      {#if nowPlaying?.cover_url}
        {#key nowPlaying.id}
          <img
            in:scale={{ start: 0.95, duration: 220, easing: cubicOut }}
            src={nowPlaying.cover_url}
            alt={nowPlaying.title}
            class="w-full h-full object-cover block"
          />
        {/key}
      {:else}
        <div class="w-full h-full flex items-center justify-center text-[var(--text-muted)]/50">
          <Music size={30} />
        </div>
      {/if}
    </div>

    <div class="min-w-0 flex-1 w-full space-y-1">
      <h1 class="text-[16px] lg:text-[19px] font-extrabold leading-tight tracking-tight truncate">
        {nowPlaying?.title ?? sg("now.idle_title")}
      </h1>
      <p class="text-[13.5px] lg:text-[14.5px] font-medium text-[var(--text-muted)] truncate">
        {nowPlaying?.artist ?? sg("now.idle_sub")}
      </p>
      {#if nowPlaying?.album}
        <div class="inline-flex items-center gap-1.5 max-w-full px-2.5 py-0.5 rounded-full text-[11.5px] text-[var(--text-muted)] bg-white/[0.05] border border-white/[0.07]">
          <Disc3 size={11} class="shrink-0 opacity-75" />
          <span class="truncate">{nowPlaying.album}</span>
        </div>
      {/if}
    </div>
  </div>

  <!-- Progress bar -->
  <div class="mt-4">
    <div class="h-1.5 rounded-full bg-white/[0.11] overflow-hidden">
      <div
        class="h-full w-full rounded-full origin-left transition-transform duration-300 ease-linear"
        style="background: linear-gradient(90deg, var(--accent), var(--accent-bright)); transform: scaleX({progressRatio});"
      ></div>
    </div>
    <div class="flex items-center justify-between mt-1.5 text-[11.5px] text-[var(--text-muted)] numeric">
      <span>{fmtTime(smoothPos)}</span>
      <span>{durationSecs ? fmtTime(durationSecs) : "--:--"}</span>
    </div>
  </div>

  <!-- Controls & Listen Here -->
  <div class="mt-4 flex flex-col items-center gap-3">
    {#if allowControls}
      <div class="flex items-center justify-center gap-3">
        <button
          type="button"
          class="w-10 h-10 rounded-full flex items-center justify-center bg-white/[0.07] border border-white/[0.12] hover:bg-white/[0.13] transition-colors active:scale-95"
          aria-label={sg("now.prev")}
          onclick={() => onControl("prev")}
        >
          <SkipBack size={18} fill="currentColor" />
        </button>
        <button
          type="button"
          class="h-11 px-5 rounded-full inline-flex items-center justify-center gap-2 font-bold text-[14px] text-white bg-[var(--accent)] hover:bg-[var(--accent-bright)] shadow-[0_6px_20px_rgba(16,185,129,0.36)] transition-all active:scale-95"
          aria-label={isPlaying ? sg("now.pause") : sg("now.play")}
          onclick={() => onControl("play_pause")}
        >
          {#if isPlaying}
            <Pause size={18} fill="currentColor" />
            <span>{sg("now.pause")}</span>
          {:else}
            <Play size={18} fill="currentColor" class="translate-x-[1px]" />
            <span>{sg("now.play")}</span>
          {/if}
        </button>
        <button
          type="button"
          class="w-10 h-10 rounded-full flex items-center justify-center bg-white/[0.07] border border-white/[0.12] hover:bg-white/[0.13] transition-colors active:scale-95"
          aria-label={sg("now.next")}
          onclick={() => onControl("next")}
        >
          <SkipForward size={18} fill="currentColor" />
        </button>
      </div>
    {/if}

    <div class="w-full flex flex-col items-center gap-2.5">
      <button
        type="button"
        class="inline-flex items-center justify-center gap-2 px-4 py-2 rounded-full text-[13px] font-semibold border transition-all active:scale-95 {listenHere
          ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
          : 'bg-white/[0.06] border-white/[0.12] text-[var(--text)] hover:bg-white/[0.12]'}"
        onclick={onToggleListen}
      >
        {#if listenHere}
          <VolumeX size={15} />
          <span>{sg("now.mute_here")}</span>
        {:else}
          <Headphones size={15} />
          <span>{sg("now.listen_here")}</span>
        {/if}
      </button>

      {#if listenHere}
        <div
          transition:fade={{ duration: 150 }}
          class="w-full max-w-[240px] flex items-center gap-2.5 px-3 py-1.5 rounded-full bg-black/25 border border-white/[0.08]"
        >
          {#if localVolume === 0}
            <VolumeX size={14} class="text-[var(--text-muted)] shrink-0" />
          {:else if localVolume < 0.5}
            <Volume1 size={14} class="text-[var(--accent-bright)] shrink-0" />
          {:else}
            <Volume2 size={14} class="text-[var(--accent-bright)] shrink-0" />
          {/if}
          <input
            type="range"
            class="spin-range flex-1 min-w-0"
            min="0"
            max="1"
            step="0.02"
            value={localVolume}
            aria-label={sg("now.local_volume")}
            oninput={(e) => onVolumeChange(Number((e.currentTarget as HTMLInputElement).value))}
          />
          <span class="text-[11px] font-semibold text-[var(--text-muted)] numeric w-8 text-right">
            {Math.round(localVolume * 100)}%
          </span>
        </div>
      {/if}
    </div>
  </div>
</section>
