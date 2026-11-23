<script lang="ts">
  import { Compass, Home, Library } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";

  let isSettings = $derived(player.activeView.kind === "settings");
</script>

<nav
  class="md:hidden relative z-30 shrink-0 min-h-14 grid grid-cols-3 bg-[var(--bg-elevated)] border-t border-[var(--border)] px-2"
  style="padding-bottom: env(safe-area-inset-bottom);"
  aria-label="Mobile navigation"
>
  <button
    class="flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold {!isSettings && player.activeTab === 'home'
      ? 'text-[var(--accent-bright)]'
      : 'text-[var(--text-muted)]'}"
    onclick={() => player.navigateTab("home")}
  >
    <Home size={19} />
    <span>{t("nav.home")}</span>
  </button>
  <button
    class="flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold {!isSettings && player.activeTab === 'explore'
      ? 'text-[var(--accent-bright)]'
      : 'text-[var(--text-muted)]'}"
    onclick={() => player.navigateTab("explore")}
  >
    <Compass size={19} />
    <span>{t("nav.explore")}</span>
  </button>
  <button
    class="flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold {!isSettings && player.activeTab === 'collection'
      ? 'text-[var(--accent-bright)]'
      : 'text-[var(--text-muted)]'}"
    onclick={() => player.navigateTab("collection")}
  >
    <Library size={19} />
    <span>{t("nav.collection")}</span>
  </button>
</nav>
