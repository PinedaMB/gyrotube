<script lang="ts">
  import { Heart, Pencil, Play } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { UserPlaylist } from "../../lib/types";
  import PlaylistIcon from "./PlaylistIcon.svelte";

  interface Props {
    onOpenFavorites: () => void;
    onOpenPlaylist: (id: string) => void;
    onEditPlaylist: (pl: UserPlaylist) => void;
  }

  let { onOpenFavorites, onOpenPlaylist, onEditPlaylist }: Props = $props();
</script>

<section class="mt-6 flex flex-col gap-3">
  <h2 class="section-title">{t("collection.local_playlists")}</h2>

  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
    <!-- Favorites card -->
    <div
      class="group flex items-center gap-3.5 p-3.5 rounded-2xl border border-white/10 hover:border-white/20 cursor-pointer transition-all duration-150 hover:-translate-y-0.5"
      style="background: linear-gradient(135deg, rgba(224, 27, 36, 0.2), var(--surface) 70%);"
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
        <strong class="block text-[14.5px] font-bold truncate">{t("collection.favorites")}</strong>
        <span class="block text-[12px] text-[var(--text-muted)]">
          {player.favorites.length === 1
            ? t("collection.song_one")
            : t("collection.song_many", { count: player.favorites.length })}
        </span>
      </div>
      <button
        class="w-9 h-9 rounded-full inline-flex items-center justify-center shrink-0 bg-white/10 text-white group-hover:bg-[var(--accent)] transition-all active:scale-95 disabled:opacity-35"
        disabled={player.favorites.length === 0}
        title="Play favorites"
        onclick={(e) => {
          e.stopPropagation();
          if (player.favorites.length) player.replaceQueueAndPlay(player.favorites, 0, "Favorites");
        }}
      >
        <Play size={15} fill="currentColor" class="translate-x-[1px]" />
      </button>
    </div>

    <!-- User playlists -->
    {#each player.userPlaylists as pl (pl.id)}
      {@const accent = pl.color_hex || "#10b981"}
      <div
        class="group flex items-center gap-3.5 p-3.5 rounded-2xl border border-[var(--border)] hover:border-white/20 cursor-pointer transition-all duration-150 hover:-translate-y-0.5"
        style="background: linear-gradient(135deg, {accent}26, var(--surface) 72%);"
        role="button"
        tabindex="0"
        onclick={() => onOpenPlaylist(pl.id)}
        onkeydown={(e) => e.key === "Enter" && onOpenPlaylist(pl.id)}
      >
        <span
          class="w-12 h-12 rounded-xl inline-flex items-center justify-center text-white shrink-0 shadow-md"
          style="background: {accent};"
        >
          <PlaylistIcon name={pl.icon_name} size={20} />
        </span>
        <div class="min-w-0 flex-1">
          <strong class="block text-[14.5px] font-bold truncate">{pl.name}</strong>
          <span class="block text-[12px] text-[var(--text-muted)] truncate">
            {pl.tracks.length} {pl.tracks.length === 1 ? "song" : "songs"}{pl.description ? ` · ${pl.description}` : ""}
          </span>
        </div>
        <div class="flex items-center gap-1 shrink-0">
          <button
            class="icon-btn !w-8 !h-8 opacity-0 group-hover:opacity-100 transition-opacity"
            title="Customize playlist"
            onclick={(e) => {
              e.stopPropagation();
              onEditPlaylist(pl);
            }}
          >
            <Pencil size={14} />
          </button>
          <button
            class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-white/10 text-white group-hover:bg-[var(--accent)] transition-all active:scale-95 disabled:opacity-35"
            disabled={pl.tracks.length === 0}
            title="Play playlist"
            onclick={(e) => {
              e.stopPropagation();
              if (pl.tracks.length) player.replaceQueueAndPlay(pl.tracks, 0, pl.name);
            }}
          >
            <Play size={15} fill="currentColor" class="translate-x-[1px]" />
          </button>
        </div>
      </div>
    {/each}
  </div>
</section>
