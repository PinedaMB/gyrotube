<script lang="ts">
  import {
    AlignCenter,
    AlignLeft,
    AlignRight,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    Copy,
    Globe,
    Languages,
    ListMusic,
    Loader2,
    PartyPopper,
    Share2,
    SlidersHorizontal,
    Type,
  } from "lucide-svelte";
  import { cubicOut } from "svelte/easing";
  import { scale } from "svelte/transition";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { LyricsAlign, LyricsFontSize, LyricsSyncKind } from "../../lib/types";

  interface Props {
    hasLyrics: boolean;
    hasRomanization: boolean;
    showSidebarActions?: boolean;
    compact?: boolean;
    onStartSelecting?: () => void;
  }

  let {
    hasLyrics,
    hasRomanization,
    showSidebarActions = false,
    compact = true,
    onStartSelecting,
  }: Props = $props();

  let open = $state(false);
  let sourcesExpanded = $state(false);
  let containerEl = $state<HTMLDivElement | null>(null);
  let lastTrackId = $state<string | null>(null);

  let offsetLabel = $derived(
    player.currentLyricsOffset === 0
      ? "0.0s"
      : `${player.currentLyricsOffset > 0 ? "+" : ""}${player.currentLyricsOffset.toFixed(1)}s`,
  );

  let targetLang = $derived(player.getTranslationTargetLang());
  let targetLangLabel = $derived(targetLang === "es" ? "Español" : "English");

  let hasActiveIndicator = $derived(
    player.currentLyricsOffset !== 0 || player.showTranslation,
  );

  const FONT_SIZES: { id: LyricsFontSize; label: string }[] = [
    { id: "sm", label: "A-" },
    { id: "md", label: "A" },
    { id: "lg", label: "A+" },
  ];

  const ALIGN_OPTIONS = [
    { id: "left" as LyricsAlign, labelKey: "lyrics.align_left", icon: AlignLeft },
    { id: "center" as LyricsAlign, labelKey: "lyrics.align_center", icon: AlignCenter },
    { id: "right" as LyricsAlign, labelKey: "lyrics.align_right", icon: AlignRight },
  ] as const;

  $effect(() => {
    const tid = player.currentTrack?.id ?? null;
    if (tid !== lastTrackId) {
      lastTrackId = tid;
      open = false;
      sourcesExpanded = false;
    }
  });

  $effect(() => {
    if (!open) return;
    const onPointerDown = (e: PointerEvent) => {
      if (containerEl && !containerEl.contains(e.target as Node)) {
        open = false;
      }
    };
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") open = false;
    };
    window.addEventListener("pointerdown", onPointerDown);
    window.addEventListener("keydown", onKeyDown);
    return () => {
      window.removeEventListener("pointerdown", onPointerDown);
      window.removeEventListener("keydown", onKeyDown);
    };
  });

  function toggleSources() {
    sourcesExpanded = !sourcesExpanded;
    if (sourcesExpanded) {
      void player.loadLyricsSources();
    }
  }

  function syncDotClass(kind: LyricsSyncKind): string {
    if (kind === "word") return "bg-sky-400";
    if (kind === "line") return "bg-amber-400";
    return "bg-zinc-400/70";
  }

  function syncKindLabel(kind: LyricsSyncKind): string {
    if (kind === "word") return t("lyrics.sync_word");
    if (kind === "line") return t("lyrics.sync_line");
    return t("lyrics.sync_plain");
  }
</script>

