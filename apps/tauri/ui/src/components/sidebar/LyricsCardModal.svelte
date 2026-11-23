<script lang="ts">
  import { Check, Copy, Download, Share2, X } from "lucide-svelte";
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
  import { player } from "../../lib/stores/player.svelte";
  import ModalSheet from "../ModalSheet.svelte";

  interface Props {
    open: boolean;
    lines: string[];
    onClose: () => void;
  }

  let { open, lines, onClose }: Props = $props();

  let canvasEl = $state<HTMLCanvasElement | null>(null);
  let theme = $state<LyricsCardThemeId>("cover");
  let aspect = $state<LyricsCardAspect>("portrait");
  let busy = $state(false);
  let copied = $state(false);

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

  async function downloadPng() {
    if (!canvasEl || busy) return;
    busy = true;
    try {
      const blob = await canvasToPngBlob(canvasEl);
      if (!blob) return;
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = lyricsCardFilename(player.currentTrack?.title ?? "lyrics");
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
      player.showToast(t("lyrics.card_saved"));
    } finally {
      busy = false;
    }
  }

  async function shareCard() {
    if (!canvasEl || busy) return;
    busy = true;
    try {
      const blob = await canvasToPngBlob(canvasEl);
      if (!blob) return;
      const filename = lyricsCardFilename(player.currentTrack?.title ?? "lyrics");
      const file = new File([blob], filename, { type: "image/png" });
      if (
        typeof navigator !== "undefined" &&
        typeof navigator.share === "function" &&
        (!navigator.canShare || navigator.canShare({ files: [file] }))
      ) {
        try {
          await navigator.share({
            files: [file],
            title: player.currentTrack?.title ?? "GyroTube",
            text: lines.join("\n"),
          });
          return;
        } catch (err) {
          if ((err as { name?: string })?.name === "AbortError") return;
        }
      }
      // Fallback to PNG download if native file share is unavailable
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
      player.showToast(t("lyrics.card_saved"));
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
        await navigator.clipboard.write([new ClipboardItem({ "image/png": blob })]);
        copied = true;
        player.showToast(t("lyrics.card_copied"));
        window.setTimeout(() => {
          copied = false;
        }, 1800);
      } else {
        await navigator.clipboard.writeText(
          `${lines.join("\n")}\n— ${player.currentTrack?.title ?? ""} · ${player.currentTrack?.artist ?? ""}`,
        );
        copied = true;
        player.showToast(t("lyrics.card_copied"));
        window.setTimeout(() => {
          copied = false;
        }, 1800);
      }
    } catch {
      player.showToast(t("spin.copy_failed"));
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
    <div class="flex justify-center">
      <canvas
        bind:this={canvasEl}
        class="w-full max-w-[330px] h-auto rounded-2xl shadow-[0_20px_50px_rgba(0,0,0,0.45)] {aspect === 'portrait'
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
        <Download size={14} />
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
