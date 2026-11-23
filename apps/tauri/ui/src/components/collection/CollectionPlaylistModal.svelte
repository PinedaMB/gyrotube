<script lang="ts">
  import { Check, Link2, Loader2, X } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track, UserPlaylist } from "../../lib/types";
  import { parseYtmUrl } from "../../lib/ytmUrl";
  import ModalSheet from "../ModalSheet.svelte";
  import PlaylistIcon, { PLAYLIST_ICON_IDS } from "./PlaylistIcon.svelte";

  const SWATCHES = [
    "#10b981",
    "#3584e4",
    "#9141ac",
    "#c061cb",
    "#e01b24",
    "#e66100",
    "#f5c211",
    "#2ec27e",
    "#0ea5e9",
    "#64748b",
  ];

  interface Props {
    open: boolean;
    editing: UserPlaylist | null;
    onClose: () => void;
    onSave: (data: {
      id: string | null;
      name: string;
      description: string;
      colorHex: string | null;
      iconName: string;
      tracks?: Track[];
    }) => void;
  }

  let { open, editing, onClose, onSave }: Props = $props();

  let formName = $state("");
  let formDesc = $state("");
  let formColor = $state<string | null>(SWATCHES[0]);
  let formIcon = $state<string>("music");
  let importUrl = $state("");
  let importing = $state(false);

  $effect(() => {
    if (open) {
      formName = editing?.name ?? "";
      formDesc = editing?.description ?? "";
      formColor = editing?.color_hex ?? SWATCHES[0];
      formIcon = editing?.icon_name ?? "music";
      importUrl = "";
      importing = false;
    }
  });

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (importing) return;

    const parsedLink = !editing ? parseYtmUrl(importUrl) : null;
    let importedTracks: Track[] | undefined;
    let clean = formName.trim();

    if (parsedLink && (parsedLink.kind === "playlist" || parsedLink.kind === "album")) {
      if (!isTauriRuntime()) return;
      importing = true;
      try {
        const [resolvedTitle, resolvedTracks] =
          parsedLink.kind === "album"
            ? await api.getAlbumTracks(parsedLink.id)
            : await api.getPlaylistTracks(parsedLink.id);
        importedTracks = resolvedTracks;
        if (!clean) clean = resolvedTitle;
      } catch {
        player.showToast(t("url.import_failed"));
        importing = false;
        return;
      } finally {
        importing = false;
      }
    }

    if (!clean) return;
    onSave({
      id: editing?.id ?? null,
      name: clean,
      description: formDesc.trim(),
      colorHex: formColor,
      iconName: formIcon,
      tracks: importedTracks,
    });
    if (importedTracks && importedTracks.length > 0) {
      player.showToast(t("url.imported_count", { count: importedTracks.length }));
    }
    onClose();
  }

  let previewColor = $derived(formColor ?? "#10b981");
</script>

<ModalSheet
  {open}
  label={editing ? t("collection.modal_edit") : t("collection.modal_new")}
  maxWidth={500}
  {onClose}
