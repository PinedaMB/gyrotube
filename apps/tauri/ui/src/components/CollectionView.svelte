<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import { formatDuration, type Track, type UserPlaylist } from "../lib/types";
  import CoverImage from "./CoverImage.svelte";

  const SWATCHES = [
    "#2190a4",
    "#e01b24",
    "#3584e4",
    "#9141ac",
    "#2ec27e",
    "#e66100",
    "#c061cb",
    "#f5c211",
  ];
  const ICONS = ["♪", "★", "♥", "⚡", "🌙", "☀", "🔥", "🎧", "🎸", "🎹", "🚗", "🏋"];

  let selectedPlaylistId = $state<string | null>(null);
  let selectedPlaylist = $derived(
    selectedPlaylistId
      ? (player.userPlaylists.find((p) => p.id === selectedPlaylistId) ?? null)
      : null,
  );

  let dialogEl = $state<HTMLDialogElement | null>(null);
  let editingId = $state<string | null>(null);
  let formName = $state("");
  let formDesc = $state("");
  let formColor = $state(SWATCHES[0]);
  let formIcon = $state("♪");

  let addMenuTrack = $state<Track | null>(null);

  function openCreateDialog() {
    editingId = null;
    formName = "";
    formDesc = "";
    formColor = SWATCHES[0];
    formIcon = "♪";
    dialogEl?.showModal();
  }

  function openEditDialog(pl: UserPlaylist) {
    editingId = pl.id;
    formName = pl.name;
    formDesc = pl.description || "";
    formColor = pl.color_hex || SWATCHES[0];
    formIcon = pl.icon_name || "♪";
    dialogEl?.showModal();
  }

  function submitPlaylist(e: SubmitEvent) {
    e.preventDefault();
    if (!formName.trim()) return;
    if (editingId) {
      player.updateUserPlaylist(editingId, formName, formDesc, formColor, formIcon);
    } else {
      const created = player.createUserPlaylist(formName, formDesc, formColor, formIcon);
      selectedPlaylistId = created.id;
    }
    dialogEl?.close();
  }

  function removePlaylist(pl: UserPlaylist) {
    player.deleteUserPlaylist(pl.id);
    if (selectedPlaylistId === pl.id) selectedPlaylistId = null;
  }
</script>

