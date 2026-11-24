<script lang="ts">
  import { ChevronDown, ChevronUp, GripVertical, Music } from "lucide-svelte";
  import { sg } from "./i18n";
  import { fmtTime, type JamTrackInfo } from "./types";

  interface Props {
    queue: JamTrackInfo[];
    isPlaying: boolean;
    allowControls: boolean;
    visibleMobile: boolean;
    onMove: (from: number, to: number) => void;
  }

  let { queue, isPlaying, allowControls, visibleMobile, onMove }: Props = $props();

  let dragFrom = $state<number | null>(null);
  let dragOver = $state<number | null>(null);
</script>

<section
  class="rounded-2xl bg-[var(--bg-elevated)] border border-[var(--border)] p-4 shadow-[0_12px_32px_rgba(0,0,0,0.28)] {visibleMobile
    ? 'block'
    : 'hidden lg:block'}"
  aria-label={sg("queue.aria")}
>
  <div class="flex items-center justify-between gap-2 mb-3">
    <span class="text-[11px] font-bold uppercase tracking-[0.09em] text-[var(--text-muted)]">
      {sg("queue.header", { count: queue.length })}
    </span>
  </div>

  {#if queue.length === 0}
    <div class="py-8 text-center text-[13.5px] text-[var(--text-muted)]">
      {sg("queue.empty")}
    </div>
  {:else}
    <div class="flex flex-col gap-1 lg:max-h-[420px] lg:overflow-y-auto scroll-thin pr-0.5">
      {#each queue as item, i (item.id + ":" + i)}
        <div
          role="listitem"
          draggable={allowControls}
          ondragstart={(e) => {
            dragFrom = i;
            e.dataTransfer?.setData("text/plain", String(i));
          }}
          ondragover={(e) => {
            if (!allowControls) return;
            e.preventDefault();
            dragOver = i;
          }}
          ondragleave={() => {
            if (dragOver === i) dragOver = null;
          }}
          ondrop={(e) => {
            e.preventDefault();
            const from = dragFrom;
            dragFrom = null;
            dragOver = null;
            if (from !== null && from !== i) onMove(from, i);
          }}
          ondragend={() => {
            dragFrom = null;
            dragOver = null;
          }}
          class="flex items-center gap-2.5 p-2 rounded-xl border transition-colors {item.is_current
            ? 'bg-[var(--accent-soft)] border-[var(--accent-border)]'
            : 'border-transparent hover:bg-white/[0.045]'} {dragFrom === i
            ? 'opacity-40'
            : ''} {dragOver === i && dragFrom !== i
            ? 'ring-1 ring-[var(--accent-bright)]'
            : ''}"
        >
          {#if allowControls}
            <span class="text-[var(--text-muted)]/50 cursor-grab shrink-0 hidden sm:flex" aria-hidden="true">
              <GripVertical size={14} />
            </span>
          {/if}

          <div class="relative w-10 h-10 rounded-lg overflow-hidden bg-white/[0.06] border border-white/[0.08] shrink-0">
            {#if item.cover_url}
              <img src={item.cover_url} alt="" loading="lazy" class="w-full h-full object-cover" />
            {:else}
              <div class="w-full h-full flex items-center justify-center text-[var(--text-muted)]/50">
                <Music size={15} />
              </div>
            {/if}
            {#if item.is_current && isPlaying}
              <div class="absolute inset-0 bg-black/50 flex items-center justify-center gap-[2px]">
                <span class="eq-bar" style="animation-delay: 0ms;"></span>
                <span class="eq-bar" style="animation-delay: 160ms;"></span>
                <span class="eq-bar" style="animation-delay: 320ms;"></span>
              </div>
            {/if}
          </div>

          <div class="min-w-0 flex-1">
            <div
              class="text-[13.5px] font-semibold truncate {item.is_current
                ? 'text-[var(--accent-bright)]'
                : 'text-[var(--text)]'}"
            >
              {item.title}
            </div>
            <div class="text-[12px] text-[var(--text-muted)] truncate">{item.artist}</div>
          </div>

          {#if item.duration_secs}
            <span class="text-[11.5px] text-[var(--text-muted)] numeric shrink-0">
              {fmtTime(item.duration_secs)}
            </span>
          {/if}

          {#if allowControls}
            <div class="flex items-center gap-1 shrink-0">
              <button
                type="button"
                disabled={i === 0}
                class="w-7 h-7 rounded-full flex items-center justify-center bg-white/[0.06] border border-white/[0.08] hover:bg-white/[0.14] transition-colors"
                aria-label={sg("queue.move_up")}
                onclick={() => onMove(i, i - 1)}
              >
                <ChevronUp size={13} />
              </button>
              <button
                type="button"
                disabled={i === queue.length - 1}
                class="w-7 h-7 rounded-full flex items-center justify-center bg-white/[0.06] border border-white/[0.08] hover:bg-white/[0.14] transition-colors"
                aria-label={sg("queue.move_down")}
                onclick={() => onMove(i, i + 1)}
              >
                <ChevronDown size={13} />
              </button>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</section>
