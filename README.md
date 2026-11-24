<p align="center">
  <img src="gyrotube.svg" width="96" height="96" alt="GyroTube logo" />
</p>

<h1 align="center">GyroTube</h1>

<p align="center">
  <strong>v1.0.0</strong> — Cliente multiplataforma de YouTube Music para escritorio (Linux, Windows, macOS) y Android, construido con Rust, Tauri v2 y Svelte 5.
</p>

## Características

- **Streaming rápido y sin cortes**: motor de audio de doble deck (`127.0.0.1` Range streamer en Rust), precarga automática de la siguiente canción, caché de audio de 1 GiB y *crossfade* configurable.
- **Descargas offline estilo Spotify**: descarga playlists locales y Favoritos con audio, carátulas y letras sincronizadas; auto-sincroniza nuevas canciones, limpia archivos huérfanos por conteo de referencias y bloquea opciones online automáticamente cuando no hay internet.
- **Inicio personalizado y Explorar global**:
  - **Inicio**: accesos rápidos a tu biblioteca, *Escuchar de nuevo*, mixes personales de 1 clic, recomendaciones *"Porque escuchaste…"* y artistas seguidos.
  - **Explorar**: selector de país, álbum destacado, nuevos lanzamientos, listas de éxitos, artistas en tendencia, estaciones y carrusel compacto de *Ánimo y géneros*.
  - **Búsqueda dedicada** (`#/search`): historial, sugerencias en vivo y filtros por canciones, álbumes, artistas y playlists.
- **Reproductor completo**: letras sincronizadas (LRCLIB + YouTube Music), modo **Video** sincronizado, **Modo Fiesta**, temporizador de apagado, cola reordenable, tema claro/oscuro/sistema e interfaz en Español e Inglés.
- **Experiencia nativa en Android**: servicio en primer plano con controles en pantalla de bloqueo (`MediaSession`), volumen gobernado por el sistema, navegación hacia atrás sin cerrar la música y enlaces profundos `gyrotube://spin/...`.
- **Respaldo local**: exportación e importación en JSON de favoritos, playlists locales y artistas seguidos desde Ajustes.
- **Salas SPIN *(Experimental)***: escucha colaborativa por Wi-Fi LAN (Jukebox) o App a App vía *iroh* con invitación compacta `GYRO-…` y código QR.

## Atajos de teclado (escritorio)

- `Ctrl+K` / `Cmd+K`: enfocar búsqueda
- `Espacio`: reproducir / pausar
- `←` / `→`: retroceder / avanzar 5 segundos
- `Shift+←` / `Shift+→`: canción anterior / siguiente
- `M`: silenciar / activar sonido

## Compilación

### Escritorio (Tauri v2)

```bash
bash scripts/build-desktop.sh
```

Genera instaladores en `output/desktop/` (`.deb`, `.rpm`, `.AppImage` en Linux; `.msi`/`.exe` en Windows; `.dmg` en macOS). En escritorio requiere `yt-dlp` y `node` disponibles en `PATH` para la extracción de flujos de YouTube.

### Android (APK aarch64)

```bash
# Release firmado -> output/android/gyrotube-android-release.apk
bash scripts/build-android.sh

# Debug -> output/android/gyrotube-android-debug.apk
bash scripts/build-android.sh --debug
```

Para firmar releases de producción con tu propio keystore, exporta `ANDROID_KEYSTORE`, `ANDROID_KEYSTORE_PASS`, `ANDROID_KEY_ALIAS` y `ANDROID_KEY_PASS`.

### Pruebas

```bash
cargo test
cd apps/tauri/ui && npm run check && npm run build
```

## Arquitectura del workspace

- `crates/gyrotube-core`: catálogo (`ytmapi-rs` + InnerTube nativo en Rust), streamer HTTP local con soporte `Range` y `/cover/{id}`, caché de audio, descargas offline, letras, persistencia atómica y SPIN/JAM.
- `apps/tauri`: aplicación principal multiplataforma (Tauri v2 + Svelte 5 + Tailwind CSS v4 + templates nativos de Android en `apps/tauri/android-templates`).
