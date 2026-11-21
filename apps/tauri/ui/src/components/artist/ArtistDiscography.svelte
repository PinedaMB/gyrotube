<script lang="ts">
  import { Film, Play } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, formatSubscribers, type ArtistDetails } from "../../lib/types";
  import Carousel from "../Carousel.svelte";
  import CoverImage from "../CoverImage.svelte";

  let { details }: { details: ArtistDetails } = $props();

  let singles = $derived(details.singles ?? []);
  let videos = $derived(details.videos ?? []);
</script>

{#if details.albums.length > 0}
  <Carousel title={t("artist.albums")}>
    {#each details.albums as album (album.id)}
      <button
        type="button"
        class="group p-2 rounded-2xl hover:bg-white/[0.06] text-left flex flex-col gap-2 transition-colors"
        onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
      >
        <CoverImage class="media-cover !rounded-xl shadow-[0_6px_18px_rgba(0,0,0,0.3)]" src={album.cover_url} alt={album.title} />
        <div class="px-0.5 min-w-0 w-full">
          <strong class="block text-[14px] font-bold truncate">{album.title}</strong>
          <span class="block text-[12.5px] text-[var(--text-muted)] truncate mt-0.5">
            {album.year ?? album.artist}
          </span>
        </div>
      </button>
    {/each}
  </Carousel>
{/if}

{#if singles.length > 0}
  <Carousel title={t("artist.singles")}>
    {#each singles as single (single.id)}
      <button
        type="button"
        class="group p-2 rounded-2xl hover:bg-white/[0.06] text-left flex flex-col gap-2 transition-colors"
        onclick={() => player.openAlbum(single.id, single.title, single.cover_url)}
      >
        <CoverImage class="media-cover !rounded-xl shadow-[0_6px_18px_rgba(0,0,0,0.3)]" src={single.cover_url} alt={single.title} />
        <div class="px-0.5 min-w-0 w-full">
          <strong class="block text-[14px] font-bold truncate">{single.title}</strong>
          <span class="block text-[12.5px] text-[var(--text-muted)] truncate mt-0.5">
            {single.year ?? single.artist}
          </span>
        </div>
      </button>
    {/each}
  </Carousel>
{/if}

{#if videos.length > 0}
  <Carousel title={t("artist.videos")} minCardWidth={240}>
    {#each videos as video, i (video.id)}
      <button
        type="button"
        class="group p-2 rounded-2xl hover:bg-white/[0.06] text-left flex flex-col gap-2 transition-colors"
        onclick={() => {
          player.replaceQueueAndPlay(videos, i, `${details.name} · ${t("artist.videos")}`);
        }}
      >
        <div class="relative w-full aspect-video rounded-xl overflow-hidden bg-black/40 border border-white/[0.08] shadow-[0_6px_18px_rgba(0,0,0,0.32)]">
          <CoverImage class="w-full h-full object-cover" src={video.cover_large_url ?? video.cover_url} alt={video.title} />
          <div class="absolute inset-0 bg-black/35 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
            <span class="w-11 h-11 rounded-full bg-[var(--accent)] text-white flex items-center justify-center shadow-lg">
              <Play size={18} fill="currentColor" class="translate-x-[1px]" />
            </span>
          </div>
          <span class="absolute top-2 left-2 inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-extrabold uppercase tracking-wider bg-black/75 text-[var(--gold-bright)] border border-white/10">
            <Film size={10} />
            VIDEO
          </span>
          {#if video.duration_secs}
            <span class="absolute bottom-2 right-2 px-1.5 py-0.5 rounded-md text-[10.5px] font-bold bg-black/75 text-white numeric">
              {formatDuration(video.duration_secs)}
            </span>
          {/if}
        </div>
        <div class="px-0.5 min-w-0 w-full">
          <strong class="block text-[14px] font-bold truncate">{video.title}</strong>
          <span class="block text-[12.5px] text-[var(--text-muted)] truncate mt-0.5">{video.artist}</span>
        </div>
      </button>
    {/each}
  </Carousel>
{/if}

{#if details.playlists.length > 0}
  <Carousel title={t("artist.playlists")}>
    {#each details.playlists as pl (pl.id)}
      <button
        type="button"
        class="group p-2 rounded-2xl hover:bg-white/[0.06] text-left flex flex-col gap-2 transition-colors"
        onclick={() => player.openAlbum(pl.id, pl.name, pl.cover_url, true)}
      >
        <CoverImage class="media-cover !rounded-xl shadow-[0_6px_18px_rgba(0,0,0,0.3)]" src={pl.cover_url} alt={pl.name} />
        <div class="px-0.5 min-w-0 w-full">
          <strong class="block text-[14px] font-bold truncate">{pl.name}</strong>
          <span class="block text-[12.5px] text-[var(--text-muted)] truncate mt-0.5">{pl.subtitle}</span>
        </div>
      </button>
    {/each}
  </Carousel>
{/if}

{#if details.similar_artists.length > 0}
  <Carousel title={t("artist.similar")} minCardWidth={154}>
    {#each details.similar_artists as sim (sim.id)}
      <button
        type="button"
        class="p-2.5 rounded-2xl flex flex-col items-center gap-2 text-center hover:bg-white/[0.06] transition-colors"
        onclick={() => player.openArtist(sim.id, sim.name)}
      >
        <CoverImage class="artist-avatar" src={sim.avatar_url} alt={sim.name} fallbackIcon="👤" />
        <strong class="text-[13.5px] font-bold truncate w-full">{sim.name}</strong>
        {#if sim.subscriber_count}
          <span class="text-[12px] text-[var(--text-muted)] -mt-1">
            {formatSubscribers(sim.subscriber_count)}
          </span>
        {/if}
      </button>
    {/each}
  </Carousel>
{/if}
