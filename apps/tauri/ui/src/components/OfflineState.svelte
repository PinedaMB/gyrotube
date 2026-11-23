<script lang="ts">
  import { Library, WifiOff } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";

  interface Props {
    compact?: boolean;
    showCollectionCta?: boolean;
    message?: string;
  }

  let { compact = false, showCollectionCta = true, message }: Props = $props();
</script>

<div
  role="status"
  class="w-full flex flex-col items-center justify-center text-center {compact
    ? 'flex-1 py-12 px-4 gap-3'
    : 'min-h-[calc(100dvh-12rem)] py-12 px-6 gap-4'}"
>
  <span
    class="{compact
      ? 'w-11 h-11'
      : 'w-16 h-16'} rounded-2xl bg-[var(--danger)]/12 border border-[var(--danger)]/25 text-[var(--danger)] inline-flex items-center justify-center shadow-[0_8px_28px_rgba(225,29,72,0.12)]"
  >
    <WifiOff size={compact ? 20 : 28} />
  </span>

  <div class="space-y-1.5 max-w-sm">
    <h2 class="{compact ? 'text-[15px]' : 'text-[22px]'} font-extrabold tracking-tight">
      {t("network.offline_title")}
    </h2>
    <p class="{compact ? 'text-[12.5px]' : 'text-[14px]'} text-[var(--text-muted)] leading-relaxed">
      {message ?? t("network.offline_desc")}
    </p>
  </div>

  {#if showCollectionCta}
    <button
      type="button"
      class="pill-btn primary mt-1 !px-5 !py-2.5 !text-[13.5px] shadow-[0_6px_20px_rgba(16,185,129,0.28)]"
      onclick={() => player.navigateTab("collection")}
    >
      <Library size={15} />
      {t("network.go_collection")}
    </button>
  {/if}
</div>
