<p align="center">
  <img src="gyrotube.svg" width="96" height="96" alt="GyroTube logo" />
</p>

<h1 align="center">GyroTube</h1>

<p align="center">
  <strong>v0.1.0</strong> — Reproductor de música de escritorio para YouTube Music construido con Rust, GTK4 y libadwaita.
</p>

## Características

- **Streaming progresivo y caché local**: inicio de reproducción inmediato mediante rangos HTTP y guardado en caché para reproducción instantánea sin conexión.
- **Navegación completa**: Inicio, Explorar (búsqueda, géneros, artistas, álbumes y playlists) y Colección (favoritos, historial y playlists locales).
- **Reproductor avanzado**: cola de reproducción, modo aleatorio, repetición, *crossfade*, temporizador de apagado y barra lateral adaptable.
- **Letras sincronizadas**: soporte de letras con desplazamiento automático.
- **Integración de escritorio**: controles multimedia MPRIS, atajos de teclado y archivo `.desktop` con ícono propio.

## Requisitos

- Rust / Cargo (edición 2024)
- Bibliotecas de desarrollo de `gtk4` (`>= 4.14`) y `libadwaita` (`>= 1.6`)
- Plugins de GStreamer para reproducción de audio (`M4A` / `WebM` / `Opus`)
- `yt-dlp` en `PATH` *(opcional, usado como respaldo)*

## Compilación y ejecución

```bash
# Ejecutar en modo desarrollo
cargo run

# Compilar versión release (target/release/gyrotube)
cargo build --release
./target/release/gyrotube
```

## Pruebas

```bash
cargo test
```
