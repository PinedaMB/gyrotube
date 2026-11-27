<script lang="ts">
  import { CheckCircle2, Disc3, Gamepad2, Loader2, Plug, Radio } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { device } from "../../lib/stores/device.svelte";
  import { player } from "../../lib/stores/player.svelte";

  let verifyingLb = $state(false);
  let lbVerifiedUser = $state<string | null>(null);
  let lbVerifyError = $state<string | null>(null);

  async function verifyListenBrainz() {
    if (verifyingLb || !isTauriRuntime()) return;
    verifyingLb = true;
    lbVerifiedUser = null;
    lbVerifyError = null;
    try {
      const user = await api.validateListenBrainzToken(player.integrations.listenBrainzToken);
      lbVerifiedUser = user;
      player.showToast(t("settings.lb_verified", { user }));
    } catch (err) {
      lbVerifyError = String(err);
    } finally {
      verifyingLb = false;
    }
  }
</script>

<section class="space-y-2.5">
  <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
    <Plug size={12} class="text-[var(--accent-bright)]" />
    {t("settings.integrations")}
  </h2>
  <div
    class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)] overflow-hidden"
    style="box-shadow: var(--card-shadow);"
  >
    {#if !device.isAndroid}
      <div class="p-4 space-y-3 transition-colors hover:bg-[var(--surface-hover)]/35">
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-3 min-w-0">
            <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
              <Gamepad2 size={16} />
            </span>
            <div class="min-w-0">
              <div class="text-[14px] font-semibold leading-snug">{t("settings.discord_rpc")}</div>
              <div class="text-[12px] text-[var(--text-muted)]">{t("settings.discord_rpc_sub")}</div>
            </div>
          </div>
          <button
            type="button"
            role="switch"
            aria-checked={player.integrations.discordRpcEnabled}
            aria-label={t("settings.discord_rpc")}
            class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.integrations.discordRpcEnabled
              ? 'bg-[var(--accent)]'
              : 'bg-[var(--control-track)]'}"
            onclick={() =>
              player.updateIntegrations({
                discordRpcEnabled: !player.integrations.discordRpcEnabled,
              })}
          >
            <span
              class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.integrations.discordRpcEnabled
                ? 'translate-x-5'
                : 'translate-x-0'}"
            ></span>
          </button>
        </div>

        {#if player.integrations.discordRpcEnabled}
          <div class="pl-12 space-y-1.5">
            <input
              type="text"
              inputmode="numeric"
              value={player.integrations.discordClientId}
              placeholder={t("settings.discord_client_id_ph")}
              class="w-full px-3 py-2 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] focus:border-[var(--accent-border)] text-[12.5px] text-[var(--text)] outline-none numeric"
              oninput={(e) =>
                player.updateIntegrations({
                  discordClientId: (e.currentTarget as HTMLInputElement).value,
                })}
            />
            <p class="text-[11.5px] text-[var(--text-muted)]">{t("settings.discord_client_id_hint")}</p>
          </div>
        {/if}
      </div>
    {/if}

    <!-- ListenBrainz -->
    <div class="p-4 space-y-3 transition-colors hover:bg-[var(--surface-hover)]/35">
      <div class="flex items-center justify-between gap-4">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Radio size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.listenbrainz")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.listenbrainz_sub")}</div>
          </div>
        </div>
        <button
          type="button"
          role="switch"
          aria-checked={player.integrations.listenBrainzEnabled}
          aria-label={t("settings.listenbrainz")}
          class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.integrations.listenBrainzEnabled
            ? 'bg-[var(--accent)]'
            : 'bg-[var(--control-track)]'}"
          onclick={() =>
            player.updateIntegrations({
              listenBrainzEnabled: !player.integrations.listenBrainzEnabled,
            })}
        >
          <span
            class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.integrations.listenBrainzEnabled
              ? 'translate-x-5'
              : 'translate-x-0'}"
          ></span>
        </button>
      </div>

      {#if player.integrations.listenBrainzEnabled}
        <div class="pl-12 space-y-2">
          <div class="flex gap-2">
            <input
              type="password"
              value={player.integrations.listenBrainzToken}
              placeholder={t("settings.lb_token_ph")}
              class="flex-1 min-w-0 px-3 py-2 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] focus:border-[var(--accent-border)] text-[12.5px] text-[var(--text)] outline-none"
              oninput={(e) => {
                lbVerifiedUser = null;
                lbVerifyError = null;
                player.updateIntegrations({
                  listenBrainzToken: (e.currentTarget as HTMLInputElement).value,
                });
              }}
            />
            <button
              type="button"
              class="pill-btn shrink-0"
              disabled={!player.integrations.listenBrainzToken.trim() || verifyingLb}
              onclick={() => void verifyListenBrainz()}
            >
              {#if verifyingLb}
                <Loader2 size={13} class="animate-spin" />
              {:else if lbVerifiedUser}
                <CheckCircle2 size={13} class="text-[var(--accent-bright)]" />
              {/if}
              {t("settings.verify")}
            </button>
          </div>
          {#if lbVerifiedUser}
            <p class="text-[11.5px] font-semibold text-[var(--accent-bright)]">
              {t("settings.lb_verified", { user: lbVerifiedUser })}
            </p>
          {:else if lbVerifyError}
            <p class="text-[11.5px] text-red-400">{lbVerifyError}</p>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Last.fm -->
    <div class="p-4 space-y-3 transition-colors hover:bg-[var(--surface-hover)]/35">
      <div class="flex items-center justify-between gap-4">
        <div class="flex items-center gap-3 min-w-0">
          <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
            <Disc3 size={16} />
          </span>
          <div class="min-w-0">
            <div class="text-[14px] font-semibold leading-snug">{t("settings.lastfm")}</div>
            <div class="text-[12px] text-[var(--text-muted)]">{t("settings.lastfm_sub")}</div>
          </div>
        </div>
        <button
          type="button"
          role="switch"
          aria-checked={player.integrations.lastFmEnabled}
          aria-label={t("settings.lastfm")}
          class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.integrations.lastFmEnabled
            ? 'bg-[var(--accent)]'
            : 'bg-[var(--control-track)]'}"
          onclick={() =>
            player.updateIntegrations({
              lastFmEnabled: !player.integrations.lastFmEnabled,
            })}
        >
          <span
            class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.integrations.lastFmEnabled
              ? 'translate-x-5'
              : 'translate-x-0'}"
          ></span>
        </button>
      </div>

      {#if player.integrations.lastFmEnabled}
        <div class="pl-12 grid grid-cols-1 sm:grid-cols-3 gap-2">
          <input
            type="password"
            value={player.integrations.lastFmApiKey}
            placeholder="API Key"
            class="px-3 py-2 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] focus:border-[var(--accent-border)] text-[12.5px] text-[var(--text)] outline-none"
            oninput={(e) =>
              player.updateIntegrations({
                lastFmApiKey: (e.currentTarget as HTMLInputElement).value,
              })}
          />
          <input
            type="password"
            value={player.integrations.lastFmApiSecret}
            placeholder="API Secret"
            class="px-3 py-2 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] focus:border-[var(--accent-border)] text-[12.5px] text-[var(--text)] outline-none"
            oninput={(e) =>
              player.updateIntegrations({
                lastFmApiSecret: (e.currentTarget as HTMLInputElement).value,
              })}
          />
          <input
            type="password"
            value={player.integrations.lastFmSessionKey}
            placeholder="Session Key"
            class="px-3 py-2 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] focus:border-[var(--accent-border)] text-[12.5px] text-[var(--text)] outline-none"
            oninput={(e) =>
              player.updateIntegrations({
                lastFmSessionKey: (e.currentTarget as HTMLInputElement).value,
              })}
          />
        </div>
      {/if}
    </div>
  </div>
</section>
