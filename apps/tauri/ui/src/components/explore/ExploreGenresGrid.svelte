<script lang="ts">
  import { colorForGenre, isMoodGenre } from "../../lib/genrePalette";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Genre } from "../../lib/types";

  let { genres }: { genres: Genre[] } = $props();

  let moods = $derived(genres.filter((g) => isMoodGenre(g.name)));
  let musicGenres = $derived(genres.filter((g) => !isMoodGenre(g.name)));
</script>

<h1 class="m-0 text-[28px] md:text-[32px] font-extrabold tracking-tight">{t("explore.title")}</h1>
<p class="muted mt-1 text-[13.5px] md:text-[14px]">{t("explore.subtitle")}</p>

{#if player.genresLoading && genres.length === 0}
  <section class="mt-6 space-y-6 animate-pulse" aria-busy="true">
    <div class="space-y-3">
      <div class="h-6 w-40 rounded-md bg-[var(--skeleton)]"></div>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-2.5">
        {#each [0, 1, 2, 3, 4, 5, 6, 7] as _}
          <div class="h-14 rounded-xl bg-[var(--skeleton-soft)] border border-[var(--border)]"></div>
        {/each}
      </div>
    </div>
    <div class="space-y-3">
      <div class="h-6 w-32 rounded-md bg-[var(--skeleton)]"></div>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-2.5">
        {#each [0, 1, 2, 3, 4, 5, 6, 7] as _}
          <div class="h-12 rounded-xl bg-[var(--skeleton-soft)] border border-[var(--border)]"></div>
        {/each}
      </div>
    </div>
  </section>
{/if}

{#if moods.length > 0}
  <section class="mt-6 space-y-3">
    <h2 class="section-title">{t("explore.moods")}</h2>
    <div class="grid grid-cols-2 md:grid-cols-4 gap-2.5">
      {#each moods as m (m.id)}
        {@const c = colorForGenre(m.name)}
        <button
          type="button"
          class="flex items-center gap-2.5 px-3.5 py-3 rounded-xl border text-left font-semibold hover:-translate-y-0.5 transition-all"
          style="background: linear-gradient(135deg, {c}26, var(--card-bg) 68%); border-color: {c}44; box-shadow: var(--card-shadow);"
          onclick={() => void player.runSearch(m.name)}
        >
          <span
            class="w-8 h-8 rounded-[10px] inline-flex items-center justify-center shrink-0"
            style="background: {c}26; color: {c};"
          >
            ♪
          </span>
          <span class="flex-1 truncate text-[14px]">{m.name}</span>
          <span class="muted">›</span>
        </button>
      {/each}
    </div>
  </section>
{/if}

{#if musicGenres.length > 0}
  <section class="mt-6 space-y-3">
    <h2 class="section-title">{t("explore.genres")}</h2>
    <div class="grid grid-cols-2 md:grid-cols-4 gap-2.5">
      {#each musicGenres as g (g.id)}
        {@const c = colorForGenre(g.name)}
        <button
          type="button"
          class="flex items-center gap-2.5 px-3.5 py-3 rounded-xl border border-[var(--border)] hover:border-[var(--border-strong)] hover:-translate-y-0.5 text-left font-semibold transition-all"
          style="background: linear-gradient(90deg, {c}1f, var(--card-bg) 62%); box-shadow: var(--card-shadow);"
          onclick={() => void player.runSearch(g.name)}
        >
          <span class="w-1 h-[22px] rounded-full shrink-0" style="background: {c};"></span>
          <span class="flex-1 truncate text-[14px]">{g.name}</span>
          <span class="muted">›</span>
        </button>
      {/each}
    </div>
  </section>
{/if}
