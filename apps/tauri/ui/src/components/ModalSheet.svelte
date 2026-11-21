<script lang="ts">
  import type { Snippet } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly, scale } from "svelte/transition";
  import { device } from "../lib/stores/device.svelte";

  interface Props {
    open: boolean;
    label: string;
    maxWidth?: number;
    onClose: () => void;
    children: Snippet;
  }

  let { open, label, maxWidth = 520, onClose, children }: Props = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (open && e.key === "Escape") {
      e.preventDefault();
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div
    transition:fade={{ duration: 150 }}
    class="fixed inset-0 z-50 bg-black/70 {device.isMobile
      ? 'flex items-end justify-center'
      : 'flex items-center justify-center p-4'}"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) onClose();
    }}
  >
    {#if device.isMobile}
      <div
        transition:fly={{ y: 260, duration: 210, easing: cubicOut }}
        role="dialog"
        aria-modal="true"
        aria-label={label}
        class="w-full max-h-[88vh] overflow-y-auto scroll-y rounded-t-3xl bg-[var(--bg-elevated)] text-[var(--text)] border-t border-[var(--border-strong)] shadow-[0_-18px_52px_rgba(0,0,0,0.65)]"
        style="padding-bottom: max(1rem, env(safe-area-inset-bottom));"
      >
        <div class="sticky top-0 z-20 flex justify-center pt-2.5 pb-1 bg-[var(--bg-elevated)]">
          <div class="w-10 h-1 rounded-full bg-white/20" aria-hidden="true"></div>
        </div>
        {@render children()}
      </div>
    {:else}
      <div
        transition:scale={{ start: 0.95, duration: 180, easing: cubicOut }}
        role="dialog"
        aria-modal="true"
        aria-label={label}
        class="max-h-[calc(100vh-48px)] overflow-y-auto scroll-y rounded-[18px] bg-[var(--bg-elevated)] text-[var(--text)] border border-[var(--border-strong)] shadow-[0_24px_64px_rgba(0,0,0,0.62)]"
        style="width: min({maxWidth}px, calc(100vw - 32px));"
      >
        {@render children()}
      </div>
    {/if}
  </div>
{/if}
