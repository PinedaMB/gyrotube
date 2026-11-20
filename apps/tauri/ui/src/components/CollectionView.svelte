<script lang="ts">
  import { Plus } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import type { UserPlaylist } from "../lib/types";
  import CollectionPlaylistDetail from "./collection/CollectionPlaylistDetail.svelte";
  import CollectionPlaylistModal from "./collection/CollectionPlaylistModal.svelte";
  import CollectionPlaylistsGrid from "./collection/CollectionPlaylistsGrid.svelte";
  import CollectionRecentlyPlayed from "./collection/CollectionRecentlyPlayed.svelte";

  type CollectionScreen =
    | { kind: "overview" }
    | { kind: "favorites" }
    | { kind: "playlist"; id: string };

  let screen = $state<CollectionScreen>({ kind: "overview" });
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
  }) {
    if (data.id) {
      player.updateUserPlaylist(data.id, data.name, data.description, data.colorHex, data.iconName);
    } else {
      const created = player.createUserPlaylist(
        data.name,
        data.description,
        data.colorHex,
        data.iconName,
      );
      screen = { kind: "playlist", id: created.id };
    }
  }

  function handleDeletePlaylist(pl: UserPlaylist) {
    player.deleteUserPlaylist(pl.id);
    if (screen.kind === "playlist" && screen.id === pl.id) {
      screen = { kind: "overview" };
    }
  }
</script>

{#if screen.kind === "favorites"}
  <CollectionPlaylistDetail
    mode="favorites"
    onBack={() => (screen = { kind: "overview" })}
  />
{:else if screen.kind === "playlist" && activePlaylist}
  <CollectionPlaylistDetail
    mode="playlist"
    playlist={activePlaylist}
    onBack={() => (screen = { kind: "overview" })}
    onEdit={openEditModal}
    onDelete={handleDeletePlaylist}
  />
{:else}
  <div class="page-clamp">
    <div class="flex items-center justify-between gap-3">
      <div>
        <h1 class="text-[28px] font-extrabold tracking-[-0.025em] leading-tight">{t("collection.title")}</h1>
        <p class="text-[13px] text-[var(--text-muted)] mt-0.5">
          {t("collection.subtitle")}
        </p>
      </div>
      <button class="pill-btn primary !px-4" onclick={openCreateModal}>
        <Plus size={15} />
        {t("collection.new_playlist")}
      </button>
    </div>

    <CollectionPlaylistsGrid
      onOpenFavorites={() => (screen = { kind: "favorites" })}
      onOpenPlaylist={(id) => (screen = { kind: "playlist", id })}
      onEditPlaylist={openEditModal}
    />

    <CollectionRecentlyPlayed onCreatePlaylist={openCreateModal} />
  </div>
{/if}

<CollectionPlaylistModal
  open={modalOpen}
  editing={editingPlaylist}
  onClose={() => (modalOpen = false)}
  onSave={handleSavePlaylist}
/>
