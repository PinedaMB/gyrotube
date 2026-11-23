<script lang="ts">
  import { Play, Trash2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import TrackRow from "../TrackRow.svelte";

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
    <div class="track-list">
      {#each player.recentTracks as track, i (track.id)}
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
              player.replaceQueueAndPlay(player.recentTracks, i, t("collection.recently_played"));
            }}
            onRemove={() => player.removeRecentTrack(track.id)}
            removeLabel={t("collection.remove_recent")}
          />
        </div>
      {/each}
    </div>
  {/if}
</section>
