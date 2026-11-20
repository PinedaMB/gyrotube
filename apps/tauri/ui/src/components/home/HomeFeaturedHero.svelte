<script lang="ts">
  import { Play } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Album } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  let { album }: { album: Album } = $props();
</script>

<section class="mt-6">
  <div
    class="flex flex-col sm:flex-row items-start sm:items-center gap-6 p-5 sm:px-6 rounded-[18px] border border-[var(--border)] shadow-[0_12px_32px_rgba(0,0,0,0.28)]"
    style="background: linear-gradient(135deg, rgba(var(--ambient-rgb), 0.24), var(--surface) 65%);"
  >
    <CoverImage class="hero-cover" src={album.cover_url} alt={album.title} />
    <div class="min-w-0 flex flex-col gap-1.5">
      <span
        class="self-start text-[11px] font-extrabold tracking-[0.08em] px-2.5 py-0.5 rounded-full bg-[var(--accent-soft)] text-[var(--accent-bright)]"
      >
        {t("home.featured")}
      </span>
      <h2 class="text-[24px] font-extrabold tracking-tight leading-tight">{album.title}</h2>
      <p class="text-[14px] text-[var(--text-muted)]">{album.artist}</p>
      <div class="flex items-center gap-2.5 mt-1.5">
        <button
          class="pill-btn primary"
          onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
        >
          <Play size={14} fill="currentColor" />
          {t("home.play")}
        </button>
        <button
          class="pill-btn"
          onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
        >
          {t("home.open_album")}
        </button>
      </div>
    </div>
  </div>
</section>
