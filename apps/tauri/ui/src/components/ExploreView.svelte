<script lang="ts">
  import { player } from "../lib/stores/player.svelte";
  import ExploreGenresGrid from "./explore/ExploreGenresGrid.svelte";
  import ExploreSearchResults from "./explore/ExploreSearchResults.svelte";
  import ExploreSkeleton from "./explore/ExploreSkeleton.svelte";
  import OfflineState from "./OfflineState.svelte";
</script>

<div class="page-clamp">
  {#if player.isOffline}
    <OfflineState />
  {:else if player.searchLoading}
    <ExploreSkeleton />
  {:else if player.searchResults}
    <ExploreSearchResults results={player.searchResults} />
  {:else}
    <ExploreGenresGrid genres={player.genres} />
  {/if}
</div>
