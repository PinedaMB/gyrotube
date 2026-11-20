<script lang="ts">
  import { Compass, Disc3, Heart, TrendingUp } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
</script>

<section class="grid grid-cols-2 lg:grid-cols-4 gap-2.5 mb-6 max-md:mb-5">
  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2 max-md:px-2.5 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(224, 27, 36, 0.16), var(--surface) 70%);"
    onclick={() => document.getElementById("new-releases")?.scrollIntoView({ behavior: "smooth" })}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0">
      <Disc3 size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.new_releases")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.new_releases_sub")}</small>
    </span>
  </button>

  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2 max-md:px-2.5 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(53, 132, 228, 0.16), var(--surface) 70%);"
    onclick={() => document.getElementById("trending-hits")?.scrollIntoView({ behavior: "smooth" })}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0">
      <TrendingUp size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.top_charts")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.top_charts_sub")}</small>
    </span>
  </button>

  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2 max-md:px-2.5 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(192, 97, 203, 0.16), var(--surface) 70%);"
    onclick={() => {
      player.clearSearch();
      player.navigateTab("explore");
    }}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0">
      <Compass size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.moods")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.moods_sub")}</small>
    </span>
  </button>

  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2 max-md:px-2.5 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(46, 194, 126, 0.16), var(--surface) 70%);"
    onclick={() => player.navigateTab("collection")}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center bg-[var(--surface-hover)] shrink-0">
      <Heart size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.collection")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.collection_sub")}</small>
    </span>
  </button>
</section>
