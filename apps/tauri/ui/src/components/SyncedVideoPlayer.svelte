<script lang="ts">
  import { Film, Maximize2 } from "lucide-svelte";
  import { onMount } from "svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";

  interface Props {
    class?: string;
  }

  let { class: className = "" }: Props = $props();

  let videoEl = $state<HTMLVideoElement | null>(null);
  let ready = $state(false);
  let playbackError = $state(false);
  let hiddenDoc = $state(typeof document !== "undefined" ? document.hidden : false);

  function syncPlayback(forceSeek = false) {
    const el = videoEl;
    if (!el || !player.videoStream) return;
    if (hiddenDoc || !player.isPlaying) {
      if (!el.paused) el.pause();
      return;
    }
    const target = Math.max(0, player.positionSecs);
    if (el.readyState >= 1) {
      const clamped = Number.isFinite(el.duration) ? Math.min(target, Math.max(0, el.duration - 0.1)) : target;
      const drift = Math.abs(el.currentTime - clamped);
      if (forceSeek || drift > 0.45) {
        try {
          el.currentTime = clamped;
        } catch {
          // ignore seek before buffered
        }
      }
    }
    if (el.paused) {
      void el.play().catch(() => {});
    }
  }

  $effect(() => {
    void player.videoStream?.url;
    ready = false;
    playbackError = false;
  });

  $effect(() => {
    void player.isPlaying;
    void player.positionSecs;
    void hiddenDoc;
    void player.videoStream?.url;
    syncPlayback(false);
  });

  function enterFullscreen() {
    if (!videoEl) return;
    if (document.fullscreenElement) {
      void document.exitFullscreen().catch(() => {});
    } else if (videoEl.requestFullscreen) {
      void videoEl.requestFullscreen().catch(() => {});
    }
  }

  onMount(() => {
    const onVis = () => {
      hiddenDoc = document.hidden;
      if (!document.hidden) syncPlayback(true);
    };
    document.addEventListener("visibilitychange", onVis);
    return () => {
      document.removeEventListener("visibilitychange", onVis);
    };
  });
</script>

<div
  class="group relative overflow-hidden bg-black/80 border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)] flex items-center justify-center {className}"
>
  {#if player.videoLoading}
    <div class="absolute inset-0 flex flex-col items-center justify-center gap-2.5 bg-white/[0.04] animate-pulse">
      <div class="w-10 h-10 rounded-full border-2 border-white/15 border-t-[var(--accent-bright)] animate-spin"></div>
    </div>
  {:else if player.videoStream && !playbackError}
    <video
      bind:this={videoEl}
      src={player.videoStream.url}
      muted
      playsinline
      preload="auto"
      class="w-full h-full object-contain transition-opacity duration-200 {ready ? 'opacity-100' : 'opacity-0'}"
      onloadeddata={() => {
        ready = true;
        syncPlayback(true);
      }}
      oncanplay={() => {
        ready = true;
        syncPlayback(false);
      }}
      onerror={() => {
        ready = false;
        playbackError = true;
      }}
    ></video>

    {#if !ready}
      <div class="absolute inset-0 flex items-center justify-center bg-black/40 pointer-events-none">
        <div class="w-9 h-9 rounded-full border-2 border-white/15 border-t-[var(--accent-bright)] animate-spin"></div>
      </div>
    {/if}

    <div class="absolute top-2.5 left-2.5 px-2 py-0.5 rounded-md text-[10.5px] font-bold bg-black/65 text-white/90 border border-white/10 numeric pointer-events-none">
      {player.videoStream.quality_label}
    </div>

    <button
      type="button"
      class="absolute bottom-2.5 right-2.5 w-8 h-8 rounded-full inline-flex items-center justify-center bg-black/65 hover:bg-black/85 text-white border border-white/15 opacity-0 group-hover:opacity-100 focus-visible:opacity-100 transition-opacity"
      title={t("player.video_fullscreen")}
      onclick={enterFullscreen}
    >
      <Maximize2 size={14} />
    </button>
  {:else}
    <div class="flex flex-col items-center justify-center gap-2 px-4 text-center text-[var(--text-muted)]">
      <Film size={24} class="opacity-45" />
      <span class="text-[12px] font-medium">{t("player.video_unavailable")}</span>
    </div>
  {/if}
</div>
