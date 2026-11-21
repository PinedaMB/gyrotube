<script lang="ts">
  import type { Snippet } from "svelte";
  import { device } from "../lib/stores/device.svelte";

  interface Props {
    open: boolean;
    label: string;
    maxWidth?: number;
    onClose: () => void;
    children: Snippet;
  }

  let { open, label, maxWidth = 520, onClose, children }: Props = $props();

  let dialogEl = $state<HTMLDialogElement | null>(null);
  let pressedOutside = false;

  const supportsClosedBy =
    typeof HTMLDialogElement !== "undefined" && "closedBy" in HTMLDialogElement.prototype;

  $effect(() => {
    const el = dialogEl;
    if (!el) return;
    if (open) {
      if (!el.open) {
        el.showModal();
      }
    } else if (el.open) {
      el.close();
    }
  });

  function isPointerOutside(e: PointerEvent, el: HTMLDialogElement): boolean {
    const r = el.getBoundingClientRect();
    return e.clientX < r.left || e.clientX > r.right || e.clientY < r.top || e.clientY > r.bottom;
  }

  function handlePointerDown(e: PointerEvent) {
    if (supportsClosedBy || !dialogEl) return;
    pressedOutside = isPointerOutside(e, dialogEl);
  }

  function handlePointerUp(e: PointerEvent) {
    if (supportsClosedBy || !dialogEl) return;
    if (pressedOutside && isPointerOutside(e, dialogEl)) {
      dialogEl.close();
    }
    pressedOutside = false;
  }

  function handleClose() {
    if (dialogEl) {
      dialogEl.returnValue = "";
    }
    if (open) {
      onClose();
    }
  }
</script>

<dialog
  bind:this={dialogEl}
  aria-label={label}
  {...({ closedby: "any" } as Record<string, string>)}
  class="modal-sheet {device.isMobile
    ? 'modal-sheet-mobile w-full max-w-none max-h-[88vh] rounded-t-3xl border-t border-[var(--border-strong)] shadow-[0_-18px_52px_rgba(0,0,0,0.65)]'
    : 'modal-sheet-desktop max-h-[calc(100vh-48px)] rounded-[18px] border border-[var(--border-strong)] shadow-[0_24px_64px_rgba(0,0,0,0.62)]'} overflow-y-auto scroll-y bg-[var(--bg-elevated)] text-[var(--text)] p-0"
  style={device.isMobile
    ? "padding-bottom: max(1rem, env(safe-area-inset-bottom));"
    : `width: min(${maxWidth}px, calc(100vw - 32px));`}
  onpointerdown={handlePointerDown}
  onpointerup={handlePointerUp}
  onclose={handleClose}
>
  {#if open}
    {#if device.isMobile}
      <div class="sticky top-0 z-20 flex justify-center pt-2.5 pb-1 bg-[var(--bg-elevated)]">
        <div class="w-10 h-1 rounded-full bg-[var(--border-strong)]" aria-hidden="true"></div>
      </div>
    {/if}
    {@render children()}
  {/if}
</dialog>

<style>
  dialog.modal-sheet:modal {
    overscroll-behavior: contain;
  }

  dialog.modal-sheet::backdrop {
    background-color: rgba(0, 0, 0, 0.7);
    overflow: hidden;
    overscroll-behavior: contain;
  }

  dialog.modal-sheet-mobile[open] {
    margin: auto 0 0 0;
  }

  dialog.modal-sheet-desktop[open] {
    margin: auto;
  }
</style>
