<script lang="ts">
  import { ArrowUpLeft, Clock, Play, Search, X } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Album, Artist, RecentSearchEntry, SearchSuggestions, Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  const EMPTY_SUGGESTIONS: SearchSuggestions = { terms: [], tracks: [], artists: [], albums: [] };

  let query = $state("");
  let suggestions = $state<SearchSuggestions>(EMPTY_SUGGESTIONS);
  let popoverOpen = $state(false);
  let selectedIdx = $state<number>(-1);
  let debounceTimer: number | undefined;
  let suggestionRequest = 0;
  let searchInput = $state<HTMLInputElement | null>(null);
  let listboxEl = $state<HTMLDivElement | null>(null);

  type NavEntry =
    | { kind: "term"; term: string }
    | { kind: "artist"; artist: Artist }
    | { kind: "track"; track: Track }
    | { kind: "album"; album: Album };

  let showingHistory = $derived(query.trim().length === 0);
  let listItems = $derived(showingHistory ? [] : suggestions.terms);
  let artistsOffset = $derived(listItems.length);
  let tracksOffset = $derived(artistsOffset + suggestions.artists.length);
  let albumsOffset = $derived(tracksOffset + suggestions.tracks.length);
  let navEntries = $derived<NavEntry[]>(
    showingHistory
      ? []
      : [
          ...suggestions.terms.map((term): NavEntry => ({ kind: "term", term })),
          ...suggestions.artists.map((artist): NavEntry => ({ kind: "artist", artist })),
          ...suggestions.tracks.map((track): NavEntry => ({ kind: "track", track })),
          ...suggestions.albums.map((album): NavEntry => ({ kind: "album", album })),
        ],
  );
  let hasRichEntities = $derived(
    !showingHistory &&
      (suggestions.artists.length > 0 ||
        suggestions.tracks.length > 0 ||
        suggestions.albums.length > 0),
  );
  let hasPopoverContent = $derived(
    showingHistory ? player.recentSearches.length > 0 : navEntries.length > 0,
  );

  function pickSuggestedArtist(artist: Artist) {
    popoverOpen = false;
    searchInput?.blur();
    player.recordRecentSearch({ kind: "artist", key: `ar:${artist.id}`, artist });
    player.openArtist(artist.id, artist.name);
  }

  function pickSuggestedTrack(track: Track) {
    popoverOpen = false;
    searchInput?.blur();
    player.recordRecentSearch({ kind: "track", key: `tr:${track.id}`, track });
    player.replaceQueueAndPlay([track], 0, track.title);
  }

  function pickSuggestedAlbum(album: Album) {
    popoverOpen = false;
    searchInput?.blur();
    player.recordRecentSearch({ kind: "album", key: `al:${album.id}`, album });
    player.openAlbum(album.id, album.title, album.cover_url);
  }

  function openRecentEntry(entry: RecentSearchEntry) {
    if (entry.kind === "query") {
      submitSearch(entry.term);
    } else if (entry.kind === "artist") {
      pickSuggestedArtist(entry.artist);
    } else if (entry.kind === "track") {
      pickSuggestedTrack(entry.track);
    } else {
      pickSuggestedAlbum(entry.album);
    }
  }

  $effect(() => {
    if (!player.searchQuery && query && document.activeElement !== searchInput) {
      query = "";
    }
  });

  $effect(() => {
    if (player.isOffline) {
      popoverOpen = false;
      selectedIdx = -1;
    }
  });

  $effect(() => {
    if (!popoverOpen || selectedIdx < 0 || !listboxEl) return;
    listboxEl
      .querySelector<HTMLElement>('[aria-selected="true"]')
      ?.scrollIntoView({ block: "nearest" });
  });

  onMount(() => {
    const onGlobalKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        if (player.isOffline) return;
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
    const request = ++suggestionRequest;
    window.clearTimeout(debounceTimer);
    const trimmed = query.trim();
    if (!trimmed) {
      suggestions = EMPTY_SUGGESTIONS;
      player.clearSearch();
      popoverOpen = player.recentSearches.length > 0;
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
    }, 150);
  }

  function submitSearch(term: string) {
    const clean = term.trim();
    if (!clean) return;
    query = clean;
    popoverOpen = false;
    searchInput?.blur();
    if (player.tryOpenYtmLink(clean)) return;
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
    const navLen = showingHistory ? player.recentSearches.length : navEntries.length;
    if (!popoverOpen || navLen === 0) {
      if (e.key === "Enter") submitSearch(query);
      else if (e.key === "Escape") searchInput?.blur();
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIdx = (selectedIdx + 1) % navLen;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIdx = ((selectedIdx < 0 ? 0 : selectedIdx) - 1 + navLen) % navLen;
    } else if (e.key === "Tab" && !e.shiftKey && !showingHistory) {
      const picked = navEntries[selectedIdx];
      if (picked?.kind === "term") {
        e.preventDefault();
        fillSuggestion(picked.term);
      }
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (showingHistory) {
        const entry = player.recentSearches[selectedIdx];
        if (entry) openRecentEntry(entry);
        else submitSearch(query);
      } else {
        const picked = navEntries[selectedIdx];
        if (!picked) {
          submitSearch(query);
        } else if (picked.kind === "term") {
          submitSearch(picked.term);
        } else if (picked.kind === "artist") {
          pickSuggestedArtist(picked.artist);
        } else if (picked.kind === "track") {
          pickSuggestedTrack(picked.track);
        } else {
          pickSuggestedAlbum(picked.album);
        }
      }
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
    class="flex items-center gap-2.5 h-11 md:h-9 px-3.5 rounded-full bg-[var(--surface)] border border-[var(--border)] transition-all {player.isOffline
      ? 'opacity-55 cursor-not-allowed'
      : 'hover:border-[var(--border-strong)] focus-within:border-[var(--accent-border)] focus-within:bg-[var(--bg)] focus-within:ring-2 focus-within:ring-[var(--accent-soft)]'}"
  >
    <Search size={15} class="text-[var(--text-muted)] shrink-0" />
    <input
      bind:this={searchInput}
      type="text"
      disabled={player.isOffline}
      enterkeyhint="search"
      autocomplete="off"
      autocapitalize="off"
      spellcheck="false"
      class="w-full bg-transparent outline-none focus-visible:!outline-none text-base md:text-[13.5px] placeholder:text-[var(--text-muted)] disabled:cursor-not-allowed"
      bind:value={query}
      placeholder={player.isOffline ? t("network.search_disabled") : t("header.search_placeholder")}
      aria-label={player.isOffline ? t("network.search_disabled") : t("header.search_placeholder")}
      onfocus={() => {
        if (!player.isOffline) popoverOpen = true;
      }}
      oninput={handleInput}
      onkeydown={handleKeydown}
    />
    {#if query.length > 0 && !player.isOffline}
      <button
        type="button"
        class="w-8 h-8 md:w-5 md:h-5 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] shrink-0 transition-colors"
        title={t("header.clear_search")}
        onmousedown={(e) => e.preventDefault()}
        onclick={clearQuery}
      >
        <X size={13} />
      </button>
    {:else if !player.isOffline}
      <kbd
        class="hidden sm:inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded-[6px] text-[10.5px] font-mono font-semibold text-[var(--text-muted)] bg-[var(--surface)] border border-[var(--border)] shrink-0 leading-none"
      >
        Ctrl K
      </kbd>
    {/if}
  </div>

  {#if popoverOpen && hasPopoverContent}
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

      <div
        bind:this={listboxEl}
        class="px-1.5 space-y-0.5 max-h-[min(420px,55dvh)] md:max-h-[420px] overflow-y-auto overscroll-contain scroll-y"
      >
        {#if showingHistory}
          {#each player.recentSearches as entry, i (entry.key)}
            <div
              class="group flex items-center gap-1 rounded-xl transition-colors {i === selectedIdx
                ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
                : 'hover:bg-white/[0.06]'}"
              role="option"
              aria-selected={i === selectedIdx}
            >
              {#if entry.kind === "query"}
                <button
                  type="button"
                  class="flex-1 min-w-0 min-h-11 md:min-h-0 flex items-center gap-2.5 px-2.5 py-2 text-left text-[13.5px]"
                  onmousedown={(e) => e.preventDefault()}
                  onclick={() => openRecentEntry(entry)}
                >
                  <Clock size={14} class="text-[var(--text-muted)] shrink-0" />
                  <span class="truncate">{entry.term}</span>
                </button>
              {:else if entry.kind === "artist"}
                <button
                  type="button"
                  class="flex-1 min-w-0 flex items-center gap-2.5 px-2.5 py-1.5 text-left"
                  onmousedown={(e) => e.preventDefault()}
                  onclick={() => openRecentEntry(entry)}
                >
                  <CoverImage
                    class="w-9 h-9 !rounded-full shrink-0 border border-[var(--border)]"
                    src={entry.artist.avatar_url}
                    alt={entry.artist.name}
                    fallbackIcon="👤"
                  />
                  <div class="min-w-0 flex-1">
                    <div class="text-[13px] font-bold truncate">{entry.artist.name}</div>
                    <div class="text-[11px] text-[var(--text-muted)]">{t("explore.artist_pill")}</div>
                  </div>
                </button>
              {:else if entry.kind === "track"}
                <button
                  type="button"
                  class="flex-1 min-w-0 flex items-center gap-2.5 px-2.5 py-1.5 text-left"
                  onmousedown={(e) => e.preventDefault()}
                  onclick={() => openRecentEntry(entry)}
                >
                  <CoverImage
                    class="track-cover !w-9 !h-9 !rounded-[7px] shrink-0"
                    src={entry.track.cover_url}
                    alt=""
                  />
                  <div class="min-w-0 flex-1">
                    <div class="text-[13px] font-bold truncate">{entry.track.title}</div>
                    <div class="text-[11px] text-[var(--text-muted)] truncate">{entry.track.artist}</div>
                  </div>
                </button>
              {:else}
                <button
                  type="button"
                  class="flex-1 min-w-0 flex items-center gap-2.5 px-2.5 py-1.5 text-left"
                  onmousedown={(e) => e.preventDefault()}
                  onclick={() => openRecentEntry(entry)}
                >
                  <CoverImage
                    class="track-cover !w-9 !h-9 !rounded-[7px] shrink-0"
                    src={entry.album.cover_url}
                    alt=""
                  />
                  <div class="min-w-0 flex-1">
                    <div class="text-[13px] font-bold truncate">{entry.album.title}</div>
                    <div class="text-[11px] text-[var(--text-muted)] truncate">
                      {t("album.badge_album")}{entry.album.artist ? ` · ${entry.album.artist}` : ""}
                    </div>
                  </div>
                </button>
              {/if}

              <button
                type="button"
                class="w-10 h-10 md:w-7 md:h-7 mr-1 rounded-lg inline-flex items-center justify-center text-[var(--text-muted)] md:opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all shrink-0"
                title={t("queue.remove")}
                onmousedown={(e) => e.preventDefault()}
                onclick={() => {
                  player.removeRecentSearch(entry.key);
                  if (player.recentSearches.length === 0) popoverOpen = false;
                }}
              >
                <X size={13} />
              </button>
            </div>
          {/each}
        {:else}
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
                <Search size={14} class="text-[var(--text-muted)] shrink-0" />
                <span class="truncate">
                  {#if prefix}
                    <span class="text-[var(--text-muted)]">{prefix}</span><strong class="font-semibold text-[var(--text)]">{rest}</strong>
                  {:else}
                    {term}
                  {/if}
                </span>
              </button>

              <button
                type="button"
                class="w-11 h-11 md:w-7 md:h-7 mr-1 rounded-lg inline-flex items-center justify-center text-[var(--text-muted)] md:opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all shrink-0"
                title={term}
                onmousedown={(e) => e.preventDefault()}
                onclick={() => fillSuggestion(term)}
              >
                <ArrowUpLeft size={14} />
              </button>
            </div>
          {/each}
        {/if}

        {#if hasRichEntities}
          {#if listItems.length > 0}
            <div class="my-1.5 mx-2 border-t border-[var(--border)]"></div>
          {/if}

          {#each suggestions.artists as artist, i (artist.id)}
            {@const idx = artistsOffset + i}
            <button
              type="button"
              role="option"
              aria-selected={idx === selectedIdx}
              class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-xl text-left transition-colors {idx === selectedIdx
                ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
                : 'hover:bg-[var(--surface-hover)]'}"
              onmouseenter={() => (selectedIdx = idx)}
              onmousedown={(e) => e.preventDefault()}
              onclick={() => pickSuggestedArtist(artist)}
            >
              <CoverImage
                class="w-9 h-9 !rounded-full shrink-0 border border-[var(--border)]"
                src={artist.avatar_url}
                alt={artist.name}
                fallbackIcon="👤"
              />
              <div class="min-w-0 flex-1">
                <div class="text-[13px] font-bold truncate">{artist.name}</div>
                <div class="text-[11px] text-[var(--text-muted)]">{t("explore.artist_pill")}</div>
              </div>
            </button>
          {/each}

          {#each suggestions.tracks as track, i (track.id)}
            {@const idx = tracksOffset + i}
            <button
              type="button"
              role="option"
              aria-selected={idx === selectedIdx}
              class="group w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-xl text-left transition-colors {idx === selectedIdx
                ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
                : 'hover:bg-[var(--surface-hover)]'}"
              onmouseenter={() => (selectedIdx = idx)}
              onmousedown={(e) => e.preventDefault()}
              onclick={() => pickSuggestedTrack(track)}
            >
              <CoverImage
                class="track-cover !w-9 !h-9 !rounded-[7px] shrink-0"
                src={track.cover_url}
                alt=""
              />
              <div class="min-w-0 flex-1">
                <div class="text-[13px] font-bold truncate">{track.title}</div>
                <div class="text-[11px] text-[var(--text-muted)] truncate">{track.artist}</div>
              </div>
              <Play
                size={14}
                fill="currentColor"
                class="text-[var(--accent-bright)] shrink-0 mr-1 transition-opacity {idx === selectedIdx
                  ? 'opacity-100'
                  : 'opacity-0 group-hover:opacity-100'}"
              />
            </button>
          {/each}

          {#each suggestions.albums as album, i (album.id)}
            {@const idx = albumsOffset + i}
            <button
              type="button"
              role="option"
              aria-selected={idx === selectedIdx}
              class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-xl text-left transition-colors {idx === selectedIdx
                ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
                : 'hover:bg-[var(--surface-hover)]'}"
              onmouseenter={() => (selectedIdx = idx)}
              onmousedown={(e) => e.preventDefault()}
              onclick={() => pickSuggestedAlbum(album)}
            >
              <CoverImage
                class="track-cover !w-9 !h-9 !rounded-[7px] shrink-0"
                src={album.cover_url}
                alt=""
              />
              <div class="min-w-0 flex-1">
                <div class="text-[13px] font-bold truncate">{album.title}</div>
                <div class="text-[11px] text-[var(--text-muted)] truncate">
                  {t("album.badge_album")}{album.artist ? ` · ${album.artist}` : ""}
                </div>
              </div>
            </button>
          {/each}
        {/if}
      </div>
    </div>
  {/if}
</div>
