<script lang="ts">
  import { ChevronDown, ChevronRight, ListMusic, MessageSquareQuote, Music, Sparkles } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import SidebarLyricsPane from "./sidebar/SidebarLyricsPane.svelte";
  import SidebarPlayerControls from "./sidebar/SidebarPlayerControls.svelte";
  import SidebarQueuePane from "./sidebar/SidebarQueuePane.svelte";
  import SidebarRelatedPane from "./sidebar/SidebarRelatedPane.svelte";

  // Kept at SidebarPlayer level so switching tabs never cancels an active sleep timer
  let sleepRemainingSecs = $state<number | null>(null);
  let sleepInterval: number | undefined;

  // Swipe-down to dismiss player sheet on mobile/compact viewports
  let scrollPaneEl = $state<HTMLDivElement | null>(null);
  let swipeDownY = $state(0);
  let swipeTracking = false;
  let swipeStartX = 0;
  let swipeStartY = 0;
  let swipeStartAt = 0;

  function handleTouchStart(e: TouchEvent) {
    if (window.innerWidth >= 1024 || e.touches.length !== 1) return;
    const target = e.target as HTMLElement | null;
    if (target?.closest("input[type='range'], [data-drag-handle]")) return;
    // Allow pull-down from top bar anytime, or from the content pane when scrolled to top
    const inHeader = Boolean(target?.closest("[data-player-header]"));
    const atTop = !scrollPaneEl || scrollPaneEl.scrollTop <= 2;
    if (!inHeader && !atTop) return;
    const touch = e.touches[0];
    swipeTracking = true;
    swipeStartX = touch.clientX;
    swipeStartY = touch.clientY;
    swipeStartAt = performance.now();
    swipeDownY = 0;
  }

  function handleTouchMove(e: TouchEvent) {
    if (!swipeTracking || e.touches.length !== 1) return;
    const touch = e.touches[0];
    const dx = touch.clientX - swipeStartX;
    const dy = touch.clientY - swipeStartY;
    if (swipeDownY === 0) {
      if (dy < 8) {
        if (Math.abs(dx) > 12 || dy < -8) swipeTracking = false;
        return;
      }
      if (Math.abs(dx) * 1.25 > dy) {
        swipeTracking = false;
        return;
      }
    }
    swipeDownY = Math.max(0, Math.min(260, dy));
  }

  function handleTouchEnd() {
    if (!swipeTracking) return;
    const dy = swipeDownY;
    const elapsed = Math.max(1, performance.now() - swipeStartAt);
    const velocity = dy / elapsed;
    swipeTracking = false;
    swipeDownY = 0;
    if (dy > 90 || (dy > 48 && velocity > 0.45)) {
      player.closePlayerSheet();
    }
  }

  function setSleepTimer(minutes: number | null) {
    window.clearInterval(sleepInterval);
    sleepInterval = undefined;
    if (!minutes) {
      sleepRemainingSecs = null;
      return;
    }
    sleepRemainingSecs = minutes * 60;
    player.showToast(`Sleep timer set for ${minutes} min`);
    sleepInterval = window.setInterval(() => {
      if (sleepRemainingSecs == null) return;
      sleepRemainingSecs -= 1;
      if (sleepRemainingSecs <= 0) {
        window.clearInterval(sleepInterval);
        sleepInterval = undefined;
        sleepRemainingSecs = null;
        if (player.isPlaying) void player.togglePlay();
        player.showToast("Sleep timer ended — playback paused");
      }
    }, 1000);
  }
</script>

<aside
  class="w-full md:w-[388px] lg:w-[360px] xl:w-[388px] shrink-0 h-full flex flex-col border-l max-md:border-l-0 border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden relative select-none"
  style="background-image: linear-gradient(180deg, rgba(var(--ambient-rgb), 0.22) 0%, rgba(var(--ambient-rgb), 0.08) 52%, transparent 100%);{swipeDownY > 0
    ? ` transform: translateY(${swipeDownY}px); transition: none;`
    : ' transition: transform 180ms cubic-bezier(0.22, 1, 0.36, 1);'}"
  ontouchstart={handleTouchStart}
  ontouchmove={handleTouchMove}
  ontouchend={handleTouchEnd}
  ontouchcancel={handleTouchEnd}
