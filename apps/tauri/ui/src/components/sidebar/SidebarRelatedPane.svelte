<script lang="ts">
  import { Play, Plus, Sparkles } from "lucide-svelte";
  import { untrack } from "svelte";
  import { fade } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";

  $effect(() => {
    const id = player.currentTrack?.id;
    if (id) {
      untrack(() => {
        void player.loadRelatedForCurrent();
      });
    }
  });

  function closeMobileSheet() {
    if (typeof window !== "undefined" && window.innerWidth < 1024) {
      player.sidebarOpen = false;
    }
  }
</script>

<div in:fade={{ duration: 150 }} class="flex-1 flex flex-col gap-5 py-2">
  {#if !player.currentTrack}
    <div class="flex-1 flex flex-col items-center justify-center text-center px-4 gap-2 text-[var(--text-muted)]">
      <Sparkles size={26} class="opacity-50" />
      <p class="text-[13.5px]">{t("related.empty")}</p>
    </div>
  {:else if player.relatedLoading}
    <div class="space-y-3 animate-pulse pt-2" aria-busy="true">
      {#each [0, 1, 2, 3, 4] as _}
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-[8px] bg-[var(--skeleton)] shrink-0"></div>
          <div class="flex-1 space-y-1.5">
            <div class="h-3.5 w-36 rounded bg-[var(--skeleton)]"></div>
            <div class="h-3 w-24 rounded bg-[var(--skeleton-soft)]"></div>
          </div>
        </div>
      {/each}
    </div>
  {:else if player.relatedFeed}
    {@const feed = player.relatedFeed}

    {#if feed.tracks.length > 0}
      <section class="space-y-2">
        <h3 class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
          {t("related.tracks")}
        </h3>
        <div class="flex flex-col">
          {#each feed.tracks as track, i (track.id + ":" + i)}
            <div
              class="group flex items-center gap-2.5 py-2 px-2 rounded-xl hover:bg-white/[0.06] cursor-pointer transition-colors"
              role="button"
              tabindex="0"
              onclick={() => player.replaceQueueAndPlay(feed.tracks, i, t("related.tracks"))}
              onkeydown={(e) => e.key === "Enter" && player.replaceQueueAndPlay(feed.tracks, i, t("related.tracks"))}
            >
              <div class="relative shrink-0">
                <CoverImage class="track-cover !w-10 !h-10 !rounded-[8px]" src={track.cover_url} alt="" />
                <div class="absolute inset-0 rounded-[8px] bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white">
                  <Play size={14} fill="currentColor" class="translate-x-[1px]" />
                </div>
              </div>
              <div class="min-w-0 flex-1">
                <div class="text-[13px] font-semibold truncate">{track.title}</div>
                <div class="text-[11.5px] text-[var(--text-muted)] truncate">{track.artist}</div>
              </div>
              <button
                type="button"
                class="icon-btn !w-7 !h-7 opacity-0 group-hover:opacity-100 shrink-0"
                title={t("queue.add")}
                onclick={(e) => {
                  e.stopPropagation();
                  player.enqueueTrack(track);
                }}
              >
                <Plus size={14} />
              </button>
              {#if track.duration_secs != null}
                <span class="text-[11.5px] text-[var(--text-muted)] numeric shrink-0">
                  {formatDuration(track.duration_secs)}
                </span>
              {/if}
            </div>
          {/each}
        </div>
      </section>
    {/if}

    {#if feed.artists.length > 0}
      <section class="space-y-2">
        <h3 class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
          {t("related.artists")}
        </h3>
        <div class="grid grid-cols-2 gap-2">
          {#each feed.artists.slice(0, 6) as artist (artist.id)}
            <button
              type="button"
              class="flex items-center gap-2.5 p-2 rounded-xl bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-left transition-colors"
              onclick={() => {
                closeMobileSheet();
                player.openArtist(artist.id, artist.name);
              }}
            >
              <CoverImage
                class="w-9 h-9 rounded-full shrink-0"
                src={artist.avatar_url}
                alt={artist.name}
                fallbackIcon="👤"
              />
              <span class="text-[12.5px] font-semibold truncate">{artist.name}</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    {#if feed.albums.length > 0}
      <section class="space-y-2">
        <h3 class="text-[11.5px] font-bold uppercase tracking-[0.08em] text-[var(--text-muted)]">
          {t("related.albums")}
        </h3>
        <div class="grid grid-cols-2 gap-2">
          {#each feed.albums.slice(0, 6) as album (album.id)}
            <button
              type="button"
              class="p-2 rounded-xl bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-left flex flex-col gap-1.5 transition-colors"
              onclick={() => {
                closeMobileSheet();
                player.openAlbum(album.id, album.title, album.cover_url);
              }}
            >
              <CoverImage class="w-full aspect-square !rounded-[8px]" src={album.cover_url} alt={album.title} />
              <span class="text-[12px] font-semibold truncate w-full">{album.title}</span>
              <span class="text-[11px] text-[var(--text-muted)] truncate w-full">{album.artist}</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    {#if feed.tracks.length === 0 && feed.artists.length === 0 && feed.albums.length === 0}
      <div class="flex-1 flex flex-col items-center justify-center text-center px-4 gap-2 text-[var(--text-muted)]">
        <Sparkles size={24} class="opacity-50" />
        <p class="text-[13px]">{t("related.empty")}</p>
      </div>
    {/if}
  {/if}
</div>
