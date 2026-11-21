<script lang="ts">
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import { onMount, type Snippet } from "svelte";
  import { t } from "../lib/i18n.svelte";

  interface Props {
    title?: string;
    id?: string;
    minCardWidth?: number;
    gap?: number;
    children: Snippet;
  }

  let { title, id, minCardWidth = 188, gap = 14, children }: Props = $props();

  let scroller = $state<HTMLDivElement | null>(null);
  let canPrev = $state(false);
  let canNext = $state(false);
  let visibleCols = $state(5);
  let cardWidth = $state(176);

  let scrollRaf: number | null = null;

  function recalcLayout() {
    if (!scroller) return;
    const width = scroller.clientWidth;
    if (width > 0) {
      const cols = Math.max(2, Math.floor((width + gap) / (minCardWidth + gap)));
      const nextWidth = (width - gap * (cols - 1)) / cols;
      if (cols !== visibleCols) visibleCols = cols;
      if (Math.abs(nextWidth - cardWidth) > 0.5) cardWidth = nextWidth;
    }
    updateEdges();
  }

  function updateEdges() {
    if (!scroller) return;
    const { scrollLeft, scrollWidth, clientWidth } = scroller;
    const nextPrev = scrollLeft > 4;
    const nextNext = scrollLeft + clientWidth < scrollWidth - 4;
    if (nextPrev !== canPrev) canPrev = nextPrev;
    if (nextNext !== canNext) canNext = nextNext;
  }

  function handleScroll() {
    if (scrollRaf !== null) return;
    scrollRaf = requestAnimationFrame(() => {
      scrollRaf = null;
      updateEdges();
    });
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
            class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-white/[0.06] border border-white/10 hover:bg-white/[0.14] hover:border-white/20 disabled:opacity-25 disabled:pointer-events-none transition-colors active:scale-95"
            disabled={!canPrev}
            title={t("carousel.prev")}
            aria-label={t("carousel.prev")}
            onclick={() => scrollPage(-1)}
          >
            <ChevronLeft size={16} />
          </button>
          <button
            type="button"
            class="w-8 h-8 rounded-full inline-flex items-center justify-center bg-white/[0.06] border border-white/10 hover:bg-white/[0.14] hover:border-white/20 disabled:opacity-25 disabled:pointer-events-none transition-colors active:scale-95"
            disabled={!canNext}
            title={t("carousel.next")}
            aria-label={t("carousel.next")}
            onclick={() => scrollPage(1)}
          >
            <ChevronRight size={16} />
          </button>
        </div>
      {/if}
    </div>
  {/if}

  <div class="relative">
    <div
      bind:this={scroller}
      class="carousel-track scroll-smooth"
      style="--carousel-card-w: {cardWidth}px; --carousel-gap: {gap}px;"
      onscroll={handleScroll}
    >
      {@render children()}
    </div>
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
