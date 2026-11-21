<div class="space-y-6 animate-pulse" aria-busy="true" aria-label="Searching">
  <!-- Filter pills skeleton -->
  <div class="flex items-center gap-2">
    {#each [64, 92, 96, 86, 98] as w}
      <div class="h-8 rounded-full bg-[var(--skeleton)]" style="width: {w}px;"></div>
    {/each}
  </div>

  <!-- Top result + Top 4 songs skeleton -->
  <div class="grid grid-cols-1 md:grid-cols-[minmax(240px,300px)_minmax(0,1fr)] gap-5">
    <div class="space-y-2.5">
      <div class="h-4 w-24 rounded-md bg-[var(--skeleton)]"></div>
      <div class="rounded-2xl p-5 bg-[var(--surface)] border border-[var(--border)] space-y-4">
        <div class="w-20 h-20 rounded-full bg-[var(--skeleton)]"></div>
        <div class="h-6 w-40 rounded-lg bg-[var(--skeleton)]"></div>
        <div class="flex items-center justify-between pt-2">
          <div class="h-5 w-28 rounded-full bg-[var(--skeleton-soft)]"></div>
          <div class="w-10 h-10 rounded-full bg-[var(--skeleton)]"></div>
        </div>
      </div>
    </div>

    <div class="space-y-2.5">
      <div class="h-4 w-20 rounded-md bg-[var(--skeleton)]"></div>
      <div class="track-list">
        {#each [0, 1, 2, 3] as _}
          <div class="flex items-center gap-3 px-3.5 py-2.5 border-b border-[var(--border)] last:border-b-0">
            <div class="w-11 h-11 rounded-lg bg-[var(--skeleton)] shrink-0"></div>
            <div class="flex-1 space-y-2">
              <div class="h-3.5 w-44 rounded bg-[var(--skeleton)]"></div>
              <div class="h-3 w-32 rounded bg-[var(--skeleton-soft)]"></div>
            </div>
            <div class="h-3 w-9 rounded bg-[var(--skeleton-soft)]"></div>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Albums carousel skeleton -->
  <div class="space-y-3">
    <div class="h-5 w-28 rounded-md bg-[var(--skeleton)]"></div>
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3">
      {#each [0, 1, 2, 3, 4] as _}
        <div class="rounded-2xl p-2.5 bg-[var(--surface)] border border-[var(--border)] space-y-2.5">
          <div class="aspect-square w-full rounded-xl bg-[var(--skeleton)]"></div>
          <div class="h-3.5 w-4/5 rounded bg-[var(--skeleton)]"></div>
          <div class="h-3 w-1/2 rounded bg-[var(--skeleton-soft)]"></div>
        </div>
      {/each}
    </div>
  </div>
</div>
