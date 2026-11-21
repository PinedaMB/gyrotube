<script lang="ts">
  import TrackRow from "../TrackRow.svelte";
  import { ArrowLeft, Play, Shuffle } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { shuffleItems } from "../../lib/stores/queueOps";
  import type { Track } from "../../lib/types";

  interface Props {
    artistName: string;
    tracks: Track[];
    onBack: () => void;
  }

  let { artistName, tracks, onBack }: Props = $props();

  let totalMins = $derived(
    Math.round(tracks.reduce((sum, tr) => sum + (tr.duration_secs ?? 0), 0) / 60),
  );

  function playAll() {
    if (!tracks.length) return;
    player.replaceQueueAndPlay(tracks, 0, artistName);
  }

  function shufflePlay() {
    if (!tracks.length) return;
    const shuffled = shuffleItems(tracks);
    player.replaceQueueAndPlay(shuffled, 0, artistName);
  }
</script>

<div class="w-full max-w-[1280px] mx-auto px-6 pt-6 pb-28 space-y-6">
  <!-- Top navigation & header -->
  <div class="space-y-4">
    <button
      type="button"
      class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--text)] bg-white/[0.06] hover:bg-white/[0.12] border border-white/10 transition-all active:scale-95"
      onclick={onBack}
    >
      <ArrowLeft size={15} />
      {artistName}
    </button>

    <div class="flex flex-wrap items-end justify-between gap-4">
      <div class="space-y-1">
        <span class="text-[12px] font-bold uppercase tracking-[0.08em] text-[var(--accent-bright)]">
          {artistName}
        </span>
        <h1 class="text-[30px] sm:text-[36px] font-black tracking-[-0.03em] leading-tight">
          {t("artist.popular")}
        </h1>
        <p class="text-[13px] text-[var(--text-muted)] numeric">
          {tracks.length === 1
            ? t("collection.song_one")
            : t("collection.song_many", { count: tracks.length })}{totalMins
            ? ` · ${totalMins} min`
            : ""}
        </p>
      </div>

      <div class="flex items-center gap-2.5">
        <button
          type="button"
          class="pill-btn primary !px-5 !py-2"
          disabled={tracks.length === 0}
          onclick={playAll}
        >
          <Play size={15} fill="currentColor" />
          {t("collection.play_all")}
        </button>
        <button
          type="button"
          class="pill-btn !px-4 !py-2 bg-white/[0.07] hover:bg-white/[0.13] border-white/15"
          disabled={tracks.length === 0}
          onclick={shufflePlay}
        >
          <Shuffle size={15} />
          {t("artist.shuffle")}
        </button>
      </div>
    </div>
  </div>

  <!-- Full numbered track list -->
  <div>
    <div class="track-list">
      {#each tracks as track, i (track.id + ":" + i)}
        <TrackRow {track} number={i + 1} onPlay={() => player.replaceQueueAndPlay(tracks, i, artistName)} />
      {/each}
    </div>
  </div>
</div>
