<p align="center">
  <img src="gyrotube.svg" width="96" height="96" alt="GyroTube logo" />
</p>

<h1 align="center">GyroTube</h1>

<p align="center">
  <strong>v0.1.0</strong> — Reproductor de música de escritorio para YouTube Music construido con Rust, GTK4 y libadwaita.
</p>

## Características

- **Streaming y precarga**: `yt-dlp` extrae la URL de audio para GStreamer sin descargar la pista completa. La siguiente canción precarga su URL y se descarga en segundo plano a la caché local; las descargas de respaldo también se conservan. Inicio reutiliza su caché local durante 15 minutos; el inicio en frío depende de YouTube y de la red.
- **Navegación completa**: Inicio, Explorar (búsqueda, géneros, artistas, álbumes y playlists) y Colección (favoritos, historial y playlists locales).
- **Reproductor avanzado**: cola de reproducción, modo aleatorio, repetición, *crossfade*, temporizador de apagado y barra lateral adaptable.
- **Letras sincronizadas**: soporte de letras con desplazamiento automático.
- **Integración de escritorio**: controles multimedia MPRIS, atajos de teclado y archivo `.desktop` con ícono propio.

## Requisitos

- Rust / Cargo (edición 2024)
- Bibliotecas de desarrollo de `gtk4` (`>= 4.14`) y `libadwaita` (`>= 1.6`)
- Plugins de GStreamer para reproducción de audio (`M4A` / `WebM` / `Opus`)
- `yt-dlp` en `PATH` (necesario para extraer audio)
- Node.js y componentes EJS de yt-dlp para resolver los desafíos de YouTube

## Compilación y ejecución

```bash
# Ejecutar en modo desarrollo
cargo run

# Compilar versión release (target/release/gyrotube)
cargo build --release
./target/release/gyrotube
```

Para medir el arranque en la aplicación, ejecuta `GYROTUBE_AUDIO_TIMINGS=1 cargo run`. Las líneas `[audio]` en la terminal registran por ID la verificación del catálogo, la extracción, la descarga de respaldo cuando se usa, la preparación en GStreamer y el tiempo desde la selección hasta que GStreamer entra en reproducción. Las etapas ya resueltas por caché o precarga pueden no aparecer después del clic.

## Paquetes Linux (x86_64)

- **Debian 13** · Habilita [Trixie Backports](https://backports.debian.org/Instructions/) para obtener un `yt-dlp` compatible, luego ejecuta `sudo apt install ./dist/gyrotube_0.1.0-1_amd64.deb`. Instala [Node.js 22 o posterior](https://nodejs.org/en/download) y asegúrate de que `node` esté en el `PATH` de la sesión gráfica.
- **Fedora 45** · `sudo dnf install ./dist/gyrotube-0.1.0-1.fc45.x86_64.rpm`
- **AppImage** · Da permiso de ejecución con `chmod +x dist/GyroTube-0.1.0-x86_64.AppImage` y ejecútalo. Requiere glibc 2.41 o posterior y usa el `yt-dlp` actualizado con soporte EJS y Node.js 22 o posterior del sistema. En sistemas sin FUSE se puede ejecutar con `APPIMAGE_EXTRACT_AND_RUN=1`.

Para regenerarlos, compila con `cargo build --release --locked` en Fedora 45 y en Debian 13. `packaging/Containerfile.debian` prepara las dependencias nativas de Debian; agrega el toolchain de Rust para compilar. Dentro de Debian, crea el AppDir con `packaging/make-appdir.sh target/release/gyrotube GyroTube.AppDir`. Con nFPM y appimagetool disponibles, ejecuta `packaging/package.sh BINARIO_DEBIAN BINARIO_FEDORA RUTA_APPDIR`. Los paquetes y `SHA256SUMS` quedan en `dist/`.

## Pruebas

```bash
cargo test
```

## Catálogo y carátulas

`ytmapi-rs` es el cliente primario de búsqueda, álbumes y artistas. RustyPipe sirve Radio y Charts de Inicio y actúa como respaldo; solo se aceptan pistas `TrackType::Track` disponibles. Las playlists del cliente primario admiten únicamente `PlaylistItem::Song`. En los álbumes, las pistas se obtienen de su playlist de audio (`audio_playlist_id`), porque las filas de la página del álbum pueden apuntar a videoclips. Las consultas de playlists usan el prefijo de navegación `VL`. No se sustituyen IDs por el primer resultado de una búsqueda. Las colas antiguas se verifican antes de reproducirse.

Este filtro identifica audio oficial de YouTube Music, no certifica que la grabación sea de estudio: un álbum en vivo también contiene canciones oficiales.

Las pistas sin imagen individual heredan la carátula del álbum. El reproductor muestra la imagen disponible sin esperar una búsqueda y prueba la miniatura si falla la descarga o la decodificación de la imagen grande. Los errores se registran en stderr.

La caché de audio guardado tiene un límite de 1 GiB. GyroTube libera espacio al arrancar y al guardar una descarga, empezando por los archivos menos usados y conservando los que tiene abiertos el reproductor. Si no puede guardar una descarga de respaldo, la reproduce desde un archivo temporal.

## Integridad de datos y pistas compartidas

La caché solo se utiliza directamente cuando la pista tiene una verificación de audio del catálogo. Las marcas antiguas se vuelven a verificar con conexión antes de reproducirse. JAM y SPIN envían el ID de la canción; el receptor consulta su tipo, disponibilidad y metadatos antes de agregarla. La API LAN responde `202 Accepted` mientras se verifica la solicitud.

Los favoritos, playlists y sesiones se guardan mediante escritura temporal y reemplazo atómico. Si un archivo no pudo cargarse, se conserva una copia con nombre `*.damaged-*` antes de reemplazarlo. Los errores de carga y guardado se muestran en la aplicación. La sesión también conserva el orden anterior al modo aleatorio; al restaurarlo no recupera pistas eliminadas ni descarta las añadidas.

## Salas SPIN

El anfitrión crea una sala y comparte el ticket `GYRO-…` con **Copiar** o mediante el QR. Los invitados pegan el ticket en GyroTube. Genera una invitación nueva al reiniciar la sala. Los códigos SPIN anteriores dejaron de funcionar.

GyroTube usa iroh para conectar cada invitado con el anfitrión. Intenta establecer una conexión directa y recurre a los relays públicos de iroh cuando hace falta. Los datos de sincronización viajan cifrados entre equipos, y cada invitado obtiene su propio audio de YouTube Music. Los relays públicos tienen límites y su disponibilidad puede variar.

Comparte el ticket solo con quienes quieras invitar. Los invitados pueden enviar solicitudes y usar los controles si el anfitrión los habilita. JAM sigue permitiendo unirse mediante la URL local.
