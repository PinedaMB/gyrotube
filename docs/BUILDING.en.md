> 🌐 **Language / Idioma:** [Español](BUILDING.es.md) | **English**

# GyroTube Build Guide

This repository is a **Cargo Workspace** with two main crates:

- `crates/gyrotube-core`: shared pure-Rust core (native YouTube Music InnerTube client, hardened loopback HTTP `AudioStreamer` with LRU disk cache, synced/static lyrics resolution, atomic persistence, and **SPIN Mode** LAN/P2P).
- `apps/tauri`: cross-platform application (**Linux, Windows, macOS, and Android**) built with **Tauri v2 + Svelte 5 + Tailwind CSS v4**, plus native Android templates in `apps/tauri/android-templates/`.

---

## 1. Prerequisites

### Base Tooling
- **Rust** (stable, via `rustup`)
- **Node.js 20+** and **pnpm** (`npm i -g pnpm`)
- **Tauri CLI v2**:
  ```bash
  cargo install tauri-cli --version "^2" --locked
  ```

### Linux System Dependencies

**Fedora / Nobara:**
```bash
sudo dnf install \
  webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel \
  librsvg2-devel openssl-devel patchelf pkgconf-pkg-config
```

**Debian / Ubuntu:**
```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev patchelf pkg-config
```

### Optional Desktop Tools
- **`yt-dlp` + `node`**: optional desktop fallback if all native InnerTube client profiles fail (the core runs without them on both desktop and Android).
- **`rpm-sign`**: optional on Fedora for signing `.rpm` packages locally (useful for RPM 6 / GNOME Software).

### Install Frontend Dependencies
```bash
pnpm --dir apps/tauri/ui install
```

---

## 2. Desktop (Tauri v2: Linux, Windows, macOS)

### Development Mode (Hot-Reload)
```bash
cd apps/tauri
cargo tauri dev
```

### Production Build (Native Installers)
Use the included script (sets `NO_STRIP=true` for Fedora compatibility, cleans previous `target/release/bundle` staging artifacts, prunes older version installers in `output/desktop/`, and copies the newly built packages):
```bash
# All available formats for your OS (.AppImage, .rpm, .deb / .msi / .dmg)
./scripts/build-desktop.sh

# Or a single bundle format (e.g., RPM on Fedora):
./scripts/build-desktop.sh --bundles rpm
```
Installers are written to `output/desktop/`.

#### Installing and Signing `.rpm` on Fedora 45+ (RPM 6)
RPM 6 enforces GPG signatures by default when installing local packages via GNOME Software or `dnf`:
- To install an unsigned local `.rpm`:
  ```bash
  sudo dnf install --nogpgcheck output/desktop/GyroTube-*.rpm
  ```
- To sign it automatically during the build (requires `rpm-sign` and a GPG key configured in `~/.rpmmacros` or via `RPM_GPG_KEY`):
  ```bash
  RPM_GPG_KEY="Your GPG Key" ./scripts/build-desktop.sh --bundles rpm
  ```

---

## 3. Android (`aarch64` APK)

### Additional Android Prerequisites
1. **Android SDK + NDK** (via Android Studio → *Settings → Android SDK*, with `Android SDK Build-Tools`, `Android API`, and NDK installed under `~/Android/Sdk`).
2. **Full JDK with `javac`** (e.g., on Fedora: `sudo dnf install java-25-openjdk-devel`; on Ubuntu: `sudo apt install openjdk-21-jdk`).
3. **Android Rust target**:
   ```bash
   rustup target add aarch64-linux-android
   ```

### Build an APK in One Command
`scripts/build-android.sh` automatically detects `ANDROID_HOME`, `NDK_HOME`, and `JAVA_HOME`, initializes `gen/android` if needed, applies the native templates (`MainActivity.kt`, `network_security_config.xml`, `PlaybackForegroundService.kt`), aligns and signs the release APK with `zipalign` + `apksigner`, and copies the output to `output/android/`.

- **Release APK (optimized and signed, ready to install):**
  ```bash
  ./scripts/build-android.sh
  # Output: output/android/gyrotube-android-release.apk
  ```
- **Debug APK (for WebView / Logcat debugging):**
  ```bash
  ./scripts/build-android.sh --debug
  # Output: output/android/gyrotube-android-debug.apk
  ```

### Sign the Release APK with Your Own Keystore
By default, `build-android.sh` signs with `~/.android/debug.keystore` (creating it if absent). To use your own distribution keystore:
```bash
# Create a keystore once
keytool -genkeypair -v -keystore ~/gyrotube.keystore -alias gyrotube \
  -keyalg RSA -keysize 2048 -validity 10000

# Build and sign automatically
ANDROID_KEYSTORE=~/gyrotube.keystore \
ANDROID_KEYSTORE_PASS="your-password" \
ANDROID_KEY_ALIAS="gyrotube" \
ANDROID_KEY_PASS="your-password" \
./scripts/build-android.sh
```

### Install and Debug on a Phone via USB
```bash
ADB=~/Android/Sdk/platform-tools/adb

# Verify connected device
$ADB devices

# Install or update APK
$ADB install -r output/android/gyrotube-android-release.apk

# View Rust, audio, and WebView logs filtered by GyroTube's PID
$ADB logcat -d --pid="$($ADB shell pidof -s com.github.gyrotube.GyroTube)" | tail -n 60
```
To inspect the WebView DOM, CSS, Console, and Network live, open **`chrome://inspect/#devices`** in Chrome/Chromium and click **Inspect** on GyroTube.

---

## 4. Tests & Quick Verification

```bash
# Strict workspace-wide Rust linting (zero warnings)
cargo clippy --workspace --all-targets -- -D warnings

# Rust core unit tests and workspace typecheck
cargo test -p gyrotube-core --lib
cargo check --workspace

# Svelte 5 frontend typecheck and production build
pnpm --dir apps/tauri/ui check
pnpm --dir apps/tauri/ui build
```

---

## 5. Automated Releases (GitHub Actions)

The repository includes workflows under `.github/workflows/`:
- `ci.yml`: runs Rust tests and `svelte-check` / `vite build` on every push or PR.
- `release-tauri.yml`: triggered when pushing a `v*` tag (e.g., `git tag v1.0.0 && git push origin v1.0.0`) or manually from *Actions*, building packages for **Linux, Windows, macOS, and Android APK**.
