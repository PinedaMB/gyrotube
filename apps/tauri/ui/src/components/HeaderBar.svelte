<script lang="ts">
  import HeaderActions from "./header/HeaderActions.svelte";
  import HeaderBrand from "./header/HeaderBrand.svelte";
  import HeaderSearch from "./header/HeaderSearch.svelte";
</script>

<header
  class="h-14 shrink-0 z-30 flex md:grid md:grid-cols-[1fr_minmax(240px,500px)_1fr] items-center gap-2 md:gap-3 px-3 md:px-4 bg-[var(--bg-elevated)]/95 backdrop-blur-md border-b border-[var(--border)] select-none"
>
  <HeaderBrand />
  <div class="flex-1 min-w-0 md:w-full">
    <HeaderSearch />
  </div>
  <HeaderActions />
</header>
