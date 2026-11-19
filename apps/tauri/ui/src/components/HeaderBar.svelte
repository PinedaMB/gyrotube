<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { player } from "../lib/stores/player.svelte";

  let query = $state("");
  let suggestions = $state<string[]>([]);
  let popoverOpen = $state(false);
  let selectedIdx = $state<number>(-1);
  let debounceTimer: number | undefined;

  let showingHistory = $derived(query.trim().length === 0);
  let listItems = $derived(showingHistory ? player.searchHistory : suggestions);

  function handleInput() {
    selectedIdx = -1;
    window.clearTimeout(debounceTimer);
    const trimmed = query.trim();
    if (!trimmed) {
      suggestions = [];
      popoverOpen = player.searchHistory.length > 0;
      return;
    }
    popoverOpen = true;
    debounceTimer = window.setTimeout(async () => {
      if (!isTauriRuntime()) return;
      try {
        suggestions = await api.getSearchSuggestions(trimmed);
        popoverOpen = suggestions.length > 0;
      } catch {
        suggestions = [];
      }
    }, 180);
  }

  function submitSearch(term: string) {
    const clean = term.trim();
    if (!clean) return;
    query = clean;
    popoverOpen = false;
    void player.runSearch(clean);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!popoverOpen || listItems.length === 0) {
      if (e.key === "Enter") submitSearch(query);
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIdx = (selectedIdx + 1) % listItems.length;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIdx = (selectedIdx - 1 + listItems.length) % listItems.length;
    } else if (e.key === "Enter") {
      e.preventDefault();
      submitSearch(selectedIdx >= 0 ? listItems[selectedIdx] : query);
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

<header
  class="h-14 shrink-0 z-30 grid grid-cols-[auto_minmax(0,1fr)_auto] items-center gap-4 px-4 bg-[var(--bg-elevated)] border-b border-[var(--border)]"
>
  <div class="flex items-center gap-2.5 shrink-0">
    <button
      class="icon-btn"
      title="Toggle player sidebar"
      onclick={() => {
        player.sidebarOpen = !player.sidebarOpen;
        void player.persistSession();
      }}
    >
      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="4" width="18" height="16" rx="3" />
        <line x1="9" y1="4" x2="9" y2="20" />
      </svg>
    </button>
    <span
      class="w-4 h-4 rounded-full bg-gradient-to-br from-[#2eb0c7] to-[#2190a4] shadow-[0_0_12px_rgba(33,144,164,0.55)] shrink-0"
      aria-hidden="true"
    ></span>
    <span class="font-extrabold text-[15px] tracking-tight hidden sm:inline">GyroTube</span>
    <span
      class="hidden md:inline-flex items-center px-2 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]"
    >
      Music
    </span>
  </div>

  <div class="w-full max-w-[460px] mx-auto relative">
    <div
      class="flex items-center gap-2.5 h-9 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--surface-hover)] transition-colors"
    >
      <svg
        class="text-[var(--text-muted)] shrink-0"
        viewBox="0 0 24 24"
        width="15"
        height="15"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <circle cx="11" cy="11" r="7" />
        <line x1="20" y1="20" x2="16.65" y2="16.65" />
      </svg>
      <input
        type="search"
        class="w-full bg-transparent outline-none text-[13.5px] placeholder:text-[var(--text-muted)]"
        bind:value={query}
        placeholder="Search songs, albums, artists…"
        onfocus={() => {
          if (listItems.length > 0) popoverOpen = true;
        }}
        onblur={() => window.setTimeout(() => (popoverOpen = false), 140)}
        oninput={handleInput}
        onkeydown={handleKeydown}
      />
    </div>

    {#if popoverOpen && listItems.length > 0}
      <div
        class="absolute left-0 right-0 top-[calc(100%+8px)] rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-2xl overflow-hidden py-1.5 z-50"
        role="listbox"
      >
        <div class="flex items-center justify-between px-3.5 py-1.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
          <span>{showingHistory ? "Recent searches" : "Suggestions"}</span>
          {#if showingHistory}
            <button
              class="text-[var(--accent-bright)] hover:underline normal-case font-semibold"
              onmousedown={() => player.clearSearchHistory()}
            >
              Clear
            </button>
          {/if}
        </div>
        {#each listItems as term, i}
          {@const [prefix, rest] = splitHighlight(term)}
          <button
            class="w-full flex items-center gap-2.5 px-3.5 py-2 text-left text-[13.5px] transition-colors {i === selectedIdx
              ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
              : 'hover:bg-[var(--surface-hover)]'}"
            onmousedown={() => submitSearch(term)}
          >
            <svg
              class="text-[var(--text-muted)] shrink-0"
              viewBox="0 0 24 24"
              width="14"
              height="14"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <circle cx="11" cy="11" r="7" />
              <line x1="20" y1="20" x2="16.65" y2="16.65" />
            </svg>
            <span class="truncate">
              {#if prefix}<span class="text-[var(--text-muted)]">{prefix}</span><strong>{rest}</strong>{:else}{term}{/if}
            </span>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="flex items-center gap-1.5 shrink-0">
    <button
      class="icon-btn"
      class:active={player.spinOpen || player.spinStatus.local_active || player.spinStatus.remote_role !== "idle"}
      title="SPIN Mode"
      onclick={() => {
        player.spinOpen = !player.spinOpen;
        if (player.spinOpen) void player.refreshSpinStatus();
      }}
    >
      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="9" cy="7" r="3" />
        <path d="M3 20c0-3.3 2.7-6 6-6s6 2.7 6 6" />
        <circle cx="17" cy="9" r="2.5" />
        <path d="M16 14.5c2.5.4 4.5 2.5 4.5 5.5" />
      </svg>
    </button>
    <button
      class="icon-btn"
      class:active={player.partyMode}
      title="Party Mode (F11)"
      onclick={() => (player.partyMode = !player.partyMode)}
    >
      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M12 3l2.2 6.2L20.5 12l-6.3 2.8L12 21l-2.2-6.2L3.5 12l6.3-2.8L12 3z" />
      </svg>
    </button>
    <span class="w-px h-5 bg-[var(--border-strong)] mx-0.5" aria-hidden="true"></span>
    <button
      class="icon-btn"
      class:active={player.settingsOpen}
      title="Settings"
      onclick={() => (player.settingsOpen = true)}
    >
      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3" />
        <path
          d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.01a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.01a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"
        />
      </svg>
    </button>
  </div>
</header>
