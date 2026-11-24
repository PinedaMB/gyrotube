<script lang="ts">
  import { Sliders, Volume2 } from "lucide-svelte";
  import { EQ_LABELS, type EqPresetId } from "../../lib/equalizer";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";

  const eqPresetList: { id: Exclude<EqPresetId, "custom">; key: Parameters<typeof t>[0] }[] = [
    { id: "flat", key: "eq.preset_flat" },
    { id: "bass", key: "eq.preset_bass" },
    { id: "vocal", key: "eq.preset_vocal" },
    { id: "rock", key: "eq.preset_rock" },
    { id: "pop", key: "eq.preset_pop" },
    { id: "electronic", key: "eq.preset_electronic" },
    { id: "acoustic", key: "eq.preset_acoustic" },
  ];
</script>

<section class="space-y-2.5">
  <h2 class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)] flex items-center gap-1.5 px-1">
    <Sliders size={12} class="text-[var(--accent-bright)]" />
    {t("eq.title")}
  </h2>
  <div
    class="rounded-2xl bg-[var(--surface)] border border-[var(--border)] divide-y divide-[var(--border)] overflow-hidden"
    style="box-shadow: var(--card-shadow);"
  >
    <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
      <div class="flex items-center gap-3 min-w-0">
        <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
          <Sliders size={16} />
        </span>
        <div class="min-w-0">
          <div class="text-[14px] font-semibold leading-snug">{t("eq.enable")}</div>
          <div class="text-[12px] text-[var(--text-muted)]">{t("eq.enable_sub")}</div>
        </div>
      </div>
      <button
        type="button"
        role="switch"
        aria-checked={player.eqSettings.enabled}
        aria-label={t("eq.enable")}
        class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.eqSettings.enabled
          ? 'bg-[var(--accent)]'
          : 'bg-[var(--control-track)]'}"
        onclick={() => player.setEqEnabled(!player.eqSettings.enabled)}
      >
        <span
          class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.eqSettings.enabled
            ? 'translate-x-5'
            : 'translate-x-0'}"
        ></span>
      </button>
    </div>

    {#if player.eqSettings.enabled}
      <div class="p-4 space-y-4 bg-[var(--bg)]/25">
        <div class="flex flex-wrap gap-1.5">
          {#each eqPresetList as preset (preset.id)}
            <button
              type="button"
              class="px-3 py-1 rounded-full text-[12px] font-semibold border transition-colors {player.eqSettings.preset === preset.id
                ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] border-[var(--accent-border)]'
                : 'bg-[var(--surface)] border-[var(--border)] text-[var(--text-muted)] hover:text-[var(--text)]'}"
              onclick={() => player.setEqPreset(preset.id)}
            >
              {t(preset.key)}
            </button>
          {/each}
        </div>

        <div class="space-y-2.5">
          {#each EQ_LABELS as label, idx (label)}
            {@const db = player.eqSettings.bands[idx]}
            {@const pct = Math.round(((db + 12) / 24) * 100)}
            <div class="flex items-center gap-3">
              <span class="w-14 text-[12px] font-semibold text-[var(--text-muted)] numeric shrink-0">
                {label}
              </span>
              <input
                type="range"
                min="-12"
                max="12"
                step="1"
                value={db}
                aria-label={label}
                class="flex-1 !h-1.5"
                style="background: linear-gradient(to right, var(--accent-bright) {pct}%, var(--control-track) {pct}%);"
                oninput={(e) => player.setEqBand(idx, Number((e.currentTarget as HTMLInputElement).value))}
              />
              <span class="w-12 text-right text-[12px] font-bold numeric shrink-0 {db !== 0 ? 'text-[var(--accent-bright)]' : 'text-[var(--text-muted)]'}">
                {db > 0 ? `+${db}` : db} dB
              </span>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <div class="p-4 flex items-center justify-between gap-4 transition-colors hover:bg-[var(--surface-hover)]/35">
      <div class="flex items-center gap-3 min-w-0">
        <span class="w-9 h-9 rounded-xl bg-[var(--bg)]/60 border border-[var(--border)] inline-flex items-center justify-center text-[var(--accent-bright)] shrink-0">
          <Volume2 size={16} />
        </span>
        <div class="min-w-0">
          <div class="text-[14px] font-semibold leading-snug">{t("eq.normalize")}</div>
          <div class="text-[12px] text-[var(--text-muted)]">{t("eq.normalize_sub")}</div>
        </div>
      </div>
      <button
        type="button"
        role="switch"
        aria-checked={player.eqSettings.normalize}
        aria-label={t("eq.normalize")}
        class="w-11 h-6 rounded-full p-0.5 transition-colors shrink-0 {player.eqSettings.normalize
          ? 'bg-[var(--accent)]'
          : 'bg-[var(--control-track)]'}"
        onclick={() => player.setEqNormalize(!player.eqSettings.normalize)}
      >
        <span
          class="block w-5 h-5 rounded-full bg-white shadow transition-transform {player.eqSettings.normalize
            ? 'translate-x-5'
            : 'translate-x-0'}"
        ></span>
      </button>
    </div>
  </div>
</section>
