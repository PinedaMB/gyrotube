<script lang="ts">
  import {
    ArrowLeft,
    CheckCircle2,
    Download,
    Heart,
    ListPlus,
    Pencil,
    Play,
    Shuffle,
    Trash2,
  } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track, UserPlaylist } from "../../lib/types";
  import PlaylistIcon from "./PlaylistIcon.svelte";

  interface Props {
    mode: "favorites" | "playlist";
    playlist?: UserPlaylist | null;
    title: string;
    description: string;
    accentColor: string;
    tracks: Track[];
    totalMins: number;
    onBack: () => void;
    onPlayAll: () => void;
    onShuffle: () => void;
    onEdit?: (pl: UserPlaylist) => void;
    onDelete?: (pl: UserPlaylist) => void;
  }

  let {
    mode,
    playlist = null,
    title,
    description,
    accentColor,
    tracks,
    totalMins,
    onBack,
    onPlayAll,
    onShuffle,
    onEdit,
    onDelete,
  }: Props = $props();

  let playlistKey = $derived(mode === "favorites" ? "__favorites__" : (playlist?.id ?? "__playlist__"));
  let markedDownloaded = $derived(player.isPlaylistMarkedDownloaded(playlistKey));
  let downloadedCount = $derived(
    tracks.reduce((acc, tr) => acc + (player.isTrackDownloaded(tr.id) ? 1 : 0), 0),
  );
  let allDownloaded = $derived(tracks.length > 0 && downloadedCount === tracks.length);
  let isDownloadingThis = $derived(player.downloadingPlaylistKey === playlistKey);
  let isDownloadingAny = $derived(player.downloadingPlaylistKey !== null);

  async function handleDownloadClick() {
    if (tracks.length === 0 || isDownloadingAny) return;
    if (!markedDownloaded && player.isOffline) return;
    const outcome = await player.togglePlaylistDownload(playlistKey, tracks);
    if (outcome === "removed") {
      player.showToast(t("offline.downloads_removed"));
    } else if (outcome === "downloaded") {
      player.showToast(t("offline.download_done"));
    } else if (outcome === "partial") {
      player.showToast(
        t("offline.download_partial", { done: downloadedCount, total: tracks.length }),
      );
    }
  }

  function queueAll() {
    if (!tracks.length) return;
    for (const tr of tracks) {
      player.queue = [...player.queue, tr];
    }
    void player.persistSession();
    player.showToast(t("toast.added_many_to_queue", { count: tracks.length }));
  }
</script>

