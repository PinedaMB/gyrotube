<script lang="ts">
  import { PanelRight, PartyPopper, Settings } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import SpinIcon from "../icons/SpinIcon.svelte";

  let spinLive = $derived(
    player.spinStatus.local_active || player.spinStatus.remote_role !== "idle",
  );
</script>

<div class="justify-self-end flex items-center gap-0.5 sm:gap-1.5 shrink-0">
  <button
    type="button"
    class="icon-btn relative"
    class:active={player.spinOpen || spinLive}
    title={t("header.spin_mode")}
    onclick={() => {
      player.spinOpen = !player.spinOpen;
      if (player.spinOpen) void player.refreshSpinStatus();
    }}
  >
    <SpinIcon size={19} spinning={spinLive} />
    {#if spinLive}
      <span
        class="w-2 h-2 rounded-full bg-[var(--gold-bright)] ring-2 ring-[var(--bg-elevated)] absolute top-1 right-1 animate-pulse"
      ></span>
    {/if}
  </button>

  <button
    type="button"
    class="icon-btn hidden sm:inline-flex"
    class:active={player.partyMode}
    title={t("header.party_mode")}
    onclick={() => (player.partyMode = !player.partyMode)}
  >
    <PartyPopper size={18} />
  </button>

  <span class="hidden sm:block w-px h-5 bg-[var(--border-strong)] mx-1" aria-hidden="true"></span>

  <button
    type="button"
    class="icon-btn"
    class:active={player.settingsOpen}
    title={t("header.settings")}
    onclick={() => (player.settingsOpen = true)}
  >
    <Settings size={18} />
  </button>

  <button
    type="button"
    class="icon-btn hidden md:inline-flex"
    class:active={player.sidebarOpen}
    title={t("header.toggle_sidebar")}
    onclick={() => {
      player.sidebarOpen = !player.sidebarOpen;
      void player.persistSession();
    }}
  >
    <PanelRight size={18} />
  </button>
</div>
