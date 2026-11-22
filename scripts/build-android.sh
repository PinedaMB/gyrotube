#!/usr/bin/env bash
set -euo pipefail

# Builds GyroTube Android APK (aarch64) and copies it into <repo>/output/android/.
# Usage:
#   ./scripts/build-android.sh           # Signed Release APK -> output/android/gyrotube-android-release.apk
#   ./scripts/build-android.sh --debug   # Signed Debug APK   -> output/android/gyrotube-android-debug.apk
#
# Optional signing overrides for release builds:
#   ANDROID_KEYSTORE, ANDROID_KEYSTORE_PASS, ANDROID_KEY_ALIAS, ANDROID_KEY_PASS

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAURI_DIR="${ROOT_DIR}/apps/tauri"
OUT_DIR="${ROOT_DIR}/output/android"

MODE="release"
EXTRA_ARGS=()
if [[ "${1:-}" == "--debug" ]]; then
  MODE="debug"
  EXTRA_ARGS+=(--debug)
fi

export ANDROID_HOME="${ANDROID_HOME:-$HOME/Android/Sdk}"
if [[ ! -d "${ANDROID_HOME}" ]]; then
  echo "Error: ANDROID_HOME not found at ${ANDROID_HOME}" >&2
  exit 1
fi

if [[ -z "${NDK_HOME:-}" ]]; then
  NDK_HOME="$(ls -d "${ANDROID_HOME}/ndk/"* 2>/dev/null | sort -V | tail -n 1 || true)"
  export NDK_HOME
fi
if [[ -z "${NDK_HOME}" || ! -d "${NDK_HOME}" ]]; then
  echo "Error: Android NDK not found under ${ANDROID_HOME}/ndk" >&2
  exit 1
fi

LLVM="${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64"
CLANG_INC="$(find "${LLVM}/lib/clang" -maxdepth 2 -name include -type d | sort -V | tail -n 1)"

if [[ -d "${LLVM}/lib" && -n "$(ls "${LLVM}/lib"/libclang.so* 2>/dev/null || true)" ]]; then
  export LIBCLANG_PATH="${LLVM}/lib"
elif [[ -d "${LLVM}/lib64" ]]; then
  export LIBCLANG_PATH="${LLVM}/lib64"
fi

export BINDGEN_EXTRA_CLANG_ARGS_aarch64_linux_android="--sysroot=${LLVM}/sysroot -isystem ${CLANG_INC}"

if [[ -z "${JAVA_HOME:-}" ]]; then
  if [[ -x "/usr/lib/jvm/java-25-openjdk/bin/javac" ]]; then
    export JAVA_HOME="/usr/lib/jvm/java-25-openjdk"
  elif [[ -x "/usr/lib/jvm/java/bin/javac" ]]; then
    export JAVA_HOME="/usr/lib/jvm/java"
  fi
fi

# Ensure Android project is initialized and patched with GyroTube templates
if [[ ! -d "${TAURI_DIR}/src-tauri/gen/android" ]]; then
  (cd "${TAURI_DIR}" && cargo tauri android init --ci)
fi
bash "${TAURI_DIR}/android-templates/apply-android-config.sh"

# Build APK
(cd "${TAURI_DIR}" && cargo tauri android build --apk --target aarch64 "${EXTRA_ARGS[@]}")

# Copy (and sign if release) APK to <repo>/output/android/
mkdir -p "${OUT_DIR}"
if [[ "${MODE}" == "debug" ]]; then
  SRC_APK="${TAURI_DIR}/src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
  DEST_APK="${OUT_DIR}/gyrotube-android-debug.apk"
  if [[ ! -f "${SRC_APK}" ]]; then
    echo "Error: expected APK not found at ${SRC_APK}" >&2
    exit 1
  fi
  cp -f "${SRC_APK}" "${DEST_APK}"
else
  SRC_APK="${TAURI_DIR}/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk"
  DEST_APK="${OUT_DIR}/gyrotube-android-release.apk"
  if [[ ! -f "${SRC_APK}" ]]; then
    echo "Error: expected APK not found at ${SRC_APK}" >&2
    exit 1
  fi

  BUILD_TOOLS="$(ls -d "${ANDROID_HOME}/build-tools/"* 2>/dev/null | sort -V | tail -n 1 || true)"
  ZIPALIGN="${BUILD_TOOLS}/zipalign"
  APKSIGNER="${BUILD_TOOLS}/apksigner"

  if [[ -x "${ZIPALIGN}" && -x "${APKSIGNER}" ]]; then
    KEYSTORE="${ANDROID_KEYSTORE:-${HOME}/.android/debug.keystore}"
    KEYSTORE_PASS="${ANDROID_KEYSTORE_PASS:-android}"
    KEY_ALIAS="${ANDROID_KEY_ALIAS:-androiddebugkey}"
    KEY_PASS="${ANDROID_KEY_PASS:-android}"

    if [[ ! -f "${KEYSTORE}" ]]; then
      mkdir -p "$(dirname "${KEYSTORE}")"
      keytool -genkeypair -v \
        -keystore "${KEYSTORE}" \
        -storepass "${KEYSTORE_PASS}" \
        -alias "${KEY_ALIAS}" \
        -keypass "${KEY_PASS}" \
        -keyalg RSA \
        -keysize 2048 \
        -validity 10000 \
        -dname "CN=GyroTube Local,O=GyroTube,C=US"
    fi

    ALIGNED_APK="${OUT_DIR}/.gyrotube-android-release-aligned.apk"
    rm -f "${ALIGNED_APK}" "${DEST_APK}" "${DEST_APK}.idsig"
    "${ZIPALIGN}" -f -p 4 "${SRC_APK}" "${ALIGNED_APK}"
    "${APKSIGNER}" -J-enable-native-access=ALL-UNNAMED sign \
      --ks "${KEYSTORE}" \
      --ks-pass "pass:${KEYSTORE_PASS}" \
      --ks-key-alias "${KEY_ALIAS}" \
      --key-pass "pass:${KEY_PASS}" \
      --out "${DEST_APK}" \
      "${ALIGNED_APK}"
    rm -f "${ALIGNED_APK}" "${DEST_APK}.idsig"
  else
    echo "Warning: zipalign/apksigner not found; copying unsigned release APK." >&2
    DEST_APK="${OUT_DIR}/gyrotube-android-release-unsigned.apk"
    cp -f "${SRC_APK}" "${DEST_APK}"
  fi
fi

echo ""
echo "APK listo en: ${DEST_APK}"
ls -lh "${DEST_APK}"
