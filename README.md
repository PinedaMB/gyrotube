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

## Pruebas

```bash
cargo test
```

## Catálogo y carátulas

`ytmapi-rs` es el cliente primario de búsqueda, álbumes y artistas. RustyPipe sirve Radio y Charts de Inicio y actúa como respaldo; solo se aceptan pistas `TrackType::Track` disponibles. Las playlists del cliente primario admiten únicamente `PlaylistItem::Song`. En los álbumes, las pistas se obtienen de su playlist de audio (`audio_playlist_id`), porque las filas de la página del álbum pueden apuntar a videoclips. Las consultas de playlists usan el prefijo de navegación `VL`. No se sustituyen IDs por el primer resultado de una búsqueda. Las colas antiguas se verifican antes de reproducirse.

Este filtro identifica audio oficial de YouTube Music, no certifica que la grabación sea de estudio: un álbum en vivo también contiene canciones oficiales.

Las pistas sin imagen individual heredan la carátula del álbum. El reproductor muestra la imagen disponible sin esperar una búsqueda y prueba la miniatura si falla la descarga o la decodificación de la imagen grande. Los errores se registran en stderr.

## Integridad de datos y pistas compartidas

La caché solo se utiliza directamente cuando la pista tiene una verificación de audio del catálogo. Las marcas antiguas se vuelven a verificar con conexión antes de reproducirse. JAM y SPIN envían el ID de la canción; el receptor consulta su tipo, disponibilidad y metadatos antes de agregarla. La API LAN responde `202 Accepted` mientras se verifica la solicitud.

Los favoritos, playlists y sesiones se guardan mediante escritura temporal y reemplazo atómico. Si un archivo no pudo cargarse, se conserva una copia con nombre `*.damaged-*` antes de reemplazarlo. Los errores de carga y guardado se muestran en la aplicación. La sesión también conserva el orden anterior al modo aleatorio; al restaurarlo no recupera pistas eliminadas ni descarta las añadidas.
