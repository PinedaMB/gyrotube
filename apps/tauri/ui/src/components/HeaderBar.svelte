<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import HeaderActions from "./header/HeaderActions.svelte";
  import HeaderBrand from "./header/HeaderBrand.svelte";
  import HeaderSearch from "./header/HeaderSearch.svelte";

  let hideOnMobile = $derived(
    player.activeView.kind === "album" ||
      player.activeView.kind === "artist" ||
      player.activeView.kind === "search" ||
      (player.activeView.kind === "collection" && Boolean(player.activeView.sub)),
  );
</script>

<header
  class="{hideOnMobile
    ? 'hidden md:grid'
    : 'flex md:grid'} relative min-h-[60px] md:min-h-0 md:h-14 pt-[max(0.5rem,env(safe-area-inset-top))] pb-2 md:py-0 shrink-0 z-30 justify-between md:grid-cols-[minmax(max-content,1fr)_minmax(0,500px)_minmax(max-content,1fr)] items-center gap-2.5 md:gap-3 px-3.5 md:px-4 bg-[var(--bg-elevated)] border-b border-[var(--border)] select-none"
>
  <HeaderBrand />
  <div class="hidden md:block flex-1 min-w-0 md:w-full">
    <HeaderSearch />
  </div>
  <HeaderActions />
</header>
