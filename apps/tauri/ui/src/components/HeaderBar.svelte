<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { isTauriRuntime } from "../lib/api";
  import { device } from "../lib/stores/device.svelte";
  import { player } from "../lib/stores/player.svelte";
  import HeaderActions from "./header/HeaderActions.svelte";
  import HeaderBrand from "./header/HeaderBrand.svelte";
  import HeaderSearch from "./header/HeaderSearch.svelte";

  const isDesktopTauri = isTauriRuntime() && !device.isAndroid;

  let hideOnMobile = $derived(
    player.activeView.kind === "album" ||
      player.activeView.kind === "artist" ||
      player.activeView.kind === "search" ||
      (player.activeView.kind === "collection" && Boolean(player.activeView.sub)),
  );

  function isInteractiveTarget(target: EventTarget | null): boolean {
    if (!(target instanceof Element)) return false;
    return Boolean(target.closest("button, input, a, select, textarea, [role='button']"));
  }

  function handleHeaderMouseDown(e: MouseEvent) {
    if (!isDesktopTauri || e.button !== 0 || isInteractiveTarget(e.target)) return;
    if (e.detail === 2) {
      void getCurrentWindow().toggleMaximize().catch(() => {});
    } else {
      void getCurrentWindow().startDragging().catch(() => {});
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<header
  data-tauri-drag-region={isDesktopTauri ? "" : undefined}
  onmousedown={handleHeaderMouseDown}
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
