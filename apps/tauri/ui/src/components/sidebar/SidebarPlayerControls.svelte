<script lang="ts">
  import {
    ChevronRight,
    Disc3,
    Film,
    Music,
    Pause,
    Play,
    Repeat,
    Shuffle,
    SkipBack,
    SkipForward,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fade, scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";
  import SyncedVideoPlayer from "../SyncedVideoPlayer.svelte";
  import SidebarActionPopovers from "./SidebarActionPopovers.svelte";

  interface Props {
    sleepRemainingSecs: number | null;
    onSetSleepTimer: (minutes: number | null) => void;
  }

  let { sleepRemainingSecs, onSetSleepTimer }: Props = $props();

  let seeking = $state(false);
  let seekDraft = $state(0);
  let resolvingArtist = $state(false);

  let displayPos = $derived(seeking ? seekDraft : player.positionSecs);
  let progressPct = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(100, (displayPos / player.durationSecs) * 100))
      : 0,
  );
</script>

<div in:fade={{ duration: 180 }} class="flex-1 flex flex-col items-center justify-between py-3 gap-4">
  <!-- Song / Video mode toggle -->
  {#if player.currentTrack}
    <div class="inline-flex items-center p-0.5 rounded-full bg-[var(--surface)] border border-[var(--border)]">
      <button
        type="button"
        class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[11.5px] font-bold transition-colors {!player.videoMode
          ? 'bg-[var(--accent)] text-white shadow-sm'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        onclick={() => player.setVideoMode(false)}
      >
        <Music size={12} />
        <span>{t("player.mode_song")}</span>
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[11.5px] font-bold transition-colors {player.videoMode
          ? 'bg-[var(--accent)] text-white shadow-sm'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        onclick={() => player.setVideoMode(true)}
      >
        <Film size={12} />
        <span>{t("player.mode_video")}</span>
      </button>
    </div>
  {/if}

  <!-- Centered large cover art or synced video -->
  <div class="flex-1 flex items-center justify-center w-full py-1">
    {#if player.videoMode && player.currentTrack}
      <SyncedVideoPlayer class="w-full aspect-video rounded-2xl" />
    {:else}
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
    {/if}
  </div>

  <!-- Lower controls stack -->
  <div class="w-full flex flex-col items-center gap-4">
    <!-- Track metadata (artist & album interactive) -->
    <div class="w-full flex flex-col items-center text-center px-1 gap-1.5">
      <h2 class="text-[19px] font-extrabold leading-tight tracking-tight line-clamp-2">
        {player.currentTrack?.title ?? t("player.nothing_playing")}
      </h2>
      {#if player.currentTrack?.artist}
        <button
          type="button"
          disabled={resolvingArtist}
          class="group inline-flex items-center justify-center gap-1.5 max-w-full px-2.5 py-0.5 rounded-full text-[14.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--accent-bright)] hover:bg-[var(--surface-hover)] transition-colors"
          title={t("ctx.go_artist")}
          onclick={async () => {
            const tr = player.currentTrack;
            if (!tr?.artist || resolvingArtist) return;
            resolvingArtist = true;
            try {
              if (typeof window !== "undefined" && window.innerWidth < 1024) {
                player.sidebarOpen = false;
              }
              await player.openArtistByNameOrId(tr.artist_id, tr.artist);
            } finally {
              resolvingArtist = false;
            }
          }}
        >
          {#if resolvingArtist}
            <span class="w-3 h-3 rounded-full border-2 border-current/30 border-t-current animate-spin shrink-0"></span>
          {/if}
          <span class="truncate">{player.currentTrack.artist}</span>
          <ChevronRight
            size={13}
            class="shrink-0 opacity-0 -ml-1 group-hover:opacity-80 group-hover:ml-0 transition-all"
          />
        </button>
      {:else}
        <p class="text-[14px] font-medium text-[var(--text-muted)] truncate">
          {t("player.select_song")}
        </p>
      {/if}
      {#if player.currentTrack?.album}
        {#if player.currentTrack.album_id}
          <button
            type="button"
            class="group inline-flex items-center justify-center gap-1.5 max-w-full px-3 py-1 rounded-full text-[12px] font-medium bg-[var(--surface)] border border-[var(--border)] text-[var(--text-muted)] hover:bg-[var(--accent-soft)] hover:border-[var(--accent-border)] hover:text-[var(--accent-bright)] transition-colors"
            title={t("player.album_prefix", { album: player.currentTrack.album })}
            onclick={() => {
              const tr = player.currentTrack;
              if (!tr?.album_id || !tr.album) return;
              if (typeof window !== "undefined" && window.innerWidth < 1024) player.sidebarOpen = false;
              player.openAlbum(tr.album_id, tr.album, tr.cover_large_url ?? tr.cover_url);
            }}
          >
            <Disc3 size={12} class="shrink-0 opacity-75 group-hover:opacity-100" />
            <span class="truncate">{player.currentTrack.album}</span>
            <ChevronRight
              size={12}
              class="shrink-0 opacity-50 group-hover:opacity-100 group-hover:translate-x-0.5 transition-all"
            />
          </button>
        {:else}
          <span class="inline-flex items-center justify-center gap-1.5 max-w-full px-2.5 py-0.5 text-[12px] text-[var(--text-muted)]/80">
            <Disc3 size={12} class="shrink-0 opacity-70" />
            <span class="truncate">{player.currentTrack.album}</span>
          </span>
        {/if}
      {/if}
    </div>

    <!-- 5-icon action row + Volume/Timer popovers -->
    <SidebarActionPopovers {sleepRemainingSecs} {onSetSleepTimer} />

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
        type="button"
        class="icon-btn relative {player.shuffle
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={t("player.shuffle")}
        onclick={() => player.toggleShuffle()}
      >
        <Shuffle size={17} />
        {#if player.shuffle}
          <span class="w-1 h-1 rounded-full bg-[var(--accent-bright)] absolute bottom-0.5"></span>
        {/if}
      </button>

      <button
        type="button"
        class="icon-btn !w-9 !h-9"
        disabled={!player.currentTrack}
        title={t("player.previous")}
        onclick={() => void player.prev()}
      >
        <SkipBack size={20} fill="currentColor" />
      </button>

      <button
        type="button"
        class="play-circle !w-[56px] !h-[56px] shrink-0"
        disabled={!player.currentTrack}
        title={t("player.play_pause")}
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
        type="button"
        class="icon-btn !w-9 !h-9"
        disabled={!player.currentTrack}
        title={t("player.next")}
        onclick={() => void player.next()}
      >
        <SkipForward size={20} fill="currentColor" />
      </button>

      <button
        type="button"
        class="icon-btn relative {player.repeat
          ? 'active'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
        title={t("player.repeat")}
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
  :global(.sidebar-seek) {
    height: 4px;
  }

  :global(.sidebar-seek::-webkit-slider-thumb) {
    width: 17px;
    height: 17px;
    background: #e5e7eb;
    border: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.45);
  }

  :global(.sidebar-seek::-moz-range-thumb) {
    width: 17px;
    height: 17px;
    background: #e5e7eb;
    border: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.45);
  }
</style>
