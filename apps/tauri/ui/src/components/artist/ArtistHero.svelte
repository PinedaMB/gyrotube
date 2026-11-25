<script lang="ts">
  import { ArrowLeft, Play, Radio, Shuffle, UserCheck, UserPlus } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { shuffleItems } from "../../lib/stores/queueOps";
  import { formatSubscribers, type ArtistDetails } from "../../lib/types";

  interface Props {
    artistId: string;
    artistName: string;
    details: ArtistDetails | null;
  }

  let { artistId, artistName, details }: Props = $props();

  let following = $derived(player.isFollowingArtist(artistId));

  let bannerLoaded = $state(false);
  let startingMix = $state(false);
  let bioExpanded = $state(false);

  $effect(() => {
    void details?.header_url;
    bannerLoaded = false;
    bioExpanded = false;
  });

  function shufflePlay() {
    if (!details?.tracks.length) return;
    const shuffled = shuffleItems(details.tracks);
    player.replaceQueueAndPlay(shuffled, 0, details.name);
  }

  async function startArtistMix() {
    if (!details?.tracks.length || startingMix || !isTauriRuntime()) return;
    startingMix = true;
    try {
      const seed = details.tracks[0];
      const radio = await api.getRadioTracks(seed.id);
      const list =
        radio.length > 0 ? [seed, ...radio.filter((t) => t.id !== seed.id)] : details.tracks;
      player.replaceQueueAndPlay(list, 0, `Mix · ${details.name}`);
    } catch (err) {
      player.showToast(String(err));
    } finally {
      startingMix = false;
    }
  }
  let hasBanner = $derived(Boolean(details?.header_url));
</script>

<section
  class="relative w-full flex flex-col justify-between overflow-hidden {hasBanner
    ? 'min-h-[270px] sm:min-h-[500px] md:min-h-[560px] lg:min-h-[620px]'
    : 'min-h-[200px] sm:min-h-[240px]'}"
