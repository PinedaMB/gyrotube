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
      transition:fade={{ duration: 160 }}
      class="fixed inset-0 z-[99] bg-black/60 backdrop-blur-[2px]"
      role="presentation"
      onclick={() => contextMenu.close()}
    ></div>
  {/if}

  <div
    bind:this={menuEl}
    in:fly={{ y: device.isMobile ? 260 : -4, duration: device.isMobile ? 210 : 110, easing: cubicOut }}
    out:fly={{ y: device.isMobile ? 220 : 0, duration: device.isMobile ? 150 : 90, opacity: device.isMobile ? 1 : 0, easing: cubicOut }}
    role="menu"
    class={device.isMobile
      ? "fixed inset-x-0 bottom-0 z-[100] max-h-[82vh] overflow-y-auto scroll-y rounded-t-[26px] bg-[var(--bg-elevated)] border-t border-[var(--border-strong)] shadow-[0_-18px_52px_rgba(0,0,0,0.68)] px-3.5 pt-2.5 select-none text-[14.5px]"
      : "fixed z-[100] w-[248px] rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_20px_52px_rgba(0,0,0,0.6)] p-1.5 select-none text-[13px]"}
    style={device.isMobile
      ? "padding-bottom: calc(1.15rem + var(--safe-bottom));"
      : `left: ${contextMenu.x}px; top: ${contextMenu.y}px;`}
  >
    {#if device.isMobile}
      <div class="w-9 h-1 rounded-full bg-white/20 mx-auto mb-3" aria-hidden="true"></div>
    {/if}
    <ContextMenuContent payload={contextMenu.payload} />
  </div>
{/if}
