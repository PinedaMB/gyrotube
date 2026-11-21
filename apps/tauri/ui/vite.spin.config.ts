import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { resolve } from "node:path";

export default defineConfig({
  plugins: [tailwindcss(), svelte({ emitCss: true })],
  base: "./",
  build: {
    outDir: "dist-spin",
    emptyOutDir: true,
    target: "esnext",
    cssCodeSplit: false,
    assetsInlineLimit: 100000,
    rollupOptions: {
      input: resolve(__dirname, "spin.html"),
      output: {
        inlineDynamicImports: true,
        entryFileNames: "spin.js",
        assetFileNames: "spin.[ext]",
      },
    },
  },
});
