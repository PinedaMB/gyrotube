<script lang="ts">
  import { ArrowLeft, Play, Radio, Shuffle } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import { formatSubscribers, type ArtistDetails } from "../../lib/types";

  interface Props {
    artistName: string;
    details: ArtistDetails | null;
  }

  let { artistName, details }: Props = $props();

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
    const shuffled = [...details.tracks].sort(() => Math.random() - 0.5);
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
</script>

<section class="relative w-full min-h-[340px] sm:min-h-[440px] flex flex-col justify-between overflow-hidden">
  {#if details?.header_url}
    <!-- Edge-to-edge full-width banner dissolving smoothly at the bottom into the page background -->
    <div
      class="absolute inset-0 pointer-events-none"
      style="-webkit-mask-image: linear-gradient(180deg, #000 0%, #000 54%, rgba(0,0,0,0.72) 72%, rgba(0,0,0,0.25) 88%, transparent 100%);
             mask-image: linear-gradient(180deg, #000 0%, #000 54%, rgba(0,0,0,0.72) 72%, rgba(0,0,0,0.25) 88%, transparent 100%);"
    >
      <img
        class="w-full h-full object-cover object-[50%_22%] transition-opacity duration-500 {bannerLoaded
          ? 'opacity-100'
          : 'opacity-0'}"
        src={details.header_url}
        alt=""
        loading="lazy"
        decoding="async"
        referrerpolicy="no-referrer"
        onload={() => (bannerLoaded = true)}
      />
    </div>
  {/if}

  <!-- Soft bottom readability gradient (transparent at top and at the very bottom edge) -->
  <div
    class="absolute inset-0 pointer-events-none"
    style="background: linear-gradient(180deg, rgba(12, 12, 15, 0.14) 0%, rgba(12, 12, 15, 0.1) 36%, rgba(12, 12, 15, 0.62) 70%, rgba(12, 12, 15, 0.25) 90%, transparent 100%);"
  ></div>

  <div class="relative z-10 w-full max-w-[1280px] mx-auto px-6 pt-4 pb-8 flex-1 flex flex-col justify-between">
    <div>
      <button
        class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-black/45 hover:bg-black/65 backdrop-blur-md border border-white/15 text-white transition-all active:scale-95"
        title={t("nav.back")}
        onclick={() => player.navigateTab("explore")}
      >
        <ArrowLeft size={17} />
      </button>
    </div>

    <div class="mt-14 sm:mt-24 max-w-2xl space-y-2.5">
      <h1 class="text-[38px] sm:text-[50px] font-black tracking-[-0.035em] leading-[1.04] text-white drop-shadow-[0_2px_14px_rgba(0,0,0,0.45)]">
        {details?.name ?? artistName}
      </h1>

      {#if details?.subscriber_count}
        <p class="text-[13.5px] font-medium text-white/80 drop-shadow-sm">
          {formatSubscribers(details.subscriber_count)}
        </p>
      {/if}

      {#if details?.description}
        <div class="pt-0.5 max-w-xl">
          <p
            class="text-[13.5px] leading-relaxed text-white/85 drop-shadow-sm {bioExpanded
              ? ''
              : 'line-clamp-2'}"
          >
            {details.description}
          </p>
          {#if details.description.length > 140}
            <button
              type="button"
              class="mt-1 text-[11.5px] font-bold uppercase tracking-wider text-white/90 hover:text-[var(--accent-bright)] transition-colors"
              onclick={() => (bioExpanded = !bioExpanded)}
            >
              {bioExpanded ? t("artist.show_less") : t("artist.more")}
            </button>
          {/if}
        </div>
      {/if}

      <div class="flex flex-wrap items-center gap-2.5 pt-2">
        <button
          class="pill-btn primary !px-5 !py-2 !text-[13.5px] shadow-[0_6px_20px_rgba(33,144,164,0.35)]"
          disabled={!details?.tracks.length}
          onclick={() => details && player.replaceQueueAndPlay(details.tracks, 0, details.name)}
        >
          <Play size={15} fill="currentColor" />
          {t("artist.play")}
        </button>

        <button
          class="pill-btn !px-4 !py-2 !text-[13.5px] bg-white/[0.08] hover:bg-white/[0.14] border-white/15 backdrop-blur-md"
          disabled={!details?.tracks.length}
          onclick={shufflePlay}
        >
          <Shuffle size={15} />
          {t("artist.shuffle")}
        </button>

        <button
          class="pill-btn !px-4 !py-2 !text-[13.5px] bg-white/[0.08] hover:bg-white/[0.14] border-white/15 backdrop-blur-md"
          disabled={!details?.tracks.length || startingMix}
          onclick={() => void startArtistMix()}
        >
          {#if startingMix}
            <span class="w-3.5 h-3.5 rounded-full border-2 border-current/30 border-t-current animate-spin"></span>
          {:else}
            <Radio size={15} />
          {/if}
          {t("artist.mix")}
        </button>
      </div>
    </div>
  </div>
</section>
