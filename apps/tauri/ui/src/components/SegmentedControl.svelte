<script module lang="ts">
  import type { Icon as LucideIcon } from "lucide-svelte";

  export interface SegmentedOption<V extends string = string> {
    value: V;
    label: string;
    icon?: typeof LucideIcon;
    disabled?: boolean;
    title?: string;
  }
</script>

<script lang="ts" generics="T extends string">
  interface Props {
    options: SegmentedOption<T>[];
    value: T;
    onchange: (value: T) => void;
    ariaLabel?: string;
    size?: "sm" | "md";
    fullWidth?: boolean;
    class?: string;
  }

  let {
    options,
    value,
    onchange,
    ariaLabel,
    size = "md",
    fullWidth = false,
    class: className = "",
  }: Props = $props();

  let iconSize = $derived(size === "sm" ? 12 : 13);

  function select(opt: SegmentedOption<T>) {
    if (opt.disabled || opt.value === value) return;
    onchange(opt.value);
  }

  function handleKeydown(e: KeyboardEvent, idx: number) {
    if (e.key !== "ArrowRight" && e.key !== "ArrowLeft") return;
    e.preventDefault();
    const dir = e.key === "ArrowRight" ? 1 : -1;
    const len = options.length;
    for (let step = 1; step <= len; step++) {
      const next = options[(idx + dir * step + len) % len];
      if (!next.disabled) {
        select(next);
        break;
      }
    }
  }
</script>

<div
  role="radiogroup"
  aria-label={ariaLabel}
  class="{fullWidth
    ? 'grid w-full'
    : 'inline-flex shrink-0'} items-center p-1 rounded-full bg-[var(--segmented-bg)] border border-[var(--border)] gap-1 {className}"
  style={fullWidth ? `grid-template-columns: repeat(${options.length}, minmax(0, 1fr));` : undefined}
>
  {#each options as opt, i (opt.value)}
    {@const active = value === opt.value}
    {@const Icon = opt.icon}
    <button
      type="button"
      role="radio"
      aria-checked={active}
      disabled={opt.disabled}
      title={opt.title}
      class="inline-flex items-center justify-center gap-1.5 rounded-full font-semibold transition-all select-none {size === 'sm'
        ? 'px-3 py-1 text-[11.5px]'
        : 'px-3 py-1.5 text-[12px]'} {active
        ? 'bg-[var(--accent)] text-[var(--accent-ink)] shadow-sm'
        : opt.disabled
          ? 'text-[var(--text-muted)] opacity-40 cursor-not-allowed'
          : 'text-[var(--text-muted)] hover:text-[var(--text)]'}"
      onclick={() => select(opt)}
      onkeydown={(e) => handleKeydown(e, i)}
    >
      {#if Icon}
        <Icon size={iconSize} />
      {/if}
      <span>{opt.label}</span>
    </button>
  {/each}
</div>
