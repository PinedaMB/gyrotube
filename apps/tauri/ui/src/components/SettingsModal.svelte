<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { player } from "../lib/stores/player.svelte";
  import type { ThemeMode } from "../lib/types";

  let dialogEl = $state<HTMLDialogElement | null>(null);
  let cacheBytes = $state(0);

  $effect(() => {
    const dlg = dialogEl;
    if (!dlg) return;
    if (player.settingsOpen && !dlg.open) {
      dlg.showModal();
      if (isTauriRuntime()) {
        void api.getCacheBytes().then((b) => (cacheBytes = b)).catch(() => {});
      }
    } else if (!player.settingsOpen && dlg.open) {
      dlg.close();
    }
  });

  let cacheMb = $derived((cacheBytes / (1024 * 1024)).toFixed(1));
  let cachePct = $derived(Math.min(100, (cacheBytes / (1024 * 1024 * 1024)) * 100));

  async function clearCache() {
    if (!isTauriRuntime()) return;
    await api.clearAudioCache();
    cacheBytes = 0;
    player.showToast("Audio cache cleared");
  }
</script>

<dialog
  bind:this={dialogEl}
  class="gt-dialog"
  onclose={() => (player.settingsOpen = false)}
  onclick={(e) => {
    if (e.target === dialogEl) player.settingsOpen = false;
  }}
>
  <div class="dlg-head">
    <h2>Settings</h2>
    <button class="icon-btn" onclick={() => (player.settingsOpen = false)}>✕</button>
  </div>

  <div class="dlg-body">
    <section class="group">
      <h3>Appearance</h3>
      <div class="card">
        <div class="row">
          <span>Theme</span>
          <select
            value={player.themeMode}
            onchange={(e) =>
              player.setThemeMode((e.currentTarget as HTMLSelectElement).value as ThemeMode)}
          >
            <option value="system">System</option>
            <option value="dark">Dark</option>
            <option value="light">Light</option>
          </select>
        </div>
      </div>
    </section>

    <section class="group">
      <h3>Playback</h3>
      <div class="card">
        <div class="row">
          <div>
            <strong>Crossfade</strong>
            <small class="muted">{player.crossfadeSecs === 0 ? "Disabled (0 s)" : `${player.crossfadeSecs} s`}</small>
          </div>
          <input
            type="range"
            min="0"
            max="10"
            step="1"
            value={player.crossfadeSecs}
            oninput={(e) => player.setCrossfadeSecs(Number((e.currentTarget as HTMLInputElement).value))}
          />
        </div>

        <label class="row">
          <div>
            <strong>Autoplay</strong>
            <small class="muted">Queue similar songs when the queue ends</small>
          </div>
          <input
            type="checkbox"
            checked={player.autoplayRelated}
            onchange={(e) => player.setAutoplay((e.currentTarget as HTMLInputElement).checked)}
          />
        </label>

        <label class="row">
          <div>
            <strong>Background playback</strong>
            <small class="muted">Keep playing music when closing the window</small>
          </div>
          <input
            type="checkbox"
            checked={player.backgroundPlayback}
            onchange={(e) =>
              player.setBackgroundPlayback((e.currentTarget as HTMLInputElement).checked)}
          />
        </label>
      </div>
    </section>

    <section class="group">
      <h3>Storage</h3>
      <div class="card">
        <div class="row">
          <div>
            <strong>Audio cache</strong>
            <small class="muted numeric">{cacheMb} MB / 1024 MB cached</small>
            <div class="cache-bar"><div class="cache-fill" style="width: {cachePct}%"></div></div>
          </div>
          <button class="pill-btn danger" disabled={cacheBytes === 0} onclick={() => void clearCache()}>
            Clear
          </button>
        </div>

        <div class="row">
          <div>
            <strong>Search history</strong>
            <small class="muted">{player.searchHistory.length} saved searches</small>
          </div>
          <button
            class="pill-btn"
            disabled={player.searchHistory.length === 0}
            onclick={() => player.clearSearchHistory()}
          >
            Clear
          </button>
        </div>
      </div>
    </section>
  </div>
</dialog>

<style>
  .dlg-head {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .dlg-head h2 {
    margin: 0;
    font-size: 17px;
  }

  .dlg-body {
    padding: 18px 20px 22px;
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  .group h3 {
    margin: 0 0 8px;
    font-size: 13px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .card {
    border-radius: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    overflow: hidden;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    padding: 12px 14px;
    border-bottom: 1px solid var(--border);
    font-size: 13.5px;
  }

  .row:last-child {
    border-bottom: none;
  }

  .row div {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .row small {
    font-size: 11.5px;
  }

  select {
    padding: 6px 10px;
    border-radius: 8px;
    border: 1px solid var(--border-strong);
    background: var(--bg);
    color: var(--text);
  }

  .cache-bar {
    width: 140px;
    height: 5px;
    margin-top: 4px;
    border-radius: 999px;
    background: var(--surface-hover);
    overflow: hidden;
  }

  .cache-fill {
    height: 100%;
    background: var(--accent);
  }

  .danger {
    background: rgba(224, 27, 36, 0.16);
    border-color: rgba(224, 27, 36, 0.42);
    color: #ff7b82;
  }
</style>
