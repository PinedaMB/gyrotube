<script lang="ts">
  import { api, isTauriRuntime } from "../lib/api";
  import { player } from "../lib/stores/player.svelte";

  let tab = $state<"local" | "remote">("local");
  let joinCode = $state("");
  let busy = $state(false);

  let isLive = $derived(player.spinStatus.local_active || player.spinStatus.remote_role !== "idle");

  async function toggleLocal(checked: boolean) {
    if (!isTauriRuntime()) return;
    busy = true;
    try {
      player.spinStatus = checked ? await api.spinStartLocal() : await api.spinStopLocal();
    } catch (err) {
      player.showToast(String(err));
    } finally {
      busy = false;
    }
  }

  async function toggleAllowControls(checked: boolean) {
    if (!isTauriRuntime()) return;
    try {
      player.spinStatus = await api.spinSetAllowControls(checked);
    } catch (err) {
      player.showToast(String(err));
    }
  }

  async function toggleRemoteHost(checked: boolean) {
    if (!isTauriRuntime()) return;
    busy = true;
    try {
      player.spinStatus = checked ? await api.spinStartRemoteHost() : await api.spinStopRemote();
    } catch (err) {
      player.showToast(String(err));
    } finally {
      busy = false;
    }
  }

  async function handleJoin() {
    if (!isTauriRuntime()) return;
    busy = true;
    try {
      if (player.spinStatus.remote_role === "guest") {
        player.spinStatus = await api.spinStopRemote();
      } else {
        player.spinStatus = await api.spinJoinRemote(joinCode.trim());
        player.showToast("Connecting to SPIN room…");
      }
    } catch (err) {
      player.showToast(String(err));
    } finally {
      busy = false;
    }
  }

  async function copyText(text: string, label: string) {
    try {
      await navigator.clipboard.writeText(text);
      player.showToast(label);
    } catch {
      player.showToast("Could not copy to clipboard");
    }
  }
</script>

<aside
  class="spin-sidebar w-[340px] shrink-0 h-full flex flex-col border-l border-[var(--border)] bg-[var(--bg-sidebar)] overflow-hidden max-xl:fixed max-xl:right-0 max-xl:top-14 max-xl:bottom-0 max-xl:z-40 max-xl:shadow-2xl"
