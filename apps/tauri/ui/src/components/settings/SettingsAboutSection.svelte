<script lang="ts">
  import { Check, Download, ExternalLink, Heart, Info, RefreshCw, ShieldCheck } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { updater } from "../../lib/stores/updater.svelte";
</script>

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
            v{updater.currentVersion}
          </span>
        </div>
        <p class="text-[12.5px] text-[var(--text-muted)] mt-0.5 leading-snug">
          {t("settings.about_desc")}
        </p>
      </div>
    </div>

    <!-- Updates row -->
    <div class="px-4 py-3 space-y-2">
      <div class="flex items-center justify-between gap-3 text-[13px]">
        <div class="min-w-0">
          <div class="font-medium text-[var(--text)]">{t("updater.label")}</div>
          <div class="text-[12px] text-[var(--text-muted)] mt-0.5">
            {#if updater.status === "checking"}
              {t("updater.checking")}
            {:else if updater.status === "installing"}
              {updater.progressPct !== null
                ? t("updater.installing", { pct: updater.progressPct })
                : t("updater.installing_indeterminate")}
            {:else if updater.status === "available" && updater.latestVersion}
              <span class="text-[var(--accent-bright)] font-semibold">
                {t("updater.available", { version: updater.latestVersion })}
              </span>
            {:else if updater.status === "up_to_date"}
              <span class="inline-flex items-center gap-1 text-[var(--accent-bright)]">
                <Check size={12} />
                {t("updater.up_to_date")}
              </span>
            {:else if updater.status === "error"}
              <span class="text-rose-400">{t("updater.error")}</span>
            {:else}
              v{updater.currentVersion}
            {/if}
          </div>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          {#if updater.status === "available" && updater.latestVersion}
            <button
              type="button"
              class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12px] font-bold bg-[var(--accent)] hover:bg-[var(--accent-bright)] text-white transition-colors active:scale-95"
              onclick={() => void updater.installOrDownload()}
            >
              {#if updater.canAutoInstall}
                <Download size={13} />
                <span>{t("updater.install_btn")}</span>
              {:else}
                <ExternalLink size={13} />
                <span>{t("updater.download_btn", { version: updater.latestVersion })}</span>
              {/if}
            </button>
          {/if}

          <button
            type="button"
            disabled={updater.status === "checking" || updater.status === "installing"}
            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[12px] font-semibold bg-[var(--bg)]/60 hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-colors disabled:opacity-50"
            onclick={() => void updater.check(false)}
          >
            <RefreshCw size={12} class={updater.status === "checking" ? "animate-spin" : ""} />
            <span>{t("updater.check_btn")}</span>
          </button>
        </div>
      </div>

      {#if updater.status === "installing"}
        <div class="h-1.5 rounded-full bg-[var(--border)] overflow-hidden">
          <div
            class="h-full rounded-full bg-[var(--accent-bright)] transition-[width] duration-200"
            style="width: {updater.progressPct ?? 35}%;"
          ></div>
        </div>
      {/if}
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
