<script lang="ts">
  import { Search, X } from "lucide-svelte";
  import TrackRow from "../TrackRow.svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";

  interface Props {
    title: string;
    tracks: Track[];
    loading: boolean;
  }

  let { title, tracks, loading }: Props = $props();
  let filterText = $state("");

  let filteredEntries = $derived.by(() => {
    const q = filterText.trim().toLowerCase();
    const out: { track: Track; originalIndex: number }[] = [];
    for (let i = 0; i < tracks.length; i++) {
      const tr = tracks[i];
      if (
        q &&
        !tr.title.toLowerCase().includes(q) &&
        !tr.artist.toLowerCase().includes(q) &&
        !(tr.album && tr.album.toLowerCase().includes(q))
      ) {
        continue;
      }
      out.push({ track: tr, originalIndex: i });
    }
    return out;
  });
</script>

<section class="min-w-0 flex flex-col gap-3">
  {#if loading}
    <div class="divide-y divide-[var(--border)] animate-pulse" aria-busy="true">
      {#each [0, 1, 2, 3, 4, 5, 6, 7] as _}
        <div class="flex items-center gap-4 py-3.5 px-3">
          <div class="w-6 h-4 rounded bg-[var(--skeleton-soft)]"></div>
          <div class="flex-1 space-y-2">
            <div class="h-3.5 w-48 rounded bg-[var(--skeleton)]"></div>
            <div class="h-3 w-28 rounded bg-[var(--skeleton-soft)]"></div>
          </div>
          <div class="h-3 w-10 rounded bg-[var(--skeleton-soft)]"></div>
        </div>
      {/each}
    </div>
  {:else if tracks.length > 0}
    {#if tracks.length > 6}
      <div class="flex items-center justify-end">
        <div
          class="flex items-center gap-2 w-full sm:w-[260px] h-8 px-3 rounded-full bg-[var(--surface)] border border-[var(--border)] focus-within:border-[var(--accent-border)] transition-colors"
        >
          <Search size={13} class="text-[var(--text-muted)] shrink-0" />
          <input
            type="text"
            bind:value={filterText}
            placeholder={t("filter.placeholder")}
            class="w-full bg-transparent outline-none focus-visible:!outline-none text-[12.5px] placeholder:text-[var(--text-muted)]"
          />
          {#if filterText}
            <button
              type="button"
              class="text-[var(--text-muted)] hover:text-[var(--text)]"
              aria-label={t("header.clear")}
              onclick={() => (filterText = "")}
            >
              <X size={13} />
            </button>
          {/if}
        </div>
      </div>
    {/if}

    {#if filteredEntries.length === 0}
      <p class="text-[13px] text-[var(--text-muted)] py-10 text-center">{t("filter.no_matches")}</p>
    {:else}
      <div class="track-list">
        {#each filteredEntries as { track, originalIndex } (track.id + ":" + originalIndex)}
          <TrackRow
            {track}
            number={originalIndex + 1}
            onPlay={() => player.replaceQueueAndPlay(tracks, originalIndex, title)}
          />
        {/each}
      </div>
    {/if}
  {:else}
    <p class="text-[var(--text-muted)] py-10 text-center">{t("album.no_tracks")}</p>
  {/if}
</section>
