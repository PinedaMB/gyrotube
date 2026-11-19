<script lang="ts">
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import { onMount, type Snippet } from "svelte";

  interface Props {
    title?: string;
    id?: string;
    minCardWidth?: number;
    gap?: number;
    children: Snippet;
  }

  let { title, id, minCardWidth = 172, gap = 12, children }: Props = $props();

  let scroller = $state<HTMLDivElement | null>(null);
  let canPrev = $state(false);
  let canNext = $state(false);
  let visibleCols = $state(5);
  let cardWidth = $state(176);

  function recalcLayout() {
    if (!scroller) return;
    const width = scroller.clientWidth;
    if (width > 0) {
      const cols = Math.max(2, Math.floor((width + gap) / (minCardWidth + gap)));
      visibleCols = cols;
      cardWidth = (width - gap * (cols - 1)) / cols;
    }
    updateEdges();
  }

  function updateEdges() {
    if (!scroller) return;
    const { scrollLeft, scrollWidth, clientWidth } = scroller;
    canPrev = scrollLeft > 4;
    canNext = scrollLeft + clientWidth < scrollWidth - 4;
  }

  function scrollPage(dir: -1 | 1) {
    if (!scroller) return;
    const pageStep = visibleCols * (cardWidth + gap);
    scroller.scrollBy({ left: dir * pageStep, behavior: "smooth" });
  }

  onMount(() => {
    recalcLayout();
    if (!scroller) return;
    const ro = new ResizeObserver(() => recalcLayout());
    ro.observe(scroller);
    const mo = new MutationObserver(() => updateEdges());
    mo.observe(scroller, { childList: true });
    return () => {
      ro.disconnect();
      mo.disconnect();
    };
  });
</script>

<section class="flex flex-col gap-3" {id}>
  {#if title}
    <div class="flex items-center justify-between gap-2">
      <h2 class="section-title">{title}</h2>
      {#if canPrev || canNext}
        <div class="flex items-center gap-1.5">
          <button
            type="button"
            class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] disabled:opacity-30 transition-all active:scale-95"
            disabled={!canPrev}
            title="Scroll left"
            aria-label="Scroll left"
            onclick={() => scrollPage(-1)}
          >
            <ChevronLeft size={16} />
          </button>
          <button
            type="button"
            class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] disabled:opacity-30 transition-all active:scale-95"
            disabled={!canNext}
            title="Scroll right"
            aria-label="Scroll right"
            onclick={() => scrollPage(1)}
          >
            <ChevronRight size={16} />
          </button>
        </div>
      {/if}
    </div>
  {/if}

  <div class="relative group">
    <div
      bind:this={scroller}
      class="carousel-track scroll-smooth"
      style="--carousel-card-w: {cardWidth}px; --carousel-gap: {gap}px;"
      onscroll={updateEdges}
    >
      {@render children()}
    </div>

    {#if canPrev}
      <button
        type="button"
        class="hidden md:inline-flex items-center justify-center w-9 h-9 rounded-full absolute left-2 top-1/2 -translate-y-1/2 bg-[var(--bg-elevated)]/90 backdrop-blur-md border border-[var(--border-strong)] shadow-xl opacity-0 group-hover:opacity-100 transition-all duration-200 hover:bg-[var(--accent)] hover:text-white hover:border-transparent active:scale-95"
        aria-label="Scroll left"
        onclick={() => scrollPage(-1)}
      >
        <ChevronLeft size={18} />
      </button>
    {/if}

    {#if canNext}
      <button
        type="button"
        class="hidden md:inline-flex items-center justify-center w-9 h-9 rounded-full absolute right-2 top-1/2 -translate-y-1/2 bg-[var(--bg-elevated)]/90 backdrop-blur-md border border-[var(--border-strong)] shadow-xl opacity-0 group-hover:opacity-100 transition-all duration-200 hover:bg-[var(--accent)] hover:text-white hover:border-transparent active:scale-95"
        aria-label="Scroll right"
        onclick={() => scrollPage(1)}
      >
        <ChevronRight size={18} />
      </button>
    {/if}
  </div>
</section>

<style>
  .carousel-track {
    display: flex;
    gap: var(--carousel-gap, 12px);
    overflow-x: auto;
    scroll-snap-type: x mandatory;
    padding-bottom: 4px;
    scrollbar-width: none;
  }

  .carousel-track::-webkit-scrollbar {
    display: none;
  }

  .carousel-track > :global(*) {
    flex: 0 0 var(--carousel-card-w, 176px) !important;
    width: var(--carousel-card-w, 176px) !important;
    scroll-snap-align: start;
  }
</style>
