#!/usr/bin/env bash
set -euo pipefail

# Builds GyroTube Desktop installers (Tauri v2) and copies them into <repo>/output/desktop/.
# Usage:
#   ./scripts/build-desktop.sh                 # All platform bundles (AppImage, rpm, deb, ...)
#   ./scripts/build-desktop.sh --bundles rpm   # Specific bundle only

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAURI_DIR="${ROOT_DIR}/apps/tauri"
BUNDLE_DIR="${ROOT_DIR}/target/release/bundle"
OUT_DIR="${ROOT_DIR}/output/desktop"

# Avoid linuxdeploy strip errors on modern glibc/ELF distros (e.g. Fedora)
export NO_STRIP="${NO_STRIP:-true}"

(cd "${TAURI_DIR}" && cargo tauri build "$@")

mkdir -p "${OUT_DIR}"
find "${BUNDLE_DIR}" -maxdepth 3 -type f \
  \( -name "*.AppImage" -o -name "*.rpm" -o -name "*.deb" -o -name "*.msi" -o -name "*.exe" -o -name "*.dmg" \) \
  -exec cp -f {} "${OUT_DIR}/" \;

echo ""
echo "Instaladores listos en: ${OUT_DIR}"
ls -lh "${OUT_DIR}"
