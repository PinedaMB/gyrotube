<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Copy, Minus, Square, X } from "lucide-svelte";
  import { onMount } from "svelte";
  import { isTauriRuntime } from "../../lib/api";
  import { t } from "../../lib/i18n.svelte";
  import { device } from "../../lib/stores/device.svelte";

  const showControls = isTauriRuntime() && !device.isAndroid;
  let isMaximized = $state(false);

  onMount(() => {
    if (!showControls) return;
    const win = getCurrentWindow();
    const sync = () => {
      void win
        .isMaximized()
        .then((v) => {
          isMaximized = v;
        })
        .catch(() => {});
    };
    sync();
    let unlisten: (() => void) | undefined;
    void win
      .onResized(() => {
        sync();
      })
      .then((fn) => {
        unlisten = fn;
      })
      .catch(() => {});
    return () => {
      unlisten?.();
    };
  });

  function minimize() {
    void getCurrentWindow().minimize().catch(() => {});
  }

  function toggleMaximize() {
    void getCurrentWindow().toggleMaximize().catch(() => {});
  }

  function closeWindow() {
    void getCurrentWindow().close().catch(() => {});
  }
</script>

{#if showControls}
  <div class="flex items-center gap-1.5 ml-1.5 pl-2.5 border-l border-[var(--border-strong)]/80 shrink-0">
    <button
      type="button"
      class="w-7 h-7 rounded-full flex items-center justify-center bg-[var(--surface)]/70 border border-[var(--border)]/70 text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] transition-all active:scale-90"
      title={t("window.minimize")}
      aria-label={t("window.minimize")}
      onclick={minimize}
    >
      <Minus size={13} strokeWidth={2.2} />
    </button>
    <button
      type="button"
      class="w-7 h-7 rounded-full flex items-center justify-center bg-[var(--surface)]/70 border border-[var(--border)]/70 text-[var(--text-muted)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] hover:border-[var(--border-strong)] transition-all active:scale-90"
      title={isMaximized ? t("window.restore") : t("window.maximize")}
      aria-label={isMaximized ? t("window.restore") : t("window.maximize")}
      onclick={toggleMaximize}
    >
      {#if isMaximized}
        <Copy size={11} strokeWidth={2.2} class="-scale-x-100" />
      {:else}
        <Square size={11} strokeWidth={2.2} />
      {/if}
    </button>
    <button
      type="button"
      class="w-7 h-7 rounded-full flex items-center justify-center bg-[var(--surface)]/70 border border-[var(--border)]/70 text-[var(--text-muted)] hover:text-white hover:bg-rose-500/90 hover:border-rose-400/60 transition-all active:scale-90"
      title={t("window.close")}
      aria-label={t("window.close")}
      onclick={closeWindow}
    >
      <X size={13.5} strokeWidth={2.2} />
    </button>
  </div>
{/if}
