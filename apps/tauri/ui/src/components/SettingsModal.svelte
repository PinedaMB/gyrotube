<script lang="ts">
  import { Globe, HardDrive, Monitor, Moon, Sun, Trash2, X } from "lucide-svelte";
  import { api, isTauriRuntime } from "../lib/api";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { AppLanguage, ThemeMode } from "../lib/types";
  import ModalSheet from "./ModalSheet.svelte";

  let cacheBytes = $state(0);

  $effect(() => {
    if (player.settingsOpen && isTauriRuntime()) {
      void api.getCacheBytes().then((b) => (cacheBytes = b)).catch(() => {});
    }
  });

  let cacheMb = $derived((cacheBytes / (1024 * 1024)).toFixed(1));
  let cachePct = $derived(Math.min(100, (cacheBytes / (1024 * 1024 * 1024)) * 100));
  let crossfadePct = $derived(Math.round((player.crossfadeSecs / 10) * 100));

  async function clearCache() {
    if (!isTauriRuntime()) return;
    await api.clearAudioCache();
    cacheBytes = 0;
    player.showToast("Audio cache cleared");
  }

  const THEMES: ThemeMode[] = ["system", "dark", "light"];
  const LANGUAGES: { id: AppLanguage; label: string }[] = [
    { id: "Auto", label: "Auto" },
    { id: "Es", label: "Español" },
    { id: "En", label: "English" },
  ];
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
              <div class="inline-flex p-1 rounded-xl bg-black/20 border border-[var(--border)] gap-1 shrink-0">
                {#each THEMES as mode}
                  <button
                    type="button"
                    class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-[12px] font-semibold transition-all {player.themeMode === mode
                      ? 'bg-[var(--accent)] text-white shadow-sm'
                      : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
                    onclick={() => player.setThemeMode(mode)}
                  >
                    {#if mode === "system"}
                      <Monitor size={13} />
                      {t("settings.theme_system")}
                    {:else if mode === "dark"}
                      <Moon size={13} />
                      {t("settings.theme_dark")}
                    {:else}
                      <Sun size={13} />
                      {t("settings.theme_light")}
                    {/if}
                  </button>
                {/each}
              </div>
            </div>

            <div class="p-3.5 flex items-center justify-between gap-3">
              <div>
                <div class="flex items-center gap-1.5 text-[14px] font-semibold">
                  <Globe size={14} class="text-[var(--accent-bright)]" />
                  {t("settings.language")}
                </div>
                <div class="text-[12px] text-[var(--text-muted)]">{t("settings.language_sub")}</div>
              </div>
              <div class="inline-flex p-1 rounded-xl bg-black/20 border border-[var(--border)] gap-1 shrink-0">
                {#each LANGUAGES as lang}
                  <button
                    type="button"
                    class="px-2.5 py-1.5 rounded-lg text-[12px] font-semibold transition-all {player.language === lang.id
                      ? 'bg-[var(--accent)] text-white shadow-sm'
                      : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
                    onclick={() => player.setLanguage(lang.id)}
                  >
                    {lang.id === "Auto" ? t("settings.lang_auto") : lang.label}
                  </button>
                {/each}
              </div>
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
                style="background: linear-gradient(to right, var(--accent-bright) {crossfadePct}%, rgba(255,255,255,0.16) {crossfadePct}%);"
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
                  : 'bg-white/15'}"
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
                  : 'bg-white/15'}"
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
                <div class="h-1.5 w-full max-w-[240px] rounded-full bg-white/10 overflow-hidden">
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
          </div>
        </section>
      </div>
</ModalSheet>