>
  <form class="p-6 flex flex-col gap-5" onsubmit={handleSubmit}>
    <div class="flex items-center justify-between">
      <h3 class="text-[18px] font-extrabold tracking-tight">
        {editing ? t("collection.modal_edit") : t("collection.modal_new")}
      </h3>
      <button type="button" class="icon-btn" onclick={onClose} title={t("common.close")}>
        <X size={17} />
      </button>
    </div>

    <!-- Live preview card -->
    <div
      class="flex items-center gap-4 p-4 rounded-2xl border border-white/10 transition-all duration-200"
      style="background: linear-gradient(135deg, {previewColor}38, var(--surface) 75%);"
    >
      <div
        class="w-16 h-16 rounded-2xl flex items-center justify-center text-white shadow-lg shrink-0 transition-colors"
        style="background: {previewColor};"
      >
        <PlaylistIcon name={formIcon} size={28} />
      </div>
      <div class="min-w-0 flex-1">
        <span class="text-[10px] font-extrabold uppercase tracking-[0.09em] text-[var(--accent-bright)]">
          {t("collection.local_playlist")}
        </span>
        <div class="text-[17px] font-extrabold truncate">
          {formName.trim() || t("collection.untitled")}
        </div>
        <div class="text-[12px] text-[var(--text-muted)] truncate">
          {formDesc.trim() || t("collection.default_desc")}
        </div>
      </div>
    </div>

    <!-- Name & description -->
    <div class="space-y-3">
      <label class="flex flex-col gap-1.5 text-[12.5px] font-semibold">
        <span class="text-[var(--text-muted)]">{t("collection.modal_name")}</span>
        <input
          type="text"
          bind:value={formName}
          placeholder={t("collection.modal_name_ph")}
          required={!parseYtmUrl(importUrl)}
          class="w-full px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border-strong)] focus:border-[var(--accent-border)] outline-none text-[14px]"
        />
      </label>

      {#if !editing && !player.isOffline}
        <label class="flex flex-col gap-1.5 text-[12.5px] font-semibold">
          <span class="text-[var(--text-muted)] inline-flex items-center gap-1.5">
            <Link2 size={13} />
            {t("url.import_label")}
          </span>
          <input
            type="url"
            bind:value={importUrl}
            placeholder={t("url.import_placeholder")}
            disabled={importing}
            class="w-full px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:border-[var(--accent-border)] outline-none text-[13px]"
          />
          {#if importing}
            <span class="inline-flex items-center gap-1.5 text-[11.5px] text-[var(--accent-bright)]">
              <Loader2 size={12} class="animate-spin" />
              {t("url.importing")}
            </span>
          {/if}
        </label>
      {/if}

      <label class="flex flex-col gap-1.5 text-[12.5px] font-semibold">
        <span class="text-[var(--text-muted)]">{t("collection.modal_desc")}</span>
        <input
          type="text"
          bind:value={formDesc}
          placeholder={t("collection.modal_desc_ph")}
          class="w-full px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:border-[var(--accent-border)] outline-none text-[13.5px]"
        />
      </label>
    </div>

    <!-- Color swatches -->
    <div class="space-y-2">
      <div class="flex items-center justify-between text-[12.5px] font-semibold">
        <span class="text-[var(--text-muted)]">{t("collection.modal_color")}</span>
        <button
          type="button"
          class="px-2.5 py-0.5 rounded-full text-[11px] font-bold transition-colors {formColor === null
            ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
            : 'bg-[var(--surface)] text-[var(--text-muted)] hover:text-[var(--text)]'}"
          onclick={() => (formColor = null)}
        >
          {t("collection.modal_auto")}
        </button>
      </div>
      <div class="flex items-center flex-wrap gap-2">
        {#each SWATCHES as hex}
          <button
            type="button"
            class="w-7 h-7 rounded-full inline-flex items-center justify-center text-white transition-transform hover:scale-110 {formColor === hex
              ? 'ring-2 ring-white ring-offset-2 ring-offset-[var(--bg-elevated)] scale-110'
              : ''}"
            style="background: {hex};"
            aria-label={hex}
            onclick={() => (formColor = hex)}
          >
            {#if formColor === hex}
              <Check size={14} strokeWidth={3} />
            {/if}
          </button>
        {/each}
      </div>
    </div>

    <!-- Icon picker -->
    <div class="space-y-2">
      <span class="text-[12.5px] font-semibold text-[var(--text-muted)]">{t("collection.modal_icon")}</span>
      <div class="grid grid-cols-8 gap-1.5">
        {#each PLAYLIST_ICON_IDS as ic}
          <button
            type="button"
            class="h-10 rounded-xl inline-flex items-center justify-center border transition-all {formIcon === ic
              ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)] scale-105'
              : 'bg-[var(--surface)] border-[var(--border)] text-[var(--text-muted)] hover:bg-[var(--surface-hover)] hover:text-[var(--text)]'}"
            aria-label={ic}
            onclick={() => (formIcon = ic)}
          >
            <PlaylistIcon name={ic} size={17} />
          </button>
        {/each}
      </div>
    </div>

    <div class="flex items-center justify-end gap-2 pt-1">
      <button type="button" class="pill-btn" onclick={onClose}>{t("collection.modal_cancel")}</button>
      <button type="submit" class="pill-btn primary !px-5">
        {editing ? t("collection.modal_save") : t("collection.modal_create")}
      </button>
    </div>
  </form>
</ModalSheet>
