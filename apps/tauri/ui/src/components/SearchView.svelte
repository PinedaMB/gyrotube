<script lang="ts">
  import { ArrowLeft, ArrowUpLeft, Clock, Search, X } from "lucide-svelte";
  import { onMount } from "svelte";
  import { api, isTauriRuntime } from "../lib/api";
  import { colorForGenre } from "../lib/genrePalette";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { Genre } from "../lib/types";
  import OfflineState from "./OfflineState.svelte";
  import ExploreSearchResults from "./explore/ExploreSearchResults.svelte";
  import ExploreSkeleton from "./explore/ExploreSkeleton.svelte";

  function genreColor(g: Genre): string {
    if (g.color_hex && g.color_hex.startsWith("#") && g.color_hex.length >= 7) {
      return g.color_hex;
    }
    return colorForGenre(g.name);
  }

  let query = $state(player.searchQuery);
  let suggestions = $state<string[]>([]);
  let editing = $state(!player.searchQuery);
  let inputEl = $state<HTMLInputElement | null>(null);
  let debounceTimer: number | undefined;
  let suggestionReq = 0;

  $effect(() => {
    const storeQ = player.searchQuery;
    if (storeQ !== query && document.activeElement !== inputEl) {
      query = storeQ;
    }
    if (!storeQ) {
      editing = true;
    } else if (player.searchResults || player.searchLoading) {
      if (document.activeElement !== inputEl) {
        editing = false;
      }
    }
  });

  onMount(() => {
    if (!player.searchQuery && !player.isOffline) {
      requestAnimationFrame(() => inputEl?.focus());
    }
    return () => {
      window.clearTimeout(debounceTimer);
      suggestionReq++;
    };
  });

  function handleInput() {
    editing = true;
    suggestions = [];
    const req = ++suggestionReq;
    window.clearTimeout(debounceTimer);
    const trimmed = query.trim();
    if (!trimmed) {
      player.clearSearch(false);
      return;
    }
    debounceTimer = window.setTimeout(async () => {
      if (!isTauriRuntime() || player.isOffline) return;
      try {
        const res = await api.getSearchSuggestions(trimmed);
        if (req === suggestionReq && query.trim() === trimmed) {
          suggestions = res;
        }
      } catch {
        // Non-fatal
      }
    }, 180);
  }

  function submit(term: string) {
    const clean = term.trim();
    if (!clean || player.isOffline) return;
    query = clean;
    editing = false;
    suggestions = [];
    inputEl?.blur();
    void player.runSearch(clean);
  }

  function clearInput() {
    query = "";
    suggestions = [];
    editing = true;
    player.clearSearch(false);
    inputEl?.focus();
  }
</script>

