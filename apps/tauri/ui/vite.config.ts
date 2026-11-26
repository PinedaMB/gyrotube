import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig(({ command }) => ({
  // A cold virtual CSS request can fall back to the raw .svelte file in Vite dev.
  plugins: [tailwindcss(), svelte({ emitCss: command === "build" })],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    target: "esnext",
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes("@tauri-apps/api/")) return "vendor-tauri";
          if (id.includes("node_modules/svelte/")) return "vendor-svelte";
          if (id.includes("node_modules/lucide-svelte/")) return "vendor-icons";
        },
      },
    },
  },
}));
