<script lang="ts">
  import { Clock3, Heart, ListPlus, Play, Plus, Volume2 } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { player } from "../../lib/stores/player.svelte";
  import { formatDuration, type Track } from "../../lib/types";
  import CoverImage from "../CoverImage.svelte";
  import PlaylistIcon from "./PlaylistIcon.svelte";

  interface Props {
    onCreatePlaylist: () => void;
  }

  let { onCreatePlaylist }: Props = $props();
  let pickerTrackId = $state<string | null>(null);

  function togglePicker(track: Track) {
    pickerTrackId = pickerTrackId === track.id ? null : track.id;
  }
</script>

<section class="mt-8 flex flex-col gap-3">
  <div class="flex items-center justify-between gap-3">
    <div class="flex items-baseline gap-2">
      <h2 class="section-title">Recently played</h2>
      {#if player.recentTracks.length > 0}
        <span class="text-[12.5px] font-semibold text-[var(--text-muted)] numeric">
          {player.recentTracks.length}
        </span>
      {/if}
    </div>

    {#if player.recentTracks.length > 0}
      <button
        class="pill-btn"
        onclick={() => player.replaceQueueAndPlay(player.recentTracks, 0, "Recently played")}
      >
        <Play size={13} fill="currentColor" />
        Play all
      </button>
    {/if}
  </div>

  {#if player.recentTracks.length === 0}
    <div class="py-10 text-center rounded-2xl bg-[var(--surface)] border border-[var(--border)]">
      <p class="text-[var(--text-muted)] text-[13.5px]">Songs you play will appear here.</p>
    </div>
  {:else}
    <div>
      <div class="grid grid-cols-[minmax(0,1fr)_auto] md:grid-cols-[minmax(0,1.5fr)_minmax(0,1fr)_auto] items-center gap-4 px-3 pb-2.5 text-[11px] font-bold uppercase tracking-wider text-[var(--text-muted)]/75 border-b border-white/[0.08]">
        <span>Title</span>
        <span class="hidden md:block">Album</span>
        <span class="w-28 flex justify-end pr-2"><Clock3 size={13} /></span>
      </div>

      <div class="flex flex-col">
        {#each player.recentTracks as track, i (track.id)}
          {@const active = player.currentTrack?.id === track.id}
          <div class="border-b border-white/[0.06] last:border-b-0">
            <div
              class="group grid grid-cols-[minmax(0,1fr)_auto] md:grid-cols-[minmax(0,1.5fr)_minmax(0,1fr)_auto] items-center gap-4 py-2.5 px-3 cursor-pointer transition-colors {active
                ? 'bg-[var(--accent-soft)]'
                : 'hover:bg-white/[0.05]'}"
              role="button"
              tabindex="0"
              onclick={() => player.replaceQueueAndPlay(player.recentTracks, i, "Recently played")}
              onkeydown={(e) =>
                e.key === "Enter" &&
                player.replaceQueueAndPlay(player.recentTracks, i, "Recently played")}
            >
              <!-- Cover + Title + Artist -->
              <div class="flex items-center gap-3.5 min-w-0">
                <div class="relative shrink-0">
                  <CoverImage class="track-cover !w-10 !h-10 !rounded-lg" src={track.cover_url} alt="" />
                  {#if active}
                    <div class="absolute inset-0 rounded-lg bg-black/55 flex items-center justify-center text-[var(--accent-bright)]">
                      <Volume2 size={16} />
                    </div>
                  {:else}
                    <div class="absolute inset-0 rounded-lg bg-black/55 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-white">
                      <Play size={14} fill="currentColor" class="translate-x-[1px]" />
                    </div>
                  {/if}
                </div>
                <div class="min-w-0">
                  <div class="text-[14px] font-semibold truncate {active ? 'text-[var(--accent-bright)] font-bold' : ''}">
                    {track.title}
                  </div>
                  <div class="text-[12px] text-[var(--text-muted)] truncate">
                    {track.artist}
                  </div>
                </div>
              </div>

              <!-- Album column (md+) -->
              <span class="hidden md:block text-[13px] text-[var(--text-muted)] truncate">
                {track.album ?? ""}
              </span>

              <!-- Actions + duration -->
              <div class="flex items-center gap-1 shrink-0">
                <button
                  class="icon-btn !w-8 !h-8"
                  class:fav-active={player.isFavorite(track.id)}
                  title="Favorite"
                  onclick={(e) => {
                    e.stopPropagation();
                    player.toggleFavorite(track);
                  }}
                >
                  <Heart size={16} fill={player.isFavorite(track.id) ? "currentColor" : "none"} />
                </button>
                <button
                  class="icon-btn !w-8 !h-8 {pickerTrackId === track.id ? 'active' : ''}"
                  title="Add to playlist"
                  onclick={(e) => {
                    e.stopPropagation();
                    togglePicker(track);
                  }}
                >
                  <ListPlus size={16} />
                </button>
                <button
                  class="icon-btn !w-8 !h-8"
                  title="Add to queue"
                  onclick={(e) => {
                    e.stopPropagation();
                    player.enqueueTrack(track);
                  }}
                >
                  <Plus size={16} />
                </button>
                <span class="text-[12.5px] text-[var(--text-muted)] numeric w-11 text-right">
                  {formatDuration(track.duration_secs)}
                </span>
              </div>
            </div>

            {#if pickerTrackId === track.id}
              <div
                transition:fly={{ y: -6, duration: 150, easing: cubicOut }}
                class="flex items-center flex-wrap gap-2 px-4 py-2.5 bg-white/[0.03] border-t border-white/[0.05]"
              >
                <span class="text-[11.5px] font-semibold text-[var(--text-muted)]">Add to playlist:</span>
                {#each player.userPlaylists as pl (pl.id)}
                  <button
                    class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[12px] font-semibold bg-[var(--surface)] hover:bg-[var(--accent-soft)] hover:text-[var(--accent-bright)] border border-[var(--border)] transition-colors"
                    onclick={() => {
                      player.addTrackToPlaylist(pl.id, track);
                      pickerTrackId = null;
                    }}
                  >
                    <PlaylistIcon name={pl.icon_name} size={13} />
                    {pl.name}
                  </button>
                {/each}
                <button
                  class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-[12px] font-semibold text-[var(--accent-bright)] bg-[var(--accent-soft)] hover:bg-[var(--accent)] hover:text-white transition-colors"
                  onclick={() => {
                    pickerTrackId = null;
                    onCreatePlaylist();
                  }}
                >
                  <Plus size={13} />
                  New playlist
                </button>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</section>
