<script lang="ts">
  import { Plus } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { UserPlaylist } from "../lib/types";
  import CollectionPlaylistDetail from "./collection/CollectionPlaylistDetail.svelte";
  import CollectionPlaylistsGrid from "./collection/CollectionPlaylistsGrid.svelte";
  import CollectionRecentlyPlayed from "./collection/CollectionRecentlyPlayed.svelte";
  import CollectionStatsCard from "./collection/CollectionStatsCard.svelte";
  import Carousel from "./Carousel.svelte";
  import CoverImage from "./CoverImage.svelte";

  type CollectionScreen =
    | { kind: "overview" }
    | { kind: "favorites" }
    | { kind: "playlist"; id: string };

  let screen = $derived.by<CollectionScreen>(() => {
    if (player.activeView.kind === "collection" && player.activeView.sub) {
      return player.activeView.sub;
    }
    return { kind: "overview" };
  });
  let modalOpen = $state(false);
  let editingPlaylist = $state<UserPlaylist | null>(null);

  let activePlaylist = $derived.by(() => {
    const s = screen;
    if (s.kind !== "playlist") return null;
    return player.userPlaylists.find((p) => p.id === s.id) ?? null;
  });

  function openCreateModal() {
    editingPlaylist = null;
    modalOpen = true;
  }

  function openEditModal(pl: UserPlaylist) {
    editingPlaylist = pl;
    modalOpen = true;
  }

  function handleSavePlaylist(data: {
    id: string | null;
    name: string;
    description: string;
    colorHex: string | null;
    iconName: string;
    coverPath?: string | null;
    tracks?: import("../lib/types").Track[];
  }) {
    if (data.id) {
      player.updateUserPlaylist(
        data.id,
        data.name,
        data.description,
        data.colorHex,
        data.iconName,
        data.coverPath ?? null,
      );
    } else {
      const created = player.createUserPlaylist(
        data.name,
        data.description,
        data.colorHex,
        data.iconName,
        data.tracks ?? [],
        data.coverPath ?? null,
      );
      player.openLocalPlaylist(created.id);
    }
  }

  function handleDeletePlaylist(pl: UserPlaylist) {
    player.deleteUserPlaylist(pl.id);
    if (screen.kind === "playlist" && screen.id === pl.id) {
      player.navigateTab("collection");
    }
  }
</script>

{#if screen.kind === "favorites"}
  <CollectionPlaylistDetail
    mode="favorites"
    onBack={() => player.goBack()}
  />
{:else if screen.kind === "playlist" && activePlaylist}
  <CollectionPlaylistDetail
    mode="playlist"
    playlist={activePlaylist}
    onBack={() => player.goBack()}
    onEdit={openEditModal}
    onDelete={handleDeletePlaylist}
  />
{:else}
  <div class="page-clamp">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div class="min-w-0 flex-1">
        <h1 class="text-[26px] sm:text-[28px] font-extrabold tracking-[-0.025em] leading-tight">{t("collection.title")}</h1>
        <p class="text-[13px] text-[var(--text-muted)] mt-0.5">
          {t("collection.subtitle")}
        </p>
      </div>
      <button type="button" class="pill-btn primary !px-3.5 sm:!px-4" onclick={openCreateModal}>
        <Plus size={15} />
        {t("collection.new_playlist")}
      </button>
    </div>

    <CollectionPlaylistsGrid
      onOpenFavorites={() => player.openLocalPlaylist("favorites")}
      onOpenPlaylist={(id) => player.openLocalPlaylist(id)}
      onEditPlaylist={openEditModal}
    />

    {#if player.followedArtists.length > 0}
      <div class="mt-7">
        <Carousel title={t("collection.followed_artists")} minCardWidth={148}>
          {#each player.followedArtists as artist (artist.id)}
            <button
              type="button"
              disabled={player.isOffline}
              class="p-2.5 rounded-2xl hover:bg-[var(--surface-hover)] flex flex-col items-center gap-2 text-center transition-colors"
              onclick={() => player.openArtist(artist.id, artist.name)}
            >
              <CoverImage class="artist-avatar" src={artist.avatar_url} alt={artist.name} fallbackIcon="👤" />
              <strong class="text-[13.5px] font-bold truncate w-full">{artist.name}</strong>
            </button>
          {/each}
        </Carousel>
      </div>
    {/if}

    <CollectionStatsCard />

    <CollectionRecentlyPlayed />
  </div>
{/if}

{#if modalOpen}
  {#await import("./collection/CollectionPlaylistModal.svelte") then { default: CollectionPlaylistModal }}
    <CollectionPlaylistModal
      open={modalOpen}
      editing={editingPlaylist}
      onClose={() => (modalOpen = false)}
      onSave={handleSavePlaylist}
    />
  {/await}
{/if}
