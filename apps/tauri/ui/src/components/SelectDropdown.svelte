<script lang="ts">
  import { Check, ChevronDown } from "lucide-svelte";
  import type { Snippet } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";

  export interface SelectOption {
    value: string;
    label: string;
  }

  interface Props {
    value: string;
    options: SelectOption[];
    onChange: (value: string) => void;
    ariaLabel?: string;
    leading?: Snippet;
  }

  let { value, options, onChange, ariaLabel = "Select option", leading }: Props = $props();

  let open = $state(false);
  let rootEl = $state<HTMLDivElement | null>(null);

  let selectedLabel = $derived(
    options.find((o) => o.value === value)?.label ?? options[0]?.label ?? "",
  );

  function select(opt: SelectOption) {
    open = false;
    if (opt.value !== value) {
      onChange(opt.value);
    }
  }

  function handleWindowClick(e: MouseEvent) {
    if (open && rootEl && !rootEl.contains(e.target as Node)) {
      open = false;
    }
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (open && e.key === "Escape") {
      e.preventDefault();
      open = false;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} onkeydown={handleWindowKeydown} />

<div bind:this={rootEl} class="relative inline-block">
  <button
    type="button"
    class="inline-flex items-center gap-2 h-8.5 px-3 rounded-full text-[12.5px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border transition-colors active:scale-[0.98] {open
      ? 'border-[var(--accent-border)] text-[var(--text)]'
      : 'border-[var(--border)] hover:border-[var(--border-strong)] text-[var(--text)]'}"
    aria-haspopup="listbox"
    aria-expanded={open}
    aria-label={ariaLabel}
    onclick={() => (open = !open)}
  >
    {#if leading}
      <span class="text-[var(--text-muted)] inline-flex items-center shrink-0">
        {@render leading()}
      </span>
    {/if}
    <span class="truncate max-w-[130px]">{selectedLabel}</span>
    <ChevronDown
      size={14}
      class="text-[var(--text-muted)] shrink-0 transition-transform duration-150 {open
        ? 'rotate-180'
        : ''}"
    />
  </button>

  {#if open}
    <div
      transition:fly={{ y: -6, duration: 150, easing: cubicOut }}
      role="listbox"
      aria-label={ariaLabel}
      class="absolute right-0 top-[calc(100%+6px)] z-50 w-48 max-h-72 overflow-y-auto scroll-y rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_18px_44px_rgba(0,0,0,0.55)] p-1.5"
    >
      {#each options as opt (opt.value)}
        {@const active = opt.value === value}
        <button
          type="button"
          role="option"
          aria-selected={active}
          class="w-full flex items-center justify-between gap-2 px-3 py-2 rounded-xl text-[13px] text-left transition-colors {active
            ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] font-semibold'
            : 'text-[var(--text)] hover:bg-white/[0.07]'}"
          onclick={() => select(opt)}
        >
          <span class="truncate">{opt.label}</span>
          {#if active}
            <Check size={14} class="shrink-0" />
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>
