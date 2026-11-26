#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FLATPAK_DIR="$ROOT_DIR/flatpak"
METAINFO="$ROOT_DIR/apps/tauri/src-tauri/linux/io.github.pinedamb.gyrotube.metainfo.xml"
DESKTOP="$ROOT_DIR/apps/tauri/src-tauri/linux/io.github.pinedamb.gyrotube.desktop"

echo "==> 1. Validating AppStream MetaInfo..."
appstreamcli validate --pedantic --explain "$METAINFO"

if command -v desktop-file-validate >/dev/null 2>&1; then
  echo "==> 2. Validating .desktop entry..."
  desktop-file-validate "$DESKTOP"
fi

echo "==> 3. Pinning latest yt-dlp release URL + sha256 in manifest..."
MANIFEST="$FLATPAK_DIR/io.github.pinedamb.gyrotube.yml"
if command -v curl >/dev/null 2>&1 && command -v sha256sum >/dev/null 2>&1; then
  YTDLP_TAG="$(curl -fsSL https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest | python3 -c 'import json,sys; print(json.load(sys.stdin)["tag_name"])' 2>/dev/null || true)"
  if [ -n "$YTDLP_TAG" ]; then
    YTDLP_URL="https://github.com/yt-dlp/yt-dlp/releases/download/${YTDLP_TAG}/yt-dlp"
    YTDLP_SHA="$(curl -fsSL "$YTDLP_URL" | sha256sum | awk '{print $1}')"
    if [ -n "$YTDLP_SHA" ]; then
      python3 - "$MANIFEST" "$YTDLP_URL" "$YTDLP_SHA" <<'PY'
import re, sys
path, url, sha = sys.argv[1:4]
text = open(path, encoding="utf-8").read()
text = re.sub(r"https://github\.com/yt-dlp/yt-dlp/releases/\S+", url, text, count=1)
text = re.sub(r"sha256:\s*\S+", f"sha256: {sha}", text, count=1)
open(path, "w", encoding="utf-8").write(text)
PY
      echo "  Pinned yt-dlp ${YTDLP_TAG} (sha256=${YTDLP_SHA})"
    fi
  fi
fi

echo "==> 4. Checking offline source generators..."
if command -v flatpak-cargo-generator >/dev/null 2>&1; then
  echo "Generating flatpak/cargo-sources.json from Cargo.lock..."
  flatpak-cargo-generator "$ROOT_DIR/Cargo.lock" -o "$FLATPAK_DIR/cargo-sources.json"
else
  echo "  [skip] flatpak-cargo-generator not found."
  echo "         Install from: https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo"
fi

if command -v flatpak-node-generator >/dev/null 2>&1; then
  if [ -f "$ROOT_DIR/apps/tauri/ui/package-lock.json" ]; then
    echo "Generating flatpak/node-sources.json from package-lock.json..."
    flatpak-node-generator npm "$ROOT_DIR/apps/tauri/ui/package-lock.json" -o "$FLATPAK_DIR/node-sources.json"
  else
    echo "  [info] Generate package-lock.json first (cd apps/tauri/ui && npm install --package-lock-only) for flatpak-node-generator."
  fi
else
  echo "  [skip] flatpak-node-generator not found."
  echo "         Install from: https://github.com/flatpak/flatpak-builder-tools/tree/master/node"
fi

echo ""
echo "==> Next steps to test locally & submit to Flathub:"
echo "  1. Pin the exact v1.0.5 git commit hash and yt-dlp release URL + sha256 in flatpak/io.github.pinedamb.gyrotube.yml"
echo "  2. Build & test locally:"
echo "     flatpak-builder --user --install --force-clean build-dir flatpak/io.github.pinedamb.gyrotube.yml"
echo "  3. Run Flathub linter:"
echo "     flatpak run --command=flatpak-builder-lint org.flatpak.Builder manifest flatpak/io.github.pinedamb.gyrotube.yml"
echo "     flatpak run --command=flatpak-builder-lint org.flatpak.Builder repo repo"
echo "  4. Fork https://github.com/flathub/flathub (branch: new-pr), add flatpak/* files, and open a PR."
