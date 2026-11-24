> 🌐 **Idioma / Language:** **Español** | [English](BUILDING.en.md)

# Guía de compilación de GyroTube

Este repositorio es un **Cargo Workspace** con dos crates principales:

- `crates/gyrotube-core`: núcleo compartido en Rust puro (cliente nativo InnerTube de YouTube Music, `AudioStreamer` HTTP loopback con caché LRU en disco, resolución de letras sincronizadas/estáticas, persistencia atómica y **Modo SPIN** LAN/P2P).
- `apps/tauri`: aplicación multiplataforma (**Linux, Windows, macOS y Android**) con **Tauri v2 + Svelte 5 + Tailwind CSS v4** y plantillas nativas de Android en `apps/tauri/android-templates/`.

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
  librsvg2-devel openssl-devel patchelf pkgconf-pkg-config
```

**Debian / Ubuntu:**
```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev patchelf pkg-config
```

### Herramientas opcionales en escritorio
- **`yt-dlp` + `node`**: respaldo opcional en escritorio si todos los perfiles nativos de InnerTube fallaran (el núcleo funciona sin ellos en escritorio y Android).
- **`rpm-sign`**: opcional en Fedora para firmar paquetes `.rpm` localmente (útil con RPM 6 / GNOME Software).

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
Usa el script incluido (define `NO_STRIP=true` para compatibilidad con Fedora, limpia artefactos previos de `target/release/bundle`, elimina instaladores de versiones anteriores en `output/desktop/` y copia los paquetes actuales):
```bash
# Todos los formatos disponibles en tu SO (.AppImage, .rpm, .deb / .msi / .dmg)
./scripts/build-desktop.sh

# O solo un formato específico (por ejemplo RPM en Fedora):
./scripts/build-desktop.sh --bundles rpm
```
Los instaladores quedan en `output/desktop/`.

#### Instalación y firma de `.rpm` en Fedora 45+ (RPM 6)
RPM 6 exige firma GPG por defecto al instalar paquetes locales desde GNOME Software o `dnf`:
- Para instalar un `.rpm` local sin firmar:
  ```bash
  sudo dnf install --nogpgcheck output/desktop/GyroTube-*.rpm
  ```
- Para firmarlo automáticamente durante el build (requiere `rpm-sign` y una clave GPG configurada en `~/.rpmmacros` o mediante `RPM_GPG_KEY`):
  ```bash
  RPM_GPG_KEY="Tu Clave GPG" ./scripts/build-desktop.sh --bundles rpm
  ```

---

## 3. Android (APK `aarch64`)

### Requisitos adicionales para Android
1. **Android SDK + NDK** (vía Android Studio → *Settings → Android SDK*, con `Android SDK Build-Tools`, `Android API` y NDK instalados en `~/Android/Sdk`).
2. **JDK completo con `javac`** (por ejemplo en Fedora: `sudo dnf install java-25-openjdk-devel`; en Ubuntu: `sudo apt install openjdk-21-jdk`).
3. **Target Rust de Android**:
   ```bash
   rustup target add aarch64-linux-android
   ```

### Compilar APK en un solo comando
El script `scripts/build-android.sh` detecta automáticamente `ANDROID_HOME`, `NDK_HOME` y `JAVA_HOME`, inicializa `gen/android` si no existe, aplica las plantillas nativas (`MainActivity.kt`, `network_security_config.xml`, `PlaybackForegroundService.kt`), alinea/firma el APK de release con `zipalign` + `apksigner` y copia el resultado a `output/android/`.

- **APK Release (optimizado y firmado, listo para instalar):**
  ```bash
  ./scripts/build-android.sh
  # Salida: output/android/gyrotube-android-release.apk
  ```
- **APK Debug (para depuración en WebView / Logcat):**
  ```bash
  ./scripts/build-android.sh --debug
  # Salida: output/android/gyrotube-android-debug.apk
  ```

### Firmar el APK de Release con tu propio keystore
Por defecto `build-android.sh` firma con `~/.android/debug.keystore` (lo crea si no existe). Para usar un keystore propio de distribución:
```bash
# Crear keystore una sola vez
keytool -genkeypair -v -keystore ~/gyrotube.keystore -alias gyrotube \
  -keyalg RSA -keysize 2048 -validity 10000

# Compilar y firmar automáticamente
ANDROID_KEYSTORE=~/gyrotube.keystore \
ANDROID_KEYSTORE_PASS="tu-password" \
ANDROID_KEY_ALIAS="gyrotube" \
ANDROID_KEY_PASS="tu-password" \
./scripts/build-android.sh
```

### Instalar y depurar en tu celular por USB
```bash
ADB=~/Android/Sdk/platform-tools/adb

# Verificar dispositivo conectado
$ADB devices

# Instalar o actualizar APK
$ADB install -r output/android/gyrotube-android-release.apk

# Ver logs de Rust, audio y WebView filtrados por el proceso de GyroTube
$ADB logcat -d --pid="$($ADB shell pidof -s com.github.gyrotube.GyroTube)" | tail -n 60
```
Para inspeccionar el DOM, CSS, Console y Network del WebView en vivo, abre **`chrome://inspect/#devices`** en Chrome/Chromium y pulsa **Inspect** sobre GyroTube.

---

## 4. Pruebas y verificación rápida

```bash
# Lint estricto de todo el workspace Rust (cero advertencias)
cargo clippy --workspace --all-targets -- -D warnings

# Pruebas unitarias del núcleo Rust y chequeo del workspace
cargo test -p gyrotube-core --lib
cargo check --workspace

# Typecheck y compilación de producción del frontend Svelte 5
pnpm --dir apps/tauri/ui check
pnpm --dir apps/tauri/ui build
```

---

## 5. Releases automáticos (GitHub Actions)

El repositorio incluye workflows en `.github/workflows/`:
- `ci.yml`: ejecuta pruebas de Rust y `svelte-check` / `vite build` en cada push o PR.
- `release-tauri.yml`: al publicar un tag `v*` (por ejemplo `git tag v1.0.0 && git push origin v1.0.0`) o ejecutarlo manualmente desde *Actions*, compila los paquetes de **Linux, Windows, macOS y el APK de Android**.
