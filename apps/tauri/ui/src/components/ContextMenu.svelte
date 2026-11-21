<script lang="ts">
  import { cubicOut } from "svelte/easing";
  import { fade, fly } from "svelte/transition";
  import { contextMenu } from "../lib/stores/contextMenu.svelte";
  import { device } from "../lib/stores/device.svelte";
  import ContextMenuContent from "./ContextMenuContent.svelte";

  let menuEl = $state<HTMLDivElement | null>(null);

  function handleWindowPointerDown(e: MouseEvent) {
    if (contextMenu.open && menuEl && !menuEl.contains(e.target as Node)) {
      contextMenu.close();
    }
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (contextMenu.open && e.key === "Escape") {
      e.preventDefault();
      contextMenu.close();
    }
  }
</script>

<svelte:window
  onmousedown={handleWindowPointerDown}
  onkeydown={handleWindowKeydown}
  onblur={() => contextMenu.close()}
/>

{#if contextMenu.open && contextMenu.payload}
  {#if device.isMobile}
    <div
      transition:fade={{ duration: 150 }}
      class="fixed inset-0 z-[99] bg-black/65"
      role="presentation"
      onclick={() => contextMenu.close()}
    ></div>
  {/if}

  <div
    bind:this={menuEl}
    in:fly={{ y: device.isMobile ? 220 : 0, duration: device.isMobile ? 190 : 0, easing: cubicOut }}
    out:fade={{ duration: 110 }}
    role="menu"
    class={device.isMobile
      ? "fixed inset-x-0 bottom-0 z-[100] max-h-[82vh] overflow-y-auto scroll-y rounded-t-3xl bg-[var(--bg-elevated)] border-t border-[var(--border-strong)] shadow-[0_-16px_48px_rgba(0,0,0,0.65)] px-3 pt-2 select-none text-[14.5px]"
      : "fixed z-[100] w-[240px] rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_20px_52px_rgba(0,0,0,0.6)] p-1.5 select-none text-[13px]"}
    style={device.isMobile
      ? "padding-bottom: max(1rem, env(safe-area-inset-bottom));"
      : `left: ${contextMenu.x}px; top: ${contextMenu.y}px;`}
  >
    {#if device.isMobile}
      <div class="w-10 h-1 rounded-full bg-white/20 mx-auto mb-2.5" aria-hidden="true"></div>
    {/if}
    <ContextMenuContent payload={contextMenu.payload} />
  </div>
{/if}
