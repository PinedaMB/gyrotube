<script lang="ts">
  import {
    Heart,
    PartyPopper,
    Radio,
    Timer,
    Volume1,
    Volume2,
    VolumeX,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { device } from "../../lib/stores/device.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration } from "../../lib/types";

  interface Props {
    sleepRemainingSecs: number | null;
    onSetSleepTimer: (minutes: number | null) => void;
  }

  let { sleepRemainingSecs, onSetSleepTimer }: Props = $props();

  type ActionPopover = "volume" | "timer" | null;
  let openPopover = $state<ActionPopover>(null);
  let prevVolume = $state(1);
  let startingRadio = $state(false);

  let volumePct = $derived(Math.round(player.volume * 100));
  let isFav = $derived(player.currentTrack ? player.isFavorite(player.currentTrack.id) : false);

  function togglePopover(kind: Exclude<ActionPopover, null>) {
    openPopover = openPopover === kind ? null : kind;
  }

  function toggleMute() {
    if (player.volume > 0.01) {
      prevVolume = player.volume;
      player.setVolume(0);
    } else {
      player.setVolume(prevVolume > 0.05 ? prevVolume : 0.8);
    }
  }
</script>

<div class="relative w-full flex items-center justify-center gap-5 py-0.5">
  {#if !device.isAndroid}
    <button
      type="button"
      class="icon-btn !w-9 !h-9 {openPopover === 'volume' || player.volume === 0
        ? 'active'
        : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
      title="{t('player.volume')} ({volumePct}%)"
      onclick={() => togglePopover("volume")}
    >
      {#if player.volume === 0}
        <VolumeX size={18} />
      {:else if player.volume < 0.5}
        <Volume1 size={18} />
      {:else}
        <Volume2 size={18} />
      {/if}
    </button>
  {/if}

  <button
    type="button"
    class="icon-btn !w-9 !h-9 transition-transform active:scale-90 {isFav
      ? 'text-[var(--coral)]'
      : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
    disabled={!player.currentTrack}
    title={isFav ? t("player.favorite_remove") : t("player.favorite_add")}
    onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
  >
    <Heart size={18} fill={isFav ? "currentColor" : "none"} />
  </button>

  <button
    type="button"
    class="icon-btn !w-9 !h-9 {startingRadio
      ? 'active'
      : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
    disabled={!player.currentTrack || startingRadio}
    title={t("player.start_radio")}
    onclick={async () => {
      openPopover = null;
      startingRadio = true;
      try {
        await player.startRadioFromCurrent();
      } finally {
        startingRadio = false;
      }
    }}
  >
    {#if startingRadio}
      <span class="w-4 h-4 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
    {:else}
      <Radio size={18} />
    {/if}
  </button>

  <button
    type="button"
    class="icon-btn !w-9 !h-9 relative {openPopover === 'timer' || sleepRemainingSecs != null
      ? 'active'
      : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
    title={sleepRemainingSecs != null
      ? `${t("player.sleep_timer")}: ${formatDuration(sleepRemainingSecs)}`
      : t("player.sleep_timer")}
    onclick={() => togglePopover("timer")}
  >
    <Timer size={18} />
    {#if sleepRemainingSecs != null}
      <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1"></span>
    {/if}
  </button>

  <button
    type="button"
    disabled={player.videoMode}
    class="icon-btn !w-9 !h-9 {player.partyMode
      ? 'active'
      : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
    title={t("header.party_mode")}
    onclick={() => {
      if (player.videoMode) return;
      openPopover = null;
      player.partyMode = true;
    }}
  >
    <PartyPopper size={18} />
  </button>

  {#if openPopover !== null}
    <button
      type="button"
      aria-label="Close popover"
      class="fixed inset-0 z-20 cursor-default bg-transparent"
      onclick={() => (openPopover = null)}
    ></button>
  {/if}

  {#if openPopover === "volume"}
    <div
      transition:fly={{ y: 8, duration: 170, easing: cubicOut }}
      class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"
    >
      <div class="flex items-center justify-between text-[12px]">
        <span class="font-semibold text-[var(--text-muted)]">{t("player.volume")}</span>
        <span class="font-bold text-[var(--accent-bright)] numeric">{volumePct}%</span>
      </div>
      <div class="flex items-center gap-3">
        <button
          type="button"
          class="icon-btn !w-8 !h-8 shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] {player.volume === 0
            ? 'text-[var(--coral)]'
            : 'text-[var(--text)]'}"
          onclick={toggleMute}
          title={player.volume === 0 ? t("player.unmute") : t("player.mute")}
        >
          {#if player.volume === 0}
            <VolumeX size={16} />
          {:else if player.volume < 0.5}
            <Volume1 size={16} />
          {:else}
            <Volume2 size={16} />
          {/if}
        </button>
        <input
          class="sidebar-seek flex-1 min-w-0"
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={player.volume}
          aria-label={t("player.volume")}
          style="background: linear-gradient(to right, var(--accent-bright) {volumePct}%, var(--control-track) {volumePct}%);"
          oninput={(e) => player.setVolume(Number((e.currentTarget as HTMLInputElement).value))}
        />
      </div>
    </div>
  {:else if openPopover === "timer"}
    <div
      transition:fly={{ y: 8, duration: 170, easing: cubicOut }}
      class="absolute bottom-12 left-1/2 -translate-x-1/2 w-[272px] p-3.5 rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.5)] z-30 space-y-2.5"
    >
      <div class="flex items-center justify-between text-[12px]">
        <span class="font-semibold text-[var(--text-muted)]">{t("player.sleep_timer")}</span>
        {#if sleepRemainingSecs != null}
          <span class="font-bold text-[var(--accent-bright)] numeric">
            {formatDuration(sleepRemainingSecs)}
          </span>
        {/if}
      </div>
      <div class="grid grid-cols-5 gap-1.5">
        {#each [null, 15, 30, 45, 60] as mins}
          <button
            type="button"
            class="py-1.5 rounded-xl text-[12px] font-semibold bg-[var(--surface)] text-[var(--text-muted)] hover:bg-[var(--accent-soft)] hover:text-[var(--accent-bright)] transition-colors"
            onclick={() => {
              onSetSleepTimer(mins);
              openPopover = null;
            }}
          >
            {mins == null ? t("player.sleep_off") : `${mins}m`}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>
