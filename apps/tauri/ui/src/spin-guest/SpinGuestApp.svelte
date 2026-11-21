<script lang="ts">
  import {
    Check,
    ChevronDown,
    ChevronUp,
    Disc3,
    GripVertical,
    Headphones,
    ListMusic,
    Music,
    Pause,
    Play,
    Plus,
    Search,
    SkipBack,
    SkipForward,
    Users,
    Volume1,
    Volume2,
    VolumeX,
    X,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly, scale } from "svelte/transition";
  import SpinIcon from "../components/icons/SpinIcon.svelte";

  interface JamTrackInfo {
    id: string;
    title: string;
    artist: string;
    album?: string | null;
    duration_secs?: number | null;
    cover_url?: string | null;
    is_current: boolean;
  }

  interface JamSnapshot {
    active: boolean;
    url: string;
    host_name: string;
    allow_controls: boolean;
    is_playing: boolean;
    position_secs: number;
    now_playing: JamTrackInfo | null;
    queue: JamTrackInfo[];
    guest_count: number;
    added_count: number;
    guest_list: string[];
  }

  interface SearchTrack {
    id: string;
    title: string;
    artist: string;
    album?: string | null;
    duration_secs?: number | null;
    cover_url?: string | null;
  }

  let snapshot = $state<JamSnapshot | null>(null);
  let mobileTab = $state<"search" | "queue">("search");
  let query = $state("");
  let searching = $state(false);
  let searchError = $state<string | null>(null);
  let results = $state<SearchTrack[]>([]);
  let hasSearched = $state(false);
  let addingIds = $state<Record<string, "adding" | "added">>({});
  let listenHere = $state(false);
  let localVolume = $state(0.85);
  let toastMessage = $state<string | null>(null);
  let dragFrom = $state<number | null>(null);
  let dragOver = $state<number | null>(null);
  let smoothPos = $state(0);

  let audioEl: HTMLAudioElement | undefined;
  let curAudioTrackId: string | null = null;
  let statePending = false;
  let toastTimer: number | undefined;
  let searchTimer: number | undefined;
  let searchGen = 0;

  let nowPlaying = $derived(snapshot?.now_playing ?? null);
  let isPlaying = $derived(Boolean(snapshot?.is_playing && nowPlaying));
  let allowControls = $derived(Boolean(snapshot?.allow_controls));
  let queue = $derived(snapshot?.queue ?? []);
  let guestCount = $derived(snapshot?.guest_count ?? 0);
  let durationSecs = $derived(nowPlaying?.duration_secs ?? 0);
  let progressRatio = $derived(
    durationSecs > 0 ? Math.max(0, Math.min(1, smoothPos / durationSecs)) : 0,
  );

  function fmt(sec: number | null | undefined): string {
    const n = Math.max(0, Math.floor(sec || 0));
    const m = Math.floor(n / 60);
    const s = n % 60;
    return `${m}:${s < 10 ? "0" : ""}${s}`;
  }

  function showToast(msg: string) {
    toastMessage = msg;
    window.clearTimeout(toastTimer);
    toastTimer = window.setTimeout(() => {
      toastMessage = null;
    }, 2800);
  }

  function stopLocalAudio() {
    if (!audioEl) return;
    audioEl.pause();
    if (curAudioTrackId !== null) {
      curAudioTrackId = null;
      audioEl.removeAttribute("src");
      audioEl.load();
    }
  }

  function localAudioFailed(msg: string) {
    listenHere = false;
    stopLocalAudio();
    showToast(msg);
  }

  function syncLocalAudio() {
    if (!audioEl) return;
    const track = snapshot?.now_playing;
    if (!listenHere || !track) {
      stopLocalAudio();
      return;
    }
    audioEl.volume = localVolume;
    if (curAudioTrackId !== track.id) {
      curAudioTrackId = track.id;
      audioEl.src = `/api/audio?id=${encodeURIComponent(track.id)}`;
    }
    const pos = snapshot?.position_secs ?? 0;
    if (audioEl.readyState >= 1 && !audioEl.seeking) {
      const target = Number.isFinite(audioEl.duration) ? Math.min(pos, audioEl.duration) : pos;
      if (Math.abs(audioEl.currentTime - target) > (snapshot?.is_playing ? 1.5 : 0.35)) {
        audioEl.currentTime = target;
      }
    }
    if (!snapshot?.is_playing) {
      audioEl.pause();
      return;
    }
    if (audioEl.paused) {
      const src = audioEl.src;
      audioEl.play().catch((err: Error) => {
        if (!listenHere || !audioEl || audioEl.src !== src || !snapshot?.is_playing || err.name === "AbortError") {
          return;
        }
        localAudioFailed(
          err.name === "NotAllowedError"
            ? "Toca «Escuchar aquí» para iniciar el audio."
            : "No se pudo reproducir el audio. Toca «Escuchar aquí» para reintentar.",
        );
      });
    }
  }

  function toggleLocalAudio() {
    listenHere = !listenHere;
    syncLocalAudio();
  }

  function setLocalVolume(v: number) {
    localVolume = Math.max(0, Math.min(1, v));
    if (audioEl) audioEl.volume = localVolume;
  }

  async function loadState() {
    if (statePending) return;
    statePending = true;
    try {
      const r = await fetch("/api/state");
      if (!r.ok) return;
      const snap = (await r.json()) as JamSnapshot;
      snapshot = snap;
      const dur = snap.now_playing?.duration_secs ?? 0;
      const pos = snap.position_secs || 0;
      smoothPos = dur > 0 ? Math.min(dur, pos) : pos;
      syncLocalAudio();
    } catch {
      // ignore transient network errors
    } finally {
      statePending = false;
    }
  }

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
      showToast(`Agregada a la cola: ${track.title}`);
      window.setTimeout(() => void loadState(), 550);
      window.setTimeout(() => {
        const next = { ...addingIds };
        delete next[track.id];
        addingIds = next;
      }, 1800);
    } catch {
      const next = { ...addingIds };
      delete next[track.id];
      addingIds = next;
      showToast("No se pudo enviar la canción");
    }
  }

  async function sendControl(action: "play_pause" | "next" | "prev") {
    try {
      await fetch("/api/control", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ action }),
      });
      window.setTimeout(() => void loadState(), 220);
    } catch {
      showToast("No se pudo enviar el comando");
    }
  }

  async function moveQueue(from: number, to: number) {
    if (from === to || from < 0 || to < 0 || to >= queue.length) return;
    try {
      const r = await fetch("/api/queue/move", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ from, to }),
      });
      if (r.ok) window.setTimeout(() => void loadState(), 140);
    } catch {
      // ignore
    }
  }

  onMount(() => {
    void loadState();
    const poll = window.setInterval(() => void loadState(), 750);
    const tick = window.setInterval(() => {
      if (isPlaying && durationSecs > 0) {
        smoothPos = Math.min(durationSecs, smoothPos + 0.25);
      }
    }, 250);
    return () => {
      window.clearInterval(poll);
      window.clearInterval(tick);
      window.clearTimeout(toastTimer);
      window.clearTimeout(searchTimer);
      stopLocalAudio();
    };
  });
