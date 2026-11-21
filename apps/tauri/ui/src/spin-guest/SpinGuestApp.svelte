<script lang="ts">
  import { ListMusic, Play, Search, Users, X } from "lucide-svelte";
  import { onMount } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import SpinIcon from "../components/icons/SpinIcon.svelte";
  import SpinNowPlaying from "./SpinNowPlaying.svelte";
  import SpinQueue from "./SpinQueue.svelte";
  import SpinSearch from "./SpinSearch.svelte";
  import type { JamSnapshot } from "./types";

  let snapshot = $state<JamSnapshot | null>(null);
  let mobileTab = $state<"search" | "queue">("search");
  let listenHere = $state(false);
  let localVolume = $state(0.85);
  let toastMessage = $state<string | null>(null);
  let smoothPos = $state(0);

  let audioEl: HTMLAudioElement | undefined;
  let curAudioTrackId: string | null = null;
  let statePending = false;
  let toastTimer: number | undefined;

  let nowPlaying = $derived(snapshot?.now_playing ?? null);
  let isPlaying = $derived(Boolean(snapshot?.is_playing && nowPlaying));
  let allowControls = $derived(Boolean(snapshot?.allow_controls));
  let queue = $derived(snapshot?.queue ?? []);
  let guestCount = $derived(snapshot?.guest_count ?? 0);
  let durationSecs = $derived(nowPlaying?.duration_secs ?? 0);

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
      stopLocalAudio();
    };
  });
</script>

<div class="min-h-[100dvh] relative overflow-x-hidden pb-16">
  <div
    class="pointer-events-none fixed inset-0 -z-10"
    style="background:
      radial-gradient(880px 420px at 14% -6%, rgba(var(--ambient-rgb), 0.19), transparent 68%),
      radial-gradient(640px 360px at 88% -4%, rgba(234, 179, 8, 0.09), transparent 64%),
      var(--bg);"
  ></div>

  <div class="max-w-[1200px] mx-auto px-3.5 sm:px-6 pt-3.5 sm:pt-6 flex flex-col gap-4 sm:gap-5">
    <header class="flex items-center justify-between gap-3">
      <div class="flex items-center gap-2.5 min-w-0">
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

    <main class="grid grid-cols-1 lg:grid-cols-[380px_minmax(0,1fr)] gap-4 sm:gap-5 items-start">
      <div class="flex flex-col gap-4 lg:sticky lg:top-5 min-w-0">
        <SpinNowPlaying
          {nowPlaying}
          {isPlaying}
          {allowControls}
          {smoothPos}
          {durationSecs}
          {listenHere}
          {localVolume}
          onControl={(action) => void sendControl(action)}
          onToggleListen={() => {
            listenHere = !listenHere;
            syncLocalAudio();
          }}
          onVolumeChange={(v) => {
            localVolume = Math.max(0, Math.min(1, v));
            if (audioEl) audioEl.volume = localVolume;
          }}
        />

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

        <SpinQueue
          {queue}
          {isPlaying}
          {allowControls}
          visibleMobile={mobileTab === "queue"}
          onMove={(from, to) => void moveQueue(from, to)}
        />
      </div>

      <SpinSearch
        {isPlaying}
        visibleMobile={mobileTab === "search"}
        onAdded={(track) => {
          showToast(`Agregada a la cola: ${track.title}`);
          window.setTimeout(() => void loadState(), 550);
        }}
        onError={(msg) => showToast(msg)}
      />
    </main>
  </div>

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
