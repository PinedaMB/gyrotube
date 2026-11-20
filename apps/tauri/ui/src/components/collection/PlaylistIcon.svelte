<script module lang="ts">
  export const PLAYLIST_ICON_IDS = [
    "music",
    "heart",
    "star",
    "flame",
    "zap",
    "moon",
    "sun",
    "headphones",
    "disc",
    "radio",
    "sparkles",
    "coffee",
    "car",
    "dumbbell",
    "rain",
    "compass",
  ] as const;
</script>

<script lang="ts">
  import {
    Car,
    CloudRain,
    Coffee,
    Compass,
    Disc3,
    Dumbbell,
    Flame,
    Headphones,
    Heart,
    Moon,
    Music,
    Radio,
    Sparkles,
    Star,
    Sun,
    Zap,
  } from "lucide-svelte";

  let { name = "music", size = 18 }: { name?: string | null; size?: number } = $props();

  let normalized = $derived.by(() => {
    const raw = (name ?? "music").trim().toLowerCase();
    if (raw === "♥" || raw === "heart") return "heart";
    if (raw === "★" || raw === "star") return "star";
    if (raw === "🔥" || raw === "flame") return "flame";
    if (raw === "⚡" || raw === "zap") return "zap";
    if (raw === "🌙" || raw === "moon") return "moon";
    if (raw === "☀" || raw === "sun") return "sun";
    if (raw === "🎧" || raw === "headphones") return "headphones";
    if (raw === "disc") return "disc";
    if (raw === "radio") return "radio";
    if (raw === "sparkles") return "sparkles";
    if (raw === "coffee") return "coffee";
    if (raw === "🚗" || raw === "car") return "car";
    if (raw === "🏋" || raw === "dumbbell") return "dumbbell";
    if (raw === "rain") return "rain";
    if (raw === "compass") return "compass";
    return "music";
  });
</script>

{#if normalized === "heart"}
  <Heart {size} fill="currentColor" />
{:else if normalized === "star"}
  <Star {size} fill="currentColor" />
{:else if normalized === "flame"}
  <Flame {size} />
{:else if normalized === "zap"}
  <Zap {size} />
{:else if normalized === "moon"}
  <Moon {size} />
{:else if normalized === "sun"}
  <Sun {size} />
{:else if normalized === "headphones"}
  <Headphones {size} />
{:else if normalized === "disc"}
  <Disc3 {size} />
{:else if normalized === "radio"}
  <Radio {size} />
{:else if normalized === "sparkles"}
  <Sparkles {size} />
{:else if normalized === "coffee"}
  <Coffee {size} />
{:else if normalized === "car"}
  <Car {size} />
{:else if normalized === "dumbbell"}
  <Dumbbell {size} />
{:else if normalized === "rain"}
  <CloudRain {size} />
{:else if normalized === "compass"}
  <Compass {size} />
{:else}
  <Music {size} />
{/if}
