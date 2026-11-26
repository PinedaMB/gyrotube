<script lang="ts">
  import { Check, ImagePlus, Link2, Loader2, Plus, Trash2, X } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { extractDominantRgb } from "../../lib/audioEngine";
  import { rgbToHex } from "../../lib/colorUtils";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track, UserPlaylist } from "../../lib/types";
  import { parseYtmUrl } from "../../lib/ytmUrl";
  import ModalSheet from "../ModalSheet.svelte";
  import SegmentedControl, { type SegmentedOption } from "../SegmentedControl.svelte";
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

  type ModalMode = "create" | "import";

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
      coverPath?: string | null;
      tracks?: Track[];
    }) => void;
  }

  let { open, editing, onClose, onSave }: Props = $props();

  let mode = $state<ModalMode>("create");
  let formName = $state("");
  let formDesc = $state("");
  let formColor = $state<string | null>(null);
  let formIcon = $state<string>("music");
  let formCover = $state<string | null>(null);
  let extractedCoverHex = $state<string | null>(null);
  let coverInputEl = $state<HTMLInputElement | null>(null);
  let importUrl = $state("");
  let importing = $state(false);

  $effect(() => {
    const cover = formCover;
    if (!cover) {
      extractedCoverHex = null;
      return;
    }
    let cancelled = false;
    void extractDominantRgb(cover).then((rgb) => {
      if (!cancelled) {
        extractedCoverHex = rgb ? rgbToHex(rgb) : null;
      }
    });
    return () => {
      cancelled = true;
    };
  });

  $effect(() => {
    if (open) {
      mode = "create";
      formName = editing?.name ?? "";
      formDesc = editing?.description ?? "";
      formColor = editing?.color_hex ?? null;
      formIcon = editing?.icon_name ?? "music";
      formCover = editing?.cover_path ?? null;
      importUrl = "";
      importing = false;
    }
  });

  async function handleCoverFile(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    input.value = "";
    if (!file) return;
    try {
      const bitmapUrl = URL.createObjectURL(file);
      const img = await new Promise<HTMLImageElement>((resolve, reject) => {
        const el = new Image();
        el.onload = () => resolve(el);
        el.onerror = () => reject(new Error("load failed"));
        el.src = bitmapUrl;
      });
      URL.revokeObjectURL(bitmapUrl);

      const size = 512;
      const canvas = document.createElement("canvas");
      canvas.width = size;
      canvas.height = size;
      const ctx = canvas.getContext("2d");
      if (!ctx) throw new Error("no 2d context");

      // Center square crop
      const srcSize = Math.min(img.naturalWidth || img.width, img.naturalHeight || img.height);
      const sx = Math.max(0, Math.floor(((img.naturalWidth || img.width) - srcSize) / 2));
      const sy = Math.max(0, Math.floor(((img.naturalHeight || img.height) - srcSize) / 2));
      ctx.drawImage(img, sx, sy, srcSize, srcSize, 0, 0, size, size);
      formCover = canvas.toDataURL("image/jpeg", 0.84);
      formColor = null;
    } catch {
      player.showToast(t("collection.cover_invalid"));
    }
  }

  let parsedImport = $derived.by(() => {
    const parsed = parseYtmUrl(importUrl);
    return parsed && (parsed.kind === "playlist" || parsed.kind === "album") ? parsed : null;
  });

  let modeOptions = $derived<SegmentedOption<ModalMode>[]>([
    { value: "create", label: t("url.tab_create"), icon: Plus },
    { value: "import", label: t("url.tab_import"), icon: Link2 },
  ]);

  let canSubmit = $derived.by(() => {
    if (importing) return false;
    if (!editing && mode === "import") return parsedImport != null;
    return formName.trim().length > 0;
  });

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!canSubmit) return;

    let importedTracks: Track[] | undefined;
    let clean = formName.trim();

    if (!editing && mode === "import" && parsedImport) {
      if (!isTauriRuntime()) return;
      importing = true;
      try {
        const [resolvedTitle, resolvedTracks] =
          parsedImport.kind === "album"
            ? await api.getAlbumTracks(parsedImport.id)
            : await api.getPlaylistTracks(parsedImport.id);
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
      coverPath: formCover,
      tracks: importedTracks,
    });
    if (importedTracks && importedTracks.length > 0) {
      player.showToast(t("url.imported_count", { count: importedTracks.length }));
    }
    onClose();
  }

  let previewColor = $derived(formColor ?? extractedCoverHex ?? "#10b981");
</script>

<ModalSheet
  {open}
  label={editing ? t("collection.modal_edit") : t("collection.modal_new")}
  maxWidth={500}
  {onClose}
