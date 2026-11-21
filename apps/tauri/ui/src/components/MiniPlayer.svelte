<script lang="ts">
  import { ChevronUp, Heart, Pause, Play, SkipBack, SkipForward } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { t } from "../lib/i18n.svelte";
  import { contextMenu } from "../lib/stores/contextMenu.svelte";
  import { player } from "../lib/stores/player.svelte";
  import CoverImage from "./CoverImage.svelte";

  let progressRatio = $derived(
    player.durationSecs > 0
      ? Math.max(0, Math.min(1, player.positionSecs / player.durationSecs))
      : 0,
  );
</script>

{#if !player.sidebarOpen && player.currentTrack}
  {@const track = player.currentTrack}
  {@const favorited = player.isFavorite(track.id)}

  <div
    transition:fly={{ y: 20, duration: 200, easing: cubicOut }}
    role="region"
    aria-label={t("player.tab_player")}
    oncontextmenu={(e) => contextMenu.openTrackMenu(e, track)}
    class="relative z-30 shrink-0 w-full overflow-hidden bg-[var(--bg-elevated)] border-t border-white/[0.08] md:fixed md:bottom-4 md:left-1/2 md:-translate-x-1/2 md:w-[min(560px,calc(100vw-32px))] md:rounded-2xl md:border md:border-white/[0.12] md:shadow-[0_18px_48px_rgba(0,0,0,0.58)]"
    style="background-image: linear-gradient(95deg, rgba(var(--ambient-rgb), 0.18) 0%, rgba(var(--ambient-rgb), 0.05) 52%, transparent 100%);"
  >
    <!-- Live thin progress bar (top on mobile, bottom on desktop; GPU scaleX) -->
    <div
      class="pointer-events-none absolute left-0 right-0 top-0 md:top-auto md:bottom-0 h-[2.5px] bg-[var(--control-track)]"
      aria-hidden="true"
    >
      <div
        class="h-full w-full bg-[var(--accent-bright)] origin-left transition-transform duration-200 ease-linear"
        style="transform: scaleX({progressRatio});"
      ></div>
    </div>

    <div class="flex items-center gap-2 sm:gap-3 px-3 sm:px-3.5 py-2.5">
      <!-- Track info trigger -> expands full player -->
      <button
        type="button"
        class="group flex-1 min-w-0 flex items-center gap-3 text-left rounded-xl -my-1 py-1 pr-1 hover:bg-white/[0.04] transition-colors"
        onclick={() => player.openPlayerSheet()}
      >
        <div class="relative shrink-0">
          <CoverImage
            class="track-cover !w-[42px] !h-[42px] !rounded-[9px] border border-white/10 shadow-sm"
            src={track.cover_url}
            alt=""
            eager={true}
          />
          <span
            class="absolute inset-0 rounded-[9px] bg-black/45 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white"
            aria-hidden="true"
          >
            <ChevronUp size={16} />
          </span>
        </div>

        <div class="min-w-0 flex-1">
          <div class="text-[13.5px] font-bold truncate leading-tight">
            {track.title}
          </div>
          <div class="text-[12px] text-[var(--text-muted)] truncate mt-0.5">
            {track.artist}
          </div>
        </div>
      </button>

      <!-- Compact playback controls -->
      <div class="flex items-center gap-1 shrink-0">
        <button
          type="button"
          class="icon-btn !w-9 !h-9"
          class:fav-active={favorited}
          title={favorited ? t("player.favorite_remove") : t("player.favorite_add")}
          onclick={() => player.toggleFavorite(track)}
        >
          <Heart size={17} fill={favorited ? "currentColor" : "none"} />
        </button>

        <button
          type="button"
          class="icon-btn !w-9 !h-9 hidden sm:inline-flex"
          title={t("player.previous")}
          onclick={() => void player.prev()}
        >
          <SkipBack size={17} fill="currentColor" />
        </button>

        <button
          type="button"
          class="play-circle !w-10 !h-10 shadow-[0_4px_14px_rgba(16,185,129,0.42)]"
          title={t("player.play_pause")}
          onclick={() => void player.togglePlay()}
        >
          {#if player.isPlaying}
            <Pause size={17} fill="currentColor" />
          {:else}
            <Play size={17} fill="currentColor" class="translate-x-[1px]" />
          {/if}
        </button>

        <button
          type="button"
          class="icon-btn !w-9 !h-9"
          title={t("player.next")}
          onclick={() => void player.next()}
        >
          <SkipForward size={17} fill="currentColor" />
        </button>
      </div>
    </div>
  </div>
{/if}
