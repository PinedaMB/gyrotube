<script lang="ts">
  import { Compass, Disc3, Heart, TrendingUp } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
</script>

<section class="grid grid-cols-2 lg:grid-cols-4 gap-2.5 mb-6 max-md:mb-5">
  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2.5 max-md:px-3 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(225, 29, 72, 0.11), var(--card-bg) 72%); box-shadow: var(--card-shadow);"
    onclick={() => document.getElementById("new-releases")?.scrollIntoView({ behavior: "smooth" })}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center shrink-0" style="background: rgba(225, 29, 72, 0.13); color: #e11d48;">
      <Disc3 size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.new_releases")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.new_releases_sub")}</small>
    </span>
  </button>

  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2.5 max-md:px-3 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(37, 99, 235, 0.11), var(--card-bg) 72%); box-shadow: var(--card-shadow);"
    onclick={() => document.getElementById("trending-hits")?.scrollIntoView({ behavior: "smooth" })}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center shrink-0" style="background: rgba(37, 99, 235, 0.13); color: #2563eb;">
      <TrendingUp size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.top_charts")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.top_charts_sub")}</small>
    </span>
  </button>

  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2.5 max-md:px-3 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(147, 51, 234, 0.11), var(--card-bg) 72%); box-shadow: var(--card-shadow);"
    onclick={() => {
      player.clearSearch();
      player.navigateTab("explore");
    }}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center shrink-0" style="background: rgba(147, 51, 234, 0.13); color: #9333ea;">
      <Compass size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.moods")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.moods_sub")}</small>
    </span>
  </button>

  <button
    class="flex items-center gap-3 px-3.5 py-3 max-md:min-h-14 max-md:gap-2.5 max-md:px-3 max-md:py-2 rounded-2xl border border-[var(--border)] hover:border-[var(--border-strong)] text-left transition-all duration-150 hover:-translate-y-0.5 active:scale-[0.98]"
    style="background: linear-gradient(135deg, rgba(16, 185, 129, 0.12), var(--card-bg) 72%); box-shadow: var(--card-shadow);"
    onclick={() => player.navigateTab("collection")}
  >
    <span class="w-9 h-9 max-md:w-8 max-md:h-8 rounded-xl inline-flex items-center justify-center shrink-0" style="background: var(--accent-soft); color: var(--accent);">
      <Heart size={18} />
    </span>
    <span class="min-w-0 flex flex-col gap-0.5">
      <strong class="text-[14px] truncate max-md:whitespace-normal max-md:text-[13px] max-md:leading-tight">{t("home.collection")}</strong>
      <small class="text-[11.5px] text-[var(--text-muted)] truncate max-md:hidden">{t("home.collection_sub")}</small>
    </span>
  </button>
</section>
