<script lang="ts">
  import { ChevronRight, ListMusic, MessageSquareQuote, Music } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import SidebarLyricsPane from "./sidebar/SidebarLyricsPane.svelte";
  import SidebarPlayerControls from "./sidebar/SidebarPlayerControls.svelte";
  import SidebarQueuePane from "./sidebar/SidebarQueuePane.svelte";

  // Kept at SidebarPlayer level so switching tabs never cancels an active sleep timer
  let sleepRemainingSecs = $state<number | null>(null);
  let sleepInterval: number | undefined;

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
  class="w-[388px] shrink-0 h-full flex flex-col border-l border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden relative select-none"
  style="background-image: linear-gradient(180deg, rgba(var(--ambient-rgb), 0.22) 0%, rgba(var(--ambient-rgb), 0.08) 52%, transparent 100%);"
>
  <!-- Top context bar -->
  <div class="shrink-0 flex items-center justify-between gap-3 px-5 pt-4 pb-2">
    <button
      class="icon-btn shrink-0 text-[var(--text-muted)] hover:text-[var(--text)]"
      title={t("player.collapse")}
      onclick={() => {
        player.sidebarOpen = false;
        void player.persistSession();
      }}
    >
      <ChevronRight size={19} />
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
    class="flex-1 min-h-0 px-7 pb-4 flex flex-col {player.sidebarTab === 'lyrics'
      ? 'overflow-hidden'
      : 'overflow-y-auto scroll-y'}"
  >
    {#if player.sidebarTab === "player"}
      <SidebarPlayerControls {sleepRemainingSecs} onSetSleepTimer={setSleepTimer} />
    {:else if player.sidebarTab === "lyrics"}
      <SidebarLyricsPane />
    {:else}
      <SidebarQueuePane />
    {/if}
  </div>

  <!-- Bottom tab bar -->
  <nav class="shrink-0 grid grid-cols-3 gap-1.5 px-3 py-2.5 border-t border-[var(--border)] bg-[var(--bg-sidebar)]">
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'player'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "player")}
    >
      <Music size={15} />
      {t("player.tab_player")}
    </button>
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'lyrics'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "lyrics")}
    >
      <MessageSquareQuote size={15} />
      {t("player.tab_lyrics")}
    </button>
    <button
      class="flex items-center justify-center gap-1.5 py-2 rounded-xl text-[13px] font-semibold border transition-colors {player.sidebarTab === 'queue'
        ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
        : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      onclick={() => (player.sidebarTab = "queue")}
    >
      <ListMusic size={15} />
      {t("player.tab_queue")}
    </button>
  </nav>
</aside>
