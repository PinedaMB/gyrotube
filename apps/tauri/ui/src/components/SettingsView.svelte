<script lang="ts">
  import {
    ArrowLeft,
    AudioLines,
    Download,
    FolderArchive,
    Globe,
    HardDrive,
    Heart,
    History,
    Info,
    Monitor,
    Moon,
    Music2,
    Palette,
    Radio,
    ShieldCheck,
    Sliders,
    Sparkles,
    Sun,
    SunMoon,
    Trash2,
    Upload,
    Volume2,
  } from "lucide-svelte";
  import { api, isTauriRuntime } from "../lib/api";
  import { EQ_LABELS, type EqPresetId } from "../lib/equalizer";
  import { t } from "../lib/i18n.svelte";
  import { device } from "../lib/stores/device.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { AppLanguage, AudioQuality, ThemeMode } from "../lib/types";
  import SegmentedControl, { type SegmentedOption } from "./SegmentedControl.svelte";

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

  let qualityOptions = $derived<SegmentedOption<AudioQuality>[]>([
    { value: "High", label: t("settings.quality_high") },
    { value: "Normal", label: t("settings.quality_normal") },
    { value: "Low", label: t("settings.quality_low") },
  ]);

  const eqPresetList: { id: Exclude<EqPresetId, "custom">; key: Parameters<typeof t>[0] }[] = [
    { id: "flat", key: "eq.preset_flat" },
    { id: "bass", key: "eq.preset_bass" },
    { id: "vocal", key: "eq.preset_vocal" },
    { id: "rock", key: "eq.preset_rock" },
    { id: "pop", key: "eq.preset_pop" },
    { id: "electronic", key: "eq.preset_electronic" },
    { id: "acoustic", key: "eq.preset_acoustic" },
  ];
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
      GyroTube v1.0.0
    </span>
  </header>

  <!-- Appearance & Language -->
  <section class="space-y-2.5">
    <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
      <Palette size={12} class="text-[var(--accent-bright)]" />
      {t("settings.appearance")}
    </h2>
    <div
      class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)] overflow-hidden"
      style="box-shadow: var(--card-shadow);"
    >
      <div class="p-3.5 sm:p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <SunMoon size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.theme")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.theme_sub")}</div>
          </div>
        </div>
        <div class="flex sm:justify-end">
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
            <Globe size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.language")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.language_sub")}</div>
          </div>
        </div>
        <div class="flex sm:justify-end">
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
      class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)] overflow-hidden"
      style="box-shadow: var(--card-shadow);"
    >
      <div class="p-3.5 sm:p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Music2 size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.audio_quality")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.audio_quality_sub")}</div>
          </div>
        </div>
        <div class="flex sm:justify-end">
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
  <section class="space-y-2.5">
    <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
      <Sliders size={12} class="text-[var(--accent-bright)]" />
      {t("eq.title")}
    </h2>
    <div
      class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)] overflow-hidden"
      style="box-shadow: var(--card-shadow);"
    >
      <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Sliders size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("eq.enable")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("eq.enable_sub")}</div>
          </div>
        </div>
        <button
          type="button"
          role="switch"
          aria-checked={player.eqSettings.enabled}
          aria-label={t("eq.enable")}
          class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.eqSettings.enabled
            ? 'bg-[var(--accent)]'
            : 'bg-[var(--control-track)]'}"
          onclick={() => player.setEqEnabled(!player.eqSettings.enabled)}
        >
          <span
            class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.eqSettings.enabled
              ? 'translate-x-5'
              : 'translate-x-0'}"
          ></span>
        </button>
      </div>

      {#if player.eqSettings.enabled}
        <div class="p-4 space-y-4 bg-[var(--bg)]/25">
          <div class="flex flex-wrap gap-1.5">
            {#each eqPresetList as preset (preset.id)}
              <button
                type="button"
                class="px-3 py-1 rounded-full text-[12px] font-semibold border transition-colors {player.eqSettings.preset === preset.id
                  ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] border-[var(--accent-border)]'
                  : 'bg-[var(--surface)] border-[var(--border)] text-[var(--text-muted)] hover:text-[var(--text)]'}"
                onclick={() => player.setEqPreset(preset.id)}
              >
                {t(preset.key)}
              </button>
            {/each}
          </div>

          <div class="space-y-2.5">
            {#each EQ_LABELS as label, idx (label)}
              {@const db = player.eqSettings.bands[idx]}
              {@const pct = Math.round(((db + 12) / 24) * 100)}
              <div class="flex items-center gap-3">
                <span class="w-14 text-[12px] font-semibold text-[var(--text-muted)] numeric shrink-0">
                  {label}
                </span>
                <input
                  type="range"
                  min="-12"
                  max="12"
                  step="1"
                  value={db}
                  aria-label={label}
                  class="flex-1 !h-1.5"
                  style="background: linear-gradient(to right, var(--accent-bright) {pct}%, var(--control-track) {pct}%);"
                  oninput={(e) => player.setEqBand(idx, Number((e.currentTarget as HTMLInputElement).value))}
                />
                <span class="w-12 text-right text-[12px] font-bold numeric shrink-0 {db !== 0 ? 'text-[var(--accent-bright)]' : 'text-[var(--text-muted)]'}">
                  {db > 0 ? `+${db}` : db} dB
                </span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Volume2 size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("eq.normalize")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("eq.normalize_sub")}</div>
          </div>
        </div>
        <button
          type="button"
          role="switch"
          aria-checked={player.eqSettings.normalize}
          aria-label={t("eq.normalize")}
          class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.eqSettings.normalize
            ? 'bg-[var(--accent)]'
            : 'bg-[var(--control-track)]'}"
          onclick={() => player.setEqNormalize(!player.eqSettings.normalize)}
        >
          <span
            class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.eqSettings.normalize
              ? 'translate-x-5'
              : 'translate-x-0'}"
          ></span>
        </button>
      </div>
    </div>
  </section>

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
  <section class="space-y-2.5">
    <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
      <Info size={12} class="text-[var(--accent-bright)]" />
      {t("settings.about")}
    </h2>
    <div
      class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)] overflow-hidden"
      style="box-shadow: var(--card-shadow);"
    >
      <div class="p-4 flex items-center gap-3.5">
        <img
          src="/gyrotube.svg"
          alt="GyroTube"
          class="w-12 h-12 shrink-0 drop-shadow-[0_4px_12px_rgba(16,185,129,0.3)]"
        />
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2 flex-wrap">
            <span class="text-[16px] font-extrabold tracking-tight">GyroTube</span>
            <span class="px-2 py-0.5 rounded-full text-[10.5px] font-bold numeric bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/40">
              v1.0.0
            </span>
          </div>
          <p class="text-[12.5px] text-[var(--text-muted)] mt-0.5 leading-snug">
            {t("settings.about_desc")}
          </p>
        </div>
      </div>

      <div class="px-4 py-3 flex items-center justify-between gap-4 text-[13px]">
        <span class="text-[var(--text-muted)] font-medium">{t("settings.about_engine")}</span>
        <span class="font-semibold text-right">{t("settings.about_engine_val")}</span>
      </div>

      <div class="px-4 py-3 flex items-center justify-between gap-4 text-[13px]">
        <span class="text-[var(--text-muted)] font-medium inline-flex items-center gap-1.5">
          <ShieldCheck size={14} class="text-[var(--accent-bright)]" />
          {t("settings.about_privacy")}
        </span>
        <span class="font-semibold text-right">{t("settings.about_privacy_val")}</span>
      </div>

      <div class="px-4 py-2.5 flex items-center justify-between gap-3 bg-[var(--bg)]/25">
        <p class="text-[12px] text-[var(--text-muted)] leading-snug inline-flex items-center gap-1.5">
          <span>Para <span class="font-semibold text-[var(--text)]/90">Naomi Alejandra</span></span>
          <Heart size={12} class="text-pink-400 fill-pink-400 shrink-0" />
        </p>
        <img
          src="/dedication.webp"
          alt="Naomi Alejandra"
          class="w-10 h-10 object-contain shrink-0 opacity-90 hover:opacity-100 hover:scale-105 transition-all select-none"
          loading="lazy"
          decoding="async"
        />
      </div>
    </div>
  </section>
</div>
