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

<header class="header">
  <div class="brand">
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
    <span class="brand-dot" aria-hidden="true"></span>
    <span class="brand-name">GyroTube</span>
    <span class="brand-badge">Music</span>
  </div>

  <div class="search-wrap">
    <div class="search-box">
      <svg class="search-icon" viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="7" />
        <line x1="20" y1="20" x2="16.65" y2="16.65" />
      </svg>
      <input
        type="search"
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
      <div class="search-popover" role="listbox">
        <div class="popover-head">
          <span>{showingHistory ? "Recent searches" : "Suggestions"}</span>
          {#if showingHistory}
            <button class="clear-link" onmousedown={() => player.clearSearchHistory()}>Clear</button>
          {/if}
        </div>
        {#each listItems as term, i}
          {@const [prefix, rest] = splitHighlight(term)}
          <button
            class="popover-row"
            class:selected={i === selectedIdx}
            onmousedown={() => submitSearch(term)}
          >
            <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="7" />
              <line x1="20" y1="20" x2="16.65" y2="16.65" />
            </svg>
            <span class="term-text">
              {#if prefix}<span class="term-prefix">{prefix}</span><strong>{rest}</strong>{:else}{term}{/if}
            </span>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="actions">
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
        <path d="M12 3l2.2 5.8L20 11l-5.8 2.2L12 19l-2.2-5.8L4 11l5.8-2.2z" />
      </svg>
    </button>
    <button class="icon-btn" title="Settings" onclick={() => (player.settingsOpen = true)}>
      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9c.26.6.85 1 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
      </svg>
    </button>
  </div>
</header>

<style>
  .header {
    height: 54px;
    padding: 0 14px;
    display: grid;
    grid-template-columns: 1fr minmax(200px, 460px) 1fr;
    align-items: center;
    gap: 12px;
    background: var(--bg-sidebar);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .brand-dot {
    width: 22px;
    height: 22px;
    border-radius: 999px;
    background: radial-gradient(circle at 35% 35%, var(--accent-bright), var(--accent));
    box-shadow: 0 0 12px rgba(33, 144, 164, 0.45);
  }

  .brand-name {
    font-weight: 800;
    font-size: 15px;
    letter-spacing: -0.02em;
  }

  .brand-badge {
    font-size: 11px;
    font-weight: 700;
    padding: 2px 9px;
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .search-wrap {
    position: relative;
    width: 100%;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 14px;
    border-radius: 999px;
    background: var(--surface);
    border: 1px solid var(--border);
    transition: border-color 150ms ease, box-shadow 150ms ease, background-color 150ms ease;
  }

  .search-box:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px rgba(33, 144, 164, 0.34);
    background: var(--surface-hover);
  }

  .search-icon {
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .search-box input {
    width: 100%;
    border: none;
    outline: none;
    background: transparent;
    font-size: 13.5px;
  }

  .search-popover {
    position: absolute;
    top: calc(100% + 6px);
    left: 0;
    right: 0;
    z-index: 40;
    padding: 8px;
    border-radius: 14px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-strong);
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.48);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .popover-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 2px 10px 6px;
    font-size: 11.5px;
    color: var(--text-muted);
  }

  .clear-link {
    font-size: 11.5px;
    color: var(--text-muted);
  }

  .clear-link:hover {
    color: var(--accent-bright);
  }

  .popover-row {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    text-align: left;
    padding: 8px 10px;
    border-radius: 9px;
    font-size: 13.5px;
    color: var(--text);
  }

  .popover-row:hover,
  .popover-row.selected {
    background: var(--surface-hover);
  }

  .term-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .term-prefix {
    opacity: 0.68;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 4px;
  }

  @media (max-width: 680px) {
    .brand-name,
    .brand-badge {
      display: none;
    }
    .header {
      grid-template-columns: auto 1fr auto;
    }
  }
</style>