>
  <!-- Mobile pull-down handle -->
  <div
    data-player-header
    class="lg:hidden shrink-0 flex justify-center pt-[max(0.6rem,env(safe-area-inset-top))] pb-0.5"
    aria-hidden="true"
  >
    <div class="w-10 h-1 rounded-full bg-[var(--border-strong)]"></div>
  </div>

  <!-- Top context bar -->
  <div data-player-header class="shrink-0 flex items-center justify-between gap-3 px-5 pt-2 lg:pt-4 pb-2">
    <button
      type="button"
      class="icon-btn shrink-0 text-[var(--text-muted)] hover:text-[var(--text)]"
      title={t("player.collapse")}
      onclick={() => player.closePlayerSheet()}
    >
      <ChevronDown size={20} class="md:hidden" />
      <ChevronRight size={19} class="hidden md:block" />
    </button>
    <div class="min-w-0 flex flex-col leading-tight text-right">
      <span class="text-[11.5px] font-medium text-[var(--text-muted)]">{t("player.playing_from")}</span>
      <span class="text-[13px] font-semibold truncate">
        {player.sourceName === "Not playing" ? t("player.not_playing") : player.sourceName}
      </span>
    </div>
  </div>

  <!-- Active pane -->
  <div
    bind:this={scrollPaneEl}
    class="flex-1 min-h-0 px-7 pb-4 flex flex-col {player.sidebarTab === 'lyrics'
      ? 'overflow-hidden'
      : 'overflow-y-auto scroll-y'}"
  >
    {#if player.sidebarTab === "player"}
      <SidebarPlayerControls {sleepRemainingSecs} onSetSleepTimer={setSleepTimer} />
    {:else if player.sidebarTab === "lyrics"}
      <SidebarLyricsPane />
    {:else if player.sidebarTab === "related"}
      <SidebarRelatedPane />
    {:else}
      <SidebarQueuePane />
    {/if}
  </div>

  <!-- Bottom tab bar -->
  <nav
    class="shrink-0 grid grid-cols-4 gap-1 px-2.5 py-2 border-t border-[var(--border)] bg-[var(--bg-sidebar)]"
    style="padding-bottom: max(0.5rem, env(safe-area-inset-bottom));"
  >
    <button
      class="flex flex-col items-center justify-center gap-0.5 py-1.5 rounded-xl text-[11px] font-semibold border transition-colors {player.sidebarTab === 'player'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "player")}
    >
      <Music size={15} />
      <span class="truncate max-w-full px-0.5">{t("player.tab_player")}</span>
    </button>
    <button
      disabled={player.videoMode}
      class="flex flex-col items-center justify-center gap-0.5 py-1.5 rounded-xl text-[11px] font-semibold border transition-colors {player.sidebarTab === 'lyrics'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => !player.videoMode && (player.sidebarTab = "lyrics")}
    >
      <MessageSquareQuote size={15} />
      <span class="truncate max-w-full px-0.5">{t("player.tab_lyrics")}</span>
    </button>
    <button
      class="flex flex-col items-center justify-center gap-0.5 py-1.5 rounded-xl text-[11px] font-semibold border transition-colors {player.sidebarTab === 'queue'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "queue")}
    >
      <ListMusic size={15} />
      <span class="truncate max-w-full px-0.5">{t("player.tab_queue")}</span>
    </button>
    <button
      class="flex flex-col items-center justify-center gap-0.5 py-1.5 rounded-xl text-[11px] font-semibold border transition-colors {player.sidebarTab === 'related'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "related")}
    >
      <Sparkles size={15} />
      <span class="truncate max-w-full px-0.5">{t("player.tab_related")}</span>
    </button>
  </nav>
</aside>
