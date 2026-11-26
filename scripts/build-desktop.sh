#!/usr/bin/env bash
set -euo pipefail

# Builds Gyro Desktop installers (Tauri v2) and copies them into <repo>/output/desktop/.
# Usage:
#   ./scripts/build-desktop.sh                 # All platform bundles (AppImage, rpm, deb, ...)
#   ./scripts/build-desktop.sh --bundles rpm   # Specific bundle only
# Optional RPM signing (for Fedora / RPM 6+ GNOME Software):
#   RPM_GPG_KEY="Your Key Name" ./scripts/build-desktop.sh --bundles rpm

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAURI_DIR="${ROOT_DIR}/apps/tauri"
BUNDLE_DIR="${ROOT_DIR}/target/release/bundle"
OUT_DIR="${ROOT_DIR}/output/desktop"

# Avoid linuxdeploy strip errors on modern glibc/ELF distros (e.g. Fedora)
export NO_STRIP="${NO_STRIP:-true}"

# Clean previous bundle staging artifacts so stale versions (e.g. 0.1.0) are never copied over
rm -rf "${BUNDLE_DIR}"

# Skip updater .sig generation during local builds when no signing private key is configured
TAURI_EXTRA_ARGS=()
if [[ -z "${TAURI_SIGNING_PRIVATE_KEY:-}" ]]; then
  TAURI_EXTRA_ARGS+=(--config '{"bundle":{"createUpdaterArtifacts":false}}')
fi

(cd "${TAURI_DIR}" && cargo tauri build "${TAURI_EXTRA_ARGS[@]}" "$@")

mkdir -p "${OUT_DIR}"

# Remove installers in output/desktop from older versions
VERSION="$(node -e "console.log(require('${TAURI_DIR}/src-tauri/tauri.conf.json').version)")"
find "${OUT_DIR}" -maxdepth 1 -type f \
  \( -name "*.AppImage" -o -name "*.rpm" -o -name "*.deb" -o -name "*.msi" -o -name "*.exe" -o -name "*.dmg" \) \
  ! -name "*${VERSION}*" -delete

find "${BUNDLE_DIR}" -maxdepth 3 -type f \
  \( -name "*.AppImage" -o -name "*.rpm" -o -name "*.deb" -o -name "*.msi" -o -name "*.exe" -o -name "*.dmg" \) \
  -exec cp -f {} "${OUT_DIR}/" \;

# Optionally sign generated RPMs if rpmsign and a GPG key are configured
if command -v rpmsign >/dev/null 2>&1; then
  SIGN_KEY="${RPM_GPG_KEY:-}"
  if [[ -z "${SIGN_KEY}" && -f "${HOME}/.rpmmacros" ]] && grep -q '^%_gpg_name' "${HOME}/.rpmmacros"; then
    SIGN_KEY="__rpmmacros__"
  fi
  if [[ -n "${SIGN_KEY}" ]]; then
    for rpm_file in "${OUT_DIR}"/*"${VERSION}"*.rpm; do
      [[ -f "${rpm_file}" ]] || continue
      if [[ "${SIGN_KEY}" == "__rpmmacros__" ]]; then
        rpmsign --addsign "${rpm_file}"
      else
        rpmsign --define "_gpg_name ${SIGN_KEY}" --addsign "${rpm_file}"
      fi
    done
  fi
fi

echo ""
echo "Instaladores listos en: ${OUT_DIR}"
ls -lh "${OUT_DIR}"
