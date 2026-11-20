#!/usr/bin/env bash
set -euo pipefail

# Applies GyroTube Android customizations after `cargo tauri android init`.
# Safe to run repeatedly (idempotent).

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GEN_MAIN="${SCRIPT_DIR}/../src-tauri/gen/android/app/src/main"
MANIFEST="${GEN_MAIN}/AndroidManifest.xml"

if [[ ! -f "${MANIFEST}" ]]; then
  echo "Error: ${MANIFEST} not found. Run 'cargo tauri android init' inside apps/tauri first." >&2
  exit 1
fi

mkdir -p "${GEN_MAIN}/res/xml"
cp "${SCRIPT_DIR}/res/xml/network_security_config.xml" "${GEN_MAIN}/res/xml/network_security_config.xml"

KOTLIN_DEST="${GEN_MAIN}/java/com/github/gyrotube/GyroTube"
mkdir -p "${KOTLIN_DEST}"
cp "${SCRIPT_DIR}/kotlin/com/github/gyrotube/GyroTube/PlaybackForegroundService.kt" \
   "${KOTLIN_DEST}/PlaybackForegroundService.kt"

python3 - "${MANIFEST}" <<'PY'
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")

permissions = [
    "android.permission.INTERNET",
    "android.permission.WAKE_LOCK",
    "android.permission.FOREGROUND_SERVICE",
    "android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK",
    "android.permission.POST_NOTIFICATIONS",
]

for perm in permissions:
    tag = f'<uses-permission android:name="{perm}" />'
    if perm not in text:
        text = text.replace("<application", f"    {tag}\n    <application", 1)

if "android:networkSecurityConfig" not in text:
    text = text.replace(
        "<application",
        '<application\n        android:networkSecurityConfig="@xml/network_security_config"',
        1,
    )

service_decl = (
    '        <service\n'
    '            android:name=".PlaybackForegroundService"\n'
    '            android:exported="false"\n'
    '            android:foregroundServiceType="mediaPlayback" />\n'
)
if "PlaybackForegroundService" not in text:
    text = text.replace("</application>", f"{service_decl}    </application>", 1)

path.write_text(text, encoding="utf-8")
print("Applied GyroTube Android manifest & foreground service configuration.")
PY
