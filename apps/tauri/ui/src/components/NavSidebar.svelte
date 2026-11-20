<script lang="ts">
  import { Compass, Heart, Home, Library } from "lucide-svelte";
  import { t } from "../lib/i18n.svelte";
  import { player } from "../lib/stores/player.svelte";
  import PlaylistIcon from "./collection/PlaylistIcon.svelte";

  let collapsed = $derived(player.navCollapsed);
</script>

<!-- Desktop / tablet left navigation sidebar -->
<aside
  class="hidden md:flex flex-col shrink-0 h-full border-r border-[var(--border)] bg-[var(--bg-sidebar)] select-none transition-[width] duration-200 ease-out overflow-hidden {collapsed
    ? 'w-[72px]'
    : 'w-[228px]'}"
>
  <!-- Primary navigation -->
  <nav class="p-3 space-y-1" aria-label="Main navigation">
    <button
      class="w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-[13.5px] font-semibold transition-all active:scale-[0.98] {collapsed
        ? 'justify-center px-0'
        : ''} {player.activeTab === 'home'
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
        : ''} {player.activeTab === 'explore'
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
        : ''} {player.activeTab === 'collection'
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
      <div class="px-2.5 pt-1 pb-1.5 text-[10.5px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)]">
        {t("collection.local_playlists")}
      </div>
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
          class="w-7 h-7 rounded-lg inline-flex items-center justify-center text-white shrink-0"
          style="background: {pl.color_hex || '#2190a4'};"
        >
          <PlaylistIcon name={pl.icon_name} size={13} />
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

<!-- Mobile bottom navigation bar (< md) -->
<nav
  class="md:hidden fixed bottom-0 left-0 right-0 z-30 h-14 grid grid-cols-3 bg-[var(--bg-elevated)]/95 backdrop-blur-lg border-t border-[var(--border)] px-2"
  aria-label="Mobile navigation"
>
  <button
    class="flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold {player.activeTab === 'home'
      ? 'text-[var(--accent-bright)]'
      : 'text-[var(--text-muted)]'}"
    onclick={() => player.navigateTab("home")}
  >
    <Home size={19} />
    <span>{t("nav.home")}</span>
  </button>
  <button
    class="flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold {player.activeTab === 'explore'
      ? 'text-[var(--accent-bright)]'
      : 'text-[var(--text-muted)]'}"
    onclick={() => player.navigateTab("explore")}
  >
    <Compass size={19} />
    <span>{t("nav.explore")}</span>
  </button>
  <button
    class="flex flex-col items-center justify-center gap-0.5 text-[11px] font-semibold {player.activeTab === 'collection'
      ? 'text-[var(--accent-bright)]'
      : 'text-[var(--text-muted)]'}"
    onclick={() => player.navigateTab("collection")}
  >
    <Library size={19} />
    <span>{t("nav.collection")}</span>
  </button>
</nav>
