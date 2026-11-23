<script lang="ts">
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import { colorForGenre } from "../../lib/genrePalette";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Genre } from "../../lib/types";

  let { genres, showHeader = true }: { genres: Genre[]; showHeader?: boolean } = $props();

  let expanded = $state(false);
  let scrollerEl = $state<HTMLDivElement | null>(null);
  let canPrev = $state(false);
  let canNext = $state(false);

  function genreColor(g: Genre): string {
    return colorForGenre(g.name);
  }

  function syncScrollButtons() {
    if (!scrollerEl) return;
    canPrev = scrollerEl.scrollLeft > 4;
    canNext = scrollerEl.scrollLeft + scrollerEl.clientWidth < scrollerEl.scrollWidth - 4;
  }

  function scrollByPage(dir: -1 | 1) {
    if (!scrollerEl) return;
    const step = Math.max(240, Math.round(scrollerEl.clientWidth * 0.78));
    scrollerEl.scrollBy({ left: dir * step, behavior: "smooth" });
  }

  $effect(() => {
    void genres.length;
    void expanded;
    if (!expanded && scrollerEl) {
      requestAnimationFrame(syncScrollButtons);
    }
  });
  let scrollerMaskStyle = $derived.by(() => {
    if (canPrev && canNext) {
      const grad = "linear-gradient(to right, transparent 0%, black 28px, black calc(100% - 28px), transparent 100%)";
      return `-webkit-mask-image: ${grad}; mask-image: ${grad};`;
    }
    if (canNext) {
      const grad = "linear-gradient(to right, black 0%, black calc(100% - 28px), transparent 100%)";
      return `-webkit-mask-image: ${grad}; mask-image: ${grad};`;
    }
    if (canPrev) {
      const grad = "linear-gradient(to right, transparent 0%, black 28px, black 100%)";
      return `-webkit-mask-image: ${grad}; mask-image: ${grad};`;
    }
    return "";
  });
</script>

{#if showHeader}
  <h1 class="m-0 text-[28px] md:text-[32px] font-extrabold tracking-tight">{t("explore.title")}</h1>
  <p class="muted mt-1 text-[13.5px] md:text-[14px]">{t("explore.subtitle")}</p>
{/if}

{#if player.genresLoading && genres.length === 0}
  <section class="mt-6 space-y-3 animate-pulse" aria-busy="true">
    <div class="h-6 w-44 rounded-md bg-[var(--skeleton)]"></div>
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2.5">
      {#each [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] as _}
        <div class="h-12 rounded-lg bg-[var(--skeleton-soft)] border border-[var(--border)]"></div>
      {/each}
    </div>
  </section>
{:else if genres.length > 0}
  <section class="mt-6 space-y-3">
    <div class="flex items-center justify-between gap-3">
      <h2 class="section-title">{t("explore.moods_genres")}</h2>
      <div class="flex items-center gap-1.5 shrink-0">
        <button
          type="button"
          class="pill-btn !py-1 !px-3 !text-[12px]"
          aria-expanded={expanded}
          onclick={() => (expanded = !expanded)}
        >
          {expanded ? t("artist.show_less") : t("artist.more")}
        </button>
        {#if !expanded}
          <button
            type="button"
            class="w-8 h-8 rounded-full inline-flex items-center justify-center border border-[var(--border)] bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text)] transition-colors disabled:opacity-30"
            disabled={!canPrev}
            title={t("carousel.prev")}
            aria-label={t("carousel.prev")}
            onclick={() => scrollByPage(-1)}
          >
            <ChevronLeft size={16} aria-hidden="true" />
          </button>
          <button
            type="button"
            class="w-8 h-8 rounded-full inline-flex items-center justify-center border border-[var(--border)] bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text)] transition-colors disabled:opacity-30"
            disabled={!canNext}
            title={t("carousel.next")}
            aria-label={t("carousel.next")}
            onclick={() => scrollByPage(1)}
          >
            <ChevronRight size={16} aria-hidden="true" />
          </button>
        {/if}
      </div>
    </div>

    {#if expanded}
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-2.5">
        {#each genres as g (g.id)}
          {@const c = genreColor(g)}
          <button
            type="button"
            class="group h-12 rounded-[9px] overflow-hidden bg-[var(--card-bg)] hover:bg-[var(--surface-hover)] border border-[var(--border)] hover:border-[var(--border-strong)] flex items-center text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.99]"
            style="box-shadow: var(--card-shadow);"
            onclick={() => void player.runSearch(g.name)}
          >
            <span class="w-[6px] self-stretch shrink-0" style="background: {c};"></span>
            <span class="flex-1 px-3.5 text-[13.5px] font-semibold truncate">{g.name}</span>
          </button>
        {/each}
      </div>
    {:else}
      <div
        bind:this={scrollerEl}
        onscroll={syncScrollButtons}
        style={scrollerMaskStyle}
        class="genre-scroller grid grid-flow-col grid-rows-4 auto-cols-[minmax(164px,196px)] sm:auto-cols-[minmax(180px,212px)] gap-2.5 overflow-x-auto py-1 scroll-smooth snap-x snap-proximity"
      >
        {#each genres as g (g.id)}
          {@const c = genreColor(g)}
          <button
            type="button"
            class="group h-12 rounded-[9px] overflow-hidden bg-[var(--card-bg)] hover:bg-[var(--surface-hover)] border border-[var(--border)] hover:border-[var(--border-strong)] flex items-center text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.99] snap-start"
            style="box-shadow: var(--card-shadow);"
            onclick={() => void player.runSearch(g.name)}
          >
            <span class="w-[6px] self-stretch shrink-0" style="background: {c};"></span>
            <span class="flex-1 px-3.5 text-[13.5px] font-semibold truncate">{g.name}</span>
          </button>
        {/each}
      </div>
    {/if}
  </section>
{/if}

<style>
  .genre-scroller {
    scrollbar-width: none;
  }
  .genre-scroller::-webkit-scrollbar {
    display: none;
  }
</style>