>
  <form class="p-5 sm:p-6 flex flex-col gap-4 sm:gap-5" onsubmit={handleSubmit}>
    <div class="flex items-center justify-between gap-2">
      <h3 class="text-[18px] font-extrabold tracking-tight">
        {editing ? t("collection.modal_edit") : t("collection.modal_new")}
      </h3>
      <button type="button" class="icon-btn" onclick={onClose} title={t("common.close")}>
        <X size={17} />
      </button>
    </div>

    {#if !editing && !player.isOffline}
      <div class="flex justify-start">
        <SegmentedControl
          options={modeOptions}
          value={mode}
          ariaLabel={t("collection.modal_new")}
          onchange={(next) => (mode = next)}
        />
      </div>
    {/if}

    <!-- Live preview card -->
    <div
      class="flex items-center gap-3.5 p-3.5 rounded-2xl border border-[var(--border)] transition-all duration-200"
      style="background: linear-gradient(135deg, {previewColor}32, var(--surface) 72%);"
    >
      <button
        type="button"
        class="group relative w-16 h-16 rounded-2xl flex items-center justify-center text-white shadow-lg shrink-0 overflow-hidden transition-colors"
        style="background: {previewColor};"
        title={t("collection.upload_cover")}
        onclick={() => coverInputEl?.click()}
      >
        {#if formCover}
          <img src={formCover} alt="" class="w-full h-full object-cover" />
        {:else}
          <PlaylistIcon name={formIcon} size={26} />
        {/if}
        <span
          class="absolute inset-0 bg-black/45 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity"
        >
          <ImagePlus size={18} class="text-white" />
        </span>
      </button>
      <input
        bind:this={coverInputEl}
        type="file"
        accept="image/*"
        class="hidden"
        onchange={(e) => void handleCoverFile(e)}
      />
      <div class="min-w-0 flex-1">
        <span
          class="text-[10px] font-extrabold uppercase tracking-[0.09em]"
          style="color: {previewColor};"
        >
          {t("collection.local_playlist")}
        </span>
        <div class="text-[16.5px] font-extrabold truncate">
          {formName.trim() || t("collection.untitled")}
        </div>
        <div class="text-[12px] text-[var(--text-muted)] truncate">
          {formDesc.trim() || t("collection.default_desc")}
        </div>
        <div class="flex items-center gap-2 mt-1.5">
          <button
            type="button"
            class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-[var(--bg)]/60 hover:bg-[var(--surface-hover)] border border-[var(--border)] transition-colors"
            onclick={() => coverInputEl?.click()}
          >
            <ImagePlus size={11} />
            {t("collection.upload_cover")}
          </button>
          {#if formCover}
            <button
              type="button"
              class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[11px] font-semibold text-red-400 hover:bg-red-500/15 transition-colors"
              onclick={() => (formCover = null)}
            >
              <Trash2 size={11} />
              {t("collection.remove_cover")}
            </button>
          {/if}
        </div>
      </div>
    </div>

    <!-- Fields -->
    <div class="space-y-3">
      {#if !editing && mode === "import"}
        <label class="flex flex-col gap-1.5 text-[12.5px] font-semibold">
          <div class="flex items-center justify-between gap-2">
            <span class="text-[var(--text-muted)] inline-flex items-center gap-1.5">
              <Link2 size={13} />
              {t("url.import_label")}
            </span>
            {#if parsedImport}
              <span class="inline-flex items-center gap-1 text-[11px] font-bold text-[var(--accent-bright)]">
                <Check size={12} strokeWidth={3} />
                {t("url.import_valid")}
              </span>
            {/if}
          </div>
          <input
            type="url"
            bind:value={importUrl}
            placeholder={t("url.import_placeholder")}
            disabled={importing}
            required
            class="w-full px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:border-[var(--accent-border)] focus:ring-2 focus:ring-[var(--accent-soft)] outline-none text-[13.5px] transition-all"
          />
        </label>
      {/if}

      <label class="flex flex-col gap-1.5 text-[12.5px] font-semibold">
        <span class="text-[var(--text-muted)]">{t("collection.modal_name")}</span>
        <input
          type="text"
          bind:value={formName}
          placeholder={t("collection.modal_name_ph")}
          required={editing != null || mode === "create"}
          class="w-full px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:border-[var(--accent-border)] focus:ring-2 focus:ring-[var(--accent-soft)] outline-none text-[13.5px] transition-all"
        />
      </label>

      {#if editing || mode === "create"}
        <label class="flex flex-col gap-1.5 text-[12.5px] font-semibold">
          <span class="text-[var(--text-muted)]">{t("collection.modal_desc")}</span>
          <input
            type="text"
            bind:value={formDesc}
            placeholder={t("collection.modal_desc_ph")}
            class="w-full px-3.5 py-2.5 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:border-[var(--accent-border)] focus:ring-2 focus:ring-[var(--accent-soft)] outline-none text-[13.5px] transition-all"
          />
        </label>
      {/if}
    </div>

    <!-- Color swatches -->
    <div class="space-y-2.5">
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
      <div class="flex items-center flex-wrap gap-2.5 px-0.5 py-0.5">
        {#each SWATCHES as hex}
          <button
            type="button"
            class="w-7 h-7 rounded-full inline-flex items-center justify-center text-white transition-transform hover:scale-105 {formColor === hex
              ? 'ring-2 ring-white ring-offset-2 ring-offset-[var(--bg-elevated)] scale-105'
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
      <div class="grid grid-cols-8 gap-2">
        {#each PLAYLIST_ICON_IDS as ic}
          <button
            type="button"
            class="h-10 rounded-xl inline-flex items-center justify-center border transition-all {formIcon === ic
              ? 'bg-[var(--accent-soft)] border-[var(--accent-border)] text-[var(--accent-bright)]'
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
      <button type="button" class="pill-btn" disabled={importing} onclick={onClose}>
        {t("collection.modal_cancel")}
      </button>
      <button type="submit" class="pill-btn primary !px-5" disabled={!canSubmit}>
        {#if importing}
          <Loader2 size={14} class="animate-spin" />
          {t("url.importing")}
        {:else if editing}
          {t("collection.modal_save")}
        {:else if mode === "import"}
          {t("url.import_submit")}
        {:else}
          {t("collection.modal_create")}
        {/if}
      </button>
    </div>
  </form>
</ModalSheet>