>
  <div class="spin-header">
    <div class="spin-title-row">
      <h3>SPIN Mode</h3>
      {#if isLive}
        <span class="live-pill">● Live</span>
      {/if}
    </div>
    <button class="icon-btn" title="Close SPIN panel" onclick={() => (player.spinOpen = false)}>
      ✕
    </button>
  </div>

  <div class="spin-body scroll-y">
    <div class="mode-switch">
      <button class:active={tab === "local"} onclick={() => (tab = "local")}>Wi-Fi Jukebox</button>
      <button class:active={tab === "remote"} onclick={() => (tab = "remote")}>App to App</button>
    </div>

    {#if tab === "local"}
      <div class="spin-card">
        <label class="switch-row">
          <span>
            <strong>Enable local Jukebox</strong>
            <small class="muted">Local web server on your Wi-Fi</small>
          </span>
          <input
            type="checkbox"
            disabled={busy}
            checked={player.spinStatus.local_active}
            onchange={(e) => void toggleLocal((e.currentTarget as HTMLInputElement).checked)}
          />
        </label>

        <label class="switch-row">
          <span>
            <strong>Allow controls</strong>
            <small class="muted">Pause, skip, and reorder queue</small>
          </span>
          <input
            type="checkbox"
            checked={player.spinStatus.allow_controls}
            onchange={(e) => void toggleAllowControls((e.currentTarget as HTMLInputElement).checked)}
          />
        </label>

        <div class="status-line">
          <span class="muted">Status</span>
          <span>
            {player.spinStatus.local_active
              ? `${player.spinStatus.guest_count} guests • ${player.spinStatus.added_count} added`
              : "Inactive"}
          </span>
        </div>
      </div>

      {#if player.spinStatus.local_active}
        <div class="spin-card qr-section">
          <span class="qr-kicker">Scan to join</span>
          {#if player.spinStatus.local_qr_svg}
            <div class="qr-box">{@html player.spinStatus.local_qr_svg}</div>
          {/if}
          <div class="copy-row">
            <code>{player.spinStatus.local_url}</code>
            <button
              class="pill-btn"
              onclick={() => void copyText(player.spinStatus.local_url, "SPIN link copied")}
            >
              Copy
            </button>
          </div>
        </div>
      {/if}
    {:else}
      <div class="spin-card">
        <label class="switch-row">
          <span>
            <strong>Broadcast as host</strong>
            <small class="muted">Create a synced listening invitation</small>
          </span>
          <input
            type="checkbox"
            disabled={busy}
            checked={player.spinStatus.remote_role === "host"}
            onchange={(e) => void toggleRemoteHost((e.currentTarget as HTMLInputElement).checked)}
          />
        </label>
      </div>

      {#if player.spinStatus.remote_role === "host" && player.spinStatus.remote_ticket}
        <div class="spin-card qr-section">
          <span class="qr-kicker">Active invitation</span>
          {#if player.spinStatus.remote_qr_svg}
            <div class="qr-box">{@html player.spinStatus.remote_qr_svg}</div>
          {/if}
          <div class="copy-row">
            <code>{player.spinStatus.remote_ticket}</code>
            <button
              class="pill-btn"
              onclick={() =>
                void copyText(player.spinStatus.remote_ticket ?? "", "Invitation copied")}
            >
              Copy
            </button>
          </div>
        </div>
      {/if}

      <div class="spin-card join-card">
        <strong>Join a room</strong>
        <p class="muted">Paste a SPIN invitation to listen in sync.</p>
        <input
          class="text-input"
          type="text"
          bind:value={joinCode}
          placeholder="GYRO-… or host URL"
        />
        <button
          class="pill-btn"
          class:primary={player.spinStatus.remote_role !== "guest"}
          disabled={busy}
          onclick={() => void handleJoin()}
        >
          {player.spinStatus.remote_role === "guest" ? "Disconnect" : "Join"}
        </button>
      </div>
    {/if}
  </div>
</aside>

<style>
  .spin-sidebar {
    width: 340px;
    flex-shrink: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-sidebar);
    border-left: 1px solid var(--border);
  }

  .spin-header {
    padding: 12px 14px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .spin-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .spin-title-row h3 {
    margin: 0;
    font-size: 16px;
  }

  .live-pill {
    font-size: 11px;
    font-weight: 700;
    padding: 2px 9px;
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .spin-body {
    flex: 1;
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .mode-switch {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4px;
    padding: 4px;
    border-radius: 999px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .mode-switch button {
    padding: 6px 10px;
    border-radius: 999px;
    font-size: 12.5px;
    font-weight: 600;
    color: var(--text-muted);
  }

  .mode-switch button.active {
    background: var(--accent-soft);
    color: var(--accent-bright);
  }

  .spin-card {
    padding: 14px;
    border-radius: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .switch-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    cursor: pointer;
  }

  .switch-row span {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 13.5px;
  }

  .switch-row small {
    font-size: 11.5px;
  }

  .status-line {
    display: flex;
    justify-content: space-between;
    font-size: 12.5px;
    padding-top: 8px;
    border-top: 1px solid var(--border);
  }

  .qr-section {
    align-items: center;
  }

  .qr-kicker {
    font-size: 12px;
    font-weight: 700;
  }

  .qr-box {
    width: 184px;
    height: 184px;
    padding: 8px;
    border-radius: 16px;
    background: #ffffff;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.32);
  }

  .qr-box :global(svg) {
    width: 100%;
    height: 100%;
    display: block;
  }

  .copy-row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .copy-row code {
    flex: 1;
    min-width: 0;
    font-size: 12px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .join-card p {
    margin: 0;
    font-size: 12.5px;
  }

  .text-input {
    width: 100%;
    padding: 8px 12px;
    border-radius: 10px;
    border: 1px solid var(--border-strong);
    background: var(--bg);
    font-size: 13px;
  }

  .join-card .pill-btn {
    align-self: flex-end;
  }

  @media (max-width: 1180px) {
    .spin-sidebar {
      position: fixed;
      top: 54px;
      right: 0;
      bottom: 0;
      z-index: 35;
      box-shadow: -12px 0 36px rgba(0, 0, 0, 0.48);
    }
  }
</style>
