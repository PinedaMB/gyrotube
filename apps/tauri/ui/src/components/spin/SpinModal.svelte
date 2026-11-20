<script lang="ts">
  import { Globe, Radio, Wifi, X } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import SpinLocalPane from "./SpinLocalPane.svelte";
  import SpinRemotePane from "./SpinRemotePane.svelte";

  let tab = $state<"local" | "remote">("local");

  let isLive = $derived(
    player.spinStatus.local_active || player.spinStatus.remote_role !== "idle",
  );

  function handleKeydown(e: KeyboardEvent) {
    if (player.spinOpen && e.key === "Escape") {
      e.preventDefault();
      player.spinOpen = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if player.spinOpen}
  <div
    transition:fade={{ duration: 160 }}
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) player.spinOpen = false;
    }}
  >
    <div
      transition:scale={{ start: 0.95, duration: 190, easing: cubicOut }}
      role="dialog"
      aria-modal="true"
      aria-label={t("spin.title")}
      class="w-[min(480px,calc(100vw-32px))] max-h-[calc(100vh-48px)] overflow-y-auto rounded-[18px] bg-[var(--bg-elevated)] text-[var(--text)] border border-[var(--border-strong)] shadow-[0_24px_64px_rgba(0,0,0,0.62)]"
    >
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-xl bg-[var(--accent-soft)] text-[var(--accent-bright)] flex items-center justify-center">
            <Radio size={17} />
          </div>
          <h2 class="text-[18px] font-extrabold tracking-tight">{t("spin.title")}</h2>
          {#if isLive}
            <span class="inline-flex items-center gap-1.5 text-[11px] font-bold px-2.5 py-0.5 rounded-full bg-emerald-500/15 text-emerald-300 border border-emerald-500/25">
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
              {t("spin.live")}
            </span>
          {/if}
        </div>
        <button
          type="button"
          class="icon-btn"
          title={t("spin.close")}
          onclick={() => (player.spinOpen = false)}
        >
          <X size={17} />
        </button>
      </div>

      <!-- Body -->
      <div class="p-6 space-y-5">
        <!-- Segmented mode switch -->
        <div class="grid grid-cols-2 gap-1 p-1 rounded-xl bg-black/25 border border-[var(--border)]">
          <button
            type="button"
            class="inline-flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-[13px] font-semibold transition-all {tab === 'local'
              ? 'bg-[var(--accent)] text-white shadow-sm'
              : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
            onclick={() => (tab = "local")}
          >
            <Wifi size={15} />
            {t("spin.tab_local")}
          </button>
          <button
            type="button"
            class="inline-flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-[13px] font-semibold transition-all {tab === 'remote'
              ? 'bg-[var(--accent)] text-white shadow-sm'
              : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
            onclick={() => (tab = "remote")}
          >
            <Globe size={15} />
            {t("spin.tab_remote")}
          </button>
        </div>

        {#if tab === "local"}
          <SpinLocalPane />
        {:else}
          <SpinRemotePane />
        {/if}
      </div>
    </div>
  </div>
{/if}
