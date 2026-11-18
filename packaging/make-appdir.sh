#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

binary=${1:?Usage: packaging/make-appdir.sh /path/to/binary /path/to/new/AppDir}
appdir=${2:?Specify a new AppDir directory}
app_id=com.github.gyrotube.GyroTube
multiarch=$(gcc -print-multiarch)
lib=/usr/lib/$multiarch
mkdir "$appdir"
appdir=$(realpath "$appdir")

install -Dm755 "$binary" "$appdir/usr/bin/gyrotube"
install -Dm755 packaging/AppRun "$appdir/AppRun"
install -Dm644 "data/$app_id.desktop" "$appdir/usr/share/applications/$app_id.desktop"
install -Dm644 gyrotube.svg "$appdir/usr/share/icons/hicolor/scalable/apps/$app_id.svg"
ln -s "usr/share/applications/$app_id.desktop" "$appdir/$app_id.desktop"
rsvg-convert -w 256 -h 256 gyrotube.svg -o "$appdir/$app_id.png"
ln -s "$app_id.png" "$appdir/.DirIcon"

gst_plugins=(coreelements playback typefindfunctions audioconvert audioresample volume autodetect
    pulseaudio soup matroska isomp4 opus vorbis ogg libav audioparsers)
scanner="$lib/gstreamer1.0/gstreamer-1.0/gst-plugin-scanner"
pixbuf="$lib/gdk-pixbuf-2.0/2.10.0"
install -Dm755 "$scanner" "$appdir/usr/libexec/gst-plugin-scanner.real"
cat > "$appdir/usr/libexec/gst-plugin-scanner" <<'RUN'
#!/bin/sh
export LD_LIBRARY_PATH="$APPDIR/usr/lib"
exec "$APPDIR/usr/libexec/gst-plugin-scanner.real" "$@"
RUN
chmod 755 "$appdir/usr/libexec/gst-plugin-scanner"

roots=("$binary" "$scanner")
for plugin in "${gst_plugins[@]}"; do
    path="$lib/gstreamer-1.0/libgst$plugin.so"
    install -Dm755 "$path" "$appdir/usr/lib/gstreamer-1.0/${path##*/}"
    roots+=("$path")
done
for module in "$lib"/gio/modules/*.so; do
    install -Dm755 "$module" "$appdir/usr/lib/gio/modules/${module##*/}"
    roots+=("$module")
done
for loader in "$pixbuf"/loaders/*.so; do
    install -Dm755 "$loader" "$appdir/usr/lib/gdk-pixbuf-2.0/loaders/${loader##*/}"
    roots+=("$loader")
done
sed "s|$pixbuf|@APPDIR@/usr/lib/gdk-pixbuf-2.0|g" "$pixbuf/loaders.cache" \
    > "$appdir/usr/lib/gdk-pixbuf-2.0/loaders.cache"

for root in "${roots[@]}"; do
    ldd "$root"
done | awk '$2 == "=>" && $3 ~ /^\// { print $1, $3 }' | sort -u |
while read -r soname path; do
    case "$soname" in
        ld-linux-*|libc.so.*|libm.so.*|libpthread.so.*|libdl.so.*|librt.so.*|libresolv.so.*|libGL*.so.*|libEGL*.so.*|libgbm.so.*|libdrm*.so.*)
            continue ;;
    esac
    install -m755 "$path" "$appdir/usr/lib/$soname"
done
install -Dm644 /usr/share/glib-2.0/schemas/gschemas.compiled \
    "$appdir/usr/share/glib-2.0/schemas/gschemas.compiled"
gio-querymodules "$appdir/usr/lib/gio/modules"
