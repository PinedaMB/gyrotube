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

<aside class="lg:sticky lg:top-8 flex flex-col items-center text-center gap-4">
  <div class="w-full flex items-center justify-start">
    <button
      class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12.5px] font-semibold text-[var(--text-muted)] hover:text-[var(--text)] bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95"
      title={t("nav.back")}
      onclick={onBack}
    >
      <ArrowLeft size={15} />
      {t("nav.back")}
    </button>
  </div>

  <!-- Large playlist artwork tile matching AlbumSidebarHero (248px) -->
  <div
    class="relative w-[248px] h-[248px] rounded-2xl flex items-center justify-center text-white overflow-hidden border border-[var(--border-strong)] shadow-[0_24px_56px_rgba(15,23,42,0.28)]"
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
          <Heart size={82} fill="currentColor" />
        {:else}
          <PlaylistIcon name={playlist?.icon_name} size={82} />
        {/if}
      </div>
    {/if}
  </div>

  <div class="space-y-1.5 max-w-[300px]">
    <h1 class="text-[26px] font-black tracking-[-0.025em] leading-tight">
      {title}
    </h1>
    {#if description}
      <p class="text-[13px] text-[var(--text-muted)] leading-snug">{description}</p>
    {/if}
    <div class="flex items-center justify-center gap-2 text-[12.5px] font-medium text-[var(--text-muted)] pt-0.5">
      <span class="px-2.5 py-0.5 rounded-full text-[10.5px] font-bold uppercase tracking-wider bg-[var(--surface)] text-[var(--text)]/90 border border-[var(--border)]">
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

  <!-- Centered circular controls matching AlbumSidebarHero -->
  <div class="flex items-center justify-center gap-4 pt-1">
    <button
      class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 disabled:opacity-35 disabled:cursor-not-allowed"
      disabled={tracks.length === 0}
      title={t("artist.shuffle")}
      onclick={onShuffle}
    >
      <Shuffle size={18} />
    </button>

    <button
      class="play-circle !w-[60px] !h-[60px] disabled:opacity-35 disabled:cursor-not-allowed"
      disabled={tracks.length === 0}
      title={t("collection.play_all")}
      onclick={onPlayAll}
    >
      <Play size={24} fill="currentColor" class="translate-x-[1.5px]" />
    </button>

    {#if mode === "playlist" && playlist}
      <button
        class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95"
        title={t("collection.customize")}
        onclick={() => onEdit?.(playlist)}
      >
        <Pencil size={17} />
      </button>
    {:else}
      <button
        class="w-11 h-11 rounded-full inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 disabled:opacity-35 disabled:cursor-not-allowed"
        disabled={tracks.length === 0}
        title={t("queue.add")}
        onclick={queueAll}
      >
        <ListPlus size={18} />
      </button>
    {/if}
  </div>

  {#if tracks.length > 0}
    <button
      type="button"
      class="pill-btn !px-4 !py-1.5 !text-[12.5px] {markedDownloaded || allDownloaded
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
      class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12px] font-semibold text-[var(--text-muted)] hover:text-red-500 hover:bg-red-500/12 transition-colors"
      onclick={() => onDelete?.(playlist)}
    >
      <Trash2 size={13} />
      {t("collection.delete")}
    </button>
  {/if}
</aside>
