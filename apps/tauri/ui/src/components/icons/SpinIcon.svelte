<script lang="ts">
  interface Props {
    size?: number;
    spinning?: boolean;
    class?: string;
  }

  let { size = 18, spinning = false, class: className = "" }: Props = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.9"
  stroke-linecap="round"
  stroke-linejoin="round"
  class={className}
  aria-hidden="true"
>
  <g class:spin-orbit={spinning}>
    <path d="M 3.5 9.7 A 8.8 8.8 0 0 1 19.2 7" />
    <path d="M 20.5 14.3 A 8.8 8.8 0 0 1 4.8 17" />
    <circle cx="19.2" cy="7" r="1.55" fill="currentColor" stroke="none" />
    <circle cx="4.8" cy="17" r="1.55" fill="currentColor" stroke="none" />
  </g>
  <g class:spin-groove={spinning}>
    <path d="M 8.3 8.3 A 5.2 5.2 0 0 1 13.3 7" stroke-width="1.65" opacity="0.72" />
    <path d="M 15.7 15.7 A 5.2 5.2 0 0 1 10.7 17" stroke-width="1.65" opacity="0.72" />
  </g>
  <circle cx="12" cy="12" r="2.2" />
</svg>

<style>
  .spin-orbit {
    transform-origin: 12px 12px;
    animation: spin-mode-rotate 5.5s linear infinite;
  }

  .spin-groove {
    transform-origin: 12px 12px;
    animation: spin-mode-rotate 5.5s linear infinite;
  }

  @keyframes spin-mode-rotate {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
