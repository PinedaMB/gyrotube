<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration } from "../lib/types";

  let lineRefs = $state<Record<number, HTMLButtonElement>>({});

  $effect(() => {
    const idx = player.activeLyricIndex;
    if (idx != null && player.partyMode) {
      lineRefs[idx]?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  });

  function lineOpacity(idx: number): number {
    if (player.activeLyricIndex == null) return 0.72;
    if (idx === player.activeLyricIndex) return 1.0;
    const diff = Math.abs(idx - player.activeLyricIndex);
    if (diff === 1) return 0.66;
    if (diff === 2) return 0.42;
    return 0.24;
  }
</script>

<div class="party-root">
  <div class="party-top">
    <div class="party-badge-row">
      <span class="party-pill">✦ Party Mode</span>
      <span class="muted">{player.sourceName}</span>
    </div>
    <button class="pill-btn" onclick={() => (player.partyMode = false)}>Exit</button>
  </div>

  <div class="party-split">
    <div class="party-left">
      {#if player.currentTrack?.cover_large_url || player.currentTrack?.cover_url}
        <img
          class="party-cover"
          src={player.currentTrack.cover_large_url || player.currentTrack.cover_url || ""}
          alt={player.currentTrack.title}
        />
      {:else}
        <div class="party-cover placeholder">♪</div>
      {/if}

      <h1 class="party-title">{player.currentTrack?.title ?? "Nothing playing"}</h1>
      <p class="party-artist muted">{player.currentTrack?.artist ?? ""}</p>

      <div class="party-progress-wrap">
        <div class="party-progress-bar">
          <div
            class="party-progress-fill"
            style="width: {player.durationSecs > 0 ? Math.min(100, (player.positionSecs / player.durationSecs) * 100) : 0}%"
          ></div>
        </div>
        <div class="party-times numeric">
          <span class="muted">{formatDuration(player.positionSecs)}</span>
          <button
            class="icon-btn"
            class:fav-active={player.currentTrack ? player.isFavorite(player.currentTrack.id) : false}
            disabled={!player.currentTrack}
            title="Favorite"
            onclick={() => player.currentTrack && player.toggleFavorite(player.currentTrack)}
          >
            ♥
          </button>
          <span class="muted">{formatDuration(player.durationSecs)}</span>
        </div>
      </div>

      <div class="party-controls">
        <button class="icon-btn" class:active={player.shuffle} onclick={() => player.toggleShuffle()}>⇄</button>
        <button class="icon-btn" onclick={() => void player.prev()}>⏮</button>
        <button class="play-circle" onclick={() => void player.togglePlay()}>
          {#if player.isPlaying}❚❚{:else}▶{/if}
        </button>
        <button class="icon-btn" onclick={() => void player.next()}>⏭</button>
        <button class="icon-btn" class:active={player.repeat} onclick={() => player.toggleRepeat()}>↻</button>
      </div>

      {#if player.nextTrack}
        <div class="up-next">
          <span class="up-next-kicker">UP NEXT</span>
          <div class="up-next-row">
            {#if player.nextTrack.cover_url}
              <img class="up-next-cover" src={player.nextTrack.cover_url} alt="" />
            {/if}
            <div class="track-meta">
              <span class="track-title">{player.nextTrack.title}</span>
              <span class="track-sub">{player.nextTrack.artist}</span>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <div class="party-right">
      <div class="party-lyrics-head">
        <h2>Lyrics</h2>
        {#if player.lyrics.type === "Synced"}
          <span class="sync-pill">● Synced</span>
        {/if}
      </div>

      <div class="party-lyrics-scroll">
        {#if player.lyrics.type === "Synced"}
          {#each player.lyrics.lines as line, i}
            <button
              bind:this={lineRefs[i]}
              class="party-line"
              class:active={player.activeLyricIndex === i}
              style="opacity: {lineOpacity(i)}"
              onclick={() => player.seek(line.time_secs)}
            >
              {line.text}
            </button>
          {/each}
        {:else if player.lyrics.type === "Plain"}
          <pre class="party-plain">{player.lyrics.text}</pre>
        {:else}
          <p class="muted">Play a song to view its lyrics.</p>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .party-root {
    position: fixed;
    inset: 0;
    z-index: 60;
    background: var(--bg);
    background-image: radial-gradient(
      circle at 30% 35%,
      rgba(var(--ambient-rgb), 0.30),
      transparent 72%
    );
    display: flex;
    flex-direction: column;
  }

  .party-top {
    padding: 16px 28px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .party-badge-row {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 13px;
  }

  .party-pill,
  .sync-pill {
    font-size: 11.5px;
    font-weight: 700;
    padding: 4px 11px;
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .party-split {
    flex: 1;
    min-height: 0;
    display: grid;
    grid-template-columns: 380px minmax(0, 1fr);
    gap: 44px;
    padding: 8px 40px 32px;
  }

  .party-left {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 14px;
  }

  .party-cover {
    width: 260px;
    height: 260px;
    border-radius: 18px;
    object-fit: cover;
    box-shadow: 0 20px 52px rgba(0, 0, 0, 0.52);
    background: var(--surface);
  }

  .party-cover.placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 56px;
    color: var(--text-muted);
  }

  .party-title {
    margin: 6px 0 0;
    font-size: 24px;
    font-weight: 850;
    text-align: center;
  }

  .party-artist {
    margin: 0;
    font-size: 15px;
  }

  .party-progress-wrap {
    width: 320px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .party-progress-bar {
    height: 5px;
    border-radius: 999px;
    background: var(--surface-hover);
    overflow: hidden;
  }

  .party-progress-fill {
    height: 100%;
    border-radius: 999px;
    background: var(--accent);
    transition: width 150ms linear;
  }

  .party-times {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
  }

  .party-controls {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .up-next {
    width: 320px;
    padding: 12px 14px;
    border-radius: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .up-next-kicker {
    font-size: 10.5px;
    font-weight: 800;
    letter-spacing: 0.08em;
    color: var(--text-muted);
  }

  .up-next-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .up-next-cover {
    width: 40px;
    height: 40px;
    border-radius: 8px;
    object-fit: cover;
  }

  .party-right {
    min-height: 0;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .party-lyrics-head {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .party-lyrics-head h2 {
    margin: 0;
    font-size: 18px;
    color: var(--text-muted);
  }

  .party-lyrics-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 36px 12px 140px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    scrollbar-width: none;
  }

  .party-lyrics-scroll::-webkit-scrollbar {
    display: none;
  }

  .party-line {
    text-align: left;
    font-size: 28px;
    line-height: 1.4;
    padding: 10px 16px;
    border-radius: 14px;
    transition:
      opacity 340ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 340ms cubic-bezier(0.22, 1, 0.36, 1),
      background-color 340ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .party-line.active {
    font-size: 34px;
    font-weight: 800;
    background: var(--accent-soft);
    transform: translateX(6px);
  }

  .party-plain {
    margin: 0;
    white-space: pre-wrap;
    font: inherit;
    font-size: 20px;
    line-height: 1.7;
  }

  @media (max-width: 900px) {
    .party-split {
      grid-template-columns: 1fr;
      overflow-y: auto;
    }
  }
</style>
