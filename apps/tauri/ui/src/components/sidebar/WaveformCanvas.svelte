<script lang="ts">
  import { onDestroy } from "svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";

  interface Props {
    peaks: number[];
    durationSecs: number;
    introCueSecs?: number | null;
    outroCueSecs?: number | null;
    showMixMarkers?: boolean;
    transitionProgress?: number | null;
    onSeek: (secs: number) => void;
  }

  let {
    peaks,
    durationSecs,
    introCueSecs = null,
    outroCueSecs = null,
    showMixMarkers = false,
    transitionProgress = null,
    onSeek,
  }: Props = $props();

  let containerEl = $state<HTMLDivElement | null>(null);
  let canvasEl = $state<HTMLCanvasElement | null>(null);
  let width = $state(0);
  let height = $state(40);
  let hoverRatio = $state<number | null>(null);
  let dragging = $state(false);
  let dragRatio = $state(0);
  let rafId: number | null = null;

  function resolveCssVar(name: string, fallback: string): string {
    if (typeof window === "undefined") return fallback;
    const val = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
    return val || fallback;
  }

  function drawFrame() {
    const canvas = canvasEl;
    if (!canvas || width <= 0 || height <= 0) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const dpr = Math.max(1, window.devicePixelRatio || 1);
    const targetW = Math.round(width * dpr);
    const targetH = Math.round(height * dpr);
    if (canvas.width !== targetW || canvas.height !== targetH) {
      canvas.width = targetW;
      canvas.height = targetH;
    }

    ctx.save();
    ctx.scale(dpr, dpr);
    ctx.clearRect(0, 0, width, height);

    const dur = Math.max(0.001, durationSecs);
    const livePos = dragging ? dragRatio * dur : player.getExactPositionSecs();
    const progress = Math.max(0, Math.min(1, livePos / dur));

    const accent = resolveCssVar("--accent-bright", "#10b981");
    const muted = resolveCssVar("--control-track", "rgba(255,255,255,0.16)");
    const hoverColor = resolveCssVar("--text-muted", "rgba(255,255,255,0.45)");

    // Optional Mix transition glow band
    if (transitionProgress !== null && transitionProgress >= 0 && transitionProgress <= 1) {
      const grad = ctx.createLinearGradient(0, 0, width, 0);
      grad.addColorStop(0, "rgba(16, 185, 129, 0.22)");
      grad.addColorStop(Math.max(0.05, Math.min(0.95, transitionProgress)), "rgba(16, 185, 129, 0.08)");
      grad.addColorStop(1, "rgba(16, 185, 129, 0)");
      ctx.fillStyle = grad;
      ctx.fillRect(0, 0, width, height);
    }

    // Draw reduced waveform bars (dynamically bucketed to canvas width for crisp visible gaps)
    const rawCount = peaks.length;
    if (rawCount > 0) {
      const gap = 1.5;
      const targetBarW = 2.5;
      const barCount = Math.max(16, Math.min(rawCount, Math.floor((width + gap) / (targetBarW + gap))));
      const step = width / barCount;
      const barW = Math.max(1.5, step - gap);
      const centerY = height * 0.5;
      const maxBarH = height - 6;

      for (let i = 0; i < barCount; i++) {
        const startIdx = Math.floor((i * rawCount) / barCount);
        const endIdx = Math.max(startIdx + 1, Math.floor(((i + 1) * rawCount) / barCount));
        let peak = 0;
        for (let j = startIdx; j < endIdx && j < rawCount; j++) {
          const v = peaks[j] ?? 0;
          if (v > peak) peak = v;
        }
        const ratio = (i + 0.5) / barCount;
        const amp = Math.max(0.08, Math.min(1, peak));
        const barH = Math.max(3, amp * maxBarH);
        const x = i * step + gap * 0.5;
        const y = centerY - barH * 0.5;

        if (ratio <= progress) {
          ctx.fillStyle = accent;
        } else if (hoverRatio !== null && ratio <= hoverRatio) {
          ctx.fillStyle = hoverColor;
        } else {
          ctx.fillStyle = muted;
        }

        const r = Math.min(barW * 0.5, 2);
        ctx.beginPath();
        ctx.roundRect(x, y, barW, barH, r);
        ctx.fill();
      }
    }

    // Draw intro & outro Mix cue markers when Mix Mode is active
    if (showMixMarkers && durationSecs > 4) {
      ctx.save();
      ctx.setLineDash([2, 2]);
      ctx.lineWidth = 1;
      ctx.strokeStyle = "rgba(16, 185, 129, 0.55)";
      if (introCueSecs != null && introCueSecs > 0.4) {
        const ix = Math.round((introCueSecs / dur) * width) + 0.5;
        ctx.beginPath();
        ctx.moveTo(ix, 2);
        ctx.lineTo(ix, height - 2);
        ctx.stroke();
      }
      if (outroCueSecs != null && outroCueSecs < dur - 1.0) {
        const ox = Math.round((outroCueSecs / dur) * width) + 0.5;
        ctx.beginPath();
        ctx.moveTo(ox, 2);
        ctx.lineTo(ox, height - 2);
        ctx.stroke();
      }
      ctx.restore();
    }

    // Playhead line
    const playheadX = Math.max(1, Math.min(width - 1, progress * width));
    ctx.fillStyle = "#ffffff";
    ctx.shadowColor = accent;
    ctx.shadowBlur = 6;
    ctx.fillRect(playheadX - 1, 1, 2, height - 2);
    ctx.restore();
  }

  function stopRaf() {
    if (rafId !== null) {
      window.cancelAnimationFrame(rafId);
      rafId = null;
    }
  }

  function startRafLoop() {
    stopRaf();
    const tick = () => {
      drawFrame();
      if (player.isPlaying || dragging) {
        rafId = window.requestAnimationFrame(tick);
      } else {
        rafId = null;
      }
    };
    rafId = window.requestAnimationFrame(tick);
  }

  // Responsive ResizeObserver for HiDPI canvas dimensions
  $effect(() => {
    const el = containerEl;
    if (!el) return;
    const initial = el.getBoundingClientRect();
    if (initial.width > 0) {
      width = Math.floor(initial.width);
      height = Math.max(32, Math.floor(initial.height));
      drawFrame();
    }
    if (typeof ResizeObserver === "undefined") return;
    const ro = new ResizeObserver((entries) => {
      const rect = entries[0]?.contentRect;
      if (rect && rect.width > 0) {
        width = Math.floor(rect.width);
        height = Math.max(32, Math.floor(rect.height));
        drawFrame();
      }
    });
    ro.observe(el);
    return () => ro.disconnect();
  });

  // Run rAF loop ONLY while playing or dragging; render single frame when paused
  $effect(() => {
    void peaks;
    void durationSecs;
    void introCueSecs;
    void outroCueSecs;
    void showMixMarkers;
    void transitionProgress;
    void player.positionSecs;
    void player.themeMode;
    void hoverRatio;

    if (player.isPlaying || dragging) {
      startRafLoop();
    } else {
      stopRaf();
      drawFrame();
    }
    return () => stopRaf();
  });

  onDestroy(() => {
    stopRaf();
  });

  function pointerToRatio(e: PointerEvent): number {
    const el = containerEl;
    if (!el) return 0;
    const rect = el.getBoundingClientRect();
    if (rect.width <= 0) return 0;
    return Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
  }

  function handlePointerDown(e: PointerEvent) {
    if (durationSecs <= 0) return;
    dragging = true;
    dragRatio = pointerToRatio(e);
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    drawFrame();
  }

  function handlePointerMove(e: PointerEvent) {
    const ratio = pointerToRatio(e);
    hoverRatio = ratio;
    if (dragging) {
      dragRatio = ratio;
      drawFrame();
    }
  }

  function handlePointerUp(e: PointerEvent) {
    if (!dragging) return;
    const ratio = pointerToRatio(e);
    dragging = false;
    onSeek(ratio * durationSecs);
    drawFrame();
  }
</script>

<div
  bind:this={containerEl}
  role="slider"
  tabindex="0"
  aria-label={t("a11y.seek")}
  aria-valuemin={0}
  aria-valuemax={Math.round(durationSecs)}
  aria-valuenow={Math.round(player.positionSecs)}
  class="relative w-full h-10 cursor-pointer select-none touch-none rounded-lg overflow-hidden"
  onpointerdown={handlePointerDown}
  onpointermove={handlePointerMove}
  onpointerup={handlePointerUp}
  onpointerleave={() => {
    hoverRatio = null;
    if (!dragging) drawFrame();
  }}
  onkeydown={(e) => {
    if (e.key === "ArrowRight") {
      e.preventDefault();
      onSeek(Math.min(durationSecs, player.getExactPositionSecs() + 5));
    } else if (e.key === "ArrowLeft") {
      e.preventDefault();
      onSeek(Math.max(0, player.getExactPositionSecs() - 5));
    }
  }}
>
  <canvas bind:this={canvasEl} class="block w-full h-full"></canvas>
</div>
