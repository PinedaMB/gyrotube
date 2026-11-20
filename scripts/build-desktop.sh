#!/usr/bin/env bash
set -euo pipefail

# Builds all configured desktop installers for the current OS into <repo>/build/.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/build"
BUNDLE_DIR="${ROOT_DIR}/target/release/bundle"

if [[ "$(uname -s)" == Linux ]] && ! command -v patchelf >/dev/null 2>&1; then
  echo "Error: falta patchelf para empaquetar AppImage." >&2
  echo "Fedora: sudo dnf install patchelf | Debian/Ubuntu: sudo apt install patchelf" >&2
  exit 1
fi

if [[ "$(uname -s)" == Linux ]]; then
  # linuxdeploy guesses Debian paths, which do not match Fedora's layout.
  GSTREAMER_PLUGINS_DIR="${GSTREAMER_PLUGINS_DIR:-$(pkg-config --variable=pluginsdir gstreamer-1.0)}"
  GSTREAMER_HELPERS_DIR="${GSTREAMER_HELPERS_DIR:-$(pkg-config --variable=pluginscannerdir gstreamer-1.0)}"
  export GSTREAMER_PLUGINS_DIR GSTREAMER_HELPERS_DIR
  if [[ ! -d "$GSTREAMER_PLUGINS_DIR" || ! -d "$GSTREAMER_HELPERS_DIR" ]]; then
    echo "Error: no se encontraron los directorios de plugins y helpers de GStreamer." >&2
    exit 1
  fi
fi

pnpm --dir "${ROOT_DIR}/apps/tauri/ui" install --frozen-lockfile
(
  cd "${ROOT_DIR}/apps/tauri"
  CARGO_TARGET_DIR="${ROOT_DIR}/target" cargo tauri build
)

shopt -s nullglob
INSTALLERS=(
  "${BUNDLE_DIR}"/deb/*.deb
  "${BUNDLE_DIR}"/rpm/*.rpm
  "${BUNDLE_DIR}"/appimage/*.AppImage
  "${BUNDLE_DIR}"/dmg/*.dmg
  "${BUNDLE_DIR}"/msi/*.msi
  "${BUNDLE_DIR}"/nsis/*.exe
)

if [[ ${#INSTALLERS[@]} -eq 0 ]]; then
  echo "Error: no se encontraron instaladores en ${BUNDLE_DIR}" >&2
  exit 1
fi

mkdir -p "${OUT_DIR}"
cp -f "${INSTALLERS[@]}" "${OUT_DIR}/"
echo "Instaladores listos en: ${OUT_DIR}"
printf '%s\n' "${INSTALLERS[@]##*/}"
