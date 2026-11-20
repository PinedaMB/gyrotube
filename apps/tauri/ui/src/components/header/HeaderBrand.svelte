<script lang="ts">
  import { PanelLeft } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
</script>

<div class="justify-self-start flex items-center gap-2 min-w-0">
  <button
    type="button"
    class="icon-btn shrink-0 hidden md:inline-flex {!player.navCollapsed
      ? 'text-[var(--text)]'
      : 'text-[var(--text-muted)]'}"
    title={t("header.toggle_sidebar")}
    onclick={() => (player.navCollapsed = !player.navCollapsed)}
  >
    <PanelLeft size={18} />
  </button>

  <button
    type="button"
    class="flex items-center gap-2 rounded-xl px-1.5 py-1 hover:bg-[var(--surface)] transition-colors active:scale-[0.98]"
    onclick={() => player.navigateTab("home")}
    title={t("header.go_home")}
  >
    <img
      src="/gyrotube.svg"
      alt=""
      class="w-6 h-6 shrink-0 drop-shadow-[0_2px_8px_rgba(33,144,164,0.45)]"
    />
    <span class="font-extrabold text-[15.5px] tracking-[-0.02em] hidden sm:inline">
      GyroTube
    </span>
    <span
      class="hidden md:inline-flex items-center px-2 py-0.5 rounded-full text-[10.5px] font-bold tracking-wide bg-[var(--accent-soft)] text-[var(--accent-bright)] border border-[var(--accent-border)]/40"
    >
      {t("brand.music")}
    </span>
  </button>
</div>
