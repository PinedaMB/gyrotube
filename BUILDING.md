# Guía de compilación de GyroTube

Este repositorio es un **Cargo Workspace** con tres crates principales:

- `crates/gyrotube-core`: núcleo compartido en Rust puro (API de YouTube Music, `AudioStreamer`, caché, letras, SPIN).
- `apps/tauri`: aplicación multiplataforma (**Linux, Windows, macOS y Android**) con **Tauri v2 + Svelte 5 + Tailwind CSS v4**.
- `apps/gtk`: aplicación nativa de Linux con **GTK4 + libadwaita + GStreamer**.

---

## 1. Requisitos previos

### Herramientas base
- **Rust** (stable, vía `rustup`)
- **Node.js 20+** y **pnpm** (`npm i -g pnpm`)
- **Tauri CLI v2**:
  ```bash
  cargo install tauri-cli --version "^2" --locked
  ```

### Dependencias de sistema en Linux

**Fedora / Nobara:**
```bash
sudo dnf install \
  webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel \
  librsvg2-devel openssl-devel gtk4-devel libadwaita-devel \
  gstreamer1-devel gstreamer1-plugins-base-devel
```

**Debian / Ubuntu:**
```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev patchelf pkg-config \
  libgtk-4-dev libadwaita-1-dev \
  libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
```

### Instalar dependencias del frontend
```bash
pnpm --dir apps/tauri/ui install
```

---

## 2. Escritorio (Tauri v2: Linux, Windows, macOS)

### Modo desarrollo (hot-reload)
```bash
cd apps/tauri
cargo tauri dev
```

### Build de producción (instaladores nativos)
Usa el script incluido (incluye `NO_STRIP=true` para compatibilidad con Fedora y copia los instaladores automáticamente a `output/desktop/`):
```bash
# Todos los formatos disponibles en tu SO (.AppImage, .rpm, .deb / .msi / .dmg)
./scripts/build-desktop.sh

# O solo un formato específico (por ejemplo RPM en Fedora):
./scripts/build-desktop.sh --bundles rpm
```
Los instaladores quedan en `output/desktop/`.

---

## 3. Android (APK `aarch64`)

### Requisitos adicionales para Android
1. **Android SDK + NDK** (vía Android Studio → *Settings → Android SDK*, con `Android SDK Build-Tools 36`, `Android API 37` y NDK instalados en `~/Android/Sdk`).
2. **JDK completo con `javac`** (por ejemplo en Fedora: `sudo dnf install java-25-openjdk-devel`; en Ubuntu: `sudo apt install openjdk-21-jdk`).
3. **Target Rust de Android**:
   ```bash
   rustup target add aarch64-linux-android
   ```

### Compilar APK en un solo comando
El script `scripts/build-android.sh` detecta automáticamente `ANDROID_HOME`, `NDK_HOME`, `LIBCLANG_PATH`, los headers del NDK para `bindgen` y `JAVA_HOME`, aplica las plantillas nativas (`MainActivity.kt`, `network_security_config.xml`, `PlaybackForegroundService.kt`) y copia el resultado a `output/android/`.

- **APK Debug (ya firmado, listo para instalar y depurar):**
  ```bash
  ./scripts/build-android.sh --debug
  # Salida: output/android/gyrotube-android-debug.apk
  ```
- **APK Release (optimizado ~47 MB, sin firmar):**
  ```bash
  ./scripts/build-android.sh
  # Salida: output/android/gyrotube-android-release-unsigned.apk
  ```

### Firmar el APK de Release (para instalar la versión optimizada)
```bash
# Crear keystore una sola vez
keytool -genkey -v -keystore ~/gyrotube.keystore -alias gyrotube \
  -keyalg RSA -keysize 2048 -validity 10000

# Firmar el APK generado
~/Android/Sdk/build-tools/36.0.0/apksigner sign \
  --ks ~/gyrotube.keystore \
  --out build/gyrotube-android-release-signed.apk \
  build/gyrotube-android-release-unsigned.apk
```

### Instalar y depurar en tu celular por USB
```bash
ADB=~/Android/Sdk/platform-tools/adb

# Verificar dispositivo conectado
$ADB devices

# Instalar o actualizar APK
$ADB install -r build/gyrotube-android-debug.apk

# Ver logs de Rust, audio y WebView filtrados por el proceso de GyroTube
$ADB logcat -d --pid="$($ADB shell pidof -s com.github.gyrotube.GyroTube)" | tail -n 60
```
Para inspeccionar el DOM, CSS, Console y Network del WebView en vivo, abre **`chrome://inspect/#devices`** en Chrome/Chromium y pulsa **Inspect** sobre GyroTube.

---

## 4. Aplicación nativa Linux GTK4

```bash
# Ejecutar en desarrollo
cargo run -p gyrotube-gtk

# Compilar binario optimizado
cargo build --release -p gyrotube-gtk
# Salida: target/release/gyrotube-gtk
```

---

## 5. Pruebas y verificación rápida

```bash
# Pruebas unitarias de core y GTK4 + chequeo de todo el workspace
cargo test -p gyrotube-core -p gyrotube-gtk
cargo check --workspace

# Typecheck y compilación de producción del frontend Svelte 5
pnpm --dir apps/tauri/ui check
pnpm --dir apps/tauri/ui build
```

---

## 6. Releases automáticos (GitHub Actions)

El repositorio incluye dos workflows en `.github/workflows/`:
- `ci.yml`: ejecuta pruebas de Rust y `svelte-check` / `vite build` en cada push o PR.
- `release-tauri.yml`: al publicar un tag `v*` (por ejemplo `git tag v0.2.0 && git push origin v0.2.0`) o ejecutarlo manualmente desde *Actions*, compila los paquetes de **Linux, Windows, macOS y el APK de Android**.
