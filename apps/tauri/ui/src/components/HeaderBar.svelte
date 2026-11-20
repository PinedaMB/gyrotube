<script lang="ts">
  import HeaderActions from "./header/HeaderActions.svelte";
  import HeaderBrand from "./header/HeaderBrand.svelte";
  import HeaderSearch from "./header/HeaderSearch.svelte";
</script>

<header
  class="h-14 shrink-0 z-30 grid grid-cols-[1fr_minmax(240px,500px)_1fr] items-center gap-3 px-4 bg-[var(--bg-elevated)]/95 backdrop-blur-md border-b border-[var(--border)] select-none"
>
  <HeaderBrand />
  <HeaderSearch />
  <HeaderActions />
</header>
