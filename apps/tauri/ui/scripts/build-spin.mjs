import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const uiDir = resolve(__dirname, "..");
const distDir = resolve(uiDir, "dist-spin");
const outFile = resolve(uiDir, "../../../crates/gyrotube-core/assets/spin.html");

execFileSync(
  process.platform === "win32" ? "npx.cmd" : "npx",
  ["vite", "build", "--config", "vite.spin.config.ts"],
  { cwd: uiDir, stdio: "inherit" },
);

const html = readFileSync(resolve(distDir, "spin.html"), "utf8");
const js = readFileSync(resolve(distDir, "spin.js"), "utf8");
const css = readFileSync(resolve(distDir, "spin.css"), "utf8");

// Inline CSS and JS into a single zero-external-asset HTML document
const inlined = html
  .replace(/<link[^>]+href="\.\/spin\.css"[^>]*>/i, () => `<style>${css}</style>`)
  .replace(
    /<script[^>]+src="\.\/spin\.js"[^>]*><\/script>/i,
    () => `<script type="module">${js.replace(/<\/script/gi, "<\\/script")}</script>`,
  );

mkdirSync(dirname(outFile), { recursive: true });
writeFileSync(outFile, inlined, "utf8");
rmSync(distDir, { recursive: true, force: true });

console.log(
  `Built self-contained SPIN web UI -> ${outFile} (${(Buffer.byteLength(inlined) / 1024).toFixed(1)} kB)`,
);
