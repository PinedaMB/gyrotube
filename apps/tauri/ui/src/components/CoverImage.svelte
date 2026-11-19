<script lang="ts">
  interface Props {
    src?: string | null;
    fallbackSrc?: string | null;
    alt?: string;
    class?: string;
    fallbackIcon?: string;
  }

  let { src = null, fallbackSrc = null, alt = "", class: className = "", fallbackIcon = "♪" }: Props = $props();

  let fallbackStage = $state(0);
  let failed = $state(false);
  let currentSrc = $state<string | null>(null);

  $effect(() => {
    currentSrc = src || fallbackSrc || null;
    fallbackStage = 0;
    failed = false;
  });

  function handleError() {
    if (fallbackStage === 0 && fallbackSrc && currentSrc !== fallbackSrc) {
      fallbackStage = 1;
      currentSrc = fallbackSrc;
      return;
    }
    if (fallbackStage < 2 && currentSrc) {
      fallbackStage = 2;
      if (currentSrc.includes("googleusercontent.com") && currentSrc.includes("=w")) {
        const downgraded = currentSrc.replace(/=w\d+-h\d+[^&?]*/, "=w226-h226-l90-rj");
        if (downgraded !== currentSrc) {
          currentSrc = downgraded;
          return;
        }
      }
      if (currentSrc.includes("i.ytimg.com") && !currentSrc.includes("hqdefault")) {
        const downgraded = currentSrc.replace(/\/(maxresdefault|sddefault|mqdefault)\./, "/hqdefault.");
        if (downgraded !== currentSrc) {
          currentSrc = downgraded;
          return;
        }
      }
    }
    failed = true;
  }
</script>

{#if currentSrc && !failed}
  <img
    class={className}
    src={currentSrc}
    {alt}
    loading="lazy"
    referrerpolicy="no-referrer"
    onerror={handleError}
  />
{:else}
  <div class="cover-fallback {className}" aria-hidden="true">{fallbackIcon}</div>
{/if}

<style>
  .cover-fallback {
    display: grid;
    place-items: center;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.08), rgba(255, 255, 255, 0.03));
    color: var(--text-muted);
    font-size: 1.4rem;
    user-select: none;
  }
</style>
