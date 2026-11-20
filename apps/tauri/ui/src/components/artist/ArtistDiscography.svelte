<script lang="ts">
  import { player } from "../../lib/stores/player.svelte";
  import { formatSubscribers, type ArtistDetails } from "../../lib/types";
  import Carousel from "../Carousel.svelte";
  import CoverImage from "../CoverImage.svelte";

  let { details }: { details: ArtistDetails } = $props();
</script>

{#if details.albums.length > 0}
  <Carousel title="Albums">
    {#each details.albums as album (album.id)}
      <button
        class="p-2.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] text-left flex flex-col gap-1.5 transition-colors"
        onclick={() => player.openAlbum(album.id, album.title, album.cover_url)}
      >
        <CoverImage class="media-cover" src={album.cover_url} alt={album.title} />
        <strong class="text-[13.5px] truncate w-full">{album.title}</strong>
        <span class="text-[12px] text-[var(--text-muted)] truncate w-full">
          {album.year ?? album.artist}
        </span>
      </button>
    {/each}
  </Carousel>
{/if}

{#if details.playlists.length > 0}
  <Carousel title="Playlists">
    {#each details.playlists as pl (pl.id)}
      <button
        class="p-2.5 rounded-2xl bg-[var(--surface)] border border-[var(--border)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] text-left flex flex-col gap-1.5 transition-colors"
        onclick={() => player.openAlbum(pl.id, pl.name, pl.cover_url, true)}
      >
        <CoverImage class="media-cover" src={pl.cover_url} alt={pl.name} />
        <strong class="text-[13.5px] truncate w-full">{pl.name}</strong>
        <span class="text-[12px] text-[var(--text-muted)] truncate w-full">{pl.subtitle}</span>
      </button>
    {/each}
  </Carousel>
{/if}

{#if details.similar_artists.length > 0}
  <Carousel title="Fans might also like" minCardWidth={144}>
    {#each details.similar_artists as sim (sim.id)}
      <button
        class="p-2.5 rounded-2xl flex flex-col items-center gap-1.5 text-center hover:bg-[var(--surface)] transition-colors"
        onclick={() => player.openArtist(sim.id, sim.name)}
      >
        <CoverImage class="artist-avatar" src={sim.avatar_url} alt={sim.name} fallbackIcon="👤" />
        <strong class="text-[13px] truncate w-full">{sim.name}</strong>
        {#if sim.subscriber_count}
          <span class="text-[11.5px] text-[var(--text-muted)]">
            {formatSubscribers(sim.subscriber_count)}
          </span>
        {/if}
      </button>
    {/each}
  </Carousel>
{/if}

{#if details.description}
  <section class="max-w-3xl rounded-2xl p-5 bg-[var(--surface)] border border-[var(--border)] space-y-2">
    <h3 class="text-[15px] font-bold">About {details.name}</h3>
    <p class="text-[13.5px] leading-relaxed text-[var(--text-muted)]">{details.description}</p>
  </section>
{/if}
