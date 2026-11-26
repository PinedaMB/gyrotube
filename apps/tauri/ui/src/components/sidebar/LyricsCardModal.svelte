<script lang="ts">
  import { Check, Copy, Download, Share2, X } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import {
    LYRICS_CARD_THEMES,
    canvasToPngBlob,
    coverSwatchGradient,
    lyricsCardFilename,
    renderLyricsCard,
    type LyricsCardAspect,
    type LyricsCardThemeId,
  } from "../../lib/lyricsCard";
  import { device } from "../../lib/stores/device.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import ModalSheet from "../ModalSheet.svelte";

  interface Props {
    open: boolean;
    lines: string[];
    onClose: () => void;
  }

  interface AndroidCardBridge {
    saveImageToGallery?: (filename: string, base64Png: string) => string;
    shareImage?: (filename: string, base64Png: string, title: string) => string;
  }

  let { open, lines, onClose }: Props = $props();

  let canvasEl = $state<HTMLCanvasElement | null>(null);
  let theme = $state<LyricsCardThemeId>("cover");
  let aspect = $state<LyricsCardAspect>("portrait");
  let busy = $state(false);
  let copied = $state(false);
  let saved = $state(false);
  let feedback = $state<{ text: string; kind: "ok" | "err" } | null>(null);
  let feedbackTimer: number | undefined;

  function showModalFeedback(text: string, kind: "ok" | "err" = "ok") {
    feedback = { text, kind };
    window.clearTimeout(feedbackTimer);
    feedbackTimer = window.setTimeout(() => {
      feedback = null;
    }, 2600);
  }

  function getAndroidBridge(): AndroidCardBridge | undefined {
    if (typeof window === "undefined") return undefined;
    return (window as unknown as { __GYROTUBE_ANDROID__?: AndroidCardBridge })
      .__GYROTUBE_ANDROID__;
  }

  function canvasToBase64Png(canvas: HTMLCanvasElement): string {
    const dataUrl = canvas.toDataURL("image/png");
    const comma = dataUrl.indexOf(",");
    return comma >= 0 ? dataUrl.slice(comma + 1) : dataUrl;
  }

  $effect(() => {
    if (!open || !canvasEl || !player.currentTrack || lines.length === 0) return;
    const track = player.currentTrack;
    const coverUrl = player.trackCoverUrl(track, true) ?? track.cover_url ?? null;
    void renderLyricsCard(canvasEl, {
      lines,
      title: track.title,
      artist: track.artist,
      coverUrl,
      ambientRgb: player.ambientRgb,
      theme,
      aspect,
    });
  });

  async function savePngInternal(): Promise<boolean> {
    if (!canvasEl) return false;
    const filename = lyricsCardFilename(player.currentTrack?.title ?? "lyrics");

    // 1. Android native bridge -> saves directly to Pictures/GyroTube in Android Gallery
    const bridge = getAndroidBridge();
    if (device.isAndroid && bridge?.saveImageToGallery) {
      const base64 = canvasToBase64Png(canvasEl);
      const res = bridge.saveImageToGallery(filename, base64);
      if (res === "ok") {
        saved = true;
        showModalFeedback(t("lyrics.card_saved"));
        window.setTimeout(() => (saved = false), 2000);
        return true;
      }
      showModalFeedback(res.replace(/^error:/, "") || t("spin.copy_failed"), "err");
      return false;
    }

    const blob = await canvasToPngBlob(canvasEl);
    if (!blob) return false;

    // 2. Desktop Tauri native command -> saves directly to Pictures/GyroTube
    if (isTauriRuntime() && !device.isAndroid) {
      try {
        const bytes = new Uint8Array(await blob.arrayBuffer());
        await api.saveLyricsCardPng(filename, bytes);
        saved = true;
        showModalFeedback(t("lyrics.card_saved"));
        window.setTimeout(() => (saved = false), 2000);
        return true;
      } catch (err) {
        console.warn("[GyroTube] saveLyricsCardPng failed, falling back to browser download:", err);
      }
    }

    // 3. Browser download fallback
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    a.remove();
    URL.revokeObjectURL(url);
    saved = true;
    showModalFeedback(t("lyrics.card_saved"));
    window.setTimeout(() => (saved = false), 2000);
    return true;
  }

  async function downloadPng() {
    if (!canvasEl || busy) return;
    busy = true;
    try {
      await savePngInternal();
    } catch {
      showModalFeedback(t("spin.copy_failed"), "err");
    } finally {
      busy = false;
    }
  }

  async function shareCard() {
    if (!canvasEl || busy) return;
    busy = true;
    try {
      const filename = lyricsCardFilename(player.currentTrack?.title ?? "lyrics");
      const title = player.currentTrack?.title ?? "Gyro";

      // 1. Android native Share Sheet via FileProvider
      const bridge = getAndroidBridge();
      if (device.isAndroid && bridge?.shareImage) {
        const base64 = canvasToBase64Png(canvasEl);
        const res = bridge.shareImage(filename, base64, title);
        if (res === "ok") {
          return;
        }
      }

      // 2. Web Share API (if supported)
      const blob = await canvasToPngBlob(canvasEl);
      if (!blob) return;
      const file = new File([blob], filename, { type: "image/png" });
      if (
        typeof navigator !== "undefined" &&
        typeof navigator.share === "function" &&
        (!navigator.canShare || navigator.canShare({ files: [file] }))
      ) {
        try {
          await navigator.share({
            files: [file],
            title,
            text: lines.join("\n"),
          });
          return;
        } catch (err) {
          if ((err as { name?: string })?.name === "AbortError") return;
        }
      }

      // 3. Fallback: save PNG natively & notify inside modal
      await savePngInternal();
    } catch {
      showModalFeedback(t("spin.copy_failed"), "err");
    } finally {
      busy = false;
    }
  }

  async function copyCard() {
    if (!canvasEl || busy) return;
    busy = true;
    try {
      const blob = await canvasToPngBlob(canvasEl);
      if (!blob) return;
      if (
        typeof navigator !== "undefined" &&
        navigator.clipboard &&
        typeof ClipboardItem !== "undefined"
      ) {
        try {
          await navigator.clipboard.write([new ClipboardItem({ "image/png": blob })]);
          copied = true;
          showModalFeedback(t("lyrics.card_copied"));
          window.setTimeout(() => (copied = false), 1800);
          return;
        } catch {
          // Some Linux/Android WebViews disallow image clipboard write; fall back to text
        }
      }
      await navigator.clipboard.writeText(
        `${lines.join("\n")}\n— ${player.currentTrack?.title ?? ""} · ${player.currentTrack?.artist ?? ""}`,
      );
      copied = true;
      showModalFeedback(t("lyrics.card_copied"));
      window.setTimeout(() => (copied = false), 1800);
    } catch {
      showModalFeedback(t("spin.copy_failed"), "err");
    } finally {
      busy = false;
    }
  }
