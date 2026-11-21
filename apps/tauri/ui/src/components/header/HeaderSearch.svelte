<script lang="ts">
  import { ArrowUpLeft, Clock, Search, X } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";

  let query = $state("");
  let suggestions = $state<string[]>([]);
  let popoverOpen = $state(false);
  let selectedIdx = $state<number>(-1);
  let debounceTimer: number | undefined;
  let suggestionRequest = 0;
  let searchInput = $state<HTMLInputElement | null>(null);

  let showingHistory = $derived(query.trim().length === 0);
  let listItems = $derived(showingHistory ? player.searchHistory : suggestions);

  $effect(() => {
    if (!player.searchQuery && query && document.activeElement !== searchInput) {
      query = "";
    }
  });

  onMount(() => {
    const onGlobalKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        searchInput?.focus();
        searchInput?.select();
      }
    };
    window.addEventListener("keydown", onGlobalKey);
    return () => {
      window.removeEventListener("keydown", onGlobalKey);
      window.clearTimeout(debounceTimer);
      suggestionRequest++;
    };
  });

  function handleInput() {
    selectedIdx = -1;
    suggestions = [];
    const request = ++suggestionRequest;
    window.clearTimeout(debounceTimer);
    const trimmed = query.trim();
    if (!trimmed) {
      player.clearSearch();
      popoverOpen = player.searchHistory.length > 0;
      return;
    }
    popoverOpen = true;
    debounceTimer = window.setTimeout(async () => {
      if (!isTauriRuntime()) return;
      try {
        const results = await api.getSearchSuggestions(trimmed);
        if (request === suggestionRequest && query.trim() === trimmed) suggestions = results;
      } catch {
        // Search can still be submitted without suggestions.
      }
    }, 180);
  }

  function submitSearch(term: string) {
    const clean = term.trim();
    if (!clean) return;
    query = clean;
    popoverOpen = false;
    searchInput?.blur();
    void player.runSearch(clean);
  }

  function fillSuggestion(term: string) {
    query = term;
    selectedIdx = -1;
    searchInput?.focus();
    handleInput();
  }

  function clearQuery() {
    query = "";
    searchInput?.focus();
    handleInput();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.isComposing) return;
    if (!popoverOpen || listItems.length === 0) {
      if (e.key === "Enter") submitSearch(query);
      else if (e.key === "Escape") searchInput?.blur();
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIdx = (selectedIdx + 1) % listItems.length;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIdx = ((selectedIdx < 0 ? 0 : selectedIdx) - 1 + listItems.length) % listItems.length;
    } else if (e.key === "Tab" && !e.shiftKey && !showingHistory && listItems[selectedIdx]) {
      e.preventDefault();
      fillSuggestion(listItems[selectedIdx]);
    } else if (e.key === "Enter") {
      e.preventDefault();
      submitSearch(listItems[selectedIdx] ?? query);
    } else if (e.key === "Escape") {
      popoverOpen = false;
    }
  }

  function splitHighlight(term: string): [string, string] {
    const typed = query.trim();
    if (!showingHistory && typed && term.toLowerCase().startsWith(typed.toLowerCase())) {
      return [term.slice(0, typed.length), term.slice(typed.length)];
    }
    return ["", term];
  }
</script>

<div
  class="w-full md:relative"
  onfocusout={(e) => {
    if (!(e.relatedTarget instanceof Node) || !e.currentTarget.contains(e.relatedTarget)) {
      popoverOpen = false;
      selectedIdx = -1;
    }
  }}
