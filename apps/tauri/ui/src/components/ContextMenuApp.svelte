<script lang="ts">
  import {
    Compass,
    Home,
    Library,
    PartyPopper,
    Pause,
    Play,
    Settings,
    SkipForward,
  } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { contextMenu } from "../lib/stores/contextMenu.svelte";
  import { player } from "../lib/stores/player.svelte";
  import SpinIcon from "./icons/SpinIcon.svelte";
</script>

{#if player.currentTrack}
  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      void player.togglePlay();
    }}
  >
    {#if player.isPlaying}
      <Pause size={15} fill="currentColor" />
      <span>{t("player.paused")}</span>
    {:else}
      <Play size={15} fill="currentColor" />
      <span>{t("home.play")}</span>
    {/if}
  </button>
  <button
    type="button"
    role="menuitem"
    class="ctx-item"
    onclick={() => {
      contextMenu.close();
      void player.next();
    }}
  >
    <SkipForward size={15} />
    <span>{t("party.up_next")}</span>
  </button>
  <div class="my-1 border-t border-[var(--border)]"></div>
{/if}

<button
  type="button"
  role="menuitem"
  class="ctx-item"
  onclick={() => {
    contextMenu.close();
    player.navigateTab("home");
  }}
>
  <Home size={15} />
  <span>{t("nav.home")}</span>
</button>
<button
  type="button"
  role="menuitem"
  class="ctx-item"
  onclick={() => {
    contextMenu.close();
    player.navigateTab("explore");
  }}
>
  <Compass size={15} />
  <span>{t("nav.explore")}</span>
</button>
<button
  type="button"
  role="menuitem"
  class="ctx-item"
  onclick={() => {
    contextMenu.close();
    player.navigateTab("collection");
  }}
>
  <Library size={15} />
  <span>{t("nav.collection")}</span>
</button>

<div class="my-1 border-t border-[var(--border)]"></div>

<button
  type="button"
  role="menuitem"
  disabled={player.videoMode}
  class="ctx-item"
  onclick={() => {
    contextMenu.close();
    player.togglePartyMode();
  }}
>
  <PartyPopper size={15} />
  <span>{t("header.party_mode")}</span>
</button>
<button
  type="button"
  role="menuitem"
  class="ctx-item"
  onclick={() => {
    contextMenu.close();
    player.openSpin();
    void player.refreshSpinStatus();
  }}
>
  <SpinIcon size={15} />
  <span>{t("header.spin_mode")}</span>
</button>
<button
  type="button"
  role="menuitem"
  class="ctx-item"
  onclick={() => {
    contextMenu.close();
    player.openSettings();
  }}
>
  <Settings size={15} />
  <span>{t("header.settings")}</span>
</button>
