<script lang="ts">
  import { BarChart3, Clock, Play, RotateCcw, Sparkles } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { resolveLocale, t } from "../../lib/i18n.svelte";
  import {
    formatListeningTime,
    topPlayedArtists,
    topPlayedTracks,
    type ArtistPlayStat,
  } from "../../lib/listeningStats";
  import { player } from "../../lib/stores/player.svelte";
  import CoverImage from "../CoverImage.svelte";

  let isSpanish = $derived(resolveLocale() === "es");
  let topTracks = $derived(topPlayedTracks(player.listeningStats, 5));
  let topArtists = $derived(topPlayedArtists(player.listeningStats, 5));
  let hasStats = $derived(player.listeningStats.totalPlays > 0);
  let expanded = $state(false);
  let resolvedArtistAvatars = $state<Record<string, string>>({});
  const fetchedArtistKeys = new Set<string>();

  function artistFallbackCover(artist: ArtistPlayStat): string | null {
    const lower = artist.name.trim().toLowerCase();
    const followed = player.followedArtists.find(
      (a) => (artist.id && a.id === artist.id) || a.name.trim().toLowerCase() === lower,
    );
    if (followed?.avatar_url) return followed.avatar_url;
    if (resolvedArtistAvatars[lower]) return resolvedArtistAvatars[lower];
    if (artist.avatarUrl) return artist.avatarUrl;
    const matchTrack = Object.values(player.listeningStats.tracks).find((entry) =>
      entry.track.artist.toLowerCase().includes(lower),
    );
    return matchTrack ? player.trackCoverUrl(matchTrack.track) : null;
  }

  $effect(() => {
    if (!expanded || player.isOffline || !isTauriRuntime()) return;
    for (const artist of topArtists) {
      const key = artist.name.trim().toLowerCase();
      if (fetchedArtistKeys.has(key)) continue;
      // Skip network if we already have a real followed avatar
      const followed = player.followedArtists.find(
        (a) => (artist.id && a.id === artist.id) || a.name.trim().toLowerCase() === key,
      );
      if (followed?.avatar_url) {
        fetchedArtistKeys.add(key);
        continue;
      }
      fetchedArtistKeys.add(key);
      void (async () => {
        try {
          if (artist.id) {
            const details = await api.getArtist(artist.id, artist.name);
            if (details.header_url) {
              resolvedArtistAvatars = { ...resolvedArtistAvatars, [key]: details.header_url };
              player.updateArtistStatAvatar(artist.name, artist.id, details.header_url);
              return;
            }
          }
          const res = await api.searchAll(artist.name);
          const match =
            res.artists.find((a) => a.name.trim().toLowerCase() === key) ?? res.artists[0];
          if (match?.avatar_url) {
            resolvedArtistAvatars = { ...resolvedArtistAvatars, [key]: match.avatar_url };
            player.updateArtistStatAvatar(artist.name, match.id, match.avatar_url);
          }
        } catch {
          // Non-fatal; fallback cover remains visible
        }
      })();
    }
  });
</script>