<div class="page-clamp">
  {#if selectedPlaylist}
    <div class="detail-head">
      <button class="icon-btn back-btn" title="Back" onclick={() => (selectedPlaylistId = null)}>
        ‹
      </button>
      <div
        class="detail-badge"
        style="background: {selectedPlaylist.color_hex || 'var(--accent)'};"
      >
        {selectedPlaylist.icon_name || "♪"}
      </div>
      <div class="detail-meta">
        <span class="kicker">LOCAL PLAYLIST</span>
        <h1 class="page-title">{selectedPlaylist.name}</h1>
        {#if selectedPlaylist.description}
          <p class="muted caption">{selectedPlaylist.description}</p>
        {/if}
        <p class="muted caption">{selectedPlaylist.tracks.length} songs</p>
        <div class="detail-actions">
          <button
            class="pill-btn primary"
            disabled={selectedPlaylist.tracks.length === 0}
            onclick={() =>
              selectedPlaylist &&
              player.replaceQueueAndPlay(selectedPlaylist.tracks, 0, selectedPlaylist.name)}
          >
            ▶ Play all
          </button>
          <button class="pill-btn" onclick={() => selectedPlaylist && openEditDialog(selectedPlaylist)}>
            Edit
          </button>
          <button
            class="pill-btn danger"
            onclick={() => selectedPlaylist && removePlaylist(selectedPlaylist)}
          >
            Delete
          </button>
        </div>
      </div>
    </div>

    {#if selectedPlaylist.tracks.length === 0}
      <p class="muted empty-note">
        This playlist is empty. Add songs from your recent history or search results.
      </p>
    {:else}
      <div class="track-list">
        {#each selectedPlaylist.tracks as track, i (track.id)}
          {@const active = player.currentTrack?.id === track.id}
          <div
            class="track-row"
            class:active
            role="button"
            tabindex="0"
            onclick={() =>
              selectedPlaylist &&
              player.replaceQueueAndPlay(selectedPlaylist.tracks, i, selectedPlaylist.name)}
            onkeydown={(e) =>
              e.key === "Enter" &&
              selectedPlaylist &&
              player.replaceQueueAndPlay(selectedPlaylist.tracks, i, selectedPlaylist.name)}
          >
            <CoverImage class="track-cover" src={track.cover_url} alt="" />
            <div class="track-meta">
              <span class="track-title">{track.title}</span>
              <span class="track-sub">{track.artist}{track.album ? ` • ${track.album}` : ""}</span>
            </div>
            <span class="muted numeric dur">{formatDuration(track.duration_secs)}</span>
            <button
              class="icon-btn"
              class:fav-active={player.isFavorite(track.id)}
              title="Favorite"
              onclick={(e) => {
                e.stopPropagation();
                player.toggleFavorite(track);
              }}
            >
              ♥
            </button>
            <button
              class="icon-btn"
              title="Remove from playlist"
              onclick={(e) => {
                e.stopPropagation();
                if (selectedPlaylist) {
                  player.removeTrackFromPlaylist(selectedPlaylist.id, track.id);
                }
              }}
            >
              ✕
            </button>
          </div>
        {/each}
      </div>
    {/if}
  {:else}
    <div class="page-head">
      <div>
        <h1 class="page-title">Collection</h1>
        <p class="muted caption">Your local playlists, recent history, and favorite songs</p>
      </div>
      <button class="pill-btn primary" onclick={openCreateDialog}>+ New playlist</button>
    </div>

    <section class="section">
      <div class="section-head">
        <h2 class="section-title">Local playlists</h2>
      </div>

      <div class="playlists-grid">
        <div
          class="playlist-card fav-card"
          role="button"
          tabindex="0"
          onclick={() =>
            player.favorites.length &&
            player.replaceQueueAndPlay(player.favorites, 0, "Favorites")}
          onkeydown={(e) =>
            e.key === "Enter" &&
            player.favorites.length &&
            player.replaceQueueAndPlay(player.favorites, 0, "Favorites")}
        >
          <span class="pl-badge fav-badge">♥</span>
          <div class="track-meta">
            <strong>Favorites</strong>
            <span class="muted small">{player.favorites.length} songs</span>
          </div>
          <button
            class="icon-btn pl-play"
            disabled={player.favorites.length === 0}
            title="Play favorites"
          >
            ▶
          </button>
        </div>

        {#each player.userPlaylists as pl (pl.id)}
          <div
            class="playlist-card"
            role="button"
            tabindex="0"
            onclick={() => (selectedPlaylistId = pl.id)}
            onkeydown={(e) => e.key === "Enter" && (selectedPlaylistId = pl.id)}
          >
            <span class="pl-badge" style="background: {pl.color_hex || 'var(--accent)'};">
              {pl.icon_name || "♪"}
            </span>
            <div class="track-meta">
              <strong>{pl.name}</strong>
              <span class="muted small">{pl.tracks.length} songs</span>
            </div>
            <button
              class="icon-btn pl-play"
              disabled={pl.tracks.length === 0}
              title="Play playlist"
              onclick={(e) => {
                e.stopPropagation();
                if (pl.tracks.length) player.replaceQueueAndPlay(pl.tracks, 0, pl.name);
              }}
            >
              ▶
            </button>
          </div>
        {/each}
      </div>
    </section>

    <section class="section">
      <div class="section-head">
        <h2 class="section-title">Recently played</h2>
        {#if player.recentTracks.length > 0}
          <button
            class="pill-btn"
            onclick={() => player.replaceQueueAndPlay(player.recentTracks, 0, "Recent")}
          >
            ▶ Play all
          </button>
        {/if}
      </div>

      {#if player.recentTracks.length === 0}
        <p class="muted">Songs you play will appear here.</p>
      {:else}
        <div class="track-list">
          {#each player.recentTracks as track, i (track.id)}
            {@const active = player.currentTrack?.id === track.id}
            <div
              class="track-row"
              class:active
              role="button"
              tabindex="0"
              onclick={() => player.replaceQueueAndPlay(player.recentTracks, i, "Recent")}
              onkeydown={(e) =>
                e.key === "Enter" && player.replaceQueueAndPlay(player.recentTracks, i, "Recent")}
            >
              <CoverImage class="track-cover" src={track.cover_url} alt="" />
              <div class="track-meta">
                <span class="track-title">{track.title}</span>
                <span class="track-sub">{track.artist}{track.album ? ` • ${track.album}` : ""}</span>
              </div>
              <span class="muted numeric dur">{formatDuration(track.duration_secs)}</span>
              <button
                class="icon-btn"
                class:fav-active={player.isFavorite(track.id)}
                title="Favorite"
                onclick={(e) => {
                  e.stopPropagation();
                  player.toggleFavorite(track);
                }}
              >
                ♥
              </button>
              {#if player.userPlaylists.length > 0}
                <button
                  class="icon-btn"
                  title="Add to playlist"
                  onclick={(e) => {
                    e.stopPropagation();
                    addMenuTrack = addMenuTrack?.id === track.id ? null : track;
                  }}
                >
                  ☰
                </button>
              {/if}
              <button
                class="icon-btn"
                title="Add to queue"
                onclick={(e) => {
                  e.stopPropagation();
                  player.enqueueTrack(track);
                }}
              >
                +
              </button>
            </div>
            {#if addMenuTrack?.id === track.id}
              <div class="inline-picker">
                <span class="muted small">Add to:</span>
                {#each player.userPlaylists as pl (pl.id)}
                  <button
                    class="pill-btn small-pill"
                    onclick={() => {
                      player.addTrackToPlaylist(pl.id, track);
                      addMenuTrack = null;
                    }}
                  >
                    {pl.icon_name || "♪"} {pl.name}
                  </button>
                {/each}
              </div>
            {/if}
          {/each}
        </div>
      {/if}
    </section>
  {/if}
</div>

<dialog bind:this={dialogEl} class="gt-dialog">
  <form class="modal-form" onsubmit={submitPlaylist}>
    <div class="modal-head">
      <h3>{editingId ? "Edit playlist" : "New playlist"}</h3>
      <button type="button" class="icon-btn" onclick={() => dialogEl?.close()}>✕</button>
    </div>

    <label class="field">
      <span>Name</span>
      <input type="text" bind:value={formName} placeholder="My playlist" required />
    </label>

    <label class="field">
      <span>Description</span>
      <input type="text" bind:value={formDesc} placeholder="Optional description" />
    </label>

    <div class="field">
      <span>Color</span>
      <div class="swatch-row">
        {#each SWATCHES as hex}
          <button
            type="button"
            class="swatch"
            class:selected={formColor === hex}
            style="background: {hex};"
            aria-label={hex}
            onclick={() => (formColor = hex)}
          ></button>
        {/each}
      </div>
    </div>

    <div class="field">
      <span>Icon</span>
      <div class="icon-grid">
        {#each ICONS as ic}
          <button
            type="button"
            class="icon-choice"
            class:selected={formIcon === ic}
            onclick={() => (formIcon = ic)}
          >
            {ic}
          </button>
        {/each}
      </div>
    </div>

    <div class="modal-actions">
      <button type="button" class="pill-btn" onclick={() => dialogEl?.close()}>Cancel</button>
      <button type="submit" class="pill-btn primary">{editingId ? "Save" : "Create"}</button>
    </div>
  </form>
</dialog>

<style>
  .page-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .page-title {
    margin: 0;
    font-size: 28px;
    font-weight: 850;
  }

  .caption {
    margin: 3px 0 0;
    font-size: 13px;
  }

  .section {
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .playlists-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 12px;
  }

  .playlist-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    cursor: pointer;
    transition: background-color 140ms ease, border-color 140ms ease;
  }

  .playlist-card:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }

  .fav-card {
    background: linear-gradient(135deg, rgba(224, 27, 36, 0.16), var(--surface) 70%);
  }

  .pl-badge {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    color: #ffffff;
    flex-shrink: 0;
  }

  .fav-badge {
    background: linear-gradient(135deg, #e01b24, #ff6b81);
  }

  .pl-play {
    flex-shrink: 0;
  }

  .small {
    font-size: 12px;
  }

  .dur {
    font-size: 12px;
    margin-right: 4px;
  }

  .fav-active {
    color: var(--coral);
  }

  .detail-head {
    display: flex;
    align-items: flex-start;
    gap: 18px;
    margin-bottom: 22px;
  }

  .back-btn {
    font-size: 22px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .detail-badge {
    width: 112px;
    height: 112px;
    border-radius: 16px;
    display: grid;
    place-items: center;
    font-size: 40px;
    color: #ffffff;
    flex-shrink: 0;
    box-shadow: 0 12px 28px rgba(0, 0, 0, 0.35);
  }

  .detail-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .kicker {
    align-self: flex-start;
    font-size: 10.5px;
    font-weight: 800;
    letter-spacing: 0.08em;
    padding: 3px 9px;
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .detail-actions {
    display: flex;
    gap: 8px;
    margin-top: 8px;
  }

  .pill-btn.danger:hover {
    background: rgba(224, 27, 36, 0.2);
    color: #ff6b81;
  }

  .empty-note {
    margin-top: 16px;
  }

  .inline-picker {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 6px;
    padding: 8px 14px;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid var(--border);
  }

  .small-pill {
    padding: 4px 10px;
    font-size: 12px;
  }

  .modal-form {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .modal-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .modal-head h3 {
    margin: 0;
    font-size: 18px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 12.5px;
    font-weight: 600;
  }

  .field input {
    padding: 9px 12px;
    border-radius: 10px;
    border: 1px solid var(--border-strong);
    background: var(--surface);
    color: var(--text);
  }

  .swatch-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .swatch {
    width: 26px;
    height: 26px;
    border-radius: 999px;
    border: 2px solid transparent;
  }

  .swatch.selected {
    border-color: #ffffff;
    transform: scale(1.1);
  }

  .icon-grid {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 6px;
  }

  .icon-choice {
    padding: 8px 0;
    border-radius: 10px;
    background: var(--surface);
    border: 1px solid var(--border);
    font-size: 16px;
  }

  .icon-choice.selected {
    background: var(--accent-soft);
    border-color: var(--accent-bright);
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 4px;
  }

  @media (max-width: 780px) {
    .playlists-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
</style>
