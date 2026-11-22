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
    ? 'py-10 px-4 gap-2.5'
    : 'min-h-[52vh] py-16 px-6 gap-3.5'}"
>
  <span
    class="{compact
      ? 'w-11 h-11'
      : 'w-14 h-14'} rounded-2xl bg-[var(--danger)]/14 border border-[var(--danger)]/30 text-[var(--danger)] inline-flex items-center justify-center"
  >
    <WifiOff size={compact ? 20 : 26} />
  </span>

  <div class="space-y-1 max-w-sm">
    <h2 class="{compact ? 'text-[15px]' : 'text-[20px]'} font-extrabold tracking-tight">
      {t("network.offline_title")}
    </h2>
    <p class="{compact ? 'text-[12.5px]' : 'text-[13.5px]'} text-[var(--text-muted)] leading-relaxed">
      {message ?? t("network.offline_desc")}
    </p>
  </div>

  {#if showCollectionCta}
    <button
      type="button"
      class="pill-btn primary mt-1 !px-4 !py-2"
      onclick={() => player.navigateTab("collection")}
    >
      <Library size={15} />
      {t("network.go_collection")}
    </button>
  {/if}
</div>
