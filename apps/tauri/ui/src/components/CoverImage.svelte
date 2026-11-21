<script module lang="ts">
  // Single shared IntersectionObserver across all CoverImage instances
  // instead of creating 50-100 separate observers per view.
  const callbacks = new WeakMap<Element, () => void>();
  let sharedObserver: IntersectionObserver | null = null;

  function observeElement(el: Element, cb: () => void): () => void {
    if (typeof IntersectionObserver === "undefined") {
      cb();
      return () => {};
    }
    if (!sharedObserver) {
      sharedObserver = new IntersectionObserver(
        (entries) => {
          for (const entry of entries) {
            if (entry.isIntersecting) {
              const fn = callbacks.get(entry.target);
              if (fn) {
                callbacks.delete(entry.target);
                sharedObserver?.unobserve(entry.target);
                fn();
              }
            }
          }
        },
        { rootMargin: "180px" },
      );
    }
    callbacks.set(el, cb);
    sharedObserver.observe(el);
    return () => {
      callbacks.delete(el);
      sharedObserver?.unobserve(el);
    };
  }
</script>

<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    src?: string | null;
    fallbackSrc?: string | null;
    alt?: string;
    class?: string;
    fallbackIcon?: string;
    eager?: boolean;
    targetPx?: number;
  }

  let {
    src = null,
    fallbackSrc = null,
    alt = "",
    class: className = "",
    fallbackIcon = "♪",
    eager = false,
    targetPx,
  }: Props = $props();

  let hostEl = $state<HTMLDivElement | null>(null);
  let inView = $state(false);
  let loaded = $state(false);
  let fallbackStage = $state(0);
  let failed = $state(false);
  let currentSrc = $state<string | null>(null);

  function inferTargetPx(): number {
    if (targetPx) return targetPx;
    if (
      className.includes("track-cover") ||
      className.includes("tile-cover") ||
      className.includes("up-next-cover") ||
      className.includes("w-10")
    ) {
      return 120;
    }
    if (
      className.includes("media-cover") ||
      className.includes("artist-avatar") ||
      className.includes("top-avatar") ||
      className.includes("hero-cover") ||
      className.includes("album-cover")
    ) {
      return 360;
    }
    return 544;
  }

  function resizeUrl(url: string | null | undefined): string | null {
    if (!url) return null;
    const px = inferTargetPx();
    if (url.includes("googleusercontent.com") && /=w\d+-h\d+/.test(url)) {
      return url.replace(/=w\d+-h\d+[^&?]*/, `=w${px}-h${px}-l90-rj`);
    }
    if (url.includes("i.ytimg.com") && px <= 120) {
      return url.replace(/\/(maxresdefault|sddefault|hqdefault)\./, "/mqdefault.");
    }
    return url;
  }

  $effect(() => {
    const next = resizeUrl(src) || resizeUrl(fallbackSrc) || null;
    if (next !== currentSrc) {
      currentSrc = next;
      fallbackStage = 0;
      failed = false;
      loaded = false;
    }
  });

  onMount(() => {
    if (eager || !hostEl) {
      inView = true;
      return;
    }
    return observeElement(hostEl, () => {
      inView = true;
    });
  });

  function handleError() {
    if (fallbackStage === 0 && fallbackSrc) {
      const fb = resizeUrl(fallbackSrc);
      if (fb && fb !== currentSrc) {
        fallbackStage = 1;
        currentSrc = fb;
        return;
      }
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

<div bind:this={hostEl} class="cover-shell {className}">
  {#if currentSrc && !failed}
    {#if !loaded}
      <div class="cover-skeleton" aria-hidden="true"></div>
    {/if}
    {#if inView}
      <img
        class="cover-img"
        class:is-loaded={loaded}
        src={currentSrc}
        {alt}
        loading={eager ? "eager" : "lazy"}
        decoding="async"
        referrerpolicy="no-referrer"
        onload={() => (loaded = true)}
        onerror={handleError}
      />
    {/if}
  {:else}
    <div class="cover-fallback" aria-hidden="true">{fallbackIcon}</div>
  {/if}
</div>

<style>
  .cover-shell {
    position: relative;
    overflow: hidden;
    background: var(--surface);
  }

  .cover-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
    opacity: 0;
    transition: opacity 180ms ease-out;
  }

  .cover-img.is-loaded {
    opacity: 1;
  }

  .cover-skeleton {
    position: absolute;
    inset: 0;
    background: rgba(255, 255, 255, 0.05);
  }

  .cover-fallback {
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.08), rgba(255, 255, 255, 0.03));
    color: var(--text-muted);
    font-size: 1.4rem;
    user-select: none;
  }
</style>
