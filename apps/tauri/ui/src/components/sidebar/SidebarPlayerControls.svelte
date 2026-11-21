<script lang="ts">
  import {
    Heart,
    PartyPopper,
    Pause,
    Play,
    Radio,
    Repeat,
    Shuffle,
    SkipBack,
    SkipForward,
    Timer,
    Volume1,
    Volume2,
    VolumeX,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, fly, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  interface Props {
    sleepRemainingSecs: number | null;
    onSetSleepTimer: (minutes: number | null) => void;
  }

  let { sleepRemainingSecs, onSetSleepTimer }: Props = $props();

  type ActionPopover = "volume" | "timer" | null;

  let seeking = $state(false);
  let seekDraft = $state(0);
  let openPopover = $state<ActionPopover>(null);
  let prevVolume = $state(1);
  let startingRadio = $state(false);

  let displayPos = $derived(seeking ? seekDraft : player.positionSecs);
  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (displayPos / player.durationSecs) * 100))
      : 0,
  );
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

<div in:fade={{ duration: 180 }} class="flex-1 flex flex-col items-center justify-between py-4 gap-5">
  <!-- Centered large cover art -->
  <div class="flex-1 flex items-center justify-center w-full py-2">
    {#key player.currentTrack?.id ?? "none"}
      <div in:scale={{ start: 0.94, duration: 260, easing: cubicOut }}>
        <CoverImage
          class="cover-large !w-[264px] !h-[264px] !rounded-2xl border border-white/10 shadow-[0_22px_50px_rgba(0,0,0,0.48)] transition-transform duration-300 hover:scale-[1.015]"
          src={player.currentTrack?.cover_large_url}
          fallbackSrc={player.currentTrack?.cover_url}
          alt={player.currentTrack?.title ?? ""}
          eager={true}
        />
      </div>
    {/key}
  </div>

  <!-- Lower controls stack -->
  <div class="w-full flex flex-col items-center gap-4">
    <!-- Track metadata -->
    <div class="w-full text-center px-1 space-y-1">
      <h2 class="text-[19px] font-extrabold leading-tight tracking-tight line-clamp-2">
        {player.currentTrack?.title ?? t("player.nothing_playing")}
      </h2>
      <p class="text-[14px] font-medium text-[var(--text-muted)] truncate">
        {player.currentTrack?.artist ?? t("player.select_song")}
      </p>
      {#if player.currentTrack?.album}
        <p class="text-[12px] text-[var(--text-muted)]/75 truncate">
          {t("player.album_prefix", { album: player.currentTrack.album })}
        </p>
      {/if}
    </div>

    <!-- 5-icon minimalist action row (Volume, Favorite, Start Radio, Sleep Timer, Party Mode) -->
    <div class="relative w-full flex items-center justify-center gap-5 py-0.5">
      <button
        class="icon-btn !w-9 !h-9 {openPopover === 'volume' || player.volume === 0
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title="Volume ({volumePct}%)"
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

      <button
        class="icon-btn !w-9 !h-9 transition-transform active:scale-90 {isFav
          ? 'text-[var(--coral)]'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        disabled={!player.currentTrack}
        title={isFav ? "Remove from favorites" : "Add to favorites"}
        onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
      >
        <Heart size={18} fill={isFav ? "currentColor" : "none"} />
      </button>

      <button
        class="icon-btn !w-9 !h-9 {startingRadio
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        disabled={!player.currentTrack || startingRadio}
        title="Start radio from this song"
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
        class="icon-btn !w-9 !h-9 relative {openPopover === 'timer' || sleepRemainingSecs != null
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={sleepRemainingSecs != null
          ? `Sleep timer: ${formatDuration(sleepRemainingSecs)}`
          : "Sleep timer"}
        onclick={() => togglePopover("timer")}
      >
        <Timer size={18} />
        {#if sleepRemainingSecs != null}
          <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)] absolute top-1 right-1"></span>
        {/if}
      </button>

      <button
        class="icon-btn !w-9 !h-9 {player.partyMode
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title="Party Mode"
        onclick={() => {
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
              class="icon-btn !w-8 !h-8 shrink-0 bg-[var(--surface)] hover:bg-[var(--surface-hover)] {player.volume === 0
                ? 'text-[var(--coral)]'
                : 'text-[var(--text)]'}"
              onclick={toggleMute}
              title={player.volume === 0 ? "Unmute" : "Mute"}
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
              aria-label="Volume"
              style="background: linear-gradient(to right, var(--accent-bright) {volumePct}%, rgba(255,255,255,0.16) {volumePct}%);"
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

    <!-- Seek bar + Time / Quality -->
    <div class="w-full space-y-2.5">
      <input
        class="sidebar-seek w-full"
        type="range"
        min="0"
        max={Math.max(1, player.durationSecs)}
        step="0.5"
        value={displayPos}
        disabled={!player.currentTrack}
        aria-label="Seek"
        style="background: linear-gradient(to right, var(--accent-bright) {progressPct}%, rgba(255,255,255,0.16) {progressPct}%);"
        onpointerdown={() => {
          seeking = true;
          seekDraft = player.positionSecs;
        }}
        oninput={(e) => {
          seekDraft = Number((e.currentTarget as HTMLInputElement).value);
        }}
        onchange={(e) => {
          const target = Number((e.currentTarget as HTMLInputElement).value);
          seeking = false;
          player.seek(target);
        }}
      />
      <div class="flex items-center justify-between text-[12px] text-[var(--text-muted)] numeric">
        <span>{formatDuration(displayPos)}</span>
        <span class="px-3 py-0.5 rounded-full text-[11px] font-bold bg-[var(--accent-soft)] text-[var(--accent-bright)]">
          {player.qualityLabel}
        </span>
        <span>{formatDuration(player.durationSecs)}</span>
      </div>
    </div>

    <!-- Transport controls -->
    <div class="w-full flex items-center justify-center gap-4 pt-1 pb-1">
      <button
        class="icon-btn relative {player.shuffle
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title="Shuffle"
        onclick={() => player.toggleShuffle()}
      >
        <Shuffle size={17} />
        {#if player.shuffle}
          <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>
        {/if}
      </button>

      <button
        class="icon-btn !w-9 !h-9"
        disabled={!player.currentTrack}
        title="Previous"
        onclick={() => void player.prev()}
      >
        <SkipBack size={20} fill="currentColor" />
      </button>

      <button
        class="play-circle !w-[56px] !h-[56px] shrink-0"
        disabled={!player.currentTrack}
        title="Play / Pause"
        onclick={() => void player.togglePlay()}
      >
        {#if player.isLoadingAudio}
          <span class="w-5 h-5 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>
        {:else if player.isPlaying}
          <Pause size={22} fill="currentColor" />
        {:else}
          <Play size={22} fill="currentColor" class="translate-x-[1.5px]" />
        {/if}
      </button>

      <button
        class="icon-btn !w-9 !h-9"
        disabled={!player.currentTrack}
        title="Next"
        onclick={() => void player.next()}
      >
        <SkipForward size={20} fill="currentColor" />
      </button>

      <button
        class="icon-btn relative {player.repeat
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title="Repeat"
        onclick={() => player.toggleRepeat()}
      >
        <Repeat size={17} />
        {#if player.repeat}
          <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  .sidebar-seek {
    height: 4px;
  }

  .sidebar-seek::-webkit-slider-thumb {
    width: 17px;
    height: 17px;
    background: #e5e7eb;
    border: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.45);
  }

  .sidebar-seek::-moz-range-thumb {
    width: 17px;
    height: 17px;
    background: #e5e7eb;
    border: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.45);
  }
</style>
