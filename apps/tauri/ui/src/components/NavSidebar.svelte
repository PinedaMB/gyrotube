<script lang="ts">
  import { Compass, Heart, Home, Library, Plus } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import CollectionPlaylistModal from "./collection/CollectionPlaylistModal.svelte";
  import PlaylistIcon from "./collection/PlaylistIcon.svelte";

  let collapsed = $derived(player.navCollapsed);
  let createModalOpen = $state(false);
  let isSettings = $derived(player.activeView.kind === "settings");
</script>

<!-- Desktop / tablet left navigation sidebar -->
<aside
  class="hidden md:flex flex-col shrink-0 h-full border-r border-[var(--border)] bg-[var(--bg-sidebar)] select-none transition-[width] duration-200 ease-out overflow-hidden {collapsed
    ? 'w-[74px]'
    : 'w-[240px]'}"
>
  <!-- Primary navigation -->
  <nav class="p-3 space-y-1" aria-label="Main navigation">
    <button
      class="w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-[13.5px] font-semibold transition-all active:scale-[0.98] {collapsed
        ? 'justify-center px-0'
        : ''} {!isSettings && player.activeTab === 'home'
        ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] shadow-sm'
        : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      title={t("nav.home")}
      onclick={() => player.navigateTab("home")}
    >
      <Home size={19} class="shrink-0" />
      {#if !collapsed}
        <span class="truncate">{t("nav.home")}</span>
      {/if}
    </button>

    <button
      class="w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-[13.5px] font-semibold transition-all active:scale-[0.98] {collapsed
        ? 'justify-center px-0'
        : ''} {!isSettings && player.activeTab === 'explore'
        ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] shadow-sm'
        : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      title={t("nav.explore")}
      onclick={() => player.navigateTab("explore")}
    >
      <Compass size={19} class="shrink-0" />
      {#if !collapsed}
        <span class="truncate">{t("nav.explore")}</span>
      {/if}
    </button>

    <button
      class="w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-[13.5px] font-semibold transition-all active:scale-[0.98] {collapsed
        ? 'justify-center px-0'
        : ''} {!isSettings && player.activeTab === 'collection'
        ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] shadow-sm'
        : 'text-[var(--text-muted)] hover:bg-[var(--surface)] hover:text-[var(--text)]'}"
      title={t("nav.collection")}
      onclick={() => player.navigateTab("collection")}
    >
      <Library size={19} class="shrink-0" />
      {#if !collapsed}
        <span class="truncate">{t("nav.collection")}</span>
      {/if}
    </button>
  </nav>

  <!-- Library shortcuts -->
  <div class="mx-3 border-t border-[var(--border)]"></div>

  <div class="flex-1 min-h-0 overflow-y-auto scroll-y p-3 space-y-1">
    {#if !collapsed}
      <div class="flex items-center justify-between gap-2 px-2.5 pt-1 pb-1.5">
        <span class="text-[10.5px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] truncate">
          {t("collection.local_playlists")}
        </span>
        <button
          type="button"
          class="w-6 h-6 rounded-lg inline-flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] transition-colors shrink-0"
          title={t("collection.new_playlist")}
          aria-label={t("collection.new_playlist")}
          onclick={() => (createModalOpen = true)}
        >
          <Plus size={14} />
        </button>
      </div>
    {:else}
      <button
        type="button"
        class="w-full flex items-center justify-center rounded-xl py-2 text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface)] transition-colors"
        title={t("collection.new_playlist")}
        aria-label={t("collection.new_playlist")}
        onclick={() => (createModalOpen = true)}
      >
        <span class="w-7 h-7 rounded-lg inline-flex items-center justify-center border border-dashed border-[var(--border-strong)]">
          <Plus size={14} />
        </span>
      </button>
    {/if}

    <!-- Favorites quick shortcut -->
    <button
      class="w-full flex items-center gap-2.5 rounded-xl px-2.5 py-2 text-left transition-colors hover:bg-[var(--surface)] {collapsed
        ? 'justify-center px-0'
        : ''}"
      title={t("collection.favorites")}
      onclick={() => {
        if (player.favorites.length > 0) {
          player.replaceQueueAndPlay(player.favorites, 0, t("collection.favorites"));
        } else {
          player.navigateTab("collection");
        }
      }}
    >
      <span
        class="w-7 h-7 rounded-lg inline-flex items-center justify-center text-white shrink-0"
        style="background: linear-gradient(135deg, #e01b24, #ff6b81);"
      >
        <Heart size={13} fill="currentColor" />
      </span>
      {#if !collapsed}
        <span class="min-w-0 flex-1">
          <span class="block text-[13px] font-semibold truncate">{t("collection.favorites")}</span>
          <span class="block text-[11px] text-[var(--text-muted)] numeric">
            {player.favorites.length === 1
              ? t("collection.song_one")
              : t("collection.song_many", { count: player.favorites.length })}
          </span>
        </span>
      {/if}
    </button>

    {#each player.userPlaylists as pl (pl.id)}
      <button
        class="w-full flex items-center gap-2.5 rounded-xl px-2.5 py-2 text-left transition-colors hover:bg-[var(--surface)] {collapsed
          ? 'justify-center px-0'
          : ''}"
        title={pl.name}
        onclick={() => {
          if (pl.tracks.length > 0) {
            player.replaceQueueAndPlay(pl.tracks, 0, pl.name);
          } else {
            player.navigateTab("collection");
          }
        }}
      >
        <span
          class="w-7 h-7 rounded-lg inline-flex items-center justify-center text-white shrink-0 overflow-hidden"
          style="background: {pl.color_hex || '#10b981'};"
        >
          {#if pl.cover_path}
            <img src={pl.cover_path} alt={pl.name} class="w-full h-full object-cover" />
          {:else}
            <PlaylistIcon name={pl.icon_name} size={13} />
          {/if}
        </span>
        {#if !collapsed}
          <span class="min-w-0 flex-1">
            <span class="block text-[13px] font-semibold truncate">{pl.name}</span>
            <span class="block text-[11px] text-[var(--text-muted)] numeric">
              {pl.tracks.length === 1
                ? t("collection.song_one")
                : t("collection.song_many", { count: pl.tracks.length })}
            </span>
          </span>
        {/if}
      </button>
    {/each}
  </div>
</aside>

<CollectionPlaylistModal
  open={createModalOpen}
  editing={null}
  onClose={() => (createModalOpen = false)}
  onSave={({ name, description, colorHex, iconName, coverPath, tracks }) => {
    player.createUserPlaylist(name, description, colorHex, iconName, tracks ?? [], coverPath ?? null);
  }}
/>
