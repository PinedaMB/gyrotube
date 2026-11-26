<script lang="ts">
  import { LogIn, LogOut, User } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import SpinQrCard from "./SpinQrCard.svelte";

  let joinCode = $state("");
  let busy = $state(false);

  let isHost = $derived(player.spinStatus.remote_role === "host");
  let isGuest = $derived(player.spinStatus.remote_role === "guest");

  // Refresh remote guest count periodically while modal is open and hosting
  $effect(() => {
    if (!player.spinOpen || !isHost || !isTauriRuntime()) return;
    const timer = window.setInterval(() => {
      void api.spinStatus().then((st) => {
        player.spinStatus = st;
      }).catch(() => {});
    }, 3000);
    return () => window.clearInterval(timer);
  });

  async function toggleRemoteHost() {
    if (!isTauriRuntime() || busy) return;
    busy = true;
    try {
      if (!isHost) {
        player.spinStatus = await api.spinStartRemoteHost();
        if (player.spinStatus.remote_role === "host") void player.syncSpinSnapshot();
      } else {
        await player.disconnectSpinRemote();
      }
    } catch (err) {
      player.showToast(String(err));
    } finally {
      busy = false;
    }
  }

  async function toggleAllowControls() {
    if (!isTauriRuntime()) return;
    try {
      player.spinStatus = await api.spinSetAllowControls(!player.spinStatus.allow_controls);
    } catch (err) {
      player.showToast(String(err));
    }
  }

  async function handleJoin() {
    if (!isTauriRuntime() || busy) return;
    busy = true;
    try {
      if (isGuest) {
        await player.disconnectSpinRemote();
      } else if (joinCode.trim()) {
        await player.joinSpinRemote(joinCode.trim());
        player.showToast(t("spin.connecting"));
      }
    } catch (err) {
      player.showToast(String(err));
    } finally {
      busy = false;
    }
  }

  let hostGuestLabel = $derived(
    player.spinStatus.remote_guest_count === 1
      ? t("spin.remote_guest_one")
      : t("spin.remote_guests", { count: player.spinStatus.remote_guest_count }),
  );
</script>

<div class="space-y-4">
  <!-- Display name -->
  <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] px-3.5 py-2.5 flex items-center gap-2.5">
    <User size={15} class="text-[var(--text-muted)] shrink-0" />
    <label for="spin-display-name" class="text-[12.5px] font-semibold text-[var(--text-muted)] shrink-0">
      {t("spin.display_name")}
    </label>
    <input
      id="spin-display-name"
      type="text"
      maxlength={28}
      value={player.spinDisplayName}
      placeholder={t("spin.display_name_placeholder")}
      class="flex-1 min-w-0 bg-transparent text-right text-[13px] font-medium outline-none placeholder:text-[var(--text-muted)]/60"
      oninput={(e) => player.setSpinDisplayName((e.currentTarget as HTMLInputElement).value)}
    />
  </div>

  <!-- Host card -->
  <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]">
    <div class="p-3.5 flex items-center justify-between gap-3">
      <div class="min-w-0">
        <div class="text-[14px] font-semibold">{t("spin.host_enable")}</div>
        <div class="text-[12px] text-[var(--text-muted)]">{t("spin.host_enable_sub")}</div>
      </div>
      <button
        type="button"
        role="switch"
        aria-checked={isHost}
        aria-label={t("spin.host_enable")}
        disabled={busy}
        class="relative w-11 h-6 rounded-full transition-colors shrink-0 disabled:opacity-50 {isHost
          ? 'bg-[var(--accent)]'
          : 'bg-[var(--control-track)]'}"
        onclick={() => void toggleRemoteHost()}
      >
        <span
          class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {isHost
            ? 'translate-x-5'
            : ''}"
        ></span>
      </button>
    </div>

    <!-- Allow guest controls -->
    <div class="p-3.5 flex items-center justify-between gap-3">
      <div class="min-w-0">
        <div class="text-[14px] font-semibold">{t("spin.allow_controls")}</div>
        <div class="text-[12px] text-[var(--text-muted)]">{t("spin.allow_controls_sub")}</div>
      </div>
      <button
        type="button"
        role="switch"
        aria-checked={player.spinStatus.allow_controls}
        aria-label={t("spin.allow_controls")}
        class="relative w-11 h-6 rounded-full transition-colors shrink-0 {player.spinStatus.allow_controls
          ? 'bg-[var(--accent)]'
          : 'bg-[var(--control-track)]'}"
        onclick={() => void toggleAllowControls()}
      >
        <span
          class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {player.spinStatus.allow_controls
            ? 'translate-x-5'
            : ''}"
        ></span>
      </button>
    </div>

    <!-- Status row -->
    <div class="px-3.5 py-2.5 flex items-center justify-between text-[12.5px] gap-2">
      <span class="text-[var(--text-muted)] font-medium">{t("spin.status")}</span>
      {#if isHost}
        <span class="font-semibold numeric text-[var(--accent-bright)]">
          {hostGuestLabel}
        </span>
      {:else if isGuest}
        <span
          class="font-semibold truncate {player.spinRemoteConn.status === 'connected'
            ? 'text-[var(--gold-bright)]'
            : 'text-[var(--text-muted)]'}"
        >
          {#if player.spinRemoteConn.status === "connected"}
            {t("spin.connected_to", { host: player.spinRemoteConn.hostName || "Host" })}
            · {player.spinRemoteConn.allowControls ? t("spin.controls_on") : t("spin.controls_off")}
          {:else if player.spinRemoteConn.status === "reconnecting"}
            {t("spin.reconnecting")}
          {:else}
            {t("spin.connecting")}
          {/if}
        </span>
      {:else}
        <span class="font-semibold text-[var(--text-muted)]">{t("spin.status_inactive")}</span>
      {/if}
    </div>
  </div>

  {#if isHost && player.spinStatus.remote_ticket}
    <SpinQrCard
      kicker={t("spin.active_invite")}
      qrSvg={player.spinStatus.remote_qr_svg}
      value={player.spinStatus.remote_ticket}
      shareUrl={`gyro://spin/${player.spinStatus.remote_ticket}`}
      copyToast={t("spin.invite_copied")}
    />
  {/if}

  <!-- Join room -->
  <form
    class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] p-4 space-y-3"
    onsubmit={(e) => {
      e.preventDefault();
      void handleJoin();
    }}
  >
    <div>
      <div class="text-[14px] font-semibold">{t("spin.join_title")}</div>
      <p class="text-[12px] text-[var(--text-muted)] mt-0.5">{t("spin.join_sub")}</p>
    </div>

    <div class="flex items-center gap-2">
      <input
        class="flex-1 min-w-0 px-3 py-2 rounded-xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] text-[13px] placeholder:text-[var(--text-muted)]/70 outline-none focus:border-[var(--accent)] transition-colors"
        type="text"
        bind:value={joinCode}
        disabled={isGuest || busy}
        placeholder={t("spin.join_placeholder")}
      />
      <button
        type="submit"
        class="pill-btn shrink-0 !px-4 !py-2 {isGuest ? 'hover:!text-red-400' : 'primary'}"
        disabled={busy || (!isGuest && !joinCode.trim())}
      >
        {#if isGuest}
          <LogOut size={14} />
          {t("spin.disconnect_btn")}
        {:else}
          <LogIn size={14} />
          {t("spin.join_btn")}
        {/if}
      </button>
    </div>
  </form>
</div>
