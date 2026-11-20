<script lang="ts">
  import { Check, Copy } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";

  interface Props {
    kicker: string;
    qrSvg?: string | null;
    value: string;
    copyToast: string;
  }

  let { kicker, qrSvg = null, value, copyToast }: Props = $props();

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
</script>

<div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] p-4 flex flex-col items-center gap-3.5">
  <span class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
    {kicker}
  </span>

  {#if qrSvg}
    <div class="qr-box w-[184px] h-[184px] p-2.5 rounded-2xl bg-white shadow-[0_10px_28px_rgba(0,0,0,0.36)]">
      {@html qrSvg}
    </div>
  {/if}

  <div class="w-full flex items-center gap-2 bg-black/25 border border-[var(--border)] rounded-xl px-3 py-2">
    <code class="flex-1 min-w-0 text-[12px] text-[var(--text)]/90 truncate select-all">
      {value}
    </code>
    <button
      type="button"
      class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-[12px] font-semibold shrink-0 transition-all active:scale-95 {copied
        ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
        : 'bg-white/[0.08] hover:bg-white/[0.15] text-[var(--text)] border border-white/10'}"
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
  </div>
</div>

<style>
  .qr-box :global(svg) {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>