</script>

<ModalSheet {open} label={t("lyrics.card_title")} maxWidth={440} {onClose}>
  <div class="flex items-center justify-between px-5 py-3.5 border-b border-[var(--border)]">
    <h2 class="text-[16px] font-extrabold tracking-tight">{t("lyrics.card_title")}</h2>
    <button
      type="button"
      class="icon-btn"
      title={t("spin.close")}
      aria-label={t("spin.close")}
      onclick={onClose}
    >
      <X size={16} />
    </button>
  </div>

  <div class="p-5 space-y-4">
    <!-- Live high-DPI canvas preview -->
    <div class="flex justify-center py-1">
      <canvas
        bind:this={canvasEl}
        class="w-full max-w-[332px] h-auto rounded-2xl shadow-[0_22px_48px_rgba(0,0,0,0.48)] {aspect === 'portrait'
          ? 'aspect-[4/5]'
          : 'aspect-square'}"
      ></canvas>
    </div>

    <!-- Theme & aspect controls -->
    <div class="flex items-center justify-between gap-3">
      <div class="flex items-center gap-2" role="radiogroup" aria-label={t("lyrics.card_theme")}>
        {#each LYRICS_CARD_THEMES as item}
          <button
            type="button"
            role="radio"
            aria-checked={theme === item.id}
            aria-label={item.id}
            class="w-7 h-7 rounded-full border-2 transition-transform active:scale-90 {theme === item.id
              ? 'border-[var(--accent-bright)] scale-110 shadow-md'
              : 'border-white/20 opacity-80 hover:opacity-100'}"
            style="background: {item.id === 'cover' ? coverSwatchGradient(player.ambientRgb) : item.swatch};"
            onclick={() => (theme = item.id)}
          ></button>
        {/each}
      </div>

      <div class="inline-flex items-center rounded-full p-0.5 bg-[var(--surface)] border border-[var(--border)] text-[11.5px] font-bold">
        <button
          type="button"
          class="px-2.5 py-1 rounded-full transition-colors {aspect === 'portrait'
            ? 'bg-[var(--accent)] text-white'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          onclick={() => (aspect = "portrait")}
        >
          4:5
        </button>
        <button
          type="button"
          class="px-2.5 py-1 rounded-full transition-colors {aspect === 'square'
            ? 'bg-[var(--accent)] text-white'
            : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
          onclick={() => (aspect = "square")}
        >
          1:1
        </button>
      </div>
    </div>

    {#if feedback}
      <div
        transition:fade={{ duration: 140 }}
        role="status"
        aria-live="polite"
        class="flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-xl text-[12px] font-semibold border {feedback.kind === 'err'
          ? 'bg-red-500/12 border-red-500/30 text-red-400'
          : 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'}"
      >
        {#if feedback.kind === "ok"}
          <Check size={13} />
        {/if}
        <span>{feedback.text}</span>
      </div>
    {/if}

    <!-- Action buttons -->
    <div class="flex items-center gap-2">
      <button
        type="button"
        class="pill-btn primary flex-1 justify-center !py-2.5"
        disabled={busy}
        onclick={() => void shareCard()}
      >
        <Share2 size={14} />
        {t("spin.share")}
      </button>
      <button
        type="button"
        class="pill-btn justify-center !px-3.5 !py-2.5"
        disabled={busy}
        title={t("lyrics.card_save")}
        onclick={() => void downloadPng()}
      >
        {#if saved}
          <Check size={14} class="text-[var(--accent-bright)]" />
        {:else}
          <Download size={14} />
        {/if}
        PNG
      </button>
      <button
        type="button"
        class="pill-btn justify-center !w-10 !h-10 !p-0 shrink-0"
        disabled={busy}
        title={t("spin.copy")}
        aria-label={t("spin.copy")}
        onclick={() => void copyCard()}
      >
        {#if copied}
          <Check size={14} class="text-[var(--accent-bright)]" />
        {:else}
          <Copy size={14} />
        {/if}
      </button>
    </div>
  </div>
</ModalSheet>