>
  <div
    class="flex items-center gap-2.5 h-9 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--border-strong)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--bg)] focus-within:ring-2 focus-within:ring-[var(--accent-soft)] transition-all"
  >
    <Search size={15} class="text-[var(--text-muted)] shrink-0" />
    <input
      bind:this={searchInput}
      type="text"
      enterkeyhint="search"
      autocomplete="off"
      autocapitalize="off"
      spellcheck="false"
      class="w-full bg-transparent outline-none text-base md:text-[13.5px] placeholder:text-[var(--text-muted)]"
      bind:value={query}
      placeholder={t("header.search_placeholder")}
      aria-label={t("header.search_placeholder")}
      onfocus={() => (popoverOpen = true)}
      oninput={handleInput}
      onkeydown={handleKeydown}
    />
    {#if query.length > 0}
      <button
        type="button"
        class="w-8 h-8 md:w-5 md:h-5 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] shrink-0 transition-colors"
        title={t("header.clear_search")}
        onmousedown={(e) => e.preventDefault()}
        onclick={clearQuery}
      >
        <X size={13} />
      </button>
    {:else}
      <kbd
        class="hidden sm:inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded-[6px] text-[10.5px] font-mono font-semibold text-[var(--text-muted)] bg-white/[0.06] border border-white/10 shrink-0 leading-none"
      >
        Ctrl K
      </kbd>
    {/if}
  </div>

  {#if popoverOpen && listItems.length > 0}
    <div
      transition:fly={{ y: -6, duration: 160, easing: cubicOut }}
      class="absolute inset-x-3 top-[calc(100%+4px)] md:inset-x-0 md:top-[calc(100%+8px)] rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_20px_48px_rgba(0,0,0,0.55)] overflow-hidden py-1.5 z-50"
      role="listbox"
    >
      <div
        class="flex items-center justify-between px-4 pt-1.5 pb-2 mb-1 text-[10.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)] border-b border-white/[0.06]"
      >
        <span>{showingHistory ? t("header.recent_searches") : t("header.suggestions")}</span>
        {#if showingHistory}
          <button
            type="button"
            class="min-h-9 md:min-h-0 text-[11.5px] text-[var(--accent-bright)] hover:text-[var(--text)] normal-case font-semibold transition-colors"
            onmousedown={(e) => e.preventDefault()}
            onclick={() => {
              player.clearSearchHistory();
              popoverOpen = false;
            }}
          >
            {t("header.clear")}
          </button>
        {/if}
      </div>

      <div class="px-1.5 space-y-0.5 max-h-[min(320px,45dvh)] md:max-h-[320px] overflow-y-auto overscroll-contain scroll-y">
        {#each listItems as term, i (term)}
          {@const [prefix, rest] = splitHighlight(term)}
          <div
            class="group flex items-center gap-1 rounded-xl transition-colors {i === selectedIdx
              ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
              : 'hover:bg-white/[0.06]'}"
            role="option"
            aria-selected={i === selectedIdx}
          >
            <button
              type="button"
              class="flex-1 min-w-0 min-h-12 md:min-h-0 flex items-center gap-2.5 px-2.5 py-2 text-left text-[13.5px]"
              onmousedown={(e) => e.preventDefault()}
              onclick={() => submitSearch(term)}
            >
              {#if showingHistory}
                <Clock size={14} class="text-[var(--text-muted)] shrink-0" />
              {:else}
                <Search size={14} class="text-[var(--text-muted)] shrink-0" />
              {/if}
              <span class="truncate">
                {#if prefix}
                  <span class="text-[var(--text-muted)]">{prefix}</span><strong class="font-semibold text-[var(--text)]">{rest}</strong>
                {:else}
                  {term}
                {/if}
              </span>
            </button>

            {#if showingHistory}
              <button
                type="button"
                class="w-11 h-11 md:w-7 md:h-7 mr-1 rounded-lg inline-flex items-center justify-center text-[var(--text-muted)] md:opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all shrink-0"
                title={t("queue.remove")}
                onmousedown={(e) => e.preventDefault()}
                onclick={() => {
                  player.removeSearchHistoryItem(term);
                  if (player.searchHistory.length === 0) popoverOpen = false;
                }}
              >
                <X size={13} />
              </button>
            {:else}
              <button
                type="button"
                class="w-11 h-11 md:w-7 md:h-7 mr-1 rounded-lg inline-flex items-center justify-center text-[var(--text-muted)] md:opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all shrink-0"
                title={term}
                onmousedown={(e) => e.preventDefault()}
                onclick={() => fillSuggestion(term)}
              >
                <ArrowUpLeft size={14} />
              </button>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
