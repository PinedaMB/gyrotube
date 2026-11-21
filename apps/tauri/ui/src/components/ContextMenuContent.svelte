<script lang="ts">
  import {
    Compass,
    Disc3,
    Heart,
    Home,
    Library,
    Link2,
    ListPlus,
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
  import { api, isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { contextMenu, type ContextPayload } from "../lib/stores/contextMenu.svelte";
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
  <div class="flex items-center gap-2.5 px-2.5 py-2 mb-1 border-b border-white/[0.08]">
    <CoverImage class="track-cover !w-9 !h-9 !rounded-[7px] shrink-0" src={track.cover_url} alt="" eager={true} />
    <div class="min-w-0 flex-1">
      <div class="text-[12.5px] font-bold truncate leading-tight">{track.title}</div>
      <div class="text-[11px] text-[var(--text-muted)] truncate mt-0.5">{track.artist}</div>
    </div>
  </div>

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

  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => (showPlaylists = !showPlaylists)}
  >
    <Plus size={15} />
    <span>{t("collection.add_to_playlist").replace(/:$/, "")}</span>
  </button>

  {#if showPlaylists}
    <div class="mx-1.5 my-1 p-1 rounded-xl bg-black/25 border border-white/[0.06] space-y-0.5 max-h-36 overflow-y-auto scroll-y">
      {#each player.userPlaylists as pl (pl.id)}
        <button
          type="button"
          role="menuitem"
          class="ctx-item !py-1.5 !text-[12px]"
          onclick={() => {
            contextMenu.close();
            player.addTrackToPlaylist(pl.id, track);
          }}
        >
          <PlaylistIcon name={pl.icon_name} size={13} />
          <span class="truncate">{pl.name}</span>
        </button>
      {/each}
      <button
        type="button"
        role="menuitem"
        class="ctx-item !py-1.5 !text-[12px] text-[var(--accent-bright)]"
        onclick={() => {
          contextMenu.close();
          player.playlistDraftTrack = track;
        }}
      >
        <Plus size={13} />
        <span>{t("collection.new_playlist")}</span>
      </button>
    </div>
  {/if}

  {#if track.artist || track.album_id}
    <div class="my-1 border-t border-white/[0.08]"></div>
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

  <div class="my-1 border-t border-white/[0.08]"></div>

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
    <div class="my-1 border-t border-white/[0.08]"></div>
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

  <div class="my-1 border-t border-white/[0.08]"></div>

  <button
    type="button"
    role="menuitem"
    disabled={player.videoMode}
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      if (!player.videoMode) player.partyMode = !player.partyMode;
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
      player.spinOpen = true;
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
      player.settingsOpen = true;
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
