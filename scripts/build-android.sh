#!/usr/bin/env bash
set -euo pipefail

# Builds GyroTube Android APK (aarch64) and copies it into <repo>/output/android/.
# Usage:
#   ./scripts/build-android.sh           # Release APK -> output/android/gyrotube-android-release-unsigned.apk
#   ./scripts/build-android.sh --debug   # Signed Debug APK -> output/android/gyrotube-android-debug.apk

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

# Copy APK to <repo>/build/
mkdir -p "${OUT_DIR}"
if [[ "${MODE}" == "debug" ]]; then
  SRC_APK="${TAURI_DIR}/src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
  DEST_APK="${OUT_DIR}/gyrotube-android-debug.apk"
else
  SRC_APK="${TAURI_DIR}/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk"
  DEST_APK="${OUT_DIR}/gyrotube-android-release-unsigned.apk"
fi

if [[ ! -f "${SRC_APK}" ]]; then
  echo "Error: expected APK not found at ${SRC_APK}" >&2
  exit 1
fi

cp -f "${SRC_APK}" "${DEST_APK}"
echo ""
echo "APK listo en: ${DEST_APK}"
ls -lh "${DEST_APK}"
