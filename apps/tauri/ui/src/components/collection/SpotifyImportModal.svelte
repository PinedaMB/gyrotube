<script lang="ts">
  import { CheckCircle2, FileSpreadsheet, Link2, Loader2, Music2, Upload, X } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { parseSpotifyCsv } from "../../lib/spotifyCsv";
  import { player } from "../../lib/stores/player.svelte";
  import type { ExternalTrackQuery, Track } from "../../lib/types";
  import ModalSheet from "../ModalSheet.svelte";
  import SegmentedControl, { type SegmentedOption } from "../SegmentedControl.svelte";

  interface Props {
    open: boolean;
    onClose: () => void;
  }

  let { open, onClose }: Props = $props();

  type SourceTab = "link" | "csv";
  let sourceTab = $state<SourceTab>("link");
  let spotifyUrl = $state("");
  let playlistName = $state("");
  let playlistCover = $state<string | null>(null);
  let queuedTracks = $state<ExternalTrackQuery[]>([]);
  let fetching = $state(false);
  let importing = $state(false);
  let cancelled = $state(false);
  let errorMsg = $state<string | null>(null);
  let progressDone = $state(0);
  let progressMatched = $state(0);
  let fileInputEl = $state<HTMLInputElement | null>(null);

  let tabOptions = $derived<SegmentedOption<SourceTab>[]>([
    { value: "link", label: t("spotify_import.tab_link"), icon: Link2 },
    { value: "csv", label: t("spotify_import.tab_csv"), icon: FileSpreadsheet },
  ]);

  let progressPct = $derived(
    queuedTracks.length > 0 ? Math.round((progressDone / queuedTracks.length) * 100) : 0,
  );

  function resetState() {
    spotifyUrl = "";
    playlistName = "";
    playlistCover = null;
    queuedTracks = [];
    fetching = false;
    importing = false;
    cancelled = false;
    errorMsg = null;
    progressDone = 0;
    progressMatched = 0;
  }

  function handleClose() {
    if (importing) {
      cancelled = true;
      return;
    }
    resetState();
    onClose();
  }

  async function loadSpotifyLink() {
    const trimmed = spotifyUrl.trim();
    if (!trimmed || fetching || importing) return;
    if (!isTauriRuntime()) {
      errorMsg = t("spotify_import.desktop_required");
      return;
    }
    errorMsg = null;
    fetching = true;
    try {
      const preview = await api.fetchSpotifyPlaylist(trimmed);
      playlistName = preview.name;
      playlistCover = preview.cover_url ?? null;
      queuedTracks = preview.tracks;
    } catch (err) {
      errorMsg = String(err);
      queuedTracks = [];
    } finally {
      fetching = false;
    }
  }

  async function handleCsvFile(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    input.value = "";
    if (!file) return;
    errorMsg = null;
    try {
      const text = await file.text();
      const tracks = parseSpotifyCsv(text);
      if (tracks.length === 0) {
        errorMsg = t("spotify_import.csv_empty");
        queuedTracks = [];
        return;
      }
      playlistName = file.name.replace(/\.csv$/i, "").replace(/[_-]+/g, " ").trim() || "Spotify Import";
      playlistCover = null;
      queuedTracks = tracks;
    } catch {
      errorMsg = t("spotify_import.csv_empty");
    }
  }

  async function startImport() {
    if (queuedTracks.length === 0 || importing) return;
    if (!isTauriRuntime()) {
      errorMsg = t("spotify_import.desktop_required");
      return;
    }
    importing = true;
    cancelled = false;
    errorMsg = null;
    progressDone = 0;
    progressMatched = 0;

    const total = queuedTracks.length;
    const matchedByIdx: (Track | null)[] = new Array(total).fill(null);
    let cursor = 0;
    const concurrency = 3;

    async function worker() {
      while (!cancelled) {
        const idx = cursor++;
        if (idx >= total) break;
        const item = queuedTracks[idx];
        try {
          const found = await api.matchExternalTrack(
            item.title,
            item.artist,
            item.duration_secs ?? null,
          );
          if (found) {
            matchedByIdx[idx] = found;
            progressMatched += 1;
          }
        } catch {
          // skip failed match
        } finally {
          progressDone += 1;
        }
      }
    }

    await Promise.all(Array.from({ length: Math.min(concurrency, total) }, () => worker()));

    const seenIds = new Set<string>();
    const finalTracks: Track[] = [];
    for (const tr of matchedByIdx) {
      if (tr && !seenIds.has(tr.id)) {
        seenIds.add(tr.id);
        finalTracks.push(tr);
      }
    }

    importing = false;

    if (cancelled) {
      if (finalTracks.length === 0) return;
    }

    if (finalTracks.length === 0) {
      errorMsg = t("spotify_import.none_matched");
      return;
    }

    const finalName = playlistName.trim() || "Spotify Import";
    const created = player.createUserPlaylist(
      finalName,
      t("spotify_import.playlist_desc"),
      "#1db954",
      "music",
      finalTracks,
      playlistCover,
    );
    const skipped = total - finalTracks.length;
    player.showToast(
      skipped > 0
        ? t("spotify_import.done_partial", { matched: finalTracks.length, skipped })
        : t("spotify_import.done_all", { matched: finalTracks.length }),
    );
    resetState();
    onClose();
    player.openLocalPlaylist(created.id);
  }
</script>

