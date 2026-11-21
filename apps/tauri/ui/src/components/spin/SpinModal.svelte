<script lang="ts">
  import { Globe, Wifi, X } from "lucide-svelte";
  import { t } from "../../lib/i18n.svelte";
  import { player } from "../../lib/stores/player.svelte";
  import SpinIcon from "../icons/SpinIcon.svelte";
  import ModalSheet from "../ModalSheet.svelte";
  import SegmentedControl, { type SegmentedOption } from "../SegmentedControl.svelte";
  import SpinLocalPane from "./SpinLocalPane.svelte";
  import SpinRemotePane from "./SpinRemotePane.svelte";

  type SpinTab = "local" | "remote";
  let tab = $state<SpinTab>("local");

  let tabOptions = $derived<SegmentedOption<SpinTab>[]>([
    { value: "local", label: t("spin.tab_local"), icon: Wifi },
    { value: "remote", label: t("spin.tab_remote"), icon: Globe },
  ]);

  let isLive = $derived(
    player.spinStatus.local_active || player.spinStatus.remote_role !== "idle",
  );
</script>

<ModalSheet
  open={player.spinOpen}
  label={t("spin.title")}
  maxWidth={480}
  onClose={() => (player.spinOpen = false)}
>
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-xl bg-[var(--gold-soft)] text-[var(--gold-bright)] flex items-center justify-center">
            <SpinIcon size={18} spinning={isLive} />
          </div>
          <h2 class="text-[18px] font-extrabold tracking-tight">{t("spin.title")}</h2>
          <span class="inline-flex items-center px-2 py-0.5 rounded-full text-[10.5px] font-bold uppercase tracking-wide bg-[var(--gold-soft)] text-[var(--gold-bright)] border border-[var(--gold-border)]">
            {t("spin.experimental")}
          </span>
          {#if isLive}
            <span class="inline-flex items-center gap-1.5 text-[11px] font-bold px-2.5 py-0.5 rounded-full bg-[var(--gold-soft)] text-[var(--gold-bright)] border border-[var(--gold-border)]">
              <span class="w-1.5 h-1.5 rounded-full bg-[var(--gold-bright)] animate-pulse"></span>
              {t("spin.live")}
            </span>
          {/if}
        </div>
        <button
          type="button"
          class="icon-btn"
          title={t("spin.close")}
          onclick={() => (player.spinOpen = false)}
        >
          <X size={17} />
        </button>
      </div>

      <!-- Body -->
      <div class="p-6 space-y-5">
        <!-- Segmented mode switch -->
        <SegmentedControl
          options={tabOptions}
          value={tab}
          fullWidth={true}
          ariaLabel={t("spin.title")}
          onchange={(v) => (tab = v)}
        />

        {#if tab === "local"}
          <SpinLocalPane />
        {:else}
          <SpinRemotePane />
        {/if}
      </div>
</ModalSheet>
