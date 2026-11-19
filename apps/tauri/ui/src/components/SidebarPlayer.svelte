<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";

  let lyricRefs = $state<Record<number, HTMLButtonElement>>({});
  let dragFrom = $state<number | null>(null);

  $effect(() => {
    const idx = player.activeLyricIndex;
    if (idx != null && player.sidebarTab === "lyrics") {
      const el = lyricRefs[idx];
      el?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  });
</script>

<aside class="sidebar">
  <div class="sidebar-top">
    <div class="playing-from">
      <span class="from-kicker">Playing from</span>
      <span class="from-source">{player.sourceName}</span>
    </div>
    <button
      class="icon-btn"
      title="Collapse player"
      onclick={() => {
        player.sidebarOpen = false;
        void player.persistSession();
      }}
    >
      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="15 18 9 12 15 6" />
      </svg>
    </button>
  </div>

  <div class="sidebar-body scroll-y">
    {#if player.sidebarTab === "player"}
      <div class="player-pane">
        <div class="cover-wrap">
          <CoverImage
            class="cover-large"
            src={player.currentTrack?.cover_large_url}
            fallbackSrc={player.currentTrack?.cover_url}
            alt={player.currentTrack?.title ?? ""}
          />
        </div>

        <div class="track-info">
          <h2 class="np-title">{player.currentTrack?.title ?? "Nothing playing"}</h2>
          <p class="np-artist">{player.currentTrack?.artist ?? "Select a song"}</p>
          {#if player.currentTrack?.album}
            <p class="np-album">Album: {player.currentTrack.album}</p>
          {/if}
        </div>

        <div class="quick-actions">
          <button
            class="icon-btn"
            class:fav-active={player.currentTrack ? player.isFavorite(player.currentTrack.id) : false}
            disabled={!player.currentTrack}
            title="Favorite"
            onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
          >
            ♥
          </button>
          <input
            class="vol-slider"
            type="range"
            min="0"
            max="1"
            step="0.02"
            value={player.volume}
            aria-label="Volume"
            oninput={(e) => player.setVolume(Number((e.currentTarget as HTMLInputElement).value))}
          />
          <button class="icon-btn" title="Party Mode" onclick={() => (player.partyMode = true)}>✦</button>
        </div>

        <div class="seek-area">
          <input
            class="seek-slider"
            type="range"
            min="0"
            max={Math.max(1, player.durationSecs)}
            step="0.5"
            value={player.positionSecs}
            disabled={!player.currentTrack}
            oninput={(e) => player.seek(Number((e.currentTarget as HTMLInputElement).value))}
          />
          <div class="time-row numeric">
            <span>{formatDuration(player.positionSecs)}</span>
            <span class="quality-pill">{player.qualityLabel}</span>
            <span>{formatDuration(player.durationSecs)}</span>
          </div>
        </div>

        <div class="transport">
          <button class="icon-btn" class:active={player.shuffle} title="Shuffle" onclick={() => player.toggleShuffle()}>
            ⇄
          </button>
          <button class="icon-btn" disabled={!player.currentTrack} title="Previous" onclick={() => void player.prev()}>
            ⏮
          </button>
          <button
            class="play-circle"
            disabled={!player.currentTrack}
            title="Play / Pause"
            onclick={() => void player.togglePlay()}
          >
            {#if player.isLoadingAudio}
              <span class="spinner"></span>
            {:else if player.isPlaying}
              ❚❚
            {:else}
              ▶
            {/if}
          </button>
          <button class="icon-btn" disabled={!player.currentTrack} title="Next" onclick={() => void player.next()}>
            ⏭
          </button>
          <button class="icon-btn" class:active={player.repeat} title="Repeat" onclick={() => player.toggleRepeat()}>
            ↻
          </button>
        </div>
      </div>
    {:else if player.sidebarTab === "lyrics"}
      <div class="lyrics-pane">
        <div class="lyrics-head">
          <div>
            <h3>{player.currentTrack?.title ?? "Lyrics"}</h3>
            {#if player.currentTrack}
              <p class="muted">{player.currentTrack.artist}</p>
            {/if}
          </div>
          {#if player.lyrics.type === "Synced"}
            <span class="badge synced">● Synced</span>
          {:else if player.lyrics.type === "Plain"}
            <span class="badge">Static</span>
          {/if}
        </div>

        {#if player.lyrics.type === "Synced"}
          <div class="lyrics-lines">
            {#each player.lyrics.lines as line, i}
              <button
                bind:this={lyricRefs[i]}
                class="lyric-line"
                class:active={player.activeLyricIndex === i}
                onclick={() => player.seek(line.time_secs)}
              >
                {line.text}
              </button>
            {/each}
          </div>
        {:else if player.lyrics.type === "Plain"}
          <pre class="lyrics-plain">{player.lyrics.text}</pre>
        {:else}
          <p class="muted empty-copy">Play a song to view its lyrics.</p>
        {/if}
      </div>
    {:else}
      <div class="queue-pane">
        <div class="queue-head">
          <h3>Queue · {player.queue.length}</h3>
          <button class="pill-btn danger-hover" onclick={() => player.clearQueue()}>Clear</button>
        </div>

        {#if player.queue.length === 0}
          <p class="muted empty-copy">The queue is empty.</p>
        {:else}
          <div class="track-list">
            {#each player.queue as track, i (track.id + ":" + i)}
              <div
                class="track-row queue-row"
                class:active={player.currentIndex === i}
                draggable="true"
                role="button"
                tabindex="0"
                ondragstart={() => (dragFrom = i)}
                ondragover={(e) => e.preventDefault()}
                ondrop={() => {
                  if (dragFrom != null) player.moveQueueItem(dragFrom, i);
                  dragFrom = null;
                }}
                onclick={() => void player.playIndex(i)}
                onkeydown={(e) => e.key === "Enter" && void player.playIndex(i)}
              >
                <span class="drag-grip" aria-hidden="true">⋮⋮</span>
                {#if track.cover_url}
                  <img class="track-cover small" src={track.cover_url} alt="" />
                {/if}
                <div class="track-meta">
                  <span class="track-title">{track.title}</span>
                  <span class="track-sub">{track.artist}</span>
                </div>
                {#if player.currentIndex === i}
                  <span class="speaker-dot" aria-label="Playing">🔊</span>
                {/if}
                <span class="muted numeric dur">{formatDuration(track.duration_secs)}</span>
                <button
                  class="icon-btn"
                  title="Remove"
                  onclick={(e) => {
                    e.stopPropagation();
                    player.removeQueueIndex(i);
                  }}
                >
                  −
                </button>
              </div>
            {/each}
          </div>
        {/if}

        <div class="autoplay-card">
          <div>
            <strong>Autoplay</strong>
            <p class="muted">Queue similar songs when queue ends</p>
          </div>
          <input
            type="checkbox"
            checked={player.autoplayRelated}
            onchange={(e) => player.setAutoplay((e.currentTarget as HTMLInputElement).checked)}
          />
        </div>

        {#if player.suggestedTracks.length > 0}
          <h4 class="suggested-title">Suggested</h4>
          <div class="track-list">
            {#each player.suggestedTracks as track (track.id)}
              <div class="track-row queue-row">
                {#if track.cover_url}
                  <img class="track-cover small" src={track.cover_url} alt="" />
                {/if}
                <div class="track-meta">
                  <span class="track-title">{track.title}</span>
                  <span class="track-sub">{track.artist}</span>
                </div>
                <button class="icon-btn" title="Add to queue" onclick={() => player.enqueueTrack(track)}>
                  +
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <nav class="sidebar-tabs">
    <button class:active={player.sidebarTab === "player"} onclick={() => (player.sidebarTab = "player")}>
      ♪ Player
    </button>
    <button class:active={player.sidebarTab === "lyrics"} onclick={() => (player.sidebarTab = "lyrics")}>
      ❞ Lyrics
    </button>
    <button class:active={player.sidebarTab === "queue"} onclick={() => (player.sidebarTab = "queue")}>
      ☰ Queue
    </button>
  </nav>
</aside>

<style>
  .sidebar {
    width: 340px;
    flex-shrink: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-sidebar);
    background-image: radial-gradient(
      circle at 50% 28%,
      rgba(var(--ambient-rgb), 0.24),
      transparent 72%
    );
    border-right: 1px solid var(--border);
  }

  .sidebar-top {
    padding: 12px 14px 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .playing-from {
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .from-kicker {
    font-size: 11px;
    color: var(--text-muted);
  }

  .from-source {
    font-size: 12.5px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .sidebar-body {
    flex: 1;
    min-height: 0;
    padding: 10px 16px 16px;
  }

  .player-pane {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding-top: 12px;
  }

  .track-info {
    text-align: center;
    width: 100%;
  }

  .np-title {
    margin: 0;
    font-size: 18px;
    font-weight: 800;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .np-artist {
    margin: 4px 0 0;
    font-size: 13.5px;
    color: var(--text-muted);
  }

  .np-album {
    margin: 2px 0 0;
    font-size: 12px;
    color: var(--text-muted);
  }

  .quick-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    justify-content: center;
  }

  .vol-slider {
    width: 110px;
  }

  .seek-area {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .seek-slider {
    width: 100%;
  }

  .time-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 11.5px;
    color: var(--text-muted);
  }

  .quality-pill {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 6px;
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .transport {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(255, 255, 255, 0.35);
    border-top-color: #ffffff;
    border-radius: 999px;
    animation: spin 700ms linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .lyrics-head,
  .queue-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    margin-bottom: 12px;
  }

  .lyrics-head h3,
  .queue-head h3 {
    margin: 0;
    font-size: 16px;
  }

  .lyrics-head p {
    margin: 2px 0 0;
    font-size: 12px;
  }

  .badge {
    font-size: 11px;
    font-weight: 700;
    padding: 3px 9px;
    border-radius: 999px;
    background: var(--surface);
    color: var(--text-muted);
  }

  .badge.synced {
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .lyrics-lines {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .lyrics-plain {
    margin: 0;
    white-space: pre-wrap;
    font: inherit;
    font-size: 14.5px;
    line-height: 1.65;
    opacity: 0.88;
  }

  .empty-copy {
    font-size: 13px;
    padding: 16px 0;
  }

  .queue-row {
    padding: 7px 10px;
    gap: 8px;
  }

  .track-cover.small {
    width: 36px;
    height: 36px;
    border-radius: 7px;
  }

  .drag-grip {
    color: var(--text-muted);
    font-size: 12px;
    cursor: grab;
  }

  .speaker-dot {
    font-size: 12px;
    color: var(--accent-bright);
  }

  .dur {
    font-size: 11.5px;
  }

  .autoplay-card {
    margin-top: 12px;
    padding: 12px;
    border-radius: 12px;
    background: var(--surface);
    border: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }

  .autoplay-card p {
    margin: 2px 0 0;
    font-size: 11.5px;
  }

  .suggested-title {
    margin: 14px 0 8px;
    font-size: 14px;
  }

  .danger-hover:hover {
    background: rgba(224, 27, 36, 0.18);
    border-color: rgba(224, 27, 36, 0.45);
    color: #ff7b82;
  }

  .sidebar-tabs {
    padding: 8px;
    border-top: 1px solid var(--border);
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 4px;
  }

  .sidebar-tabs button {
    padding: 7px 6px;
    border-radius: 10px;
    font-size: 12.5px;
    font-weight: 600;
    border: 1px solid transparent;
    color: var(--text-muted);
    transition: background-color 140ms ease, border-color 140ms ease, color 140ms ease;
  }

  .sidebar-tabs button:hover {
    background: var(--surface);
    color: var(--text);
  }

  .sidebar-tabs button.active {
    background: var(--accent-soft);
    border-color: var(--accent-border);
    color: var(--accent-bright);
  }

  @media (max-width: 850px) {
    .sidebar {
      position: fixed;
      inset: 54px 0 0 0;
      width: 100%;
      z-index: 30;
    }
  }
</style>
