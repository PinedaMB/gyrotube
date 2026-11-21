<script module lang="ts">
  export interface SelectOption {
    value: string;
    label: string;
  }
</script>

<script lang="ts">
  import { Check, ChevronDown } from "lucide-svelte";
  import type { Snippet } from "svelte";
  import { cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";

  interface Props {
    value: string;
    options: SelectOption[];
    onChange: (value: string) => void;
    ariaLabel?: string;
    leading?: Snippet;
  }

  let { value, options, onChange, ariaLabel = "Select option", leading }: Props = $props();

  let open = $state(false);
  let activeIdx = $state(0);
  let rootEl = $state<HTMLDivElement | null>(null);
  let triggerEl = $state<HTMLButtonElement | null>(null);
  let optionEls = $state<Record<number, HTMLButtonElement>>({});
  const listboxId = $props.id();

  let selectedIndex = $derived(Math.max(0, options.findIndex((o) => o.value === value)));
  let selectedLabel = $derived(
    options.find((o) => o.value === value)?.label ?? options[0]?.label ?? "",
  );

  function openMenu(focusIndex = selectedIndex) {
    open = true;
    activeIdx = Math.max(0, Math.min(options.length - 1, focusIndex));
    queueMicrotask(() => {
      optionEls[activeIdx]?.focus();
    });
  }

  function closeMenu(returnFocus = false) {
    open = false;
    if (returnFocus) {
      triggerEl?.focus();
    }
  }

  function select(opt: SelectOption) {
    closeMenu(true);
    if (opt.value !== value) {
      onChange(opt.value);
    }
  }

  function handleTriggerKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault();
      if (!open) {
        openMenu(selectedIndex);
      }
    }
  }

  function handleListKeydown(e: KeyboardEvent) {
    if (!open || options.length === 0) return;
    if (e.key === "Escape") {
      e.preventDefault();
      closeMenu(true);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      activeIdx = (activeIdx + 1) % options.length;
      optionEls[activeIdx]?.focus();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      activeIdx = (activeIdx - 1 + options.length) % options.length;
      optionEls[activeIdx]?.focus();
    } else if (e.key === "Home") {
      e.preventDefault();
      activeIdx = 0;
      optionEls[activeIdx]?.focus();
    } else if (e.key === "End") {
      e.preventDefault();
      activeIdx = options.length - 1;
      optionEls[activeIdx]?.focus();
    } else if (e.key === "Tab") {
      closeMenu(false);
    }
  }

  function handleWindowClick(e: MouseEvent) {
    if (open && rootEl && !rootEl.contains(e.target as Node)) {
      closeMenu(false);
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div bind:this={rootEl} class="relative inline-block">
  <button
    bind:this={triggerEl}
    type="button"
    class="inline-flex items-center gap-2 h-8.5 px-3 rounded-full text-[12.5px] font-semibold bg-[var(--surface)] hover:bg-[var(--surface-hover)] border transition-colors active:scale-[0.98] {open
      ? 'border-[var(--accent-border)] text-[var(--text)]'
      : 'border-[var(--border)] hover:border-[var(--border-strong)] text-[var(--text)]'}"
    aria-haspopup="listbox"
    aria-expanded={open}
    aria-controls={open ? listboxId : undefined}
    aria-label={ariaLabel}
    onclick={() => (open ? closeMenu(false) : openMenu())}
    onkeydown={handleTriggerKeydown}
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
      id={listboxId}
      transition:fly={{ y: -6, duration: 150, easing: cubicOut }}
      role="listbox"
      aria-label={ariaLabel}
      aria-activedescendant={`${listboxId}-opt-${activeIdx}`}
      tabindex="-1"
      class="absolute right-0 top-[calc(100%+6px)] z-50 w-48 max-h-72 overflow-y-auto scroll-y rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border-strong)] shadow-[0_18px_44px_rgba(0,0,0,0.55)] p-1.5"
      onkeydown={handleListKeydown}
    >
      {#each options as opt, i (opt.value)}
        {@const active = opt.value === value}
        <button
          id={`${listboxId}-opt-${i}`}
          bind:this={optionEls[i]}
          type="button"
          role="option"
          aria-selected={active}
          tabindex={i === activeIdx ? 0 : -1}
          class="w-full flex items-center justify-between gap-2 px-3 py-2 rounded-xl text-[13px] text-left transition-colors {active
            ? 'bg-[var(--accent-soft)] text-[var(--accent-bright)] font-semibold'
            : 'text-[var(--text)] hover:bg-[var(--surface-hover)]'}"
          onfocus={() => (activeIdx = i)}
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
