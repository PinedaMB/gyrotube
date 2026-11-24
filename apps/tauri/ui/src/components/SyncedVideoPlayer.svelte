<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import {
    Film,
    Heart,
    Maximize2,
    Minimize2,
    Pause,
    Play,
    SkipBack,
    SkipForward,
    Volume2,
    VolumeX,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { device } from "../lib/stores/device.svelte";
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";

  interface Props {
    class?: string;
  }

  let { class: className = "" }: Props = $props();

  let videoEl = $state<HTMLVideoElement | null>(null);
  let ready = $state(false);
  let playbackError = $state(false);
  let isFullscreen = $state(false);
  let hudVisible = $state(true);
  let hudHovered = $state(false);
  let seeking = $state(false);
  let seekDraft = $state(0);
  let savedVolume = $state(1);
  let hiddenDoc = $state(typeof document !== "undefined" ? document.hidden : false);
  let lastSeekAt = 0;
  let notifiedReadyForUrl: string | null = null;
  let hideHudTimer: number | null = null;

  let displayPos = $derived(seeking ? seekDraft : player.positionSecs);
  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (displayPos / player.durationSecs) * 100))
      : 0,
  );
  let volumePct = $derived(Math.round(player.volume * 100));
  let isFav = $derived(player.currentTrack ? player.isFavorite(player.currentTrack.id) : false);
  let showFullscreenHud = $derived(isFullscreen && (hudVisible || hudHovered || !player.isPlaying || seeking));

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

  function scheduleHudHide() {
    if (hideHudTimer !== null) {
      window.clearTimeout(hideHudTimer);
      hideHudTimer = null;
    }
    hudVisible = true;
    if (!isFullscreen) return;
    hideHudTimer = window.setTimeout(() => {
      hideHudTimer = null;
      if (isFullscreen && player.isPlaying && !hudHovered && !seeking) {
        hudVisible = false;
      }
    }, 2800);
  }

  function toggleMute() {
    if (player.volume > 0) {
      savedVolume = player.volume;
      player.setVolume(0);
    } else {
      player.setVolume(savedVolume > 0.05 ? savedVolume : 0.8);
    }
  }

  function setFullscreen(next: boolean) {
    if (isFullscreen === next) return;
    isFullscreen = next;
    hudHovered = false;
    if (next) {
      scheduleHudHide();
    } else if (hideHudTimer !== null) {
      window.clearTimeout(hideHudTimer);
      hideHudTimer = null;
      hudVisible = true;
    }
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
      if (!isFullscreen) return;
      const target = e.target as HTMLElement | null;
      if (target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA")) return;

      if (e.key === "Escape") {
        e.preventDefault();
        setFullscreen(false);
      } else if (e.key === " ") {
        e.preventDefault();
        scheduleHudHide();
        void player.togglePlay();
      } else if (e.key === "ArrowRight") {
        e.preventDefault();
        scheduleHudHide();
        player.seek(Math.min(player.durationSecs || 0, player.getExactPositionSecs() + 5));
      } else if (e.key === "ArrowLeft") {
        e.preventDefault();
        scheduleHudHide();
        player.seek(Math.max(0, player.getExactPositionSecs() - 5));
      } else if (e.key === "f" || e.key === "F") {
        e.preventDefault();
        setFullscreen(false);
      }
    };
    document.addEventListener("visibilitychange", onVis);
    window.addEventListener("keydown", onKeyDown);
    return () => {
      if (hideHudTimer !== null) window.clearTimeout(hideHudTimer);
      document.removeEventListener("visibilitychange", onVis);
      window.removeEventListener("keydown", onKeyDown);
      if (isFullscreen && isTauriRuntime()) {
        void getCurrentWindow().setFullscreen(false).catch(() => {});
      }
    };
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class={isFullscreen
    ? `group fixed inset-0 z-[80] w-screen h-screen bg-black flex items-center justify-center overflow-hidden select-none ${showFullscreenHud ? "" : "cursor-none"}`
    : `group relative overflow-hidden bg-black/80 border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)] flex items-center justify-center ${className}`}
  onmousemove={() => {
    if (isFullscreen) scheduleHudHide();
  }}
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
      onclick={() => {
        if (isFullscreen) {
          scheduleHudHide();
          void player.togglePlay();
        }
      }}
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

    {#if isFullscreen}
      <!-- Top fullscreen HUD -->
      <div
        class="absolute inset-x-0 top-0 px-6 pt-5 pb-12 bg-gradient-to-b from-black/80 via-black/35 to-transparent flex items-start justify-between gap-4 transition-opacity duration-200 {showFullscreenHud
          ? 'opacity-100'
          : 'opacity-0 pointer-events-none'}"
        onmouseenter={() => (hudHovered = true)}
        onmouseleave={() => {
          hudHovered = false;
          scheduleHudHide();
        }}
      >
        <div class="min-w-0">
          {#if player.currentTrack}
            <div class="text-[18px] font-extrabold text-white truncate drop-shadow">
              {player.currentTrack.title}
            </div>
            <div class="text-[13.5px] font-medium text-white/75 truncate">
              {player.currentTrack.artist}
            </div>
          {/if}
        </div>
        <div class="flex items-center gap-2 shrink-0">
          <span class="px-2.5 py-1 rounded-md text-[11px] font-bold bg-black/60 text-white/90 border border-white/15 numeric">
            {player.videoStream.quality_label}
          </span>
          <button
            type="button"
            class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-black/60 hover:bg-black/85 text-white border border-white/15 transition-colors"
            title={t("player.video_fullscreen")}
            onclick={toggleFullscreen}
          >
            <Minimize2 size={16} />
          </button>
        </div>
      </div>

      <!-- Bottom fullscreen HUD -->
      <div
        class="absolute inset-x-0 bottom-0 px-6 pb-6 pt-14 bg-gradient-to-t from-black/85 via-black/45 to-transparent flex flex-col gap-3 transition-opacity duration-200 {showFullscreenHud
          ? 'opacity-100'
          : 'opacity-0 pointer-events-none'}"
        onmouseenter={() => (hudHovered = true)}
        onmouseleave={() => {
          hudHovered = false;
          scheduleHudHide();
        }}
      >
        <!-- Seekbar -->
        <div class="flex items-center gap-3">
          <span class="text-[12px] font-semibold text-white/80 numeric min-w-[38px]">
            {formatDuration(displayPos)}
          </span>
          <input
            type="range"
            class="flex-1 h-1.5"
            min="0"
            max={Math.max(1, player.durationSecs)}
            step="0.5"
            value={displayPos}
            aria-label={t("a11y.seek")}
            style="background: linear-gradient(to right, var(--accent-bright) {progressPct}%, rgba(255,255,255,0.28) {progressPct}%);"
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
              scheduleHudHide();
            }}
          />
          <span class="text-[12px] font-semibold text-white/80 numeric min-w-[38px] text-right">
            {formatDuration(player.durationSecs)}
          </span>
        </div>

        <!-- Transport & Volume row -->
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-2 min-w-[140px]">
            {#if player.currentTrack}
              <button
                type="button"
                class="w-9 h-9 rounded-full inline-flex items-center justify-center hover:bg-white/15 transition-colors {isFav
                  ? 'text-[var(--fav-coral)]'
                  : 'text-white/80 hover:text-white'}"
                title={isFav ? t("player.favorite_remove") : t("player.favorite_add")}
                onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
              >
                <Heart size={18} fill={isFav ? "currentColor" : "none"} />
              </button>
            {/if}
          </div>

          <div class="flex items-center gap-4">
            <button
              type="button"
              class="w-10 h-10 rounded-full inline-flex items-center justify-center text-white/90 hover:text-white hover:bg-white/15 transition-colors"
              title={t("player.previous")}
              onclick={() => void player.prev()}
            >
              <SkipBack size={21} fill="currentColor" />
            </button>
            <button
              type="button"
              class="play-circle !w-12 !h-12"
              title={t("player.play_pause")}
              onclick={() => void player.togglePlay()}
            >
              {#if player.isPlaying}
                <Pause size={21} fill="currentColor" />
              {:else}
                <Play size={21} fill="currentColor" class="translate-x-[1.5px]" />
              {/if}
            </button>
            <button
              type="button"
              class="w-10 h-10 rounded-full inline-flex items-center justify-center text-white/90 hover:text-white hover:bg-white/15 transition-colors"
              title={t("player.next")}
              onclick={() => void player.next()}
            >
              <SkipForward size={21} fill="currentColor" />
            </button>
          </div>

          <div class="flex items-center justify-end gap-2.5 min-w-[140px]">
            {#if !device.isAndroid}
              <button
                type="button"
                class="w-8 h-8 rounded-full inline-flex items-center justify-center text-white/85 hover:text-white hover:bg-white/15 transition-colors"
                title={t("player.volume")}
                onclick={toggleMute}
              >
                {#if player.volume === 0}
                  <VolumeX size={17} />
                {:else}
                  <Volume2 size={17} />
                {/if}
              </button>
              <input
                type="range"
                class="w-24"
                min="0"
                max="1"
                step="0.02"
                value={player.volume}
                aria-label={t("player.volume")}
                style="background: linear-gradient(to right, var(--accent-bright) {volumePct}%, rgba(255,255,255,0.28) {volumePct}%);"
                oninput={(e) => player.setVolume(Number((e.currentTarget as HTMLInputElement).value))}
              />
            {/if}
          </div>
        </div>
      </div>
    {:else}
      <div class="absolute top-2.5 left-2.5 px-2 py-0.5 rounded-md text-[10.5px] font-bold bg-black/65 text-white/90 border border-white/10 numeric pointer-events-none">
        {player.videoStream.quality_label}
      </div>

      <button
        type="button"
        class="absolute bottom-2.5 right-2.5 w-8 h-8 rounded-full inline-flex items-center justify-center bg-black/65 hover:bg-black/85 text-white border border-white/15 opacity-0 group-hover:opacity-100 focus-visible:opacity-100 transition-opacity"
        title={t("player.video_fullscreen")}
        onclick={toggleFullscreen}
      >
        <Maximize2 size={14} />
      </button>
    {/if}
  {:else}
    <div class="flex flex-col items-center justify-center gap-2 px-4 text-center text-white/75">
      <Film size={24} class="opacity-65" />
      <span class="text-[12px] font-medium">{t("player.video_unavailable")}</span>
    </div>
  {/if}
</div>
