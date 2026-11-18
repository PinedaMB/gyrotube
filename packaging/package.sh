#!/usr/bin/env bash
set -euo pipefail
deb_binary=$(realpath "${1:?Usage: packaging/package.sh DEBIAN_BINARY FEDORA_BINARY APPDIR}")
rpm_binary=$(realpath "${2:?Specify the Fedora binary}")
appdir=$(realpath "${3:?Specify the prepared AppDir}")
cmp "$deb_binary" "$appdir/usr/bin/gyrotube"
cd "$(dirname "$0")/.."

nfpm=${NFPM:-nfpm}
appimagetool=${APPIMAGETOOL:-appimagetool}
version=$(awk -F'"' '/^version = / { print $2; exit }' Cargo.toml)
mkdir -p dist
export GYROTUBE_PACKAGE_VERSION="$version"
GYROTUBE_PACKAGE_BINARY="$deb_binary" GYROTUBE_PACKAGE_RELEASE=1 \
    "$nfpm" package --config packaging/nfpm.yaml --packager deb --target "dist/gyrotube_${version}-1_amd64.deb"
GYROTUBE_PACKAGE_BINARY="$rpm_binary" GYROTUBE_PACKAGE_RELEASE=1.fc45 \
    "$nfpm" package --config packaging/nfpm.yaml --packager rpm --target "dist/gyrotube-$version-1.fc45.x86_64.rpm"

runtime=()
if [[ -n "${APPIMAGE_RUNTIME:-}" ]]; then
    runtime=(--runtime-file "$APPIMAGE_RUNTIME")
fi
ARCH=x86_64 APPIMAGE_EXTRACT_AND_RUN=1 "$appimagetool" --no-appstream "${runtime[@]}" \
    "$appdir" "dist/GyroTube-$version-x86_64.AppImage"
(
    cd dist
    sha256sum "gyrotube_${version}-1_amd64.deb" "gyrotube-$version-1.fc45.x86_64.rpm" \
        "GyroTube-$version-x86_64.AppImage" > SHA256SUMS
)
