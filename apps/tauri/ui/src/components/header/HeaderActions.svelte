<script lang="ts">
  import { PanelRight, PartyPopper, Search, Settings } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import SpinIcon from "../icons/SpinIcon.svelte";
  import WindowControls from "./WindowControls.svelte";

  let spinLive = $derived(
    player.spinStatus.local_active || player.spinStatus.remote_role !== "idle",
  );
</script>

<div class="justify-self-end flex items-center gap-1 sm:gap-1.5 shrink-0">
  <button
    type="button"
    disabled={player.isOffline}
    class="icon-btn !w-9 !h-9 md:hidden"
    title={t("header.search_placeholder")}
    aria-label={t("header.search_placeholder")}
    onclick={() => player.openSearch()}
  >
    <Search size={19} />
  </button>
  <button
    type="button"
    disabled={player.isOffline && !spinLive}
    class="icon-btn max-md:!w-9 max-md:!h-9 relative"
    class:active={player.spinOpen || spinLive}
    title={t("header.spin_mode")}
    onclick={() => player.toggleSpin()}
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
    disabled={player.videoMode}
    class="icon-btn hidden sm:inline-flex"
    class:active={player.partyMode}
    title={t("header.party_mode")}
    onclick={() => player.togglePartyMode()}
  >
    <PartyPopper size={18} />
  </button>

  <button
    type="button"
    class="icon-btn max-md:!w-9 max-md:!h-9"
    class:active={player.activeView.kind === "settings"}
    title={t("header.settings")}
    aria-label={t("header.settings")}
    onclick={() => player.openSettings()}
  >
    <Settings size={18} />
  </button>

  <button
    type="button"
    class="icon-btn hidden md:inline-flex"
    class:active={player.sidebarOpen}
    title={t("header.toggle_sidebar")}
    onclick={() => {
      player.togglePlayerSheet();
    }}
  >
    <PanelRight size={18} />
  </button>

  <WindowControls />
</div>
