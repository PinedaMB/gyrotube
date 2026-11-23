<script lang="ts">
  import { Download, Globe, HardDrive, Monitor, Moon, Sun, Trash2, Upload, X } from "lucide-svelte";
  import { api, isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { AppLanguage, ThemeMode } from "../lib/types";
  import ModalSheet from "./ModalSheet.svelte";
  import SegmentedControl, { type SegmentedOption } from "./SegmentedControl.svelte";

  let cacheBytes = $state(0);
  let downloadsBytes = $state(0);
  let fileInputEl = $state<HTMLInputElement | null>(null);

  $effect(() => {
    if (player.settingsOpen && isTauriRuntime()) {
      void api.getCacheBytes().then((b) => (cacheBytes = b)).catch(() => {});
      void api.getDownloadsBytes().then((b) => (downloadsBytes = b)).catch(() => {});
    }
  });

  let cacheMb = $derived((cacheBytes / (1024 * 1024)).toFixed(1));
  let downloadsMb = $derived((downloadsBytes / (1024 * 1024)).toFixed(1));
  let downloadedCount = $derived(player.downloadedTrackIds.size);
  let cachePct = $derived(Math.min(100, (cacheBytes / (1024 * 1024 * 1024)) * 100));
  let crossfadePct = $derived(Math.round((player.crossfadeSecs / 10) * 100));

  async function clearCache() {
    if (!isTauriRuntime()) return;
    await api.clearAudioCache();
    cacheBytes = 0;
    player.showToast(t("settings.cache_cleared"));
  }

  async function clearDownloads() {
    if (!isTauriRuntime()) return;
    try {
      await player.clearAllDownloads();
      downloadsBytes = 0;
      player.showToast(t("offline.downloads_removed"));
    } catch (err) {
      player.showToast(String(err));
    }
  }

  function exportLibrary() {
    try {
      const json = player.exportLibraryJson();
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      const date = new Date().toISOString().slice(0, 10);
      a.href = url;
      a.download = `gyrotube-library-${date}.json`;
      document.body.appendChild(a);
      a.click();
      a.remove();
      window.setTimeout(() => URL.revokeObjectURL(url), 1000);
      player.showToast(t("settings.export_done"));
    } catch (err) {
      player.showToast(String(err));
    }
  }

  async function handleImportFile(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    input.value = "";
    if (!file) return;
    try {
      const text = await file.text();
      const res = player.importLibraryJson(text);
      if (res.favoritesAdded === 0 && res.playlistsAdded === 0 && res.artistsAdded === 0) {
        player.showToast(t("settings.import_nothing"));
      } else {
        player.showToast(
          t("settings.import_done", {
            favorites: res.favoritesAdded,
            playlists: res.playlistsAdded,
          }),
        );
      }
    } catch {
      player.showToast(t("settings.import_invalid"));
    }
  }

  let themeOptions = $derived<SegmentedOption<ThemeMode>[]>([
    { value: "system", label: t("settings.theme_system"), icon: Monitor },
    { value: "dark", label: t("settings.theme_dark"), icon: Moon },
    { value: "light", label: t("settings.theme_light"), icon: Sun },
  ]);

  let languageOptions = $derived<SegmentedOption<AppLanguage>[]>([
    { value: "Auto", label: t("settings.lang_auto") },
    { value: "Es", label: "Español" },
    { value: "En", label: "English" },
  ]);
</script>

<ModalSheet
  open={player.settingsOpen}
  label={t("settings.title")}
  maxWidth={520}
  onClose={() => (player.settingsOpen = false)}
>
      <div class="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]">
        <h2 class="text-[18px] font-extrabold tracking-tight">{t("settings.title")}</h2>
        <button class="icon-btn" title={t("common.close")} onclick={() => (player.settingsOpen = false)}>
          <X size={17} />
        </button>
      </div>

      <div class="p-6 space-y-5">
        <!-- Appearance & Language -->
        <section class="space-y-2.5">
          <h3 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
            {t("settings.appearance")}
          </h3>
          <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]">
            <div class="p-3.5 flex items-center justify-between gap-3">
              <div>
                <div class="text-[14px] font-semibold">{t("settings.theme")}</div>
                <div class="text-[12px] text-[var(--text-muted)]">{t("settings.theme_sub")}</div>
              </div>
              <SegmentedControl
                options={themeOptions}
                value={player.themeMode}
                ariaLabel={t("settings.theme")}
                onchange={(mode) => player.setThemeMode(mode)}
              />
            </div>

            <div class="p-3.5 flex items-center justify-between gap-3">
              <div>
                <div class="flex items-center gap-1.5 text-[14px] font-semibold">
                  <Globe size={14} class="text-[var(--accent-bright)]" />
                  {t("settings.language")}
                </div>
                <div class="text-[12px] text-[var(--text-muted)]">{t("settings.language_sub")}</div>
              </div>
              <SegmentedControl
                options={languageOptions}
                value={player.language}
                ariaLabel={t("settings.language")}
                onchange={(lang) => player.setLanguage(lang)}
              />
            </div>
          </div>
        </section>

        <!-- Playback -->
        <section class="space-y-2.5">
          <h3 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
            {t("settings.playback")}
          </h3>
          <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]">
            <div class="p-3.5 flex items-center justify-between gap-4">
              <div class="min-w-0">
                <div class="text-[14px] font-semibold">{t("settings.crossfade")}</div>
                <div class="text-[12px] text-[var(--text-muted)] numeric">
                  {player.crossfadeSecs === 0
                    ? t("settings.crossfade_off")
                    : t("settings.crossfade_secs", { secs: player.crossfadeSecs })}
                </div>
              </div>
              <input
                class="w-36 !h-1.5"
                type="range"
                min="0"
                max="10"
                step="1"
                value={player.crossfadeSecs}
                aria-label={t("settings.crossfade")}
                style="background: linear-gradient(to right, var(--accent-bright) {crossfadePct}%, var(--control-track) {crossfadePct}%);"
                oninput={(e) => player.setCrossfadeSecs(Number((e.currentTarget as HTMLInputElement).value))}
              />
            </div>

            <div class="p-3.5 flex items-center justify-between gap-4">
              <div class="min-w-0">
                <div class="text-[14px] font-semibold">{t("settings.autoplay")}</div>
                <div class="text-[12px] text-[var(--text-muted)]">{t("settings.autoplay_sub")}</div>
              </div>
              <button
                type="button"
                role="switch"
                aria-checked={player.autoplayRelated}
                aria-label={t("settings.autoplay")}
                class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.autoplayRelated
                  ? 'bg-[var(--accent)]'
                  : 'bg-[var(--control-track)]'}"
                onclick={() => player.setAutoplay(!player.autoplayRelated)}
              >
                <span
                  class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.autoplayRelated
                    ? 'translate-x-5'
                    : 'translate-x-0'}"
                ></span>
              </button>
            </div>

            <div class="p-3.5 flex items-center justify-between gap-4">
              <div class="min-w-0">
                <div class="text-[14px] font-semibold">{t("settings.background")}</div>
                <div class="text-[12px] text-[var(--text-muted)]">{t("settings.background_sub")}</div>
              </div>
              <button
                type="button"
                role="switch"
                aria-checked={player.backgroundPlayback}
                aria-label={t("settings.background")}
                class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.backgroundPlayback
                  ? 'bg-[var(--accent)]'
                  : 'bg-[var(--control-track)]'}"
                onclick={() => player.setBackgroundPlayback(!player.backgroundPlayback)}
              >
                <span
                  class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.backgroundPlayback
                    ? 'translate-x-5'
                    : 'translate-x-0'}"
                ></span>
              </button>
            </div>
          </div>
        </section>

        <!-- Storage -->
        <section class="space-y-2.5">
          <h3 class="text-[11px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
            {t("settings.storage")}
          </h3>
          <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]">
            <div class="p-3.5 flex items-center justify-between gap-4">
              <div class="min-w-0 flex-1 space-y-1.5">
                <div class="flex items-center gap-2 text-[14px] font-semibold">
                  <HardDrive size={15} class="text-[var(--accent-bright)]" />
                  {t("settings.cache")}
                </div>
                <div class="text-[12px] text-[var(--text-muted)] numeric">
                  {t("settings.cache_sub", { mb: cacheMb })}
                </div>
                <div class="h-1.5 w-full max-w-[240px] rounded-full bg-[var(--control-track)] overflow-hidden">
                  <div
                    class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-300"
                    style="width: {cachePct}%"
                  ></div>
                </div>
              </div>
              <button
                class="pill-btn hover:!bg-red-500/20 hover:!border-red-500/40 hover:!text-red-400 shrink-0"
                disabled={cacheBytes === 0}
                onclick={() => void clearCache()}
              >
                <Trash2 size={13} />
                {t("settings.clear")}
              </button>
            </div>

            <div class="p-3.5 flex items-center justify-between gap-4">
              <div class="min-w-0 flex-1 space-y-1">
                <div class="flex items-center gap-2 text-[14px] font-semibold">
                  <Download size={15} class="text-[var(--accent-bright)]" />
                  {t("settings.downloads")}
                </div>
                <div class="text-[12px] text-[var(--text-muted)] numeric">
                  {downloadedCount === 0 && downloadsBytes === 0
                    ? t("settings.downloads_empty")
                    : t("settings.downloads_sub", { mb: downloadsMb, count: downloadedCount })}
                </div>
              </div>
              <button
                class="pill-btn hover:!bg-red-500/20 hover:!border-red-500/40 hover:!text-red-400 shrink-0"
                disabled={downloadsBytes === 0 && downloadedCount === 0}
                onclick={() => void clearDownloads()}
              >
                <Trash2 size={13} />
                {t("settings.clear")}
              </button>
            </div>

            <div class="p-3.5 flex items-center justify-between gap-4">
              <div class="min-w-0">
                <div class="text-[14px] font-semibold">{t("settings.search_history")}</div>
                <div class="text-[12px] text-[var(--text-muted)] numeric">
                  {player.searchHistory.length === 1
                    ? t("settings.search_history_one")
                    : t("settings.search_history_many", { count: player.searchHistory.length })}
                </div>
              </div>
              <button
                class="pill-btn shrink-0"
                disabled={player.searchHistory.length === 0}
                onclick={() => player.clearSearchHistory()}
              >
                {t("settings.clear")}
              </button>
            </div>

            <div class="p-3.5 flex flex-wrap items-center justify-between gap-3">
              <div class="min-w-0 flex-1">
                <div class="text-[14px] font-semibold">{t("settings.backup")}</div>
                <div class="text-[12px] text-[var(--text-muted)]">{t("settings.backup_sub")}</div>
              </div>
              <div class="flex items-center gap-2 shrink-0">
                <button type="button" class="pill-btn" onclick={exportLibrary}>
                  <Download size={13} />
                  {t("settings.export")}
                </button>
                <button type="button" class="pill-btn" onclick={() => fileInputEl?.click()}>
                  <Upload size={13} />
                  {t("settings.import")}
                </button>
                <input
                  bind:this={fileInputEl}
                  type="file"
                  accept=".json,application/json"
                  class="hidden"
                  onchange={(e) => void handleImportFile(e)}
                />
              </div>
            </div>
          </div>
        </section>
      </div>
</ModalSheet>
