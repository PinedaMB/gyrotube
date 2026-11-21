<div class="mt-6 space-y-7 animate-pulse" aria-busy="true" aria-label="Loading home">
  <!-- Featured release hero skeleton -->
  <div class="rounded-2xl p-5 bg-[var(--surface)] border border-[var(--border)] flex flex-col sm:flex-row items-center gap-5">
    <div class="w-36 h-36 rounded-2xl bg-white/[0.08] shrink-0"></div>
    <div class="flex-1 w-full space-y-3">
      <div class="h-5 w-24 rounded-full bg-white/[0.08]"></div>
      <div class="h-7 w-64 max-w-full rounded-lg bg-white/[0.09]"></div>
      <div class="h-4 w-40 rounded bg-white/[0.06]"></div>
      <div class="flex items-center gap-2.5 pt-1">
        <div class="h-9 w-28 rounded-full bg-white/[0.09]"></div>
        <div class="h-9 w-28 rounded-full bg-white/[0.06]"></div>
      </div>
    </div>
  </div>

  <!-- New releases carousel skeleton -->
  <div class="space-y-3">
    <div class="h-6 w-44 rounded-md bg-white/[0.08]"></div>
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3.5">
      {#each [0, 1, 2, 3, 4] as _}
        <div class="p-2 space-y-2.5">
          <div class="aspect-square w-full rounded-xl bg-white/[0.08]"></div>
          <div class="h-4 w-4/5 rounded bg-white/[0.08]"></div>
          <div class="h-3 w-3/5 rounded bg-white/[0.05]"></div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Trending charts list skeleton -->
  <div class="space-y-3">
    <div class="flex items-center justify-between">
      <div class="h-6 w-48 rounded-md bg-white/[0.08]"></div>
      <div class="h-8 w-40 rounded-full bg-white/[0.06]"></div>
    </div>
    <div class="space-y-1">
      {#each [0, 1, 2, 3, 4] as _}
        <div class="flex items-center gap-3 px-2 py-2">
          <div class="w-5 h-3.5 rounded bg-white/[0.06] shrink-0"></div>
          <div class="w-11 h-11 rounded-[8px] bg-white/[0.08] shrink-0"></div>
          <div class="flex-1 space-y-2">
            <div class="h-4 w-52 max-w-[70%] rounded bg-white/[0.08]"></div>
            <div class="h-3 w-36 max-w-[50%] rounded bg-white/[0.05]"></div>
          </div>
          <div class="h-3 w-10 rounded bg-white/[0.06] shrink-0"></div>
        </div>
      {/each}
    </div>
  </div>
</div>
