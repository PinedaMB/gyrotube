<script lang="ts">
  import { ArrowLeft, Clock3, Heart, ListPlus, Play, Shuffle, Volume2 } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

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
    const shuffled = [...tracks].sort(() => Math.random() - 0.5);
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
    <div
      class="grid grid-cols-[28px_minmax(0,1fr)_auto] lg:grid-cols-[28px_minmax(0,1.65fr)_minmax(0,1fr)_auto] items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]"
    >
      <span class="text-center">#</span>
      <span>{t("collection.col_title")}</span>
      <span class="hidden lg:block">{t("collection.col_album")}</span>
      <span class="w-[108px] flex justify-end"><Clock3 size={13} /></span>
    </div>

    <div class="flex flex-col">
      {#each tracks as track, i (track.id + ":" + i)}
        {@const active = player.currentTrack?.id === track.id}
        {@const favorited = player.isFavorite(track.id)}
        <div
          class="group grid grid-cols-[28px_minmax(0,1fr)_auto] lg:grid-cols-[28px_minmax(0,1.65fr)_minmax(0,1fr)_auto] items-center gap-4 py-2.5 px-3 border-b border-white/[0.06] last:border-b-0 cursor-pointer transition-colors {active
            ? 'bg-[var(--accent-soft)]'
            : 'hover:bg-white/[0.05]'}"
          role="button"
          tabindex="0"
          onclick={() => player.replaceQueueAndPlay(tracks, i, artistName)}
          onkeydown={(e) => e.key === "Enter" && player.replaceQueueAndPlay(tracks, i, artistName)}
        >
          <div class="flex items-center justify-center shrink-0">
            {#if active}
              <Volume2 size={15} class="text-[var(--accent-bright)]" />
            {:else}
              <span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric group-hover:hidden">
                {i + 1}
              </span>
              <Play size={13} fill="currentColor" class="hidden group-hover:block translate-x-[1px]" />
            {/if}
          </div>

          <div class="flex items-center gap-3.5 min-w-0">
            <CoverImage class="track-cover !w-10 !h-10 !rounded-[8px] shrink-0" src={track.cover_url} alt="" />
            <div class="min-w-0">
              <div
                class="text-[14px] font-semibold truncate {active
                  ? 'text-[var(--accent-bright)] font-bold'
                  : ''}"
              >
                {track.title}
              </div>
              <div class="text-[12px] text-[var(--text-muted)] truncate">
                {track.artist}
              </div>
            </div>
          </div>

          <span class="hidden lg:block text-[13px] text-[var(--text-muted)] truncate">
            {track.album ?? ""}
          </span>

          <div class="flex items-center justify-end gap-1 shrink-0 w-[108px]">
            <button
              type="button"
              class="w-8 h-8 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1] transition-all"
              title={t("queue.add")}
              onclick={(e) => {
                e.stopPropagation();
                player.enqueueTrack(track);
              }}
            >
              <ListPlus size={16} />
            </button>
            <button
              type="button"
              class="w-8 h-8 rounded-full inline-flex items-center justify-center transition-all {favorited
                ? 'text-[var(--fav-coral)] opacity-100 hover:bg-white/[0.08]'
                : 'text-[var(--text-muted)] opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-[var(--text)] hover:bg-white/[0.1]'}"
              title={favorited ? t("player.favorite_remove") : t("player.favorite_add")}
              onclick={(e) => {
                e.stopPropagation();
                player.toggleFavorite(track);
              }}
            >
              <Heart size={16} fill={favorited ? "currentColor" : "none"} />
            </button>
            <span class="text-[12.5px] text-[var(--text-muted)] numeric w-11 text-right">
              {formatDuration(track.duration_secs)}
            </span>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
