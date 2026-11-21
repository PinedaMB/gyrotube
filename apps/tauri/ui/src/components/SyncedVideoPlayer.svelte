<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Film, Maximize2, Minimize2 } from "lucide-svelte";
  import { onMount } from "svelte";
  import { isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";

  interface Props {
    class?: string;
  }

  let { class: className = "" }: Props = $props();

  let videoEl = $state<HTMLVideoElement | null>(null);
  let ready = $state(false);
  let playbackError = $state(false);
  let isFullscreen = $state(false);
  let hiddenDoc = $state(typeof document !== "undefined" ? document.hidden : false);
  let lastSeekAt = 0;
  let notifiedReadyForUrl: string | null = null;

  function syncPlayback(forceSeek = false) {
    const el = videoEl;
    if (!el || !player.videoStream) return;
    if (hiddenDoc || !player.isPlaying) {
      if (!el.paused) el.pause();
      if (el.playbackRate !== 1) el.playbackRate = 1;
      return;
    }
    if (el.seeking) return;

    const target = Math.max(0, player.getExactPositionSecs());
    if (el.readyState >= 1) {
      const clamped = Number.isFinite(el.duration)
        ? Math.min(target, Math.max(0, el.duration - 0.1))
        : target;
      const signedDrift = clamped - el.currentTime;
      const drift = Math.abs(signedDrift);
      const now = Date.now();

      if (forceSeek || (drift > 0.6 && now - lastSeekAt > 600)) {
        lastSeekAt = now;
        if (el.playbackRate !== 1) el.playbackRate = 1;
        try {
          el.currentTime = clamped;
        } catch {
          // ignore seek before buffered
        }
      } else if (drift > 0.08) {
        el.playbackRate = signedDrift > 0 ? 1.06 : 0.94;
      } else if (drift <= 0.05 && el.playbackRate !== 1) {
        el.playbackRate = 1;
      }
    }
    if (el.paused) {
      void el.play().catch(() => {});
    }
  }

  function handleVideoReady() {
    const el = videoEl;
    const url = player.videoStream?.url ?? null;
    if (!el || !url) return;
    ready = true;
    const target = Math.max(0, player.getExactPositionSecs());
    if (el.readyState >= 1 && Math.abs(el.currentTime - target) > 0.12 && !el.seeking) {
      const clamped = Number.isFinite(el.duration)
        ? Math.min(target, Math.max(0, el.duration - 0.1))
        : target;
      try {
        lastSeekAt = Date.now();
        el.currentTime = clamped;
      } catch {
        // ignore seek before buffered
      }
    }
    if (notifiedReadyForUrl !== url) {
      notifiedReadyForUrl = url;
      player.notifyVideoReady(el);
    }
    syncPlayback(false);
  }

  $effect(() => {
    void player.videoStream?.url;
    ready = false;
    playbackError = false;
    lastSeekAt = 0;
    notifiedReadyForUrl = null;
  });

  $effect(() => {
    void player.isPlaying;
    void player.positionSecs;
    void hiddenDoc;
    void player.videoStream?.url;
    syncPlayback(false);
  });

  $effect(() => {
    if (!player.isPlaying || !ready || hiddenDoc || !player.videoStream) return;
    const timer = window.setInterval(() => syncPlayback(false), 120);
    return () => window.clearInterval(timer);
  });

  function setFullscreen(next: boolean) {
    if (isFullscreen === next) return;
    isFullscreen = next;
    // Use Tauri window fullscreen + fixed viewport overlay instead of
    // videoEl.requestFullscreen(), which triggers a WebKitGTK/GStreamer
    // compositor freeze & flicker bug on Linux.
    if (isTauriRuntime()) {
      void getCurrentWindow().setFullscreen(next).catch(() => {});
    }
  }

  function toggleFullscreen() {
    setFullscreen(!isFullscreen);
  }

  onMount(() => {
    const onVis = () => {
      hiddenDoc = document.hidden;
      if (!document.hidden) syncPlayback(true);
    };
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && isFullscreen) {
        e.preventDefault();
        setFullscreen(false);
      }
    };
    document.addEventListener("visibilitychange", onVis);
    window.addEventListener("keydown", onKeyDown);
    return () => {
      document.removeEventListener("visibilitychange", onVis);
      window.removeEventListener("keydown", onKeyDown);
      if (isFullscreen && isTauriRuntime()) {
        void getCurrentWindow().setFullscreen(false).catch(() => {});
      }
    };
  });
</script>

<div
  class={isFullscreen
    ? "group fixed inset-0 z-[80] w-screen h-screen bg-black flex items-center justify-center overflow-hidden"
    : `group relative overflow-hidden bg-black/80 border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)] flex items-center justify-center ${className}`}
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
      ondblclick={toggleFullscreen}
      onloadeddata={handleVideoReady}
      oncanplay={handleVideoReady}
      onerror={() => {
        ready = false;
        playbackError = true;
        player.notifyVideoUnavailable();
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
      onclick={toggleFullscreen}
    >
      {#if isFullscreen}
        <Minimize2 size={14} />
      {:else}
        <Maximize2 size={14} />
      {/if}
    </button>
  {:else}
    <div class="flex flex-col items-center justify-center gap-2 px-4 text-center text-[var(--text-muted)]">
      <Film size={24} class="opacity-45" />
      <span class="text-[12px] font-medium">{t("player.video_unavailable")}</span>
    </div>
  {/if}
</div>
