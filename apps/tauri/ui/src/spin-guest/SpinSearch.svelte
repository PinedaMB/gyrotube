<script lang="ts">
  import { Check, Music, Plus, Search, X } from "lucide-svelte";
  import SpinIcon from "../components/icons/SpinIcon.svelte";
  import { fmtTime, type SearchTrack } from "./types";

  interface Props {
    isPlaying: boolean;
    visibleMobile: boolean;
    onAdded: (track: SearchTrack) => void;
    onError: (msg: string) => void;
  }

  let { isPlaying, visibleMobile, onAdded, onError }: Props = $props();

  let query = $state("");
  let searching = $state(false);
  let searchError = $state<string | null>(null);
  let results = $state<SearchTrack[]>([]);
  let hasSearched = $state(false);
  let addingIds = $state<Record<string, "adding" | "added">>({});

  let searchTimer: number | undefined;
  let searchGen = 0;

  async function runSearch(raw: string) {
    const q = raw.trim();
    if (!q) {
      results = [];
      hasSearched = false;
      searchError = null;
      searching = false;
      return;
    }
    const gen = ++searchGen;
    searching = true;
    searchError = null;
    try {
      const r = await fetch(`/api/search?q=${encodeURIComponent(q)}`);
      if (!r.ok) throw new Error("Search failed");
      const items = (await r.json()) as SearchTrack[];
      if (gen !== searchGen) return;
      results = Array.isArray(items) ? items : [];
      hasSearched = true;
    } catch {
      if (gen !== searchGen) return;
      searchError = "Error al buscar canciones";
      results = [];
      hasSearched = true;
    } finally {
      if (gen === searchGen) searching = false;
    }
  }

  function onQueryInput(val: string) {
    query = val;
    window.clearTimeout(searchTimer);
    if (!val.trim()) {
      results = [];
      hasSearched = false;
      searchError = null;
      searching = false;
      return;
    }
    searchTimer = window.setTimeout(() => {
      void runSearch(val);
    }, 340);
  }

  function onSearchSubmit(e: SubmitEvent) {
    e.preventDefault();
    window.clearTimeout(searchTimer);
    void runSearch(query);
  }

  async function addTrack(track: SearchTrack) {
    if (addingIds[track.id]) return;
    addingIds = { ...addingIds, [track.id]: "adding" };
    try {
      const r = await fetch("/api/queue/add", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(track),
      });
      if (!r.ok) throw new Error("Failed");
      addingIds = { ...addingIds, [track.id]: "added" };
      onAdded(track);
      window.setTimeout(() => {
        const next = { ...addingIds };
        delete next[track.id];
        addingIds = next;
      }, 1800);
    } catch {
      const next = { ...addingIds };
      delete next[track.id];
      addingIds = next;
      onError("No se pudo enviar la canción");
    }
  }
</script>

<section
  class="rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border)] p-4 sm:p-5 shadow-[0_12px_32px_rgba(0,0,0,0.28)] min-w-0 {visibleMobile
    ? 'block'
    : 'hidden lg:block'}"
  aria-label="Buscar canciones"
>
  <div class="flex items-center justify-between gap-2 mb-3.5">
    <span class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)]">
      Agregar a la cola
    </span>
    {#if searching}
      <span class="w-4 h-4 rounded-full border-2 border-white/20 border-t-[var(--accent-bright)] animate-spin"></span>
    {/if}
  </div>

  <form class="flex items-center gap-2" onsubmit={onSearchSubmit}>
    <div class="relative flex-1 min-w-0">
      <Search
        size={16}
        class="absolute left-3.5 top-1/2 -translate-y-1/2 text-[var(--text-muted)] pointer-events-none"
      />
      <input
        type="search"
        autocomplete="off"
        value={query}
        placeholder="Buscar canción, artista o álbum…"
        aria-label="Buscar canción en YouTube Music"
        class="w-full h-11 pl-10 pr-9 rounded-full bg-black/25 border border-[var(--border-strong)] text-[14.5px] placeholder:text-[var(--text-muted)]/75 focus:outline-none focus:border-[var(--accent-bright)] transition-colors"
        oninput={(e) => onQueryInput((e.currentTarget as HTMLInputElement).value)}
      />
      {#if query}
        <button
          type="button"
          class="absolute right-2 top-1/2 -translate-y-1/2 w-7 h-7 rounded-full flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-white/10"
          aria-label="Limpiar búsqueda"
          onclick={() => onQueryInput("")}
        >
          <X size={14} />
        </button>
      {/if}
    </div>
    <button
      type="submit"
      class="h-11 px-5 rounded-full font-bold text-[14px] text-white bg-[var(--accent)] hover:bg-[var(--accent-bright)] shadow-[0_4px_14px_rgba(16,185,129,0.34)] transition-all active:scale-95 shrink-0"
    >
      Buscar
    </button>
  </form>

  <div class="mt-4 flex flex-col gap-1">
    {#if searching && results.length === 0}
      <div class="py-12 text-center text-[14px] text-[var(--text-muted)]">
        Buscando canciones…
      </div>
    {:else if searchError}
      <div class="py-12 text-center text-[14px] text-rose-400">
        {searchError}
      </div>
    {:else if results.length > 0}
      {#each results as track (track.id)}
        {@const addState = addingIds[track.id]}
        <div class="flex items-center gap-3 p-2.5 rounded-xl hover:bg-white/[0.045] transition-colors">
          <div class="w-11 h-11 rounded-[9px] overflow-hidden bg-white/[0.06] border border-white/[0.08] shrink-0">
            {#if track.cover_url}
              <img src={track.cover_url} alt="" loading="lazy" class="w-full h-full object-cover" />
            {:else}
              <div class="w-full h-full flex items-center justify-center text-[var(--text-muted)]/50">
                <Music size={16} />
              </div>
            {/if}
          </div>

          <div class="min-w-0 flex-1">
            <div class="text-[14.5px] font-semibold truncate">{track.title}</div>
            <div class="text-[12.5px] text-[var(--text-muted)] truncate">
              {track.artist}{track.album ? ` · ${track.album}` : ""}
            </div>
          </div>

          {#if track.duration_secs}
            <span class="hidden sm:inline text-[12px] text-[var(--text-muted)] numeric shrink-0">
              {fmtTime(track.duration_secs)}
            </span>
          {/if}

          <button
            type="button"
            disabled={addState === "adding"}
            class="inline-flex items-center gap-1.5 px-3.5 py-2 rounded-full text-[13px] font-semibold border transition-all active:scale-95 shrink-0 {addState === 'added'
              ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
              : 'bg-[var(--accent)] border-transparent text-white hover:bg-[var(--accent-bright)]'}"
            onclick={() => void addTrack(track)}
          >
            {#if addState === "added"}
              <Check size={14} />
              <span>Agregada</span>
            {:else}
              <Plus size={14} />
              <span>Agregar</span>
            {/if}
          </button>
        </div>
      {/each}
    {:else if hasSearched}
      <div class="py-12 text-center text-[14px] text-[var(--text-muted)]">
        No se encontraron canciones
      </div>
    {:else}
      <div class="py-12 text-center space-y-2">
        <div class="w-11 h-11 mx-auto rounded-2xl bg-[var(--gold-soft)] text-[var(--gold-bright)] flex items-center justify-center">
          <SpinIcon size={22} spinning={isPlaying} />
        </div>
        <p class="text-[14px] text-[var(--text-muted)]">
          Escribe el nombre de una canción o artista para agregarla a la cola.
        </p>
      </div>
    {/if}
  </div>
</section>
