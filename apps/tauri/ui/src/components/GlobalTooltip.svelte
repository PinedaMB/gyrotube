<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { device } from "../lib/stores/device.svelte";

  let visible = $state(false);
  let text = $state("");
  let x = $state(0);
  let y = $state(0);
  let placement = $state<"top" | "bottom">("top");

  let activeEl: HTMLElement | null = null;
  let savedTitle: string | null = null;
  let showTimer: number | undefined;
  let lastHideAt = 0;

  function restoreTitle() {
    if (activeEl && savedTitle !== null) {
      if (!activeEl.hasAttribute("title")) {
        activeEl.setAttribute("title", savedTitle);
      }
    }
    activeEl = null;
    savedTitle = null;
  }

  function hide() {
    window.clearTimeout(showTimer);
    showTimer = undefined;
    if (visible) {
      visible = false;
      lastHideAt = performance.now();
    }
    restoreTitle();
  }

  function positionFor(el: HTMLElement) {
    const rect = el.getBoundingClientRect();
    const estWidth = Math.min(280, Math.max(80, text.length * 6.8 + 22));
    const center = rect.left + rect.width / 2;
    const clampedX = Math.max(
      12 + estWidth / 2,
      Math.min(window.innerWidth - 12 - estWidth / 2, center),
    );
    x = clampedX;
    if (rect.top < 46) {
      placement = "bottom";
      y = rect.bottom + 8;
    } else {
      placement = "top";
      y = rect.top - 8;
    }
  }

  function handlePointerOver(e: PointerEvent) {
    if (device.isTouch || e.pointerType === "touch") return;
    const raw = e.target as HTMLElement | null;
    const el = raw?.closest<HTMLElement>("[title], [data-tooltip]") ?? null;
    if (!el || el === activeEl) return;

    const tip = (el.getAttribute("title") ?? el.getAttribute("data-tooltip") ?? "").trim();
    if (!tip) return;

    // For non-interactive text elements (like truncated titles), only show tooltip if actually truncated
    const isInteractive = el.matches("button, a, input, [role='button'], [role='menuitem']");
    if (!isInteractive && el.scrollWidth <= el.clientWidth + 1) {
      return;
    }

    hide();
    activeEl = el;
    if (el.hasAttribute("title")) {
      savedTitle = el.getAttribute("title");
      el.removeAttribute("title");
    }

    const delay = performance.now() - lastHideAt < 240 ? 70 : 320;
    showTimer = window.setTimeout(() => {
      if (activeEl !== el || !document.contains(el)) return;
      text = tip;
      positionFor(el);
      visible = true;
    }, delay);
  }

  function handlePointerOut(e: PointerEvent) {
    if (!activeEl) return;
    const next = e.relatedTarget as Node | null;
    if (next && activeEl.contains(next)) return;
    hide();
  }

  onMount(() => {
    const onScroll = () => hide();
    document.addEventListener("pointerover", handlePointerOver, true);
    document.addEventListener("pointerout", handlePointerOut, true);
    document.addEventListener("pointerdown", hide, true);
    document.addEventListener("scroll", onScroll, true);
    window.addEventListener("blur", hide);
    window.addEventListener("keydown", hide);
    return () => {
      hide();
      document.removeEventListener("pointerover", handlePointerOver, true);
      document.removeEventListener("pointerout", handlePointerOut, true);
      document.removeEventListener("pointerdown", hide, true);
      document.removeEventListener("scroll", onScroll, true);
      window.removeEventListener("blur", hide);
      window.removeEventListener("keydown", hide);
    };
  });
</script>

{#if visible && text}
  <div
    transition:fade={{ duration: 110 }}
    role="tooltip"
    class="fixed z-[200] pointer-events-none select-none max-w-[280px] px-2.5 py-1 rounded-lg bg-[var(--bg-elevated)] border border-[var(--border-strong)] text-[12px] font-medium leading-snug text-[var(--text)] shadow-[0_10px_28px_rgba(0,0,0,0.45)] whitespace-nowrap truncate"
    style="left: {x}px; top: {y}px; transform: translate(-50%, {placement === 'top' ? '-100%' : '0'});"
  >
    {text}
  </div>
{/if}
