<script lang="ts">
  import { Play } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Album } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  let { album }: { album: Album } = $props();
</script>

<section class="mt-5 md:mt-6">
  <div
    class="flex items-center gap-3 p-3 md:gap-6 md:py-5 md:px-6 rounded-[18px] border border-[var(--border)] md:shadow-[0_12px_32px_rgba(0,0,0,0.28)]"
    style="background: linear-gradient(135deg, rgba(var(--ambient-rgb), 0.24), var(--surface) 65%);"
  >
    <CoverImage class="hero-cover max-md:!w-22 max-md:!h-22" src={album.cover_url} alt={album.title} />
    <div class="min-w-0 flex flex-col gap-1.5">
      <span
        class="self-start text-[11px] font-extrabold tracking-[0.08em] px-2.5 py-0.5 rounded-full bg-[var(--accent-soft)] text-[var(--accent-bright)]"
      >
        {t("home.featured")}
      </span>
      <h2 class="text-[18px] md:text-[24px] font-extrabold tracking-tight leading-tight max-md:line-clamp-2">{album.title}</h2>
      <p class="text-[13px] md:text-[14px] text-[var(--text-muted)] max-md:truncate">{album.artist}</p>
      <div class="flex items-center gap-2.5 mt-1.5">
        <button
          class="pill-btn primary max-md:!hidden"
          onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
        >
          <Play size={14} fill="currentColor" />
          {t("home.play")}
        </button>
        <button
          class="pill-btn max-md:min-h-11"
          onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
        >
          {t("home.open_album")}
        </button>
      </div>
    </div>
  </div>
</section>
