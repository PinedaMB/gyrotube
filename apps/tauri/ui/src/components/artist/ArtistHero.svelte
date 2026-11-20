<script lang="ts">
  import { ArrowLeft, BadgeCheck, Play, Radio, Shuffle } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { player } from "../../lib/stores/player.svelte";
  import { formatSubscribers, type ArtistDetails } from "../../lib/types";

  interface Props {
    artistName: string;
    details: ArtistDetails | null;
  }

  let { artistName, details }: Props = $props();

  let bannerLoaded = $state(false);
  let startingMix = $state(false);

  $effect(() => {
    void details?.header_url;
    bannerLoaded = false;
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

<section class="relative w-full min-h-[340px] sm:min-h-[380px] flex flex-col justify-between overflow-hidden">
  {#if details?.header_url}
    <img
      class="absolute inset-0 w-full h-full object-cover object-[65%_25%] transition-opacity duration-500 {bannerLoaded
        ? 'opacity-100'
        : 'opacity-0'}"
      src={details.header_url}
      alt=""
      loading="lazy"
      decoding="async"
      referrerpolicy="no-referrer"
      onload={() => (bannerLoaded = true)}
    />
  {/if}

  <!-- Seamless directional fade into GyroTube background -->
  <div
    class="absolute inset-0 pointer-events-none"
    style="background:
      linear-gradient(180deg, rgba(18, 18, 22, 0.25) 0%, rgba(18, 18, 22, 0.42) 55%, var(--bg) 100%),
      linear-gradient(90deg, var(--bg) 0%, rgba(30, 30, 33, 0.78) 38%, rgba(30, 30, 33, 0.18) 72%, transparent 100%);"
  ></div>

  <div class="relative z-10 w-full max-w-[1280px] mx-auto px-6 pt-4 pb-6 flex-1 flex flex-col justify-between">
    <div>
      <button
        class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-black/40 hover:bg-black/60 backdrop-blur-md border border-white/10 text-white transition-all active:scale-95"
        title="Back"
        onclick={() => player.navigateTab("explore")}
      >
        <ArrowLeft size={17} />
      </button>
    </div>

    <div class="mt-16 max-w-2xl space-y-2.5">
      <div class="inline-flex items-center gap-1.5 text-[12px] font-bold text-[var(--accent-bright)]">
        <BadgeCheck size={15} />
        <span>Verified Artist</span>
      </div>

      <h1 class="text-[36px] sm:text-[46px] font-black tracking-[-0.035em] leading-[1.05] drop-shadow-sm">
        {details?.name ?? artistName}
      </h1>

      {#if details?.subscriber_count}
        <p class="text-[14px] font-medium text-[var(--text-muted)]">
          {formatSubscribers(details.subscriber_count)}
        </p>
      {/if}

      <div class="flex flex-wrap items-center gap-2.5 pt-2">
        <button
          class="pill-btn primary !px-5 !py-2 !text-[13.5px] shadow-[0_6px_20px_rgba(33,144,164,0.35)]"
          disabled={!details?.tracks.length}
          onclick={() => details && player.replaceQueueAndPlay(details.tracks, 0, details.name)}
        >
          <Play size={15} fill="currentColor" />
          Play
        </button>

        <button
          class="pill-btn !px-4 !py-2 !text-[13.5px] bg-white/[0.08] hover:bg-white/[0.14] border-white/15 backdrop-blur-md"
          disabled={!details?.tracks.length}
          onclick={shufflePlay}
        >
          <Shuffle size={15} />
          Shuffle
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
          Mix
        </button>
      </div>
    </div>
  </div>
</section>
