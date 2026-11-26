<script lang="ts">
  import type { ContextPayload } from "../lib/stores/contextMenu.svelte";
  import ContextMenuApp from "./ContextMenuApp.svelte";
  import ContextMenuTrack from "./ContextMenuTrack.svelte";

  let { payload }: { payload: ContextPayload } = $props();
</script>

<div class="ctx-menu-root">
  {#if payload.kind === "track"}
    <ContextMenuTrack {payload} />
  {:else}
    <ContextMenuApp />
  {/if}
</div>

<style>
  .ctx-menu-root :global(.ctx-item) {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 11px;
    text-align: left;
    font-weight: 500;
    color: var(--text);
    transition: background-color 110ms ease;
  }

  @media (max-width: 767px) {
    .ctx-menu-root :global(.ctx-item) {
      padding: 12px 12px;
      gap: 12px;
    }
  }

  .ctx-menu-root :global(.ctx-item:hover) {
    background: var(--border);
  }
</style>
