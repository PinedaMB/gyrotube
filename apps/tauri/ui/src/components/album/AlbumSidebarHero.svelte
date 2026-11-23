<script lang="ts">
  import { ArrowLeft, ListPlus, Play, Radio, Shuffle } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { shuffleItems } from "../../lib/stores/queueOps";
  import type { Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  interface Props {
    id: string;
    title: string;
    coverUrl?: string | null;
    isPlaylist?: boolean;
    tracks: Track[];
  }

  let { id, title, coverUrl = null, isPlaylist = false, tracks }: Props = $props();
  let startingRadio = $state(false);

  async function startPlaylistRadio() {
    if (!isPlaylist || startingRadio || !isTauriRuntime()) return;
    startingRadio = true;
    try {
      const radio = await api.getPlaylistRadio(id);
      if (radio.length > 0) {
        player.replaceQueueAndPlay(radio, 0, `Radio · ${title}`);
      } else {
        player.showToast(t("toast.radio_empty"));
      }
    } catch (err) {
      player.showToast(String(err));
    } finally {
      startingRadio = false;
    }
  }

  let totalMins = $derived(
    Math.round(tracks.reduce((sum, t) => sum + (t.duration_secs ?? 0), 0) / 60),
  );
  let artistLabel = $derived(tracks[0]?.artist ?? "");

  function playAll() {
    if (!tracks.length) return;
    player.replaceQueueAndPlay(tracks, 0, title);
  }

  function shuffleAll() {
    if (!tracks.length) return;
    const shuffled = shuffleItems(tracks);
    player.replaceQueueAndPlay(shuffled, 0, title);
  }

  function queueAll() {
    if (!tracks.length) return;
    for (const t of tracks) {
      player.queue = [...player.queue, t];
    }
    void player.persistSession();
    player.showToast(t("toast.added_many_to_queue", { count: tracks.length }));
  }
</script>

<aside class="lg:sticky lg:top-8 flex flex-col items-center text-center gap-4">
  <div class="w-full flex items-center justify-start">
    <button
      class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--text)] bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95"
      title={t("nav.back")}
      onclick={() => player.goBack()}
    >
      <ArrowLeft size={15} />
      {t("nav.back")}
    </button>
  </div>

  {#if artistLabel}
    <span class="text-[13px] font-semibold text-[var(--text-muted)] truncate max-w-full">
      {artistLabel}
    </span>
  {/if}

  <CoverImage
    class="album-cover !w-[248px] !h-[248px] !rounded-2xl border border-white/10 shadow-[0_22px_52px_rgba(0,0,0,0.52)]"
    src={coverUrl}
    fallbackSrc={tracks[0]?.cover_large_url ?? tracks[0]?.cover_url}
    alt={title}
    eager={true}
  />

  <div class="space-y-1.5 max-w-[300px]">
    <h1 class="text-[24px] font-black tracking-[-0.025em] leading-tight">
      {title}
    </h1>
    <div class="flex items-center justify-center gap-2 text-[12.5px] font-medium text-[var(--text-muted)]">
      <span class="px-2 py-0.5 rounded-full text-[10.5px] font-bold uppercase tracking-wider bg-[var(--accent-soft)] text-[var(--accent-bright)]">
        {isPlaylist ? t("album.badge_playlist") : t("album.badge_album")}
      </span>
      <span>·</span>
      <span class="numeric">
        {tracks.length === 1
          ? t("collection.song_one")
          : t("collection.song_many", { count: tracks.length })}{totalMins ? ` · ${totalMins} min` : ""}
      </span>
    </div>
  </div>

  <!-- 3 centered circular controls -->
  <div class="flex items-center justify-center gap-4 pt-2">
    <button
      class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 disabled:opacity-40"
      disabled={tracks.length === 0}
      title={t("player.shuffle")}
      onclick={shuffleAll}
    >
      <Shuffle size={18} />
    </button>

    <button
      class="play-circle !w-[60px] !h-[60px]"
      disabled={tracks.length === 0}
      title={t("home.play")}
      onclick={playAll}
    >
      <Play size={24} fill="currentColor" class="translate-x-[1.5px]" />
    </button>

    <button
      class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 disabled:opacity-40"
      disabled={tracks.length === 0}
      title={t("queue.add")}
      onclick={queueAll}
    >
      <ListPlus size={18} />
    </button>

    {#if isPlaylist}
      <button
        class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 disabled:opacity-40"
        disabled={startingRadio}
        title={t("album.start_radio")}
        onclick={() => void startPlaylistRadio()}
      >
        {#if startingRadio}
          <span class="w-4 h-4 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
        {:else}
          <Radio size={18} />
        {/if}
      </button>
    {/if}
  </div>
</aside>