{#if hasLyrics}
  <div class="relative" bind:this={containerEl}>
    {#if compact}
      <button
        type="button"
        class="relative w-7 h-7 rounded-full inline-flex items-center justify-center transition-all active:scale-95 border {open
          ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] border-[var(--accent-border)]'
          : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] border-[var(--border)]'}"
        title={t("lyrics.options")}
        aria-label={t("lyrics.options")}
        aria-expanded={open}
        onclick={() => (open = !open)}
      >
        <SlidersHorizontal size={13} />
        {#if hasActiveIndicator}
          <span class="absolute -top-0.5 -right-0.5 w-2 h-2 rounded-full bg-[var(--accent-bright)]"></span>
        {/if}
      </button>
    {:else}
      <button
        type="button"
        class="relative inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[11.5px] font-semibold border transition-colors {open
          ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] border-[var(--accent-border)]'
          : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] border-[var(--border)]'}"
        title={t("lyrics.options")}
        aria-label={t("lyrics.options")}
        aria-expanded={open}
        onclick={() => (open = !open)}
      >
        <SlidersHorizontal size={13} />
        <span>{t("lyrics.options")}</span>
        {#if hasActiveIndicator}
          <span class="w-1.5 h-1.5 rounded-full bg-[var(--accent-bright)]"></span>
        {/if}
      </button>
    {/if}

    {#if open}
      <div
        transition:scale={{ start: 0.95, duration: 130, easing: cubicOut }}
        class="absolute right-0 top-9 z-30 w-[280px] max-h-[72vh] overflow-y-auto p-2 rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_16px_40px_rgba(0,0,0,0.55)] space-y-1.5 origin-top-right"
      >
        <!-- Lyrics source selector -->
        {#if player.currentTrack && !player.isOffline}
          <div>
            <button
              type="button"
              class="w-full flex items-center justify-between gap-2 px-2.5 py-2 rounded-xl hover:bg-[var(--surface-hover)] text-[12.5px] font-semibold text-[var(--text)] transition-colors"
              onclick={toggleSources}
            >
              <span class="flex items-center gap-2 min-w-0">
                <ListMusic size={14} class="text-[var(--accent-bright)] shrink-0" />
                <span class="truncate">{t("lyrics.source_label")}</span>
              </span>
              <span class="flex items-center gap-1 text-[11px] text-[var(--text-muted)] shrink-0">
                {#if player.lyricsSourcesLoading}
                  <Loader2 size={12} class="animate-spin" />
                {:else}
                  <ChevronDown
                    size={14}
                    class="transition-transform duration-150 {sourcesExpanded ? 'rotate-180' : ''}"
                  />
                {/if}
              </span>
            </button>

            {#if sourcesExpanded}
              <div class="mt-1 mx-1 p-1 rounded-xl bg-[var(--bg)]/55 border border-[var(--border)] space-y-0.5">
                {#if player.lyricsSourcesLoading}
                  <div class="px-2.5 py-2 text-[11.5px] text-[var(--text-muted)] flex items-center gap-2">
                    <Loader2 size={12} class="animate-spin" />
                    {t("lyrics.source_loading")}
                  </div>
                {:else if player.lyricsSources.length === 0}
                  <div class="px-2.5 py-2 text-[11.5px] text-[var(--text-muted)]">
                    {t("lyrics.source_empty")}
                  </div>
                {:else}
                  {#each player.lyricsSources as src (src.id)}
                    {@const selected = player.activeLyricsSourceId === src.id}
                    <button
                      type="button"
                      class="w-full flex items-center justify-between gap-2 px-2 py-1.5 rounded-lg text-left transition-colors {selected
                        ? 'bg-[var(--accent-soft)] text-[var(--text)]'
                        : 'hover:bg-[var(--surface-hover)] text-[var(--text)]'}"
                      onclick={() => {
                        player.selectLyricsSource(src);
                        sourcesExpanded = false;
                      }}
                    >
                      <span class="flex items-center gap-2 min-w-0">
                        <span class="w-2 h-2 rounded-full shrink-0 {syncDotClass(src.sync_kind)}"></span>
                        <span class="text-[12px] font-semibold truncate">{src.label}</span>
                      </span>
                      <span class="flex items-center gap-1.5 shrink-0">
                        <span class="text-[10.5px] text-[var(--text-muted)]">
                          {syncKindLabel(src.sync_kind)}
                        </span>
                        {#if selected}
                          <Check size={12} class="text-[var(--accent-bright)]" />
                        {/if}
                      </span>
                    </button>
                  {/each}
                {/if}
              </div>
            {/if}
          </div>
        {/if}

        <!-- Translation toggle + quick language switch -->
        {#if player.lyrics.type === "Synced" && !player.isOffline}
          <div class="flex items-center justify-between gap-1.5 px-2.5 py-2 rounded-xl hover:bg-[var(--surface-hover)] transition-colors">
            <button
              type="button"
              class="flex-1 flex items-center gap-2 min-w-0 text-left text-[12.5px] font-semibold text-[var(--text)]"
              onclick={() => player.setShowTranslation(!player.showTranslation)}
            >
              {#if player.translationLoading}
                <Loader2 size={14} class="text-[var(--accent-bright)] animate-spin shrink-0" />
              {:else}
                <Globe size={14} class="text-[var(--accent-bright)] shrink-0" />
              {/if}
              <span class="truncate">{t("lyrics.translate_label")}</span>
            </button>
            <div class="flex items-center gap-1.5 shrink-0">
              <button
                type="button"
                class="px-1.5 py-0.5 rounded-md text-[10.5px] font-bold bg-[var(--surface)] hover:bg-[var(--accent-soft)] text-[var(--text-muted)] hover:text-[var(--accent-bright)] border border-[var(--border)] transition-colors"
                title={targetLangLabel}
                onclick={() => player.setTranslationLang(targetLang === "es" ? "en" : "es")}
              >{targetLang.toUpperCase()}</button>
              <button
                type="button"
                role="switch"
                aria-checked={player.showTranslation}
                aria-label={t("lyrics.translate_label")}
                class="w-8 h-4.5 rounded-full p-0.5 transition-colors inline-flex items-center {player.showTranslation
                  ? 'bg-[var(--accent)] justify-end'
                  : 'bg-[var(--control-track)] justify-start'}"
                onclick={() => player.setShowTranslation(!player.showTranslation)}
              >
                <span class="w-3.5 h-3.5 rounded-full bg-white shadow"></span>
              </button>
            </div>
          </div>
        {/if}

        <!-- Romanization toggle -->
        {#if hasRomanization}
          <button
            type="button"
            class="w-full flex items-center justify-between gap-2 px-2.5 py-2 rounded-xl hover:bg-[var(--surface-hover)] text-[12.5px] font-semibold text-[var(--text)] transition-colors"
            onclick={() => player.setShowRomanization(!player.showRomanization)}
          >
            <span class="flex items-center gap-2">
              <Languages size={14} class="text-[var(--accent-bright)]" />
              {t("lyrics.romanization_label")}
            </span>
            <span
              class="w-8 h-4.5 rounded-full p-0.5 transition-colors inline-flex items-center {player.showRomanization
                ? 'bg-[var(--accent)] justify-end'
                : 'bg-[var(--control-track)] justify-start'}"
            >
              <span class="w-3.5 h-3.5 rounded-full bg-white shadow"></span>
            </span>
          </button>
        {/if}

        <!-- Sync offset compact row -->
        {#if player.lyrics.type === "Synced" && player.currentTrack}
          <div class="px-2.5 py-1.5 space-y-1.5">
            <div class="flex items-center justify-between gap-2">
              <span class="text-[12.5px] font-semibold text-[var(--text)]">
                {t("lyrics.offset_label")}
              </span>
              <div class="inline-flex items-center gap-1">
                <button
                  type="button"
                  class="w-6 h-6 rounded-md inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] border border-[var(--border)] active:scale-95"
                  title="-0.1s"
                  onclick={() => player.adjustLyricsOffset(-0.1)}
                >
                  <ChevronLeft size={13} />
                </button>
                <button
                  type="button"
                  class="min-w-[46px] px-1.5 py-0.5 rounded-md text-[11.5px] font-bold numeric transition-colors {player.currentLyricsOffset !== 0
                    ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)]'
                    : 'text-[var(--text-muted)]'}"
                  title={t("lyrics.offset_reset")}
                  disabled={player.currentLyricsOffset === 0}
                  onclick={() => player.resetLyricsOffset()}
                >{offsetLabel}</button>
                <button
                  type="button"
                  class="w-6 h-6 rounded-md inline-flex items-center justify-center bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] border border-[var(--border)] active:scale-95"
                  title="+0.1s"
                  onclick={() => player.adjustLyricsOffset(0.1)}
                >
                  <ChevronRight size={13} />
                </button>
              </div>
            </div>
            <div class="flex items-center justify-end gap-1">
              <button
                type="button"
                class="px-2 py-0.5 rounded-md text-[10.5px] font-bold numeric bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] border border-[var(--border)]"
                onclick={() => player.adjustLyricsOffset(-0.5)}
              >-0.5s</button>
              <button
                type="button"
                class="px-2 py-0.5 rounded-md text-[10.5px] font-bold numeric bg-[var(--surface)] hover:bg-[var(--surface-hover)] text-[var(--text-muted)] hover:text-[var(--text)] border border-[var(--border)]"
                onclick={() => player.adjustLyricsOffset(0.5)}
              >+0.5s</button>
            </div>
          </div>
        {/if}

        <!-- Text size -->
        <div class="flex items-center justify-between gap-2 px-2.5 py-1.5">
          <span class="flex items-center gap-2 text-[12.5px] font-semibold text-[var(--text)]">
            <Type size={14} class="text-[var(--accent-bright)]" />
            {t("lyrics.font_size")}
          </span>
          <div class="inline-flex items-center p-0.5 rounded-lg bg-[var(--surface)] border border-[var(--border)]">
            {#each FONT_SIZES as fs (fs.id)}
              {@const active = player.lyricsFontSize === fs.id}
              <button
                type="button"
                class="px-2 py-0.5 rounded-md text-[11px] font-bold transition-colors {active
                  ? 'bg-[var(--accent)] text-white'
                  : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
                onclick={() => player.setLyricsFontSize(fs.id)}
              >{fs.label}</button>
            {/each}
          </div>
        </div>

        <!-- Alignment -->
        <div class="flex items-center justify-between gap-2 px-2.5 py-1.5">
          <span class="flex items-center gap-2 text-[12.5px] font-semibold text-[var(--text)]">
            <AlignLeft size={14} class="text-[var(--accent-bright)]" />
            {t("lyrics.align")}
          </span>
          <div class="inline-flex items-center p-0.5 rounded-lg bg-[var(--surface)] border border-[var(--border)]">
            {#each ALIGN_OPTIONS as al (al.id)}
              {@const active = player.lyricsAlign === al.id}
              {@const Icon = al.icon}
              <button
                type="button"
                class="px-2 py-1 rounded-md transition-colors {active
                  ? 'bg-[var(--accent)] text-white'
                  : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
                title={t(al.labelKey)}
                aria-label={t(al.labelKey)}
                aria-pressed={active}
                onclick={() => player.setLyricsAlign(al.id)}
              >
                <Icon size={12} />
              </button>
            {/each}
          </div>
        </div>

        <div class="h-px bg-[var(--border)] my-1"></div>

        <!-- Quick actions -->
        <div class="space-y-0.5">
          <button
            type="button"
            class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-xl hover:bg-[var(--surface-hover)] text-[12.5px] font-semibold text-[var(--text)] transition-colors"
            onclick={() => {
              open = false;
              void player.copyFullLyrics();
            }}
          >
            <Copy size={14} class="text-[var(--accent-bright)]" />
            {t("lyrics.copy_all")}
          </button>

          {#if showSidebarActions}
            <button
              type="button"
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-xl hover:bg-[var(--surface-hover)] text-[12.5px] font-semibold text-[var(--text)] transition-colors"
              onclick={() => {
                open = false;
                onStartSelecting?.();
              }}
            >
              <Share2 size={14} class="text-[var(--accent-bright)]" />
              {t("lyrics.share_card")}
            </button>

            <button
              type="button"
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-xl hover:bg-[var(--surface-hover)] text-[12.5px] font-semibold text-[var(--text)] transition-colors"
              onclick={() => {
                open = false;
                player.openPartyMode();
              }}
            >
              <PartyPopper size={14} class="text-[var(--accent-bright)]" />
              {t("party.badge")}
            </button>
          {/if}
        </div>
      </div>
    {/if}
  </div>
{/if}
