<script lang="ts">
  import { Play, Trash2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import TrackRow from "../TrackRow.svelte";
</script>

<section class="mt-8 flex flex-col gap-3">
  <div class="flex items-center justify-between gap-3">
    <div class="flex items-baseline gap-2">
      <h2 class="section-title">{t("collection.recently_played")}</h2>
      {#if player.recentTracks.length > 0}
        <span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric">
          {player.recentTracks.length}
        </span>
      {/if}
    </div>

    {#if player.recentTracks.length > 0}
      <div class="flex items-center gap-2">
        <button
          type="button"
          class="pill-btn hover:!text-red-400 hover:!border-red-500/30 hover:!bg-red-500/10"
          onclick={() => player.clearRecentTracks()}
        >
          <Trash2 size={13} />
          {t("collection.clear_recent")}
        </button>
        <button
          type="button"
          class="pill-btn"
          onclick={() =>
            player.replaceQueueAndPlay(player.recentTracks, 0, t("collection.recently_played"))}
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
        <TrackRow
          {track}
          onPlay={() =>
            player.replaceQueueAndPlay(player.recentTracks, i, t("collection.recently_played"))}
          onRemove={() => player.removeRecentTrack(track.id)}
          removeLabel={t("collection.remove_recent")}
        />
      {/each}
    </div>
  {/if}
</section>
