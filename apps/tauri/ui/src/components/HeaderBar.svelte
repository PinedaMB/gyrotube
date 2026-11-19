<script lang="ts">
  import { PanelLeft, PartyPopper, Search, Settings, Users, X } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { api, isTauriRuntime } from "../lib/api";
  import { player } from "../lib/stores/player.svelte";

  let query = $state("");
  let suggestions = $state<string[]>([]);
  let popoverOpen = $state(false);
  let selectedIdx = $state<number>(-1);
  let debounceTimer: number | undefined;
  let searchInput = $state<HTMLInputElement | null>(null);

  let showingHistory = $derived(query.trim().length === 0);
  let listItems = $derived(showingHistory ? player.searchHistory : suggestions);
  let spinLive = $derived(
    player.spinStatus.local_active || player.spinStatus.remote_role !== "idle",
  );

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
    return () => window.removeEventListener("keydown", onGlobalKey);
  });

  function handleInput() {
    selectedIdx = -1;
    window.clearTimeout(debounceTimer);
    const trimmed = query.trim();
    if (!trimmed) {
      suggestions = [];
      player.clearSearch();
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
    searchInput?.blur();
    void player.runSearch(clean);
  }

  function clearQuery() {
    query = "";
    suggestions = [];
    selectedIdx = -1;
    player.clearSearch();
    popoverOpen = player.searchHistory.length > 0;
    searchInput?.focus();
  }

  function handleKeydown(e: KeyboardEvent) {
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
  class="h-14 shrink-0 z-30 grid grid-cols-[1fr_minmax(220px,480px)_1fr] items-center gap-3 px-4 bg-[var(--bg-elevated)]/95 backdrop-blur-md border-b border-[var(--border)] select-none"
>
  <!-- Left: Sidebar toggle + Brand -->
  <div class="justify-self-start flex items-center gap-2.5 min-w-0">
    <button
      class="icon-btn shrink-0 {player.sidebarOpen ? 'text-[var(--text)]' : 'text-[var(--text-muted)]'}"
      title="Toggle player sidebar"
      onclick={() => {
        player.sidebarOpen = !player.sidebarOpen;
        void player.persistSession();
      }}
    >
      <PanelLeft size={18} />
    </button>

    <button
      class="flex items-center gap-2 rounded-xl px-1.5 py-1 hover:bg-[var(--surface)] transition-colors"
      onclick={() => player.navigateTab("home")}
      title="Go to Home"
    >
      <img src="/gyrotube.svg" alt="" class="w-6 h-6 shrink-0 drop-shadow-[0_2px_8px_rgba(33,144,164,0.45)]" />
      <span class="font-extrabold text-[15.5px] tracking-[-0.02em] hidden sm:inline">GyroTube</span>
      <span
        class="hidden md:inline-flex items-center px-2 py-0.5 rounded-full text-[10.5px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/40"
      >
        Music
      </span>
    </button>
  </div>

  <!-- Center: Mathematically centered Spotlight Search -->
  <div class="w-full relative">
    <div
      class="flex items-center gap-2.5 h-9 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--border-strong)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--bg)] focus-within:ring-2 focus-within:ring-[var(--accent-soft)] transition-all"
    >
      <Search size={15} class="text-[var(--text-muted)] shrink-0" />
      <input
        bind:this={searchInput}
        type="text"
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
      {#if query.length > 0}
        <button
          type="button"
          class="w-5 h-5 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] shrink-0 transition-colors"
          title="Clear search"
          onmousedown={(e) => {
            e.preventDefault();
            clearQuery();
          }}
        >
          <X size={13} />
        </button>
      {:else}
        <kbd
          class="hidden sm:inline-flex items-center px-1.5 py-0.5 rounded-md text-[10px] font-semibold text-[var(--text-muted)] bg-white/[0.05] border border-white/10 shrink-0"
        >
          Ctrl K
        </kbd>
      {/if}
    </div>

    {#if popoverOpen && listItems.length > 0}
      <div
        transition:fly={{ y: -8, duration: 170, easing: cubicOut }}
        class="absolute left-0 right-0 top-[calc(100%+8px)] rounded-2xl bg-[var(--bg-elevated)]/98 backdrop-blur-xl border border-[var(--border-strong)] shadow-[0_18px_44px_rgba(0,0,0,0.5)] overflow-hidden py-1.5 z-50"
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
            <Search size={14} class="text-[var(--text-muted)] shrink-0" />
            <span class="truncate">
              {#if prefix}<span class="text-[var(--text-muted)]">{prefix}</span><strong>{rest}</strong>{:else}{term}{/if}
            </span>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Right: SPIN + Party Mode + Divider + Settings -->
  <div class="justify-self-end flex items-center gap-1.5">
    <button
      class="icon-btn relative"
      class:active={player.spinOpen || spinLive}
      title="SPIN Mode"
      onclick={() => {
        player.spinOpen = !player.spinOpen;
        if (player.spinOpen) void player.refreshSpinStatus();
      }}
    >
      <Users size={18} />
      {#if spinLive}
        <span class="w-2 h-2 rounded-full bg-emerald-400 ring-2 ring-[var(--bg-elevated)] absolute top-1 right-1"></span>
      {/if}
    </button>
    <button
      class="icon-btn"
      class:active={player.partyMode}
      title="Party Mode"
      onclick={() => (player.partyMode = !player.partyMode)}
    >
      <PartyPopper size={18} />
    </button>
    <span class="w-px h-5 bg-[var(--border-strong)] mx-1" aria-hidden="true"></span>
    <button
      class="icon-btn"
      class:active={player.settingsOpen}
      title="Settings"
      onclick={() => (player.settingsOpen = true)}
    >
      <Settings size={18} />
    </button>
  </div>
</header>
