<script lang="ts">
  import { CheckCircle2, Heart, Play } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track, UserPlaylist } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";
  import PlaylistIcon from "../collection/PlaylistIcon.svelte";

  type ShortcutItem =
    | { kind: "favorites"; tracks: Track[] }
    | { kind: "playlist"; playlist: UserPlaylist }
    | { kind: "track"; track: Track };

  let items = $derived.by<ShortcutItem[]>(() => {
    const out: ShortcutItem[] = [];
    if (player.favorites.length > 0) {
      out.push({ kind: "favorites", tracks: player.favorites });
    }
    for (const pl of player.sortedUserPlaylists) {
      if (out.length >= 6) break;
      out.push({ kind: "playlist", playlist: pl });
    }
    const seenTrackIds = new Set<string>();
    for (const tr of player.recentTracks) {
      if (out.length >= 6) break;
      if (seenTrackIds.has(tr.id)) continue;
      seenTrackIds.add(tr.id);
      out.push({ kind: "track", track: tr });
    }
    return out;
  });

  function isAllDownloaded(tracks: Track[]): boolean {
    return tracks.length > 0 && tracks.every((tr) => player.isTrackDownloaded(tr.id));
  }
</script>

{#if items.length > 0}
  <section class="grid grid-cols-2 lg:grid-cols-3 gap-2.5 mb-6 max-md:mb-5">
    {#each items as item, idx (item.kind === "favorites" ? "fav" : item.kind === "playlist" ? item.playlist.id : `tr:${item.track.id}:${idx}`)}
      {#if item.kind === "favorites"}
        <div
          role="button"
          tabindex="0"
          class="group flex items-center gap-2.5 sm:gap-3 pr-2.5 rounded-xl overflow-hidden border border-[var(--border)] hover:border-[var(--border-strong)] bg-[var(--card-bg)] cursor-pointer transition-all duration-150 hover:-translate-y-0.5"
          style="box-shadow: var(--card-shadow);"
          onclick={() => player.openLocalPlaylist("favorites")}
          onkeydown={(e) => e.key === "Enter" && player.openLocalPlaylist("favorites")}
        >
          <span
            class="w-13 h-13 sm:w-14 sm:h-14 shrink-0 inline-flex items-center justify-center text-white"
            style="background: linear-gradient(135deg, #e01b24, #ff6b81);"
          >
            <Heart size={20} fill="currentColor" />
          </span>
          <div class="min-w-0 flex-1 py-1.5">
            <div class="flex items-center gap-1">
              <strong class="text-[13px] sm:text-[13.5px] font-bold truncate">{t("collection.favorites")}</strong>
              {#if isAllDownloaded(item.tracks)}
                <CheckCircle2 size={12} class="text-[var(--accent-bright)] shrink-0" />
              {/if}
            </div>
            <span class="block text-[11.5px] text-[var(--text-muted)] truncate">
              {item.tracks.length === 1
                ? t("collection.song_one")
                : t("collection.song_many", { count: item.tracks.length })}
            </span>
          </div>
          <button
            type="button"
            class="w-7 h-7 sm:w-8 sm:h-8 rounded-full inline-flex items-center justify-center shrink-0 bg-[var(--accent)] text-white opacity-100 sm:opacity-0 sm:group-hover:opacity-100 focus-visible:opacity-100 shadow-md transition-all active:scale-95"
            title={t("home.play")}
            onclick={(e) => {
              e.stopPropagation();
              player.replaceQueueAndPlay(item.tracks, 0, t("collection.favorites"));
            }}
          >
            <Play size={13} fill="currentColor" class="translate-x-[1px]" />
          </button>
        </div>
      {:else if item.kind === "playlist"}
        {@const pl = item.playlist}
        {@const accent = pl.color_hex || "#10b981"}
        <div
          role="button"
          tabindex="0"
          class="group flex items-center gap-2.5 sm:gap-3 pr-2.5 rounded-xl overflow-hidden border border-[var(--border)] hover:border-[var(--border-strong)] bg-[var(--card-bg)] cursor-pointer transition-all duration-150 hover:-translate-y-0.5"
          style="box-shadow: var(--card-shadow);"
          onclick={() => player.openLocalPlaylist(pl.id)}
          onkeydown={(e) => e.key === "Enter" && player.openLocalPlaylist(pl.id)}
        >
          <span
            class="w-13 h-13 sm:w-14 sm:h-14 shrink-0 inline-flex items-center justify-center text-white overflow-hidden"
            style="background: {accent};"
          >
            {#if pl.cover_path}
              <img src={pl.cover_path} alt={pl.name} class="w-full h-full object-cover" />
            {:else}
              <PlaylistIcon name={pl.icon_name} size={20} />
            {/if}
          </span>
          <div class="min-w-0 flex-1 py-1.5">
            <div class="flex items-center gap-1">
              <strong class="text-[13px] sm:text-[13.5px] font-bold truncate">{pl.name}</strong>
              {#if isAllDownloaded(pl.tracks)}
                <CheckCircle2 size={12} class="text-[var(--accent-bright)] shrink-0" />
              {/if}
            </div>
            <span class="block text-[11.5px] text-[var(--text-muted)] truncate">
              {pl.tracks.length === 1
                ? t("collection.song_one")
                : t("collection.song_many", { count: pl.tracks.length })}
            </span>
          </div>
          {#if pl.tracks.length > 0}
            <button
              type="button"
              class="w-7 h-7 sm:w-8 sm:h-8 rounded-full inline-flex items-center justify-center shrink-0 bg-[var(--accent)] text-white opacity-100 sm:opacity-0 sm:group-hover:opacity-100 focus-visible:opacity-100 shadow-md transition-all active:scale-95"
              title={t("home.play")}
              onclick={(e) => {
                e.stopPropagation();
                player.replaceQueueAndPlay(pl.tracks, 0, pl.name);
              }}
            >
              <Play size={13} fill="currentColor" class="translate-x-[1px]" />
            </button>
          {/if}
        </div>
      {:else}
        {@const tr = item.track}
        <button
          type="button"
          class="group flex items-center gap-2.5 sm:gap-3 pr-2.5 rounded-xl overflow-hidden border border-[var(--border)] hover:border-[var(--border-strong)] bg-[var(--card-bg)] text-left transition-all duration-150 hover:-translate-y-0.5"
          style="box-shadow: var(--card-shadow);"
          onclick={() => player.replaceQueueAndPlay([tr], 0, tr.title)}
        >
          <CoverImage
            class="w-13 h-13 sm:w-14 sm:h-14 shrink-0 !rounded-none"
            src={player.trackCoverUrl(tr, false)}
            fallbackSrc={player.offlineCoverUrl(tr.id)}
            alt={tr.title}
          />
          <div class="min-w-0 flex-1 py-1.5">
            <strong class="block text-[13px] sm:text-[13.5px] font-bold truncate">{tr.title}</strong>
            <span class="block text-[11.5px] text-[var(--text-muted)] truncate">{tr.artist}</span>
          </div>
          <span
            class="hidden sm:inline-flex w-8 h-8 rounded-full items-center justify-center shrink-0 bg-[var(--accent)] text-white opacity-0 group-hover:opacity-100 shadow-md transition-all"
            aria-hidden="true"
          >
            <Play size={13} fill="currentColor" class="translate-x-[1px]" />
          </span>
        </button>
      {/if}
    {/each}
  </section>
{/if}