<ModalSheet {open} label={t("spotify_import.title")} maxWidth={520} onClose={handleClose}>
  <div class="p-5 sm:p-6 space-y-4">
    <div class="flex items-start justify-between gap-3">
      <div>
        <h2 class="text-[18px] font-extrabold tracking-tight leading-snug">
          {t("spotify_import.title")}
        </h2>
        <p class="text-[12.5px] text-[var(--text-muted)] mt-0.5">
          {t("spotify_import.subtitle")}
        </p>
      </div>
      <button
        type="button"
        class="icon-btn !w-8 !h-8 shrink-0 text-[var(--text-muted)] hover:text-[var(--text)]"
        aria-label={t("lyrics.cancel")}
        onclick={handleClose}
      >
        <X size={16} />
      </button>
    </div>

    {#if !importing}
      <SegmentedControl
        options={tabOptions}
        value={sourceTab}
        ariaLabel={t("spotify_import.title")}
        onchange={(tab) => {
          sourceTab = tab;
          errorMsg = null;
        }}
      />

      {#if sourceTab === "link"}
        <div class="space-y-2">
          <div class="flex gap-2">
            <input
              type="url"
              bind:value={spotifyUrl}
              placeholder="https://open.spotify.com/playlist/..."
              class="flex-1 min-w-0 px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:border-[var(--accent-border)] text-[13px] text-[var(--text)] outline-none"
              onkeydown={(e) => e.key === "Enter" && void loadSpotifyLink()}
            />
            <button
              type="button"
              class="pill-btn primary !px-4 shrink-0"
              disabled={!spotifyUrl.trim() || fetching}
              onclick={() => void loadSpotifyLink()}
            >
              {#if fetching}
                <Loader2 size={14} class="animate-spin" />
              {/if}
              {t("spotify_import.fetch")}
            </button>
          </div>
          <p class="text-[11.5px] text-[var(--text-muted)]">
            {t("spotify_import.link_hint")}
          </p>
        </div>
      {:else}
        <div class="space-y-2">
          <button
            type="button"
            class="w-full py-5 px-4 rounded-2xl border border-dashed border-[var(--border-strong)] hover:border-[var(--accent-border)] bg-[var(--surface)] hover:bg-[var(--surface-hover)] flex flex-col items-center justify-center gap-2 transition-colors"
            onclick={() => fileInputEl?.click()}
          >
            <Upload size={20} class="text-[var(--accent-bright)]" />
            <span class="text-[13px] font-semibold">{t("spotify_import.csv_pick")}</span>
            <span class="text-[11.5px] text-[var(--text-muted)] text-center">
              {t("spotify_import.csv_hint")}
            </span>
          </button>
          <input
            bind:this={fileInputEl}
            type="file"
            accept=".csv,text/csv"
            class="hidden"
            onchange={(e) => void handleCsvFile(e)}
          />
        </div>
      {/if}
    {/if}

    {#if errorMsg}
      <div class="px-3.5 py-2.5 rounded-xl bg-red-500/12 border border-red-500/30 text-[12.5px] text-red-400">
        {errorMsg}
      </div>
    {/if}

    {#if queuedTracks.length > 0}
      <div class="p-3.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] space-y-3">
        <div class="flex items-center gap-3">
          <span class="w-10 h-10 rounded-xl bg-[var(--accent-soft)] text-[var(--accent-bright)] inline-flex items-center justify-center shrink-0">
            <Music2 size={18} />
          </span>
          <div class="min-w-0 flex-1 space-y-1">
            <input
              type="text"
              bind:value={playlistName}
              disabled={importing}
              placeholder={t("collection.modal_name_ph")}
              class="w-full px-2.5 py-1 rounded-lg bg-[var(--bg)]/60 border border-[var(--border)] focus:border-[var(--accent-border)] text-[13.5px] font-bold text-[var(--text)] outline-none"
            />
            <div class="text-[12px] text-[var(--text-muted)] numeric px-0.5">
              {t("spotify_import.tracks_ready", { count: queuedTracks.length })}
            </div>
          </div>
        </div>

        {#if importing}
          <div class="space-y-1.5">
            <div class="flex items-center justify-between text-[12px] numeric">
              <span class="text-[var(--text-muted)]">
                {t("spotify_import.matching", { done: progressDone, total: queuedTracks.length })}
              </span>
              <span class="font-bold text-[var(--accent-bright)]">
                {t("spotify_import.found_count", { matched: progressMatched })}
              </span>
            </div>
            <div class="h-2 w-full rounded-full bg-[var(--control-track)] overflow-hidden">
              <div
                class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-200"
                style="width: {progressPct}%;"
              ></div>
            </div>
          </div>
        {/if}
      </div>
    {/if}

    <div class="flex items-center justify-end gap-2 pt-1">
      <button type="button" class="pill-btn" onclick={handleClose}>
        {importing ? t("spotify_import.stop") : t("lyrics.cancel")}
      </button>
      <button
        type="button"
        class="pill-btn primary !px-4"
        disabled={queuedTracks.length === 0 || importing || fetching}
        onclick={() => void startImport()}
      >
        {#if importing}
          <Loader2 size={14} class="animate-spin" />
        {:else}
          <CheckCircle2 size={14} />
        {/if}
        {t("spotify_import.import_btn")}
      </button>
    </div>
  </div>
</ModalSheet>