<div class="w-full min-h-full flex flex-col">
  <!-- Mobile dedicated YouTube Music-style search bar -->
  <div
    class="md:hidden sticky top-0 z-20 flex items-center gap-2 px-3 pt-[max(0.625rem,env(safe-area-inset-top))] pb-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border)]"
  >
    <button
      type="button"
      class="icon-btn shrink-0"
      title={t("nav.back")}
      aria-label={t("nav.back")}
      onclick={() => player.goBack()}
    >
      <ArrowLeft size={19} />
    </button>

    <div
      class="flex-1 min-w-0 flex items-center gap-2 h-10 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--bg)] focus-within:ring-2 focus-within:ring-[var(--accent-soft)] transition-all"
    >
      <Search size={15} class="text-[var(--text-muted)] shrink-0" aria-hidden="true" />
      <input
        bind:this={inputEl}
        type="text"
        disabled={player.isOffline}
        enterkeyhint="search"
        autocomplete="off"
        autocapitalize="off"
        spellcheck="false"
        class="w-full bg-transparent outline-none focus-visible:!outline-none text-[15px] placeholder:text-[var(--text-muted)] disabled:cursor-not-allowed"
        bind:value={query}
        placeholder={player.isOffline ? t("network.search_disabled") : t("header.search_placeholder")}
        aria-label={t("header.search_placeholder")}
        onfocus={() => (editing = true)}
        oninput={handleInput}
        onkeydown={(e) => {
          if (e.key === "Enter") submit(query);
          else if (e.key === "Escape") inputEl?.blur();
        }}
      />
      {#if query.length > 0 && !player.isOffline}
        <button
          type="button"
          class="w-7 h-7 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] shrink-0"
          title={t("header.clear_search")}
          aria-label={t("header.clear_search")}
          onmousedown={(e) => e.preventDefault()}
          onclick={clearInput}
        >
          <X size={14} />
        </button>
      {/if}
    </div>
  </div>

  <div class="page-clamp">
    {#if player.isOffline}
      <OfflineState />
    {:else if !editing && player.searchLoading}
      <ExploreSkeleton />
    {:else if !editing && player.searchResults}
      <ExploreSearchResults results={player.searchResults} />
    {:else if query.trim().length > 0 && suggestions.length > 0}
      <!-- Live suggestions list -->
      <section class="space-y-2">
        <h2 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)] px-1">
          {t("header.suggestions")}
        </h2>
        <div class="flex flex-col divide-y divide-[var(--border)] rounded-2xl bg-[var(--card-bg)] border border-[var(--border)] overflow-hidden">
          {#each suggestions as term (term)}
            <div class="flex items-center gap-1">
              <button
                type="button"
                class="flex-1 min-w-0 flex items-center gap-3 px-4 py-3 text-left text-[14px] hover:bg-[var(--surface-hover)] transition-colors"
                onclick={() => submit(term)}
              >
                <Search size={15} class="text-[var(--text-muted)] shrink-0" aria-hidden="true" />
                <span class="truncate">{term}</span>
              </button>
              <button
                type="button"
                class="w-11 h-11 inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] shrink-0"
                title={term}
                aria-label={term}
                onclick={() => {
                  query = term;
                  handleInput();
                  inputEl?.focus();
                }}
              >
                <ArrowUpLeft size={16} />
              </button>
            </div>
          {/each}
        </div>
      </section>
    {:else}
      <div class="space-y-6">
        {#if player.searchHistory.length > 0}
          <!-- Recent searches (YouTube Music clean row list) -->
          <section class="space-y-1.5">
            <div class="flex items-center justify-between px-1">
              <h2 class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
                {t("header.recent_searches")}
              </h2>
              <button
                type="button"
                class="text-[12.5px] font-semibold text-[var(--accent-bright)] hover:underline"
                onclick={() => player.clearSearchHistory()}
              >
                {t("header.clear")}
              </button>
            </div>
            <div class="flex flex-col -mx-1.5">
              {#each player.searchHistory as term (term)}
                <div class="flex items-center gap-0.5 rounded-xl hover:bg-[var(--surface-hover)] transition-colors">
                  <button
                    type="button"
                    class="flex-1 min-w-0 flex items-center gap-3 px-2.5 py-2.5 text-left text-[14.5px] font-medium"
                    onclick={() => submit(term)}
                  >
                    <Clock size={16} class="text-[var(--text-muted)] shrink-0" aria-hidden="true" />
                    <span class="truncate">{term}</span>
                  </button>
                  <button
                    type="button"
                    class="w-9 h-9 inline-flex items-center justify-center rounded-full text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface)] shrink-0"
                    title={term}
                    aria-label={term}
                    onclick={() => {
                      query = term;
                      handleInput();
                      inputEl?.focus();
                    }}
                  >
                    <ArrowUpLeft size={16} />
                  </button>
                  <button
                    type="button"
                    class="w-9 h-9 inline-flex items-center justify-center rounded-full text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface)] shrink-0"
                    title={t("queue.remove")}
                    aria-label={t("queue.remove")}
                    onclick={() => player.removeSearchHistoryItem(term)}
                  >
                    <X size={15} />
                  </button>
                </div>
              {/each}
            </div>
          </section>
        {/if}

        {#if player.genres.length > 0}
          <!-- Quick genre discovery grid so the search screen never feels empty -->
          <section class="space-y-2.5">
            <h2 class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)] px-1">
              {t("explore.moods_genres")}
            </h2>
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
              {#each player.genres.slice(0, 12) as g (g.id)}
                {@const c = genreColor(g)}
                <button
                  type="button"
                  class="h-11 rounded-lg overflow-hidden bg-[var(--card-bg)] hover:bg-[var(--surface-hover)] border border-[var(--border)] hover:border-[var(--border-strong)] flex items-center text-left transition-colors"
                  style="box-shadow: var(--card-shadow);"
                  onclick={() => submit(g.name)}
                >
                  <span class="w-[4px] self-stretch shrink-0" style="background: {c};"></span>
                  <span class="flex-1 px-3 text-[13px] font-semibold truncate">{g.name}</span>
                </button>
              {/each}
            </div>
          </section>
        {:else if player.searchHistory.length === 0}
          <div class="min-h-[42vh] flex flex-col items-center justify-center text-center gap-3 text-[var(--text-muted)]">
            <span class="w-14 h-14 rounded-2xl bg-[var(--surface)] border border-[var(--border)] inline-flex items-center justify-center">
              <Search size={24} />
            </span>
            <p class="text-[14px] max-w-xs">{t("header.search_placeholder")}</p>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
