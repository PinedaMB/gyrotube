<script lang="ts">
  import { LogIn, LogOut } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import SpinQrCard from "./SpinQrCard.svelte";

  let joinCode = $state("");
  let busy = $state(false);

  let isHost = $derived(player.spinStatus.remote_role === "host");
  let isGuest = $derived(player.spinStatus.remote_role === "guest");

  async function toggleRemoteHost() {
    if (!isTauriRuntime() || busy) return;
    busy = true;
    try {
      player.spinStatus = !isHost ? await api.spinStartRemoteHost() : await api.spinStopRemote();
      if (player.spinStatus.remote_role === "host") void player.syncSpinSnapshot();
    } catch (err) {
      player.showToast(String(err));
    } finally {
      busy = false;
    }
  }

  async function handleJoin() {
    if (!isTauriRuntime() || busy) return;
    busy = true;
    try {
      if (isGuest) {
        player.spinStatus = await api.spinStopRemote();
      } else if (joinCode.trim()) {
        player.spinStatus = await api.spinJoinRemote(joinCode.trim());
        player.showToast(t("spin.connecting"));
      }
    } catch (err) {
      player.showToast(String(err));
    } finally {
      busy = false;
    }
  }
</script>

<div class="space-y-4">
  <!-- Host toggle -->
  <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] p-3.5 flex items-center justify-between gap-3">
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

  {#if isHost && player.spinStatus.remote_ticket}
    <SpinQrCard
      kicker={t("spin.active_invite")}
      qrSvg={player.spinStatus.remote_qr_svg}
      value={player.spinStatus.remote_ticket}
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