{#if hasStats}
  <section class="mt-7 rounded-2xl bg-[var(--surface)] border border-[var(--border)] overflow-hidden">
    <div class="px-4 py-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-2.5 sm:gap-3">
      <button
        type="button"
        class="flex items-center gap-3 text-left min-w-0 flex-1"
        aria-expanded={expanded}
        onclick={() => (expanded = !expanded)}
      >
        <span class="w-9 h-9 rounded-xl bg-[var(--accent-soft)] text-[var(--accent-bright)] inline-flex items-center justify-center shrink-0">
          <BarChart3 size={18} />
        </span>
        <div class="min-w-0 flex-1">
          <div class="flex flex-wrap items-baseline gap-x-2 gap-y-0.5">
            <span class="text-[15px] font-extrabold tracking-tight whitespace-nowrap">{t("stats.title")}</span>
            <span class="text-[11.5px] font-semibold text-[var(--text-muted)] numeric whitespace-nowrap">
              {formatListeningTime(player.listeningStats.totalSeconds, isSpanish)} · {player.listeningStats.totalPlays} {t("stats.total_plays").toLowerCase()}
            </span>
          </div>
          <p class="text-[11.5px] text-[var(--text-muted)] leading-snug mt-0.5">{t("stats.subtitle")}</p>
        </div>
      </button>

      <div class="flex items-center justify-end gap-2 shrink-0 pl-12 sm:pl-0">
        <button
          type="button"
          class="px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--bg)]/60 hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-colors"
          onclick={() => (expanded = !expanded)}
        >
          {expanded ? t("artist.show_less") : t("artist.more")}
        </button>
        {#if expanded}
          <button
            type="button"
            class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--bg)]/60 hover:bg-red-500/20 hover:text-red-400 border border-[var(--border)] transition-colors"
            title={t("stats.reset")}
            onclick={() => player.resetListeningStats()}
          >
            <RotateCcw size={12} />
            {t("stats.reset")}
          </button>
        {/if}
      </div>
    </div>

    {#if expanded}
      <div class="px-4 pb-4 pt-1 border-t border-[var(--border)] space-y-4">
        <!-- Quick stat pills -->
        <div class="grid grid-cols-2 gap-2.5 pt-2">
          <div class="p-3 rounded-xl bg-[var(--bg)]/50 border border-[var(--border)]">
            <div class="text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)] flex items-center gap-1.5">
              <Clock size={12} />
              {t("stats.total_time")}
            </div>
            <div class="text-[20px] font-black tracking-tight numeric mt-1">
              {formatListeningTime(player.listeningStats.totalSeconds, isSpanish)}
            </div>
          </div>
          <div class="p-3 rounded-xl bg-[var(--bg)]/50 border border-[var(--border)]">
            <div class="text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)] flex items-center gap-1.5">
              <Sparkles size={12} />
              {t("stats.total_plays")}
            </div>
            <div class="text-[20px] font-black tracking-tight numeric mt-1">
              {player.listeningStats.totalPlays}
            </div>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          {#if topTracks.length > 0}
            <div class="space-y-1.5">
              <h4 class="text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
                {t("stats.top_tracks")}
              </h4>
              <div class="space-y-1">
                {#each topTracks as entry, idx (entry.track.id)}
                  <button
                    type="button"
                    class="w-full flex items-center gap-2.5 p-1.5 rounded-xl hover:bg-[var(--surface-hover)] text-left transition-colors group"
                    onclick={() =>
                      player.replaceQueueAndPlay(
                        topTracks.map((e) => e.track),
                        idx,
                        t("stats.title"),
                      )}
                  >
                    <span class="w-4 text-center text-[11.5px] font-bold text-[var(--text-muted)] numeric">
                      {idx + 1}
                    </span>
                    <CoverImage
                      class="track-cover !w-9 !h-9 !rounded-[7px] shrink-0"
                      src={player.trackCoverUrl(entry.track)}
                      alt=""
                    />
                    <div class="min-w-0 flex-1">
                      <div class="text-[12.5px] font-bold truncate">{entry.track.title}</div>
                      <div class="text-[11px] text-[var(--text-muted)] truncate">{entry.track.artist}</div>
                    </div>
                    <span class="text-[11px] font-semibold text-[var(--text-muted)] numeric shrink-0 group-hover:hidden">
                      {t("stats.plays_count", { count: entry.plays })}
                    </span>
                    <Play size={14} fill="currentColor" class="hidden group-hover:block text-[var(--accent-bright)] shrink-0 mr-1" />
                  </button>
                {/each}
              </div>
            </div>
          {/if}

          {#if topArtists.length > 0}
            <div class="space-y-1.5">
              <h4 class="text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
                {t("stats.top_artists")}
              </h4>
              <div class="space-y-1">
                {#each topArtists as artist, idx (artist.name)}
                  <button
                    type="button"
                    disabled={player.isOffline}
                    class="w-full flex items-center gap-2.5 p-2 rounded-xl hover:bg-[var(--surface-hover)] text-left transition-colors disabled:opacity-50"
                    onclick={() => void player.openArtistByNameOrId(artist.id, artist.name)}
                  >
                    <span class="w-4 text-center text-[11.5px] font-bold text-[var(--text-muted)] numeric">
                      {idx + 1}
                    </span>
                    <CoverImage
                      class="w-9 h-9 !rounded-full shrink-0 border border-[var(--border)]"
                      src={artistFallbackCover(artist)}
                      alt={artist.name}
                      fallbackIcon="👤"
                    />
                    <span class="text-[13px] font-bold truncate flex-1">{artist.name}</span>
                    <span class="text-[11px] font-semibold text-[var(--text-muted)] numeric shrink-0">
                      {t("stats.plays_count", { count: artist.plays })}
                    </span>
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </section>
{/if}
