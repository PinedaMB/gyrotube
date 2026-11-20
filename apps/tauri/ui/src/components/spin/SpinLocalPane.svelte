<script lang="ts">
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import SpinQrCard from "./SpinQrCard.svelte";

  let busy = $state(false);

  async function toggleLocal() {
    if (!isTauriRuntime() || busy) return;
    busy = true;
    const next = !player.spinStatus.local_active;
    try {
      player.spinStatus = next ? await api.spinStartLocal() : await api.spinStopLocal();
      if (next) void player.syncSpinSnapshot();
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
</script>

<div class="space-y-4">
  <div class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)]">
    <!-- Enable local Jukebox -->
    <div class="p-3.5 flex items-center justify-between gap-3">
      <div class="min-w-0">
        <div class="text-[14px] font-semibold">{t("spin.local_enable")}</div>
        <div class="text-[12px] text-[var(--text-muted)]">{t("spin.local_enable_sub")}</div>
      </div>
      <button
        type="button"
        role="switch"
        aria-checked={player.spinStatus.local_active}
        aria-label={t("spin.local_enable")}
        disabled={busy}
        class="relative w-11 h-6 rounded-full transition-colors shrink-0 disabled:opacity-50 {player.spinStatus.local_active
          ? 'bg-[var(--accent)]'
          : 'bg-white/15'}"
        onclick={() => void toggleLocal()}
      >
        <span
          class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {player.spinStatus.local_active
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
          : 'bg-white/15'}"
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
    <div class="px-3.5 py-2.5 flex items-center justify-between text-[12.5px]">
      <span class="text-[var(--text-muted)] font-medium">{t("spin.status")}</span>
      <span
        class="font-semibold numeric {player.spinStatus.local_active
          ? 'text-[var(--accent-bright)]'
          : 'text-[var(--text-muted)]'}"
      >
        {player.spinStatus.local_active
          ? t("spin.status_active", {
              guests: player.spinStatus.guest_count,
              added: player.spinStatus.added_count,
            })
          : t("spin.status_inactive")}
      </span>
    </div>
  </div>

  {#if player.spinStatus.local_active}
    <SpinQrCard
      kicker={t("spin.scan_to_join")}
      qrSvg={player.spinStatus.local_qr_svg}
      value={player.spinStatus.local_url}
      copyToast={t("spin.link_copied")}
    />
  {/if}
</div>