>
  {#if details?.header_url}
    <!-- Edge-to-edge full-width banner: mask directly on <img> so mobile h-auto never shows a hard bottom edge -->
    <div class="absolute inset-0 pointer-events-none">
      <img
        class="w-full h-auto sm:h-full object-contain sm:object-cover object-top sm:object-[50%_15%] transition-opacity duration-500 {bannerLoaded
          ? 'opacity-100'
          : 'opacity-0'}"
        style="-webkit-mask-image: linear-gradient(180deg, #000 0%, #000 56%, rgba(0,0,0,0.72) 75%, rgba(0,0,0,0.22) 90%, transparent 100%);
               mask-image: linear-gradient(180deg, #000 0%, #000 56%, rgba(0,0,0,0.72) 75%, rgba(0,0,0,0.22) 90%, transparent 100%);"
        src={details.header_url}
        alt=""
        loading="lazy"
        decoding="async"
        referrerpolicy="no-referrer"
        onload={() => (bannerLoaded = true)}
      />
    </div>

    <!-- Soft bottom readability gradient over banner -->
    <div
      class="absolute inset-0 pointer-events-none"
      style="background: linear-gradient(180deg, rgba(12, 12, 15, 0.08) 0%, rgba(12, 12, 15, 0.14) 42%, rgba(12, 12, 15, 0.62) 74%, rgba(12, 12, 15, 0.22) 92%, transparent 100%);"
    ></div>
  {/if}

  <div class="relative z-10 w-full max-w-[1280px] mx-auto px-6 pt-[calc(0.75rem+var(--safe-top))] md:pt-4 pb-8 sm:pb-10 flex-1 flex flex-col justify-between">
    <div>
      <button
        class="w-9 h-9 rounded-full inline-flex items-center justify-center transition-all active:scale-95 {hasBanner
          ? 'bg-black/60 hover:bg-black/75 border border-white/15 text-white'
          : 'bg-[var(--surface)] hover:bg-[var(--surface-hover)] border border-[var(--border)] text-[var(--text)]'}"
        title={t("nav.back")}
        onclick={() => player.goBack()}
      >
        <ArrowLeft size={17} />
      </button>
    </div>

    <div class="{hasBanner ? 'mt-20 sm:mt-44 md:mt-52 lg:mt-60' : 'mt-8 sm:mt-10'} max-w-2xl space-y-2.5">
      <h1
        class="text-[38px] sm:text-[50px] font-black tracking-[-0.035em] leading-[1.04] {hasBanner
          ? 'text-white drop-shadow-[0_2px_14px_rgba(0,0,0,0.55)]'
          : 'text-[var(--text)]'}"
      >
        {details?.name ?? artistName}
      </h1>

      {#if details?.subscriber_count}
        <p
          class="text-[13.5px] font-medium {hasBanner
            ? 'text-white/85 drop-shadow-sm'
            : 'text-[var(--text-muted)]'}"
        >
          {formatSubscribers(details.subscriber_count)}
        </p>
      {/if}

      {#if details?.description}
        <div class="hidden sm:block pt-0.5 max-w-xl">
          <p
            class="text-[13.5px] leading-relaxed {hasBanner
              ? 'text-white/85 drop-shadow-sm'
              : 'text-[var(--text-muted)]'} {bioExpanded
              ? ''
              : 'line-clamp-2'}"
          >
            {details.description}
          </p>
          {#if details.description.length > 140}
            <button
              type="button"
              class="mt-1 text-[11.5px] font-bold uppercase tracking-wider transition-colors {hasBanner
                ? 'text-white/90 hover:text-[var(--accent-bright)]'
                : 'text-[var(--accent-bright)] hover:underline'}"
              onclick={() => (bioExpanded = !bioExpanded)}
            >
              {bioExpanded ? t("artist.show_less") : t("artist.more")}
            </button>
          {/if}
        </div>
      {/if}

      <div class="flex flex-nowrap sm:flex-wrap items-center gap-2 sm:gap-2.5 pt-2">
        <button
          class="pill-btn primary !px-4 sm:!px-5 !py-2 !text-[13px] sm:!text-[13.5px] shadow-[0_6px_20px_rgba(16,185,129,0.35)]"
          disabled={!details?.tracks.length}
          onclick={() => details && player.replaceQueueAndPlay(details.tracks, 0, details.name)}
        >
          <Play size={15} fill="currentColor" />
          {t("artist.play")}
        </button>

        <button
          class="pill-btn justify-center !gap-0 sm:!gap-2 !w-9 !h-9 !p-0 sm:!w-auto sm:!h-auto sm:!px-4 sm:!py-2 !text-[13.5px] {hasBanner
            ? '!bg-black/55 hover:!bg-black/70 !border-white/20 !text-white'
            : ''}"
          disabled={!details?.tracks.length}
          title={t("artist.shuffle")}
          aria-label={t("artist.shuffle")}
          onclick={shufflePlay}
        >
          <Shuffle size={15} />
          <span class="hidden sm:inline">{t("artist.shuffle")}</span>
        </button>

        <button
          class="pill-btn justify-center !gap-0 sm:!gap-2 !w-9 !h-9 !p-0 sm:!w-auto sm:!h-auto sm:!px-4 sm:!py-2 !text-[13.5px] {hasBanner
            ? '!bg-black/55 hover:!bg-black/70 !border-white/20 !text-white'
            : ''}"
          disabled={!details?.tracks.length || startingMix}
          title={t("artist.mix")}
          aria-label={t("artist.mix")}
          onclick={() => void startArtistMix()}
        >
          {#if startingMix}
            <span class="w-3.5 h-3.5 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
          {:else}
            <Radio size={15} />
          {/if}
          <span class="hidden sm:inline">{t("artist.mix")}</span>
        </button>

        <button
          type="button"
          class="pill-btn !px-3.5 sm:!px-4 !py-2 !text-[13px] sm:!text-[13.5px] {hasBanner
            ? following
              ? '!bg-black/60 !border-[var(--accent)] !text-emerald-300'
              : '!bg-black/55 hover:!bg-black/70 !border-white/20 !text-white'
            : following
              ? '!bg-[var(--accent-soft)] !border-[var(--accent-border)] !text-[var(--accent-bright)]'
              : ''}"
          onclick={() =>
            player.toggleFollowArtist({
              id: artistId,
              name: details?.name ?? artistName,
              avatar_url: details?.header_url ?? null,
              subscriber_count: details?.subscriber_count ?? null,
            })}
        >
          {#if following}
            <UserCheck size={15} />
            {t("artist.following")}
          {:else}
            <UserPlus size={15} />
            {t("artist.follow")}
          {/if}
        </button>
      </div>
    </div>
  </div>
</section>
