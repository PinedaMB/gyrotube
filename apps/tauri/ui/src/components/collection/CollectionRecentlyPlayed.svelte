<script lang="ts">
  import { Play, Trash2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";
  import TrackRow from "../TrackRow.svelte";

  type HistoryBucketKey = "today" | "yesterday" | "week" | "older";

  let groupedRecent = $derived.by(() => {
    const now = new Date();
    const startOfToday = new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime();
    const startOfYesterday = startOfToday - 24 * 60 * 60 * 1000;
    const startOfWeek = startOfToday - 7 * 24 * 60 * 60 * 1000;

    const buckets: Record<HistoryBucketKey, { track: Track; index: number }[]> = {
      today: [],
      yesterday: [],
      week: [],
      older: [],
    };

    player.recentTracks.forEach((track, index) => {
      const ts = player.recentPlayedAt[track.id];
      if (!ts || ts >= startOfToday) {
        buckets.today.push({ track, index });
      } else if (ts >= startOfYesterday) {
        buckets.yesterday.push({ track, index });
      } else if (ts >= startOfWeek) {
        buckets.week.push({ track, index });
      } else {
        buckets.older.push({ track, index });
      }
    });

    const order: { key: HistoryBucketKey; label: string }[] = [
      { key: "today", label: t("collection.history_today") },
      { key: "yesterday", label: t("collection.history_yesterday") },
      { key: "week", label: t("collection.history_week") },
      { key: "older", label: t("collection.history_older") },
    ];

    return order
      .map((entry) => ({ ...entry, items: buckets[entry.key] }))
      .filter((group) => group.items.length > 0);
  });

  function playAllRecent() {
    const list = player.isOffline
      ? player.recentTracks.filter((tr) => player.isTrackDownloaded(tr.id))
      : player.recentTracks;
    if (!list.length) {
      if (player.isOffline) player.showToast(t("offline.not_downloaded"));
      return;
    }
    player.replaceQueueAndPlay(list, 0, t("collection.recently_played"));
  }
</script>

<section class="mt-8 flex flex-col gap-3">
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2.5">
    <div class="flex items-center gap-2 min-w-0">
      <h2 class="section-title !text-[20px] sm:!text-[22px] truncate">{t("collection.recently_played")}</h2>
      {#if player.recentTracks.length > 0}
        <span class="px-2 py-0.5 rounded-full bg-[var(--surface)] border border-[var(--border)] text-[11.5px] font-bold text-[var(--text-muted)] numeric shrink-0">
          {player.recentTracks.length}
        </span>
      {/if}
    </div>

    {#if player.recentTracks.length > 0}
      <div class="flex items-center gap-2 shrink-0">
        <button
          type="button"
          class="pill-btn !text-[12.5px] hover:!text-red-400 hover:!border-red-500/30 hover:!bg-red-500/10"
          onclick={() => player.clearRecentTracks()}
        >
          <Trash2 size={13} />
          {t("collection.clear_recent")}
        </button>
        <button
          type="button"
          class="pill-btn !text-[12.5px]"
          onclick={playAllRecent}
        >
          <Play size={13} fill="currentColor" />
          {t("collection.play_all")}
        </button>
      </div>
    {/if}
  </div>

  {#if player.recentTracks.length === 0}
    <div class="py-10 text-center rounded-2xl bg-[var(--surface)] border border-[var(--border)]">
      <p class="text-[var(--text-muted)] text-[13.5px]">{t("collection.empty_recent")}</p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each groupedRecent as group (group.key)}
        <div class="space-y-1.5">
          {#if groupedRecent.length > 1 || group.key !== "today"}
            <h3 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)] px-1">
              {group.label}
            </h3>
          {/if}
          <div class="track-list">
            {#each group.items as { track, index } (track.id)}
              {@const unavailableOffline = player.isOffline && !player.isTrackDownloaded(track.id)}
              <div class={unavailableOffline ? "opacity-45" : ""}>
                <TrackRow
                  {track}
                  onPlay={() => {
                    if (unavailableOffline) {
                      player.showToast(t("offline.not_downloaded"));
                      return;
                    }
                    if (player.isOffline) {
                      const downloaded = player.recentTracks.filter((tr) => player.isTrackDownloaded(tr.id));
                      const idx = downloaded.findIndex((tr) => tr.id === track.id);
                      if (idx >= 0) {
                        player.replaceQueueAndPlay(downloaded, idx, t("collection.recently_played"));
                      }
                      return;
                    }
                    player.replaceQueueAndPlay(player.recentTracks, index, t("collection.recently_played"));
                  }}
                  onRemove={() => player.removeRecentTrack(track.id)}
                  removeLabel={t("collection.remove_recent")}
                />
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</section>
