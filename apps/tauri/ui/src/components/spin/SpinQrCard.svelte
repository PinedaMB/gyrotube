<script lang="ts">
  import { Check, Copy, Share2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";

  interface Props {
    kicker: string;
    qrSvg?: string | null;
    value: string;
    copyToast: string;
    shareUrl?: string | null;
  }

  let { kicker, qrSvg = null, value, copyToast, shareUrl = null }: Props = $props();

  let copied = $state(false);
  let copyTimer: number | null = null;

  async function handleCopy() {
    if (!value) return;
    try {
      await navigator.clipboard.writeText(value);
      player.showToast(copyToast);
      copied = true;
      if (copyTimer) window.clearTimeout(copyTimer);
      copyTimer = window.setTimeout(() => {
        copied = false;
      }, 1800);
    } catch {
      player.showToast(t("spin.copy_failed"));
    }
  }

  async function handleShare() {
    const link = shareUrl || value;
    if (!link) return;
    if (typeof navigator !== "undefined" && typeof navigator.share === "function") {
      try {
        await navigator.share({
          title: "Gyro SPIN",
          text: value,
          url: link,
        });
        return;
      } catch (err) {
        if ((err as { name?: string })?.name === "AbortError") return;
      }
    }
    try {
      await navigator.clipboard.writeText(link);
      player.showToast(copyToast);
    } catch {
      player.showToast(t("spin.copy_failed"));
    }
  }
</script>

<div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] p-4 flex flex-col items-center gap-3.5">
  <span class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
    {kicker}
  </span>

  {#if qrSvg}
    <div class="qr-box w-[184px] h-[184px] p-2.5 rounded-2xl bg-white border border-[var(--border)] shadow-[0_8px_24px_rgba(15,23,42,0.16)]">
      <img
        src={`data:image/svg+xml;charset=utf-8,${encodeURIComponent(qrSvg)}`}
        alt={kicker}
        class="w-full h-full block"
      />
    </div>
  {/if}

  <div class="w-full flex items-center gap-1.5 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-xl px-3 py-2">
    <code class="flex-1 min-w-0 text-[12px] text-[var(--text)]/90 truncate select-all">
      {value}
    </code>
    <button
      type="button"
      class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-[12px] font-semibold shrink-0 transition-all active:scale-95 {copied
        ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]'
        : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text)] border border-[var(--border)]'}"
      onclick={() => void handleCopy()}
    >
      {#if copied}
        <Check size={13} />
        {t("spin.copied")}
      {:else}
        <Copy size={13} />
        {t("spin.copy")}
      {/if}
    </button>
    {#if shareUrl}
      <button
        type="button"
        class="inline-flex items-center justify-center w-7 h-7 rounded-lg shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text)] border border-[var(--border)] transition-all active:scale-95"
        title={t("spin.share")}
        aria-label={t("spin.share")}
        onclick={() => void handleShare()}
      >
        <Share2 size={13} />
      </button>
    {/if}
  </div>
</div>

<style>
  .qr-box :global(svg) {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>
