<script lang="ts">
  import { Heart, Play, Radio, Sparkles } from "lucide-svelte";
  import { api, isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import type { Track } from "../../lib/types";

  interface PersonalMix {
    key: string;
    title: string;
    subtitle: string;
    seed: Track;
    color: string;
    icon: "heart" | "radio" | "sparkles";
  }

  let loadingKey = $state<string | null>(null);

  let mixes = $derived.by<PersonalMix[]>(() => {
    const out: PersonalMix[] = [];
    const usedIds = new Set<string>();

    // 1. Favorites Mix
    if (player.favorites.length > 0) {
      const seed = player.favorites[0];
      usedIds.add(seed.id);
      out.push({
        key: "favorites",
        title: t("home.mix_favorites"),
        subtitle: t("home.mix_favorites_sub"),
        seed,
        color: "#e11d48",
        icon: "heart",
      });
    }

    // 2. Top Artist Mix (most frequent artist across recent + favorites)
    const pool = [...player.recentTracks, ...player.favorites];
    if (pool.length > 0) {
      const counts = new Map<string, { count: number; track: Track }>();
      for (const tr of pool) {
        const key = tr.artist.split(",")[0]?.trim() || tr.artist;
        if (!key) continue;
        const prev = counts.get(key);
        if (prev) prev.count += 1;
        else counts.set(key, { count: 1, track: tr });
      }
      const best = [...counts.entries()].sort((a, b) => b[1].count - a[1].count)[0];
      if (best) {
        const [artistName, { track }] = best;
        if (!usedIds.has(track.id) || out.length === 0) {
          usedIds.add(track.id);
          out.push({
            key: `artist:${artistName}`,
            title: t("home.mix_artist", { artist: artistName }),
            subtitle: t("home.mix_artist_sub", { artist: artistName }),
            seed: track,
            color: "#7c3aed",
            icon: "radio",
          });
        }
      }
    }

    // 3. Discovery Mix (another recent track not used yet)
    const discoverySeed = player.recentTracks.find((tr) => !usedIds.has(tr.id));
    if (discoverySeed) {
      out.push({
        key: `discovery:${discoverySeed.id}`,
        title: t("home.mix_discovery"),
        subtitle: t("home.mix_discovery_sub"),
        seed: discoverySeed,
        color: "#0ea5e9",
        icon: "sparkles",
      });
    }

    return out;
  });

  async function startMix(mix: PersonalMix) {
    if (!isTauriRuntime() || loadingKey || player.isOffline) return;
    loadingKey = mix.key;
    try {
      const radio = await api.getRadioTracks(mix.seed.id);
      const unique = radio.filter((tr) => tr.id !== mix.seed.id);
      const list = [mix.seed, ...unique];
      player.replaceQueueAndPlay(list, 0, mix.title);
    } catch (err) {
      player.showToast(String(err));
    } finally {
      loadingKey = null;
    }
  }
</script>

{#if mixes.length > 0}
  <section class="mt-6 flex flex-col gap-3">
    <h2 class="section-title">{t("home.made_for_you")}</h2>
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each mixes as mix (mix.key)}
        {@const bgUrl = mix.seed.cover_large_url || mix.seed.cover_url}
        <button
          type="button"
          disabled={loadingKey !== null}
          class="group relative overflow-hidden rounded-2xl p-4 border border-[var(--border-strong)] hover:border-[var(--accent-border)] text-left flex flex-col justify-between min-h-[136px] bg-[#16151d] text-white shadow-[0_6px_20px_rgba(15,23,42,0.12)] transition-all duration-200 hover:-translate-y-0.5"
          onclick={() => void startMix(mix)}
        >
          {#if bgUrl}
            <img
              src={bgUrl}
              alt=""
              loading="lazy"
              decoding="async"
              referrerpolicy="no-referrer"
              class="absolute inset-0 w-full h-full object-cover opacity-35 scale-105 group-hover:scale-110 transition-transform duration-500 pointer-events-none"
            />
          {/if}
          <div
            class="absolute inset-0 pointer-events-none"
            style="background: linear-gradient(145deg, {mix.color}cc 0%, {mix.color}55 45%, rgba(16, 15, 22, 0.92) 100%);"
          ></div>
          <div
            class="absolute inset-0 pointer-events-none"
            style="background: linear-gradient(180deg, rgba(12,12,18,0.12) 0%, rgba(12,12,18,0.45) 55%, rgba(12,12,18,0.82) 100%);"
          ></div>

          <div class="relative z-10 flex items-center justify-between gap-2">
            <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10.5px] font-bold uppercase tracking-wider bg-black/45 text-white border border-white/15">
              {#if mix.icon === "heart"}
                <Heart size={11} fill="currentColor" />
              {:else if mix.icon === "radio"}
                <Radio size={11} />
              {:else}
                <Sparkles size={11} />
              {/if}
              Mix
            </span>
            <span class="w-9 h-9 rounded-full inline-flex items-center justify-center bg-black/45 group-hover:bg-[var(--accent)] border border-white/20 text-white shadow-md transition-colors">
              {#if loadingKey === mix.key}
                <span class="w-4 h-4 rounded-full border-2 border-white/30 border-t-white animate-spin"></span>
              {:else}
                <Play size={15} fill="currentColor" class="translate-x-[1px]" />
              {/if}
            </span>
          </div>

          <div class="relative z-10 mt-3">
            <strong class="block text-[17px] font-extrabold tracking-tight leading-snug text-white drop-shadow-[0_1px_6px_rgba(0,0,0,0.45)] truncate">
              {mix.title}
            </strong>
            <span class="block text-[12px] text-white/85 truncate mt-0.5">
              {mix.subtitle}
            </span>
          </div>
        </button>
      {/each}
    </div>
  </section>
{/if}
