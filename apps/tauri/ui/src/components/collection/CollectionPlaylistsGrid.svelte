<script lang="ts">
  import { CheckCircle2, Heart, Pencil, Pin, Play } from "lucide-svelte";
  import { extractDominantRgb } from "../../lib/audioEngine";
  import { rgbToHex } from "../../lib/colorUtils";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track, UserPlaylist } from "../../lib/types";
  import PlaylistIcon from "./PlaylistIcon.svelte";

  interface Props {
    onOpenFavorites: () => void;
    onOpenPlaylist: (id: string) => void;
    onEditPlaylist: (pl: UserPlaylist) => void;
  }

  let { onOpenFavorites, onOpenPlaylist, onEditPlaylist }: Props = $props();

  let coverColors = $state<Record<string, string>>({});
  let extractedFor = new Map<string, string>();

  $effect(() => {
    for (const pl of player.userPlaylists) {
      const cover = pl.cover_path;
      if (!cover) {
        if (extractedFor.has(pl.id)) {
          extractedFor.delete(pl.id);
          const next = { ...coverColors };
          delete next[pl.id];
          coverColors = next;
        }
        continue;
      }
      if (extractedFor.get(pl.id) === cover) continue;
      extractedFor.set(pl.id, cover);
      const pid = pl.id;
      void extractDominantRgb(cover).then((rgb) => {
        if (extractedFor.get(pid) === cover && rgb) {
          coverColors = { ...coverColors, [pid]: rgbToHex(rgb) };
        }
      });
    }
  });

  function allDownloaded(tracks: Track[]): boolean {
    return tracks.length > 0 && tracks.every((tr) => player.isTrackDownloaded(tr.id));
  }

  function playTracks(tracks: Track[], source: string) {
    const list = player.isOffline
      ? tracks.filter((tr) => player.isTrackDownloaded(tr.id))
      : tracks;
    if (!list.length) {
      if (player.isOffline) player.showToast(t("offline.not_downloaded"));
      return;
    }
    player.replaceQueueAndPlay(list, 0, source);
  }
</script>

<section class="mt-6 flex flex-col gap-3">
  <h2 class="section-title">{t("collection.local_playlists")}</h2>

  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
    <!-- Favorites card -->
    <div
      class="group flex items-center gap-3.5 p-3.5 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] cursor-pointer transition-all duration-150 hover:-translate-y-0.5"
      style="background: linear-gradient(135deg, rgba(224, 27, 36, 0.16), var(--card-bg) 70%); box-shadow: var(--card-shadow);"
      role="button"
      tabindex="0"
      onclick={onOpenFavorites}
      onkeydown={(e) => e.key === "Enter" && onOpenFavorites()}
    >
      <span
        class="w-12 h-12 rounded-xl inline-flex items-center justify-center text-white shrink-0 shadow-md"
        style="background: linear-gradient(135deg, #e01b24, #ff6b81);"
      >
        <Heart size={20} fill="currentColor" />
      </span>
      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-1.5">
          <strong class="text-[14.5px] font-bold truncate">{t("collection.favorites")}</strong>
          {#if allDownloaded(player.favorites)}
            <span title={t("offline.downloaded")} class="text-[var(--accent-bright)] shrink-0">
              <CheckCircle2 size={13} />
            </span>
          {/if}
        </div>
        <span class="block text-[12px] text-[var(--text-muted)]">
          {player.favorites.length === 1
            ? t("collection.song_one")
            : t("collection.song_many", { count: player.favorites.length })}
        </span>
      </div>
      <button
        class="w-9 h-9 rounded-full inline-flex items-center justify-center shrink-0 bg-[var(--surface-hover)] text-[var(--text)] group-hover:bg-[var(--accent)] group-hover:text-white transition-all active:scale-95 disabled:opacity-35"
        disabled={player.favorites.length === 0}
        title={t("home.play")}
        onclick={(e) => {
          e.stopPropagation();
          playTracks(player.favorites, t("collection.favorites"));
        }}
      >
        <Play size={15} fill="currentColor" class="translate-x-[1px]" />
      </button>
    </div>

    <!-- User playlists (pinned first) -->
    {#each player.sortedUserPlaylists as pl (pl.id)}
      {@const accent = pl.color_hex || (pl.cover_path ? coverColors[pl.id] : null) || "#10b981"}
      {@const pinned = player.isPlaylistPinned(pl.id)}
      <div
        class="group flex items-center gap-3.5 p-3.5 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] cursor-pointer transition-all duration-150 hover:-translate-y-0.5"
        style="background: linear-gradient(135deg, {accent}22, var(--card-bg) 72%); box-shadow: var(--card-shadow);"
        role="button"
        tabindex="0"
        onclick={() => onOpenPlaylist(pl.id)}
        onkeydown={(e) => e.key === "Enter" && onOpenPlaylist(pl.id)}
      >
        <span
          class="w-12 h-12 rounded-xl inline-flex items-center justify-center text-white shrink-0 shadow-md overflow-hidden"
          style="background: {accent};"
        >
          {#if pl.cover_path}
            <img src={pl.cover_path} alt={pl.name} class="w-full h-full object-cover" />
          {:else}
            <PlaylistIcon name={pl.icon_name} size={20} />
          {/if}
        </span>
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-1.5">
            {#if pinned}
              <Pin size={12} class="text-[var(--accent-bright)] shrink-0 rotate-45" />
            {/if}
            <strong class="text-[14.5px] font-bold truncate">{pl.name}</strong>
            {#if allDownloaded(pl.tracks)}
              <span title={t("offline.downloaded")} class="text-[var(--accent-bright)] shrink-0">
                <CheckCircle2 size={13} />
              </span>
            {/if}
          </div>
          <span class="block text-[12px] text-[var(--text-muted)] truncate">
            {pl.tracks.length === 1
              ? t("collection.song_one")
              : t("collection.song_many", { count: pl.tracks.length })}{pl.description ? ` · ${pl.description}` : ""}
          </span>
        </div>
        <div class="flex items-center gap-1 shrink-0">
          <button
            class="icon-btn !w-8 !h-8 transition-opacity {pinned
              ? 'text-[var(--accent-bright)] opacity-100'
              : 'text-[var(--text-muted)] opacity-85 [@media(hover:hover)]:opacity-0 [@media(hover:hover)]:group-hover:opacity-100 [@media(hover:hover)]:focus-visible:opacity-100'}"
            title={pinned ? t("collection.unpin") : t("collection.pin")}
            onclick={(e) => {
              e.stopPropagation();
              player.togglePinPlaylist(pl.id);
            }}
          >
            <Pin size={13} class={pinned ? "fill-current" : ""} />
          </button>
          <button
            class="icon-btn !w-8 !h-8 text-[var(--text-muted)] opacity-85 [@media(hover:hover)]:opacity-0 [@media(hover:hover)]:group-hover:opacity-100 [@media(hover:hover)]:focus-visible:opacity-100 transition-opacity"
            title={t("collection.modal_edit")}
            onclick={(e) => {
              e.stopPropagation();
              onEditPlaylist(pl);
            }}
          >
            <Pencil size={14} />
          </button>
          <button
            class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-[var(--surface-hover)] text-[var(--text)] group-hover:bg-[var(--accent)] group-hover:text-white transition-all active:scale-95 disabled:opacity-35"
            disabled={pl.tracks.length === 0}
            title={t("home.play")}
            onclick={(e) => {
              e.stopPropagation();
              playTracks(pl.tracks, pl.name);
            }}
          >
            <Play size={15} fill="currentColor" class="translate-x-[1px]" />
          </button>
        </div>
      </div>
    {/each}
  </div>
</section>
