<script lang="ts">
  import {
    ArrowLeft,
    AudioLines,
    Check,
    ChevronDown,
    Download,
    FastForward,
    FolderArchive,
    Globe,
    HardDrive,
    History,
    Monitor,
    Moon,
    Music2,
    Palette,
    Radio,
    Sparkles,
    Sun,
    SunMoon,
    Trash2,
    Upload,
  } from "lucide-svelte";
  import { getAndroidBridge } from "../lib/androidBridge";
  import { api, isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { device } from "../lib/stores/device.svelte";
  import { player } from "../lib/stores/player.svelte";
  import { updater } from "../lib/stores/updater.svelte";
  import type { AppLanguage, AudioQuality, ThemeMode } from "../lib/types";
  import SegmentedControl, { type SegmentedOption } from "./SegmentedControl.svelte";
  import SettingsAboutSection from "./settings/SettingsAboutSection.svelte";
  import SettingsEqSection from "./settings/SettingsEqSection.svelte";
  import SettingsIntegrationsSection from "./settings/SettingsIntegrationsSection.svelte";

  let cacheBytes = $state(0);
  let downloadsBytes = $state(0);
  let fileInputEl = $state<HTMLInputElement | null>(null);

  $effect(() => {
    if (player.activeView.kind === "settings" && isTauriRuntime()) {
      void api.getCacheBytes().then((b) => (cacheBytes = b)).catch(() => {});
      void api.getDownloadsBytes().then((b) => (downloadsBytes = b)).catch(() => {});
    }
  });

  let cacheMb = $derived((cacheBytes / (1024 * 1024)).toFixed(1));
  let downloadsMb = $derived((downloadsBytes / (1024 * 1024)).toFixed(1));
  let downloadedCount = $derived(player.downloadedTrackIds.size);
  let downloadsSongLabel = $derived(
    downloadedCount === 1
      ? t("collection.song_one")
      : t("collection.song_many", { count: downloadedCount }),
  );
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

  async function exportLibrary() {
    try {
      const json = player.exportLibraryJson();
      const date = new Date().toISOString().slice(0, 10);
      const filename = `gyro-library-${date}.json`;

      // 1. Android native bridge -> saves directly to Downloads/Gyro via MediaStore
      const androidBridge = getAndroidBridge();
      if (device.isAndroid && androidBridge?.saveBackupJson) {
        const res = androidBridge.saveBackupJson(filename, json);
        if (res === "ok") {
          player.showToast(t("settings.export_done"));
          return;
        }
        throw new Error(res.replace(/^error:/, "") || "Backup export failed");
      }

      // 2. Desktop Tauri native command -> saves directly to Downloads/
      if (isTauriRuntime() && !device.isAndroid) {
        await api.saveBackupJson(filename, json);
        player.showToast(t("settings.export_done"));
        return;
      }

      // 3. Browser fallback
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
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

  let qualityOptions = $derived<SegmentedOption<AudioQuality>[]>([
    { value: "High", label: t("settings.quality_high") },
    { value: "Normal", label: t("settings.quality_normal") },
    { value: "Low", label: t("settings.quality_low") },
  ]);

  const PALETTE_SWATCHES = [
    { id: "gyro", labelKey: "settings.palette_gyro", colors: ["#131218", "#1e1c26", "#10b981"] },
    { id: "amoled", labelKey: "settings.palette_amoled", colors: ["#000000", "#101014", "#10b981"] },
    { id: "catppuccin", labelKey: "settings.palette_catppuccin", colors: ["#1e1e2e", "#28283d", "#cba6f7"] },
    { id: "nord", labelKey: "settings.palette_nord", colors: ["#2e3440", "#3b4252", "#88c0d0"] },
    { id: "rose-pine", labelKey: "settings.palette_rose_pine", colors: ["#1b131a", "#f8b4cc", "#f48fb1"] },
    { id: "tokyo-night", labelKey: "settings.palette_tokyo_night", colors: ["#1a1b26", "#24283b", "#7aa2f7"] },
    { id: "gruvbox", labelKey: "settings.palette_gruvbox", colors: ["#282828", "#3c3836", "#fe8019"] },
  ] as const;

  let paletteMenuOpen = $state(false);
  let paletteMenuEl = $state<HTMLDivElement | null>(null);
  let activePaletteSwatch = $derived(
    PALETTE_SWATCHES.find((s) => s.id === player.themePalette) ?? PALETTE_SWATCHES[0],
  );

  $effect(() => {
    if (!paletteMenuOpen) return;
    const onPointerDown = (e: PointerEvent) => {
      if (paletteMenuEl && !paletteMenuEl.contains(e.target as Node)) {
        paletteMenuOpen = false;
      }
    };
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") paletteMenuOpen = false;
    };
    window.addEventListener("pointerdown", onPointerDown);
    window.addEventListener("keydown", onKeyDown);
    return () => {
      window.removeEventListener("pointerdown", onPointerDown);
      window.removeEventListener("keydown", onKeyDown);
    };
  });
</script>

<div class="page-clamp !max-w-[780px] space-y-6">
  <!-- Header -->
  <header class="flex items-center justify-between gap-3">
    <div class="flex items-center gap-3 min-w-0">
      <button
        type="button"
        class="w-9 h-9 rounded-full inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-all active:scale-95 shrink-0"
        title={t("nav.back")}
        aria-label={t("nav.back")}
        onclick={() => player.goBack()}
      >
        <ArrowLeft size={16} />
      </button>
      <div class="min-w-0">
        <h1 class="text-[24px] sm:text-[28px] font-extrabold tracking-[-0.025em] leading-tight">
          {t("settings.title")}
        </h1>
      </div>
    </div>
    <span class="px-2.5 py-1 rounded-full text-[11px] font-bold numeric bg-[var(--surface)] border border-[var(--border)] text-[var(--text-muted)] shrink-0">
      Gyro v{updater.currentVersion}
    </span>
  </header>

  <!-- Appearance & Language -->
  <section class="space-y-2.5">
    <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
      <Palette size={12} class="text-[var(--accent-bright)]" />
      {t("settings.appearance")}
    </h2>
    <div
      class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]"
      style="box-shadow: var(--card-shadow);"
    >
      <div class="p-3.5 sm:p-4 rounded-t-2xl flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <SunMoon size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.theme")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.theme_sub")}</div>
          </div>
        </div>
        <div class="flex sm:justify-end w-full sm:w-auto [&>*]:w-full sm:[&>*]:w-auto [&_button]:flex-1 sm:[&_button]:flex-initial">
          <SegmentedControl
            options={themeOptions}
            value={player.themeMode}
            ariaLabel={t("settings.theme")}
            onchange={(mode) => player.setThemeMode(mode)}
          />
        </div>
      </div>

      <div class="p-3.5 sm:p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Palette size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.palette")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.palette_sub")}</div>
          </div>
        </div>
        <div class="relative w-full sm:w-52 shrink-0" bind:this={paletteMenuEl}>
          <button
            type="button"
            class="w-full flex items-center justify-between gap-2.5 px-3 py-2 rounded-xl bg-[var(--bg)]/65 hover:bg-[var(--surface-hover)] border transition-colors text-left {paletteMenuOpen
              ? 'border-[var(--accent-border)]'
              : 'border-[var(--border)]'}"
            aria-haspopup="listbox"
            aria-expanded={paletteMenuOpen}
            aria-label={t("settings.palette")}
            onclick={() => (paletteMenuOpen = !paletteMenuOpen)}
          >
            <span class="flex items-center gap-2.5 min-w-0">
              <span class="flex items-center -space-x-1 shrink-0" aria-hidden="true">
                {#each activePaletteSwatch.colors as col}
                  <span
                    class="w-3.5 h-3.5 rounded-full border border-white/25"
                    style="background: {col};"
                  ></span>
                {/each}
              </span>
              <span class="text-[13px] font-semibold text-[var(--text)] truncate">
                {t(activePaletteSwatch.labelKey)}
              </span>
            </span>
            <ChevronDown
              size={15}
              class="text-[var(--text-muted)] shrink-0 transition-transform duration-150 {paletteMenuOpen ? 'rotate-180' : ''}"
            />
          </button>

          {#if paletteMenuOpen}
            <div
              role="listbox"
              aria-label={t("settings.palette")}
              class="absolute right-0 left-0 mt-1.5 z-30 rounded-xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] p-1.5 shadow-[0_14px_34px_rgba(0,0,0,0.42)] space-y-0.5"
            >
              {#each PALETTE_SWATCHES as sw (sw.id)}
                {@const active = player.themePalette === sw.id}
                <button
                  type="button"
                  role="option"
                  aria-selected={active}
                  class="w-full flex items-center justify-between gap-2.5 px-2.5 py-2 rounded-lg text-left transition-colors {active
                    ? 'bg-[var(--accent-soft)] text-[var(--text)]'
                    : 'hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)]'}"
                  onclick={() => {
                    player.setThemePalette(sw.id);
                    paletteMenuOpen = false;
                  }}
                >
                  <span class="flex items-center gap-2.5 min-w-0">
                    <span class="flex items-center -space-x-1 shrink-0" aria-hidden="true">
                      {#each sw.colors as col}
                        <span
                          class="w-3.5 h-3.5 rounded-full border border-white/25"
                          style="background: {col};"
                        ></span>
                      {/each}
                    </span>
                    <span class="text-[12.5px] font-semibold truncate">{t(sw.labelKey)}</span>
                  </span>
                  {#if active}
                    <Check size={14} class="text-[var(--accent-bright)] shrink-0" />
                  {/if}
                </button>
              {/each}
            </div>
          {/if}
        </div>
      </div>

      <div class="p-3.5 sm:p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Globe size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.language")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.language_sub")}</div>
          </div>
        </div>
        <div class="flex sm:justify-end w-full sm:w-auto [&>*]:w-full sm:[&>*]:w-auto [&_button]:flex-1 sm:[&_button]:flex-initial">
          <SegmentedControl
            options={languageOptions}
            value={player.language}
            ariaLabel={t("settings.language")}
            onchange={(lang) => player.setLanguage(lang)}
          />
        </div>
      </div>
    </div>
  </section>

  <!-- Playback -->
  <section class="space-y-2.5">
    <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
      <Music2 size={12} class="text-[var(--accent-bright)]" />
      {t("settings.playback")}
    </h2>
    <div
      class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]"
      style="box-shadow: var(--card-shadow);"
    >
      <div class="p-3.5 sm:p-4 rounded-t-2xl flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Music2 size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.audio_quality")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.audio_quality_sub")}</div>
          </div>
        </div>
        <div class="flex sm:justify-end w-full sm:w-auto [&>*]:w-full sm:[&>*]:w-auto [&_button]:flex-1 sm:[&_button]:flex-initial">
          <SegmentedControl
            options={qualityOptions}
            value={player.audioQuality}
            ariaLabel={t("settings.audio_quality")}
            onchange={(q) => player.setAudioQuality(q)}
          />
        </div>
      </div>

      <div class="p-4 flex flex-wrap sm:flex-nowrap items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <AudioLines size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.crossfade")}</div>
            <div class="text-[12px] text-[var(--text-muted)] numeric">
              {player.crossfadeSecs === 0
                ? t("settings.crossfade_off")
                : t("settings.crossfade_secs", { secs: player.crossfadeSecs })}
            </div>
          </div>
        </div>
        <input
          class="w-full sm:w-44 !h-1.5 shrink-0"
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

      <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Sparkles size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.autoplay")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.autoplay_sub")}</div>
          </div>
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

      <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <FastForward size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.sponsorblock")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.sponsorblock_sub")}</div>
          </div>
        </div>
        <button
          type="button"
          role="switch"
          aria-checked={player.sponsorBlockEnabled}
          aria-label={t("settings.sponsorblock")}
          class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.sponsorBlockEnabled
            ? 'bg-[var(--accent)]'
            : 'bg-[var(--control-track)]'}"
          onclick={() => player.setSponsorBlockEnabled(!player.sponsorBlockEnabled)}
        >
          <span
            class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.sponsorBlockEnabled
              ? 'translate-x-5'
              : 'translate-x-0'}"
          ></span>
        </button>
      </div>

      {#if !device.isAndroid}
        <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
          <div class="flex items-center gap-3 min-w-0">
            <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
              <Radio size={16} />
            </span>
            <div class="min-w-0">
              <div class="text-[14px] font-semibold leading-snug">{t("settings.background")}</div>
              <div class="text-[12px] text-[var(--text-muted)]">{t("settings.background_sub")}</div>
            </div>
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
      {/if}
    </div>
  </section>

  <!-- Equalizer & Normalization -->
  <SettingsEqSection />

  <!-- Opt-in Integrations (Discord, ListenBrainz, Last.fm) -->
  <SettingsIntegrationsSection />

  <!-- Storage & Backup -->
  <section class="space-y-2.5">
    <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
      <HardDrive size={12} class="text-[var(--accent-bright)]" />
      {t("settings.storage")}
    </h2>
    <div
      class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)] overflow-hidden"
      style="box-shadow: var(--card-shadow);"
    >
      <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0 flex-1">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <HardDrive size={16} />
          </span>
          <div class="min-w-0 flex-1 space-y-1.5">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.cache")}</div>
            <div class="text-[12px] text-[var(--text-muted)] numeric">
              {t("settings.cache_sub", { mb: cacheMb })}
            </div>
            <div class="h-1.5 w-full max-w-[260px] rounded-full bg-[var(--control-track)] overflow-hidden">
              <div
                class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-300"
                style="width: {cachePct}%"
              ></div>
            </div>
          </div>
        </div>
        <button
          type="button"
          class="pill-btn hover:!bg-red-500/20 hover:!border-red-500/40 hover:!text-red-400 shrink-0"
          disabled={cacheBytes === 0}
          onclick={() => void clearCache()}
        >
          <Trash2 size={13} />
          {t("settings.clear")}
        </button>
      </div>

      <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0 flex-1">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Download size={16} />
          </span>
          <div class="min-w-0 flex-1">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.downloads")}</div>
            <div class="text-[12px] text-[var(--text-muted)] numeric">
              {downloadedCount === 0 && downloadsBytes === 0
                ? t("settings.downloads_empty")
                : `${downloadsMb} MB · ${downloadsSongLabel}`}
            </div>
          </div>
        </div>
        <button
          type="button"
          class="pill-btn hover:!bg-red-500/20 hover:!border-red-500/40 hover:!text-red-400 shrink-0"
          disabled={downloadsBytes === 0 && downloadedCount === 0}
          onclick={() => void clearDownloads()}
        >
          <Trash2 size={13} />
          {t("settings.clear")}
        </button>
      </div>

      <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <History size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.search_history")}</div>
            <div class="text-[12px] text-[var(--text-muted)] numeric">
              {player.recentSearches.length === 1
                ? t("settings.search_history_one")
                : t("settings.search_history_many", { count: player.recentSearches.length })}
            </div>
          </div>
        </div>
        <button
          type="button"
          class="pill-btn shrink-0"
          disabled={player.recentSearches.length === 0 && player.searchHistory.length === 0}
          onclick={() => player.clearSearchHistory()}
        >
          <Trash2 size={13} />
          {t("settings.clear")}
        </button>
      </div>

      <div class="p-3.5 sm:p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0 flex-1">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <FolderArchive size={16} />
          </span>
          <div class="min-w-0 flex-1">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.backup")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.backup_sub")}</div>
          </div>
        </div>
        <div class="flex items-center gap-2 w-full sm:w-auto shrink-0">
          <button type="button" class="pill-btn flex-1 sm:flex-none justify-center" onclick={exportLibrary}>
            <Download size={13} />
            {t("settings.export")}
          </button>
          <button type="button" class="pill-btn flex-1 sm:flex-none justify-center" onclick={() => fileInputEl?.click()}>
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

  <!-- About -->
  <SettingsAboutSection />
</div>