</script>

<div class="min-h-[100dvh] relative overflow-x-hidden pb-16">
  <!-- Ambient radial glow -->
  <div
    class="pointer-events-none fixed inset-0 -z-10"
    style="background:
      radial-gradient(880px 420px at 14% -6%, rgba(var(--ambient-rgb), 0.19), transparent 68%),
      radial-gradient(640px 360px at 88% -4%, rgba(234, 179, 8, 0.09), transparent 64%),
      var(--bg);"
  ></div>

  <div class="max-w-[1200px] mx-auto px-3.5 sm:px-6 pt-3.5 sm:pt-6 flex flex-col gap-4 sm:gap-5">
    <!-- Top bar -->
    <header class="flex items-center justify-between gap-3">
      <div class="flex items-center gap-2.5 min-w-0">
        <!-- GyroTube brand emblem (gyrotube.svg palette) -->
        <span
          class="w-9 h-9 rounded-xl flex items-center justify-center shrink-0 shadow-[0_6px_18px_rgba(16,185,129,0.28)]"
          style="background: radial-gradient(circle at 50% 25%, #34d399, #10b981 48%, #047857 100%);"
          aria-hidden="true"
        >
          <span class="w-[26px] h-[26px] rounded-full bg-[#16141c] flex items-center justify-center border border-emerald-950/80">
            <span class="w-3 h-3 rounded-full bg-gradient-to-b from-[#ff5c6c] to-[#e11d48] ring-1 ring-amber-300/70 flex items-center justify-center">
              <Play size={7} fill="#fff" class="text-white translate-x-[0.5px]" />
            </span>
          </span>
        </span>
        <div class="flex items-center gap-2 min-w-0">
          <span class="font-extrabold text-[18px] sm:text-[20px] tracking-tight truncate">GyroTube</span>
          <span class="inline-flex items-center gap-1.5 text-[11px] font-bold tracking-wider uppercase px-2.5 py-0.5 rounded-full bg-[var(--gold-soft)] border border-[var(--gold-border)] text-[var(--gold-bright)]">
            <SpinIcon size={12} spinning={isPlaying} />
            SPIN
          </span>
        </div>
      </div>

      <div class="inline-flex items-center gap-2 text-[12.5px] font-semibold px-3 py-1.5 rounded-full bg-[var(--bg-elevated)] border border-[var(--border-strong)] shrink-0">
        <span class="w-2 h-2 rounded-full bg-[var(--accent-bright)] shadow-[0_0_8px_var(--accent-bright)] animate-pulse"></span>
        <Users size={13} class="text-[var(--text-muted)]" />
        <span class="numeric">{guestCount} {guestCount === 1 ? "invitado" : "invitados"}</span>
      </div>
    </header>

    <!-- Main responsive layout -->
    <main class="grid grid-cols-1 lg:grid-cols-[380px_minmax(0,1fr)] gap-4 sm:gap-5 items-start">
      <!-- Left column: Now Playing + Queue (desktop) -->
      <div class="flex flex-col gap-4 lg:sticky lg:top-5 min-w-0">
        <!-- Now Playing Card -->
        <section
          class="rounded-2xl border border-[var(--border)] p-4 sm:p-5 shadow-[0_14px_38px_rgba(0,0,0,0.34)] relative overflow-hidden"
          style="background: linear-gradient(160deg, rgba(16, 185, 129, 0.14) 0%, var(--bg-elevated) 58%);"
          aria-label="Sonando ahora"
        >
          <div class="flex items-center justify-between gap-2 mb-3.5">
            <span class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)]">
              Sonando ahora
            </span>
            {#if nowPlaying}
              <span
                class="inline-flex items-center gap-1.5 text-[11px] font-bold uppercase tracking-[0.06em] px-2 py-0.5 rounded-full {isPlaying
                  ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
                  : 'bg-white/[0.06] text-[var(--text-muted)]'}"
              >
                {#if isPlaying}
                  <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] animate-pulse"></span>
                  Reproduciendo
                {:else}
                  En pausa
                {/if}
              </span>
            {/if}
          </div>

          <!-- Cover + Metadata -->
          <div class="flex lg:flex-col items-center lg:text-center gap-3.5 lg:gap-4">
            <div class="relative w-[68px] h-[68px] lg:w-[208px] lg:h-[208px] rounded-xl lg:rounded-2xl overflow-hidden bg-[#16141c] border border-white/10 shadow-[0_14px_34px_rgba(0,0,0,0.48)] shrink-0">
              {#if nowPlaying?.cover_url}
                {#key nowPlaying.id}
                  <img
                    in:scale={{ start: 0.95, duration: 220, easing: cubicOut }}
                    src={nowPlaying.cover_url}
                    alt={nowPlaying.title}
                    class="w-full h-full object-cover block"
                  />
                {/key}
              {:else}
                <div class="w-full h-full flex items-center justify-center text-[var(--text-muted)]/50">
                  <Music size={30} />
                </div>
              {/if}
            </div>

            <div class="min-w-0 flex-1 w-full space-y-1">
              <h1 class="text-[16px] lg:text-[19px] font-extrabold leading-tight tracking-tight truncate">
                {nowPlaying?.title ?? "Sin reproducción"}
              </h1>
              <p class="text-[13.5px] lg:text-[14.5px] font-medium text-[var(--text-muted)] truncate">
                {nowPlaying?.artist ?? "Busca una canción para empezar"}
              </p>
              {#if nowPlaying?.album}
                <div class="inline-flex items-center gap-1.5 max-w-full px-2.5 py-0.5 rounded-full text-[11.5px] text-[var(--text-muted)] bg-white/[0.05] border border-white/[0.07]">
                  <Disc3 size={11} class="shrink-0 opacity-75" />
                  <span class="truncate">{nowPlaying.album}</span>
                </div>
              {/if}
            </div>
          </div>

          <!-- Progress bar -->
          <div class="mt-4">
            <div class="h-1.5 rounded-full bg-white/[0.11] overflow-hidden">
              <div
                class="h-full w-full rounded-full origin-left transition-transform duration-300 ease-linear"
                style="background: linear-gradient(90deg, var(--accent), var(--accent-bright)); transform: scaleX({progressRatio});"
              ></div>
            </div>
            <div class="flex items-center justify-between mt-1.5 text-[11.5px] text-[var(--text-muted)] numeric">
              <span>{fmt(smoothPos)}</span>
              <span>{durationSecs ? fmt(durationSecs) : "--:--"}</span>
            </div>
          </div>

          <!-- Controls & Listen Here -->
          <div class="mt-4 flex flex-col items-center gap-3">
            {#if allowControls}
              <div class="flex items-center justify-center gap-3">
                <button
                  type="button"
                  class="w-10 h-10 rounded-full flex items-center justify-center bg-white/[0.07] border border-white/[0.12] hover:bg-white/[0.13] transition-colors active:scale-95"
                  aria-label="Anterior"
                  onclick={() => void sendControl("prev")}
                >
                  <SkipBack size={18} fill="currentColor" />
                </button>
                <button
                  type="button"
                  class="h-11 px-5 rounded-full inline-flex items-center justify-center gap-2 font-bold text-[14px] text-white bg-[var(--accent)] hover:bg-[var(--accent-bright)] shadow-[0_6px_20px_rgba(16,185,129,0.36)] transition-all active:scale-95"
                  aria-label={isPlaying ? "Pausar" : "Reproducir"}
                  onclick={() => void sendControl("play_pause")}
                >
                  {#if isPlaying}
                    <Pause size={18} fill="currentColor" />
                    <span>Pausar</span>
                  {:else}
                    <Play size={18} fill="currentColor" class="translate-x-[1px]" />
                    <span>Reproducir</span>
                  {/if}
                </button>
                <button
                  type="button"
                  class="w-10 h-10 rounded-full flex items-center justify-center bg-white/[0.07] border border-white/[0.12] hover:bg-white/[0.13] transition-colors active:scale-95"
                  aria-label="Siguiente"
                  onclick={() => void sendControl("next")}
                >
                  <SkipForward size={18} fill="currentColor" />
                </button>
              </div>
            {/if}

            <div class="w-full flex flex-col items-center gap-2.5">
              <button
                type="button"
                class="inline-flex items-center justify-center gap-2 px-4 py-2 rounded-full text-[13px] font-semibold border transition-all active:scale-95 {listenHere
                  ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
                  : 'bg-white/[0.06] border-white/[0.12] text-[var(--text)] hover:bg-white/[0.12]'}"
                onclick={toggleLocalAudio}
              >
                {#if listenHere}
                  <VolumeX size={15} />
                  <span>Silenciar aquí</span>
                {:else}
                  <Headphones size={15} />
                  <span>Escuchar aquí</span>
                {/if}
              </button>

              {#if listenHere}
                <div
                  transition:fade={{ duration: 150 }}
                  class="w-full max-w-[240px] flex items-center gap-2.5 px-3 py-1.5 rounded-full bg-black/25 border border-white/[0.08]"
                >
                  {#if localVolume === 0}
                    <VolumeX size={14} class="text-[var(--text-muted)] shrink-0" />
                  {:else if localVolume < 0.5}
                    <Volume1 size={14} class="text-[var(--accent-bright)] shrink-0" />
                  {:else}
                    <Volume2 size={14} class="text-[var(--accent-bright)] shrink-0" />
                  {/if}
                  <input
                    type="range"
                    class="spin-range flex-1 min-w-0"
                    min="0"
                    max="1"
                    step="0.02"
                    value={localVolume}
                    aria-label="Volumen local"
                    oninput={(e) => setLocalVolume(Number((e.currentTarget as HTMLInputElement).value))}
                  />
                  <span class="text-[11px] font-semibold text-[var(--text-muted)] numeric w-8 text-right">
                    {Math.round(localVolume * 100)}%
                  </span>
                </div>
              {/if}
            </div>
          </div>

          <audio
            bind:this={audioEl}
            preload="auto"
            class="hidden"
            onloadedmetadata={syncLocalAudio}
            onerror={() => {
              if (listenHere && curAudioTrackId !== null) {
                localAudioFailed("No se pudo cargar el audio. Toca «Escuchar aquí» para reintentar.");
              }
            }}
          ></audio>
        </section>

        <!-- Mobile tab switcher -->
        <div
          class="grid lg:hidden grid-cols-2 gap-1 p-1 rounded-full bg-[var(--bg-elevated)] border border-[var(--border)]"
          role="tablist"
          aria-label="Secciones de SPIN"
        >
          <button
            type="button"
            role="tab"
            aria-selected={mobileTab === "search"}
            class="inline-flex items-center justify-center gap-1.5 py-2 px-3 rounded-full text-[13px] font-bold transition-colors {mobileTab === 'search'
              ? 'bg-[var(--accent)] text-white shadow-sm'
              : 'text-[var(--text-muted)]'}"
            onclick={() => (mobileTab = "search")}
          >
            <Search size={14} />
            <span>Buscar</span>
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={mobileTab === "queue"}
            class="inline-flex items-center justify-center gap-1.5 py-2 px-3 rounded-full text-[13px] font-bold transition-colors {mobileTab === 'queue'
              ? 'bg-[var(--accent)] text-white shadow-sm'
              : 'text-[var(--text-muted)]'}"
            onclick={() => (mobileTab = "queue")}
          >
            <ListMusic size={14} />
            <span>Cola ({queue.length})</span>
          </button>
        </div>

        <!-- Queue Card -->
        <section
          class="rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border)] p-4 shadow-[0_12px_32px_rgba(0,0,0,0.28)] {mobileTab === 'queue'
            ? 'block'
            : 'hidden lg:block'}"
          aria-label="Cola de reproducción"
        >
          <div class="flex items-center justify-between gap-2 mb-3">
            <span class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)]">
              Cola ({queue.length})
            </span>
          </div>

          {#if queue.length === 0}
            <div class="py-8 text-center text-[13.5px] text-[var(--text-muted)]">
              La cola está vacía
            </div>
          {:else}
            <div class="flex flex-col gap-1 lg:max-h-[420px] lg:overflow-y-auto scroll-thin pr-0.5">
              {#each queue as item, i (item.id + ":" + i)}
                <div
                  role="listitem"
                  draggable={allowControls}
                  ondragstart={(e) => {
                    dragFrom = i;
                    e.dataTransfer?.setData("text/plain", String(i));
                  }}
                  ondragover={(e) => {
                    if (!allowControls) return;
                    e.preventDefault();
                    dragOver = i;
                  }}
                  ondragleave={() => {
                    if (dragOver === i) dragOver = null;
                  }}
                  ondrop={(e) => {
                    e.preventDefault();
                    const from = dragFrom;
                    dragFrom = null;
                    dragOver = null;
                    if (from !== null && from !== i) void moveQueue(from, i);
                  }}
                  ondragend={() => {
                    dragFrom = null;
                    dragOver = null;
                  }}
                  class="flex items-center gap-2.5 p-2 rounded-xl border transition-colors {item.is_current
                    ? 'bg-[var(--accent-soft)] border-[var(--accent-border)]'
                    : 'border-transparent hover:bg-white/[0.045]'} {dragFrom === i
                    ? 'opacity-40'
                    : ''} {dragOver === i && dragFrom !== i
                    ? 'ring-1 ring-[var(--accent-bright)]'
                    : ''}"
                >
                  {#if allowControls}
                    <span class="text-[var(--text-muted)]/50 cursor-grab shrink-0 hidden sm:flex" aria-hidden="true">
                      <GripVertical size={14} />
                    </span>
                  {/if}

                  <div class="relative w-10 h-10 rounded-lg overflow-hidden bg-white/[0.06] border border-white/[0.08] shrink-0">
                    {#if item.cover_url}
                      <img src={item.cover_url} alt="" loading="lazy" class="w-full h-full object-cover" />
                    {:else}
                      <div class="w-full h-full flex items-center justify-center text-[var(--text-muted)]/50">
                        <Music size={15} />
                      </div>
                    {/if}
                    {#if item.is_current && isPlaying}
                      <div class="absolute inset-0 bg-black/50 flex items-center justify-center gap-[2px]">
                        <span class="eq-bar" style="animation-delay: 0ms;"></span>
                        <span class="eq-bar" style="animation-delay: 160ms;"></span>
                        <span class="eq-bar" style="animation-delay: 320ms;"></span>
                      </div>
                    {/if}
                  </div>

                  <div class="min-w-0 flex-1">
                    <div
                      class="text-[13.5px] font-semibold truncate {item.is_current
                        ? 'text-[var(--accent-bright)]'
                        : 'text-[var(--text)]'}"
                    >
                      {item.title}
                    </div>
                    <div class="text-[12px] text-[var(--text-muted)] truncate">{item.artist}</div>
                  </div>

                  {#if item.duration_secs}
                    <span class="text-[11.5px] text-[var(--text-muted)] numeric shrink-0">
                      {fmt(item.duration_secs)}
                    </span>
                  {/if}

                  {#if allowControls}
                    <div class="flex items-center gap-1 shrink-0">
                      <button
                        type="button"
                        disabled={i === 0}
                        class="w-7 h-7 rounded-full flex items-center justify-center bg-white/[0.06] border border-white/[0.08] hover:bg-white/[0.14] transition-colors"
                        aria-label="Subir"
                        onclick={() => void moveQueue(i, i - 1)}
                      >
                        <ChevronUp size={13} />
                      </button>
                      <button
                        type="button"
                        disabled={i === queue.length - 1}
                        class="w-7 h-7 rounded-full flex items-center justify-center bg-white/[0.06] border border-white/[0.08] hover:bg-white/[0.14] transition-colors"
                        aria-label="Bajar"
                        onclick={() => void moveQueue(i, i + 1)}
                      >
                        <ChevronDown size={13} />
                      </button>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </section>
      </div>

      <!-- Right column: Search & Add to Queue -->
      <section
        class="rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border)] p-4 sm:p-5 shadow-[0_12px_32px_rgba(0,0,0,0.28)] min-w-0 {mobileTab === 'search'
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
                    {fmt(track.duration_secs)}
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
    </main>
  </div>

  <!-- Floating toast -->
  {#if toastMessage}
    <div
      transition:fly={{ y: 12, duration: 180, easing: cubicOut }}
      class="fixed bottom-5 left-1/2 -translate-x-1/2 z-50 max-w-[92vw] px-4 py-2.5 rounded-2xl bg-[#24212e] border border-[var(--border-strong)] shadow-[0_14px_36px_rgba(0,0,0,0.55)] flex items-center gap-3 text-[13.5px] font-medium"
      role="status"
      aria-live="polite"
    >
      <span>{toastMessage}</span>
      <button
        type="button"
        class="w-6 h-6 rounded-full flex items-center justify-center bg-white/[0.08] hover:bg-white/[0.16]"
        aria-label="Cerrar notificación"
        onclick={() => (toastMessage = null)}
      >
        <X size={13} />
      </button>
    </div>
  {/if}
</div>