<aside class="lg:sticky lg:top-8 flex flex-col items-center text-center gap-3 lg:gap-4">
  <div class="w-full flex items-center justify-between gap-2">
    <button
      type="button"
      class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--text)] bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95"
      title={t("nav.back")}
      onclick={onBack}
    >
      <ArrowLeft size={15} />
      {t("nav.back")}
    </button>

    {#if mode === "playlist" && playlist}
      <button
        type="button"
        class="lg:hidden inline-flex items-center justify-center w-9 h-9 rounded-full text-[var(--text-muted)] hover:text-red-500 bg-[var(--surface)] hover:bg-red-500/12 border border-[var(--border)] transition-all active:scale-95"
        title={t("collection.delete")}
        aria-label={t("collection.delete")}
        onclick={() => onDelete?.(playlist)}
      >
        <Trash2 size={15} />
      </button>
    {/if}
  </div>

  <!-- Responsive playlist artwork tile: compact 176px on mobile, 248px on desktop -->
  <div
    class="relative w-44 h-44 sm:w-[216px] sm:h-[216px] lg:w-[248px] lg:h-[248px] rounded-2xl flex items-center justify-center text-white overflow-hidden border border-[var(--border-strong)] shadow-[0_20px_48px_rgba(15,23,42,0.28)]"
    style={mode === "favorites"
      ? "background: linear-gradient(135deg, #e01b24 0%, #ff6b81 100%);"
      : `background: linear-gradient(135deg, ${accentColor} 0%, ${accentColor}b8 100%);`}
  >
    {#if mode === "playlist" && playlist?.cover_path}
      <img src={playlist.cover_path} alt={title} class="w-full h-full object-cover" />
    {:else}
      <div
        class="pointer-events-none absolute inset-0 bg-gradient-to-br from-white/20 via-transparent to-black/30"
      ></div>
      <div class="relative z-10 drop-shadow-[0_10px_24px_rgba(0,0,0,0.35)]">
        {#if mode === "favorites"}
          <Heart size={68} fill="currentColor" class="lg:w-[82px] lg:h-[82px]" />
        {:else}
          <PlaylistIcon name={playlist?.icon_name} size={68} />
        {/if}
      </div>
    {/if}
  </div>

  <div class="space-y-1 max-w-[300px]">
    <h1 class="text-[22px] sm:text-[26px] font-black tracking-[-0.025em] leading-tight">
      {title}
    </h1>
    {#if description}
      <p class="text-[12.5px] sm:text-[13px] text-[var(--text-muted)] leading-snug line-clamp-2">
        {description}
      </p>
    {/if}
    <div class="flex items-center justify-center gap-2 text-[12px] sm:text-[12.5px] font-medium text-[var(--text-muted)] pt-0.5">
      <span class="px-2 py-0.5 rounded-full text-[10px] sm:text-[10.5px] font-bold uppercase tracking-wider bg-[var(--surface)] text-[var(--text)]/90 border border-[var(--border)]">
        {mode === "favorites" ? t("collection.favorites") : t("collection.local_badge")}
      </span>
      <span>·</span>
      <span class="numeric">
        {tracks.length === 1
          ? t("collection.song_one")
          : t("collection.song_many", { count: tracks.length })}{totalMins
          ? ` · ${totalMins} min`
          : ""}
      </span>
    </div>
  </div>

  <!-- Centered circular controls (Play dead-center; Download integrated on mobile to save vertical space) -->
  <div class="flex items-center justify-center gap-3 sm:gap-4 pt-0.5">
    {#if tracks.length > 0}
      <button
        type="button"
        class="lg:hidden w-10 h-10 rounded-full inline-flex items-center justify-center border transition-all active:scale-95 disabled:opacity-35 disabled:cursor-not-allowed {markedDownloaded || allDownloaded
          ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
          : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] border-[var(--border)]'}"
        disabled={isDownloadingAny || (!markedDownloaded && player.isOffline)}
        title={markedDownloaded ? t("offline.remove_downloads") : t("offline.download")}
        aria-label={markedDownloaded ? t("offline.remove_downloads") : t("offline.download")}
        onclick={() => void handleDownloadClick()}
      >
        {#if isDownloadingThis}
          <span class="w-4 h-4 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
        {:else if markedDownloaded || allDownloaded}
          <CheckCircle2 size={17} />
        {:else}
          <Download size={17} />
        {/if}
      </button>
    {/if}

    <button
      type="button"
      class="w-10 h-10 sm:w-11 sm:h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 disabled:opacity-35 disabled:cursor-not-allowed"
      disabled={tracks.length === 0}
      title={t("artist.shuffle")}
      aria-label={t("artist.shuffle")}
      onclick={onShuffle}
    >
      <Shuffle size={17} />
    </button>

    <button
      type="button"
      class="play-circle !w-14 !h-14 sm:!w-[60px] sm:!h-[60px] disabled:opacity-35 disabled:cursor-not-allowed"
      disabled={tracks.length === 0}
      title={t("collection.play_all")}
      aria-label={t("collection.play_all")}
      onclick={onPlayAll}
    >
      <Play size={22} fill="currentColor" class="translate-x-[1.5px]" />
    </button>

    {#if mode === "playlist" && playlist}
      <button
        type="button"
        class="w-10 h-10 sm:w-11 sm:h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95"
        title={t("collection.customize")}
        aria-label={t("collection.customize")}
        onclick={() => onEdit?.(playlist)}
      >
        <Pencil size={16} />
      </button>
    {/if}

    <button
      type="button"
      class="w-10 h-10 sm:w-11 sm:h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 disabled:opacity-35 disabled:cursor-not-allowed"
      disabled={tracks.length === 0}
      title={t("queue.add")}
      aria-label={t("queue.add")}
      onclick={queueAll}
    >
      <ListPlus size={17} />
    </button>
  </div>

  {#if isDownloadingThis && player.downloadProgress}
    <div class="lg:hidden inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[11.5px] font-semibold bg-[var(--accent-soft)] border border-[var(--accent-border)] text-[var(--accent-bright)]">
      <span class="w-3 h-3 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
      <span>
        {t("offline.downloading", {
          done: player.downloadProgress.done,
          total: player.downloadProgress.total,
        })}
      </span>
    </div>
  {/if}

  <!-- Desktop-only pill buttons (plenty of vertical space in sticky sidebar) -->
  {#if tracks.length > 0}
    <button
      type="button"
      class="hidden lg:inline-flex pill-btn !px-4 !py-1.5 !text-[12.5px] {markedDownloaded || allDownloaded
        ? '!bg-[var(--accent-soft)] !border-[var(--accent-border)] !text-[var(--accent-bright)]'
        : ''}"
      disabled={isDownloadingAny || (!markedDownloaded && player.isOffline)}
      title={markedDownloaded ? t("offline.remove_downloads") : t("offline.download")}
      onclick={() => void handleDownloadClick()}
    >
      {#if isDownloadingThis && player.downloadProgress}
        <span class="w-3.5 h-3.5 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
        <span>
          {t("offline.downloading", {
            done: player.downloadProgress.done,
            total: player.downloadProgress.total,
          })}
        </span>
      {:else if markedDownloaded && allDownloaded}
        <CheckCircle2 size={14} />
        <span>{t("offline.downloaded")}</span>
      {:else if markedDownloaded}
        <CheckCircle2 size={14} />
        <span>{t("offline.downloaded")} ({downloadedCount}/{tracks.length})</span>
      {:else}
        <Download size={14} />
        <span>
          {downloadedCount > 0
            ? `${t("offline.download")} (${downloadedCount}/${tracks.length})`
            : t("offline.download")}
        </span>
      {/if}
    </button>
  {/if}

  {#if mode === "playlist" && playlist}
    <button
      type="button"
      class="hidden lg:inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12px] font-semibold text-[var(--text-muted)] hover:text-red-500 hover:bg-red-500/12 transition-colors"
      onclick={() => onDelete?.(playlist)}
    >
      <Trash2 size={13} />
      {t("collection.delete")}
    </button>
  {/if}
</aside>
