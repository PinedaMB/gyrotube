<script lang="ts">
  import {
    Check,
    ChevronDown,
    Compass,
    Disc3,
    Eye,
    EyeOff,
    Heart,
    Home,
    Library,
    Link2,
    ListPlus,
    ListStart,
    PartyPopper,
    Pause,
    Play,
    Plus,
    Radio,
    Settings,
    SkipForward,
    Trash2,
    User,
  } from "lucide-svelte";
  import { slide } from "svelte/transition";
  import { api, isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { contextMenu, type ContextPayload } from "../lib/stores/contextMenu.svelte";
  import { device } from "../lib/stores/device.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { Track } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";
  import PlaylistIcon from "./collection/PlaylistIcon.svelte";
  import SpinIcon from "./icons/SpinIcon.svelte";

  let { payload }: { payload: ContextPayload } = $props();
  let showPlaylists = $state(false);

  async function startTrackRadio(track: Track) {
    contextMenu.close();
    if (!isTauriRuntime()) return;
    try {
      const radio = await api.getRadioTracks(track.id);
      const list = [track, ...radio.filter((r) => r.id !== track.id)];
      player.replaceQueueAndPlay(list, 0, `Radio · ${track.title}`);
    } catch (err) {
      player.showToast(String(err));
    }
  }

  async function copyTrackLink(track: Track) {
    contextMenu.close();
    try {
      await navigator.clipboard.writeText(`https://music.youtube.com/watch?v=${track.id}`);
      player.showToast(t("ctx.link_copied"));
    } catch {
      player.showToast(t("spin.copy_failed"));
    }
  }
</script>

{#if payload.kind === "track"}
  {@const track = payload.track}
  {@const fav = player.isFavorite(track.id)}

  <!-- Track header -->
  <div class="flex items-center gap-3 px-2.5 py-2 mb-2 border-b border-[var(--border)]">
    <CoverImage
      class={device.isMobile
        ? "track-cover !w-11 !h-11 !rounded-[10px] shrink-0"
        : "track-cover !w-9 !h-9 !rounded-[7px] shrink-0"}
      src={player.trackCoverUrl(track, false)}
      fallbackSrc={player.offlineCoverUrl(track.id)}
      alt=""
      eager={true}
    />
    <div class="min-w-0 flex-1">
      <div class="{device.isMobile ? 'text-[14.5px]' : 'text-[12.5px]'} font-bold truncate leading-tight">
        {track.title}
      </div>
      <div class="{device.isMobile ? 'text-[12px]' : 'text-[11px]'} text-[var(--text-muted)] truncate mt-0.5">
        {track.artist}{track.album ? ` · ${track.album}` : ""}
      </div>
    </div>
    {#if device.isMobile}
      <button
        type="button"
        class="icon-btn !w-9 !h-9 shrink-0 rounded-full {fav
          ? 'text-[var(--fav-coral)] bg-[var(--fav-coral)]/12'
          : 'text-[var(--text-muted)] bg-[var(--surface)]'}"
        title={fav ? t("player.favorite_remove") : t("player.favorite_add")}
        aria-label={fav ? t("player.favorite_remove") : t("player.favorite_add")}
        onclick={() => {
          player.toggleFavorite(track);
        }}
      >
        <Heart size={17} fill={fav ? "currentColor" : "none"} />
      </button>
    {/if}
  </div>

  {#if device.isMobile}
    <!-- Mobile quick-action tile row (thumb-friendly, reduces vertical height) -->
    <div class="grid grid-cols-4 gap-2 mb-2.5">
      <button
        type="button"
        role="menuitem"
        class="flex flex-col items-center justify-center gap-1.5 py-2.5 px-1 rounded-2xl bg-[var(--accent-soft)] border border-[var(--accent-border)] text-[var(--accent-bright)] active:scale-95 transition-transform"
        onclick={() => {
          contextMenu.close();
          if (payload.onPlay) payload.onPlay();
          else player.replaceQueueAndPlay([track], 0, track.title);
        }}
      >
        <Play size={17} fill="currentColor" class="translate-x-[1px]" />
        <span class="text-[11px] font-semibold truncate max-w-full">{t("home.play")}</span>
      </button>
      <button
        type="button"
        role="menuitem"
        class="flex flex-col items-center justify-center gap-1.5 py-2.5 px-1 rounded-2xl bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-[var(--text)] active:scale-95 transition-transform"
        onclick={() => {
          contextMenu.close();
          player.playNext(track);
        }}
      >
        <ListStart size={17} />
        <span class="text-[11px] font-semibold truncate max-w-full">{t("party.up_next")}</span>
      </button>
      <button
        type="button"
        role="menuitem"
        class="flex flex-col items-center justify-center gap-1.5 py-2.5 px-1 rounded-2xl bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-[var(--text)] active:scale-95 transition-transform"
        onclick={() => {
          contextMenu.close();
          player.enqueueTrack(track);
        }}
      >
        <ListPlus size={17} />
        <span class="text-[11px] font-semibold truncate max-w-full">{t("player.tab_queue")}</span>
      </button>
      <button
        type="button"
        role="menuitem"
        class="flex flex-col items-center justify-center gap-1.5 py-2.5 px-1 rounded-2xl bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-[var(--text)] active:scale-95 transition-transform"
        onclick={() => void startTrackRadio(track)}
      >
        <Radio size={17} />
        <span class="text-[11px] font-semibold truncate max-w-full">Radio</span>
      </button>
    </div>
  {:else}
    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        if (payload.onPlay) payload.onPlay();
        else player.replaceQueueAndPlay([track], 0, track.title);
      }}
    >
      <Play size={15} fill="currentColor" />
      <span>{t("home.play")}</span>
    </button>

    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        player.playNext(track);
      }}
    >
      <ListStart size={15} />
      <span>{t("queue.play_next")}</span>
    </button>

    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        player.enqueueTrack(track);
      }}
    >
      <ListPlus size={15} />
      <span>{t("queue.add")}</span>
    </button>

    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => void startTrackRadio(track)}
    >
      <Radio size={15} />
      <span>{t("ctx.start_radio")}</span>
    </button>

    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        player.toggleFavorite(track);
      }}
    >
      <Heart
        size={15}
        class={fav ? "text-[var(--fav-coral)]" : ""}
        fill={fav ? "currentColor" : "none"}
      />
      <span>{fav ? t("player.favorite_remove") : t("player.favorite_add")}</span>
    </button>
  {/if}

  <button
    type="button"
    role="menuitem"
    aria-expanded={showPlaylists}
    class="ctx-item justify-between"
    onclick={() => (showPlaylists = !showPlaylists)}
  >
    <span class="inline-flex items-center gap-2.5 min-w-0">
      <Plus size={15} class="shrink-0" />
      <span class="truncate">{t("collection.add_to_playlist").replace(/:$/, "")}</span>
    </span>
    <ChevronDown
      size={14}
      class="shrink-0 text-[var(--text-muted)] transition-transform duration-150 {showPlaylists ? 'rotate-180' : ''}"
    />
  </button>

  {#if showPlaylists}
    <div
      transition:slide={{ duration: 150 }}
      class="mx-1 my-1.5 p-1.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] space-y-0.5 max-h-40 overflow-y-auto scroll-y"
    >
      {#each player.userPlaylists as pl (pl.id)}
        {@const alreadyIn = pl.tracks.some((tr) => tr.id === track.id)}
        <button
          type="button"
          role="menuitem"
          class="ctx-item !py-2 !text-[13px] justify-between"
          onclick={() => {
            contextMenu.close();
            player.addTrackToPlaylist(pl.id, track);
          }}
        >
          <span class="inline-flex items-center gap-2 min-w-0">
            <span
              class="w-6 h-6 rounded-md inline-flex items-center justify-center text-white shrink-0"
              style="background: {pl.color_hex || '#10b981'};"
            >
              <PlaylistIcon name={pl.icon_name} size={12} />
            </span>
            <span class="truncate">{pl.name}</span>
          </span>
          {#if alreadyIn}
            <Check size={14} class="text-[var(--accent-bright)] shrink-0" />
          {/if}
        </button>
      {/each}
      <button
        type="button"
        role="menuitem"
        class="ctx-item !py-2 !text-[13px] text-[var(--accent-bright)]"
        onclick={() => {
          contextMenu.close();
          player.playlistDraftTrack = track;
        }}
      >
        <span class="w-6 h-6 rounded-md inline-flex items-center justify-center bg-[var(--accent-soft)] shrink-0">
          <Plus size={13} />
        </span>
        <span>{t("collection.new_playlist")}</span>
      </button>
    </div>
  {/if}

  {#if track.artist || track.album_id}
    <div class="my-1 border-t border-[var(--border)]"></div>
  {/if}

  {#if track.artist}
    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        void player.openArtistByNameOrId(track.artist_id, track.artist);
      }}
    >
      <User size={15} />
      <span>{t("ctx.go_artist")}</span>
    </button>
  {/if}

  {#if track.album_id && track.album}
    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        if (track.album_id && track.album) {
          player.openAlbum(track.album_id, track.album, track.cover_large_url ?? track.cover_url);
        }
      }}
    >
      <Disc3 size={15} />
      <span>{t("ctx.go_album")}</span>
    </button>
  {/if}

  <div class="my-1 border-t border-[var(--border)]"></div>

  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      player.toggleHideTrack(track);
    }}
  >
    {#if player.isTrackHidden(track.id)}
      <Eye size={15} />
      <span>{t("ctx.unhide_track")}</span>
    {:else}
      <EyeOff size={15} />
      <span>{t("ctx.hide_track")}</span>
    {/if}
  </button>

  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => void copyTrackLink(track)}
  >
    <Link2 size={15} />
    <span>{t("ctx.copy_link")}</span>
  </button>

  {#if payload.onRemove}
    <button
      type="button"
      role="menuitem"
      class="ctx-item text-red-400 hover:!bg-red-500/15"
      onclick={() => {
        contextMenu.close();
        payload.onRemove?.();
      }}
    >
      <Trash2 size={15} />
      <span>{payload.removeLabel ?? t("queue.remove")}</span>
    </button>
  {/if}
{:else}
  <!-- General app context menu -->
  {#if player.currentTrack}
    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        void player.togglePlay();
      }}
    >
      {#if player.isPlaying}
        <Pause size={15} fill="currentColor" />
        <span>{t("player.paused")}</span>
      {:else}
        <Play size={15} fill="currentColor" />
        <span>{t("home.play")}</span>
      {/if}
    </button>
    <button
      type="button"
      role="menuitem"
      class="ctx-item"
      onclick={() => {
        contextMenu.close();
        void player.next();
      }}
    >
      <SkipForward size={15} />
      <span>{t("party.up_next")}</span>
    </button>
    <div class="my-1 border-t border-[var(--border)]"></div>
  {/if}

  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      player.navigateTab("home");
    }}
  >
    <Home size={15} />
    <span>{t("nav.home")}</span>
  </button>
  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      player.navigateTab("explore");
    }}
  >
    <Compass size={15} />
    <span>{t("nav.explore")}</span>
  </button>
  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      player.navigateTab("collection");
    }}
  >
    <Library size={15} />
    <span>{t("nav.collection")}</span>
  </button>

  <div class="my-1 border-t border-[var(--border)]"></div>

  <button
    type="button"
    role="menuitem"
    disabled={player.videoMode}
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      player.togglePartyMode();
    }}
  >
    <PartyPopper size={15} />
    <span>{t("header.party_mode")}</span>
  </button>
  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      player.openSpin();
      void player.refreshSpinStatus();
    }}
  >
    <SpinIcon size={15} />
    <span>{t("header.spin_mode")}</span>
  </button>
  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      player.openSettings();
    }}
  >
    <Settings size={15} />
    <span>{t("header.settings")}</span>
  </button>
{/if}

<style>
  .ctx-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 11px;
    text-align: left;
    font-weight: 500;
    color: var(--text);
    transition: background-color 110ms ease;
  }

  @media (max-width: 767px) {
    .ctx-item {
      padding: 12px 12px;
      gap: 12px;
    }
  }

  .ctx-item:hover {
    background: var(--border);
  }
</style>
