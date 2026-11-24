> 🌐 **Idioma / Language:** **Español** | [English](DESIGN.en.md)

# GyroTube — Documento de Arquitectura y Diseño

GyroTube es un cliente multiplataforma de **YouTube Music** para **Linux, Windows, macOS y Android**, construido con un núcleo en Rust puro (**Edition 2024**, `crates/gyrotube-core`), **Tauri v2**, **Svelte 5** (runes), **Tailwind CSS v4** y red P2P **Iroh**. Se enfoca en reproducción de audio progresiva por bloques de baja latencia, una estética adaptativa *Obsidian Vinyl + Jade Emerald* con iluminación ambiental dinámica según la carátula, letras tipo karaoke priorizando sincronización con tarjetas compartibles (*Lyrics Cards*) y sesiones colaborativas en **Modo SPIN**.

---

## 1. Arquitectura de alto nivel

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                   Interfaz Tauri v2 + Svelte 5 + Tailwind CSS v4            │
│  ┌──────┐ ┌──────────┐ ┌───────────┐ ┌─────────┐ ┌───────────────────────┐  │
│  │Inicio│ │ Explorar │ │ Colección │ │ Ajustes │ │ Modo Fiesta (Karaoke) │  │
│  └──┬───┘ └────┬─────┘ └─────┬─────┘ └────┬────┘ └───────────┬───────────┘  │
│     └──────────┴─────────────┴────────────┴──────────────────┤              │
│                                                              ▼              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │        Reproductor lateral / Mini-Player / Bottom Sheets / SPIN       │  │
│  │         (Reproductor • Letras + Lyrics Cards • Cola • Relacionadas)   │  │
│  └──────────────────┬─────────────────────────────────┬──────────────────┘  │
│                     ▼                                 ▼                     │
│  ┌──────────────────────────────────────┐  ┌─────────────────────────────┐  │
│  │      DualDeckAudioEngine (TS)        │  │    Stores Svelte 5          │  │
│  │  Audio HTML5 A/B • Crossfade • EQ    │  │ player • localPrefs • queue │  │
│  │  Web Audio 5 bandas y normalización  │  │ session • nativeEvents      │  │
│  └──────────────────┬───────────────────┘  └──────────────┬──────────────┘  │
└─────────────────────┼─────────────────────────────────────┼─────────────────┘
                      │ HTTP Loopback (127.0.0.1)           │ Comandos IPC Tauri
                      ▼                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Núcleo Rust (`crates/gyrotube-core`)                     │
│  ┌──────────────────────────────────┐  ┌─────────────────────────────────┐  │
│  │  Servidor HTTP Loopback seguro   │  │  InnerTube nativo y Catálogo    │  │
│  │  (`audio_streamer.rs`)           │  │  • `innertube/{player,browse,   │  │
│  │  • `/stream/{id}.{ext}` (206)    │  │     parsers}` (5 perfiles)      │  │
│  │  • `/cover/{id}` • `/proxy/{k}`  │  │  • `backend/{catalog,streaming, │  │
│  │  • `verify_audio_tail` (todo SO) │  │     lyrics,parsers}`            │  │
│  └──────────────────┬───────────────┘  │  • LRCLIB priorizando sincr.    │  │
│                     ▼                  │  • Respaldo `yt-dlp` escritorio │  │
│  ┌──────────────────────────────────┐  └─────────────────────────────────┘  │
│  │   Caché de audio LRU (1 GiB)     │  ┌─────────────────────────────────┐  │
│  │   (`audio_cache.rs`)             │  │      Modo SPIN Colaborativo     │  │
│  └──────────────────────────────────┘  │  • Jukebox LAN HTTP+QR (`jam`)  │  │
│  ┌──────────────────────────────────┐  │  • P2P Iroh QUIC (`spin_remote`)│  │
│  │  Persistencia JSON (`models.rs`) │  │    ALPN `gyrotube/spin/1`       │  │
│  │  Servicio Android (`android-tmpl`)  └─────────────────────────────────┘  │
│  └──────────────────────────────────┘                                       │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Estructura del Workspace y Módulos

### 2.1. Núcleo Rust (`crates/gyrotube-core/src/`)

| Módulo | Ruta | Responsabilidad |
| :--- | :--- | :--- |
| **Orquestador Backend** | `backend/mod.rs` | Estado compartido `Backend`, cachés TTL en memoria (`stream_cache`, `music_proofs`, `home_cache`) y coordinación de calidad/sesión. |
| **Operaciones de Catálogo** | `backend/catalog.rs` | Inicio, búsqueda, detalles de artista/álbum/playlist, radio/relacionadas, contrapartes oficiales Canción↔Video (`RDAMVM`) y verificación estricta de procedencia musical. |
| **Resolución de Streams** | `backend/streaming.rs` | Resolutor adaptativo (InnerTube en Rust puro con 5 perfiles + respaldo opcional `yt-dlp` en escritorio), verificación del último byte y refresco de URL a mitad de reproducción. |
| **Pipeline de Letras** | `backend/lyrics.rs` | Resolución priorizando letras sincronizadas: `ANDROID_MUSIC` con marcas de tiempo → LRCLIB `/api/get` exacto → LRCLIB `/api/search` sincronizado y validado (±8 s de duración + coincidencia de título/artista) → respaldos estáticos (`ytm_plain`, LRCLIB plano). |
| **Parsers del Backend** | `backend/parsers.rs` | Normalización, filtrado de insignias, extracción de duración/metadatos y pruebas unitarias (`backend/tests.rs`). |
| **Cliente InnerTube Nativo** | `innertube/{mod,player,browse,parsers,tests}.rs` | Cliente API InnerTube de YouTube Music en Rust puro: 5 perfiles de reproducción (`ANDROID_VR`, `VISIONOS`, `ANDROID_VR_LEGACY`, `TV_EMBEDDED`, `IOS`), caché `/next`, endpoints de exploración/búsqueda/radio y sanitización de badges localizados. |
| **Streamer Progresivo Loopback** | `audio_streamer.rs` | Servidor HTTP local endurecido (`127.0.0.1:<puerto>`) para `/stream/<id>.<ext>` (`206 Partial Content`), `/cover/<id>` y `/proxy/<key>`. Descarga por bloques `Range`, inicia reproducción temprana, valida el byte final en todas las plataformas y guarda pistas completas en caché. |
| **Caché de Audio en Disco** | `audio_cache.rs` | Caché LRU con conteo de referencias (`~/.cache/gyrotube/streams/`, tope `1 GiB`) que fija los archivos `LocalAudio` activos para que la limpieza nunca elimine la pista en reproducción. |
| **Modelos y Persistencia** | `models.rs` | Tipos de dominio (`Track`, `Album`, `Artist`, `Playlist`, `StreamInfo`, `LyricsData`, `AudioQuality`), parser LRC, sanitizador de títulos y persistencia JSON atómica (`~/.local/share/gyrotube/`). |
| **Modo SPIN — Jukebox LAN** | `jam.rs` | Servidor HTTP LAN integrado (`0.0.0.0:<puerto>`) con interfaz web móvil (`assets/spin.html`) y generador de QR SVG mediante el crate `qrcode` (zona de silencio de 4 módulos). |
| **Modo SPIN — Remoto P2P** | `spin_remote.rs` | Escucha sincronizada App a App sobre **Iroh** QUIC (`gyrotube/spin/1`) con tickets `GYRO-...`, controles remotos bidireccionales (`allow_controls`), cola sincronizada (hasta 20 pistas, `MAX_FRAME = 65536`) y conteo RAII de invitados conectados (`remote_guest_count`). |
| **TLS e Internacionalización** | `tls.rs`, `i18n.rs` | Inicialización TLS con Rustls/WebPKI para escritorio y Android; capa ligera de traducción ES/EN (`tr(es, en)`) con detección de idioma. |

### 2.2. Capa Tauri y Android (`apps/tauri/`)

| Capa | Ruta | Responsabilidad |
| :--- | :--- | :--- |
| **Puente Rust de Tauri** | `src-tauri/src/{lib.rs, commands/}` | Registra plugins de Tauri v2, integración de escritorio/AppStream y comandos IPC (`catalog.rs`, `player.rs`, `library.rs`, `spin.rs`). |
| **Plantillas Nativas Android** | `android-templates/` | Aplica a `gen/android` `PlaybackForegroundService.kt` (notificación multimedia, controles en pantalla de bloqueo y wake lock), `MainActivity.kt` (enlaces profundos `gyrotube://spin/...` y teclas multimedia) y `network_security_config.xml` (loopback local). |
| **Estado y Motores Frontend** | `ui/src/lib/` | `audioEngine.ts` (`DualDeckAudioEngine`), `stores/player.svelte.ts`, `stores/localPrefs.ts`, `stores/queueOps.ts`, `stores/sessionPersistence.ts`, `stores/nativeEvents.ts`, `lyricsFormat.ts`, `lyricsCard.ts` (generador PNG `<canvas>` de alta densidad), `listeningStats.ts`, `ytmUrl.ts`, `i18n.svelte.ts`. |
| **Vistas y Componentes UI** | `ui/src/{views/, components/}` | `HomeView`, `ExploreView`, `CollectionView`, `ArtistView`, `AlbumView`, `PlaylistView`, `SearchView`, `SettingsView` (con `SettingsEqSection` y `SettingsAboutSection`), `SidebarPlayer` + `SidebarLyricsPane`, `PartyMode`, `SpinModal`, `SpinRemotePane`, `LyricsCardModal`, `MiniPlayer`, `ModalSheet`, `ContextMenu`. |

---

## 3. Pipeline de Audio Progresivo por Bloques

El CDN de YouTube (`googlevideo.com`) limita conexiones HTTP `GET` abiertas y puede entregar flujos truncados (~768 KB / 42–48 s) o `HTTP 403` en perfiles restringidos. GyroTube resuelve y reproduce audio de forma fluida sin esperar la descarga completa:

1. **Precarga especulativa**:
   - Al pasar el cursor por una pista o iniciar la reproducción, se resuelve en segundo plano la pista siguiente para transiciones instantáneas.
2. **Extracción adaptativa en Rust puro (5 perfiles)**:
   - `innertube/player.rs` prueba perfiles adaptativos (`ANDROID_VR`, `VISIONOS`, `ANDROID_VR_LEGACY`, `TV_EMBEDDED`, `IOS`) y elige el flujo acorde a la preferencia `AudioQuality` (`Low`, `Medium`, `High`).
   - Antes de aceptar una URL candidata, `verify_audio_tail` solicita el último byte (`Range: bytes={total-1}-{total-1}`) en **todas las plataformas** (escritorio y Android) para descartar perfiles truncados a los 42–48 s antes de reproducir.
   - En escritorio, si todos los perfiles nativos fallaran, `backend/streaming.rs` recurre a `yt-dlp` cuando está disponible.
3. **Descarga por bloques (`AudioStreamer`)**:
   - Descarga el audio en bloques `Range` dentro del directorio de caché y libera la reproducción apenas se almacenan los bytes iniciales.
   - Una tarea Tokio en segundo plano descarga los bloques restantes por HTTP keep-alive; ante un `HTTP 403` o corte de socket, refresca la URL automáticamente y retoma desde el desplazamiento exacto.
4. **Servidor Loopback con Seek**:
   - Sirve `http://127.0.0.1:<puerto>/stream/<sesión>.<ext>` con `Accept-Ranges: bytes` y `206 Partial Content`.
   - Si el usuario adelanta más allá de lo descargado, la conexión espera a `progress_rx.changed().await` hasta que el escritor alcance ese rango.
5. **Confirmación en caché LRU de 1 GiB**:
   - Las pistas completadas se consolidan en la caché LRU para reproducirse al instante sin consumo de red.

---

## 4. Reproducción Doble Deck, Crossfade y DSP Web Audio (`audioEngine.ts`)

- **Dos Decks HTML5 `<audio>` independientes (`Deck A` y `Deck B`)**:
  - `activeDeck` reproduce la pista actual mientras `standbyDeck` precarga e inicia la siguiente durante las transiciones de *crossfade* (`0–10 s`).
- **Grafo Web Audio bajo demanda (Lazy)**:
  - Se inicializa solo si el usuario activa el **Ecualizador de 5 bandas** (`60 Hz`, `230 Hz`, `910 Hz`, `3.6 kHz`, `14 kHz` con presets) o la **Normalización de volumen** (`DynamicsCompressorNode`), manteniendo sobrecarga cero por defecto en WebKitGTK y Android WebView.
- **Contrapartes oficiales Canción ↔ Video**:
  - Usa consultas `RDAMVM` de InnerTube para alternar sin fricción entre el audio oficial y su video musical (y viceversa).

---

## 5. Escucha Colaborativa ("Modo SPIN")

GyroTube ofrece dos modos colaborativos integrados bajo **Modo SPIN**, accesibles desde `SpinModal`, `SpinRemotePane`, una píldora dorada de estado en vivo en el reproductor y enlaces profundos `gyrotube://spin/...`:

### 5.1. Control Remoto / Jukebox LAN (`jam.rs`)
- **Caso de uso**: reuniones donde un dispositivo reproduce en los altavoces y los invitados en el mismo Wi-Fi agregan canciones desde el navegador de su teléfono sin instalar nada.
- **Implementación**:
  - Levanta un servidor HTTP en `0.0.0.0:<puerto>` con token (`/?t=<token>`) y código QR SVG (`qrcode`, renderizado de forma segura mediante `data:image/svg+xml`).
  - Sirve `assets/spin.html` para ver la pista actual, consultar la cola, buscar en YouTube Music y encolar pistas musicales verificadas.

### 5.2. Escucha Sincronizada Remota — App a App (`spin_remote.rs`)
- **Caso de uso**: amigos en distintos dispositivos o redes usando GyroTube con reproducción sincronizada y control compartido de la cola.
- **Implementación**:
  - Utiliza **Iroh** (`iroh::Endpoint`) sobre QUIC con ALPN `gyrotube/spin/1` y tickets compartibles `GYRO-...` (con nombre personalizable y botón Compartir).
  - **Controles bidireccionales**: cuando el host habilita `allow_controls`, los invitados pueden enviar comandos `spin_remote_control` (play/pausa, siguiente, anterior, seek) y `spin_remote_add_track`.
  - **Sincronización de cola y presencia**: transmite instantáneas `RemoteStatePayload` con la pista actual, cola próxima sincronizada (hasta 20 canciones con carátulas preservadas, `MAX_FRAME = 65536`), marca de tiempo para corrección de deriva y conteo en vivo `remote_guest_count`.

---

## 6. Sistema de Diseño Visual y UX

### 6.1. Identidad de marca (`gyrotube.svg`)
La identidad visual nace de `gyrotube.svg` — una paleta **Obsidian Vinyl + Jade Emerald + Spin Gold + Ruby Crimson** consistente en Escritorio, Android y la interfaz web de invitados SPIN:

- **Superficies Obsidian Vinyl**: tonos obsidiana oscuros (`#131218` lienzo, `#17151d` barra lateral, `#1e1c26` tarjetas/modales/mini-player) y tonos perla-jade en Modo Claro (`#f6f7f6`, `#eef0ef`, `#ffffff`).
- **Acento primario Jade Emerald**: `#10b981` / `#34d399` (Oscuro) y `#059669` / `#047857` (Claro) para acciones principales, estados activos, progreso y resplandor ambiental por defecto (`16, 185, 129`).
- **Acento secundario Spin Gold**: `#eab308` / `#fde047` (Oscuro) y `#ca8a04` / `#b45309` (Claro) para insignias de **Modo SPIN**, estado de invitados en vivo y el ícono orbital `SpinIcon`.
- **Favoritos y acciones destructivas Ruby Crimson**: `#ff5c6c` para favoritos/corazones y `#e11d48` para acciones destructivas.

### 6.2. Diseño adaptativo y funciones de biblioteca
- **Barra de navegación colapsable y reproductor lateral**:
  - Rail izquierdo en escritorio (`240px` expandido / `74px` colapsado) con playlists fijadas (máx. 4) y barra derecha (`360–388px`) con 4 pestañas: **Reproductor**, **Letras** (con controles compactos de reproducción y seek al pie), **Cola** y **Relacionadas**.
  - Ruta dedicada `#/settings` (`SettingsView` + `SettingsEqSection` + `SettingsAboutSection`).
- **Adaptación móvil y táctil**:
  - Detección reactiva (`isMobile`, `isCompact`, `isTouch`, `isAndroid`) que transforma modales (`SPIN`, `Editor de Playlist`, `Lyrics Card`) y el **Menú Contextual** en **Bottom Sheets** en pantallas móviles.
  - `ArtistHero` adaptativo (portada con máscara integrada en móvil y biografía oculta; hero amplio en escritorio).
- **Flujos de biblioteca y descubrimiento**:
  - **Smart Shuffle / Mejorar (`✨`)** en playlists locales mediante recomendaciones de radio.
  - **Playlists fijadas** (máx. 4, sincronizadas entre Colección, NavSidebar y HomeShortcuts).
  - **Historial agrupado por fecha** (*Hoy / Ayer / Esta semana / Anteriores*), **Ocultar canciones** (`EyeOff` con salto automático), aviso de duplicados y **Búsquedas recientes** unificadas (`query`, `artist`, `track`, `album`).

### 6.3. Iluminación ambiental dinámica, letras pulidas y Lyrics Cards
- **Extracción de color de carátula**: muestrea el color RGB dominante de la portada activa (respaldo Jade Emerald `16, 185, 129`) para teñir `.ambient-surface`, la barra lateral y el mini-player.
- **Letras sincronizadas y estáticas unificadas (`lyricsFormat.ts`, `SidebarLyricsPane`, `PartyMode`)**:
  - Parser compartido con puntos de espera de intro (`• • •`), curva asimétrica de opacidad (líneas pasadas más tenues, próximas más legibles) y agrupación por estrofas con encabezados `[Sección]` en letras estáticas.
  - La línea activa anima únicamente con `transform: scale(1.03)` / `scale(1.035)` y `opacity` en GPU (sin `filter: blur` ni reflows de `font-size`).
- **Tarjetas de letra compartibles (`lyricsCard.ts`, `LyricsCardModal.svelte`)**:
  - Exportador PNG `<canvas>` de alta densidad con formatos `4:5` y `1:1`, 6 temas (incluyendo HSL dinámico de carátula y negro puro), selección de 1 a 5 líneas en letras sincronizadas o estáticas, emblema `/gyrotube.svg` real y acciones de Compartir / Descargar / Copiar.

---

## 7. Tokens Visuales y Tipografía (`apps/tauri/ui/src/styles/app.css` y `spin.css`)

### 7.1. Tokens de color principales

| Token | Modo Oscuro | Modo Claro | Rol |
| :--- | :--- | :--- | :--- |
| `--bg` | `#131218` | `#f6f7f6` | Lienzo principal (Obsidian Vinyl / Pearl Jade) |
| `--bg-sidebar` | `#17151d` | `#eef0ef` | Rail de navegación y barra lateral del reproductor |
| `--bg-elevated` | `#1e1c26` | `#ffffff` | Modales, bottom sheets, mini-player, tarjetas SPIN |
| `--surface` / `--surface-hover` | `rgba(255,255,255,0.048)` / `0.09` | `rgba(22,20,28,0.04)` / `0.08` | Contenedores interactivos y píldoras |
| `--border` / `--border-strong` | `rgba(255,255,255,0.08)` / `0.15` | `rgba(22,20,28,0.09)` / `0.16` | Divisores finos y bordes elevados |
| `--text` / `--text-muted` | `#f5f4f8` / `rgba(245,244,248,0.72)` | `#16141c` / `rgba(22,20,28,0.70)` | Tipografía primaria y secundaria de alto contraste |
| `--accent` / `--accent-bright` | `#10b981` / `#34d399` | `#059669` / `#047857` | Acciones primarias Jade Emerald, estados activos, progreso |
| `--accent-soft` / `--accent-border` | `rgba(16,185,129,0.16)` / `0.40` | `rgba(5,150,105,0.12)` / `0.34` | Píldoras activas, filas seleccionadas, insignias de calidad |
| `--gold` / `--gold-bright` | `#eab308` / `#fde047` | `#ca8a04` / `#b45309` | Ícono de **Modo SPIN**, insignias en vivo y acentos orbitales |
| `--gold-soft` / `--gold-border` | `rgba(234,179,8,0.16)` / `0.38` | `rgba(202,138,4,0.14)` / `0.36` | Emblema de modal SPIN y píldora de sesión activa |
| `--fav-coral` / `--danger` | `#ff5c6c` / `#e11d48` | `#ff5c6c` / `#e11d48` | Favoritos Ruby Crimson y acciones destructivas |
| `--ambient-rgb` | `16, 185, 129` (por defecto) | `16, 185, 129` (por defecto) | Resplandor radial ambiental dinámico de la carátula |

### 7.2. Escala tipográfica y legibilidad

| Rol | Tamaño y peso | Detalles |
| :--- | :--- | :--- |
| **Fuente (`--lyrics-font`)** | `"Plus Jakarta Sans Variable", "Plus Jakarta Sans", system-ui` | `tabular-nums` (`.numeric`) para duraciones, contadores y tiempos de seek |
| **Saludo de Inicio / Hero de Artista** | `32px`, `800` | Tracking ajustado (`-0.02em`), acompañado de subtítulo atenuado de `14px` |
| **Títulos de sección (`.section-title`)** | `22px`, `800` | Tracking ajustado (`-0.02em`) |
| **Título en reproductor lateral** | `19px`, `800` | `line-clamp-2`, con botón interactivo de artista(s) y chip de álbum `Disc3` |
| **Título / Subtítulo en fila de pista** | `14px`–`15px` (`600`) / `12.5px`–`13px` | Portada `44×44px` en escritorio (`40×40px` en móvil) |
| **Letras en barra lateral / Modo Fiesta** | `18px`–`34px`, `700`–`800` | Curva asimétrica de opacidad + `scale(1.03–1.035)` en GPU |
| **Insignias y etiquetas micro** | `11px`–`11.5px`, `700` | Mayúsculas con tracking (`0.08em`), píldoras de calidad y SPIN |

### 7.3. Reglas de rendimiento y renderizado
- **Sin `backdrop-blur` persistente en modales/chrome**: las superficies elevadas usan `--bg-elevated` opaco con bordes sutiles para evitar caídas de composición en WebKitGTK y Android WebView.
- **Solo animaciones de `transform` y `opacity`**: barras de progreso (`scaleX`), foco en letras (`scale` + `opacity`) y rotación de `SpinIcon` corren exclusivamente en el hilo compositor.
- **Movimiento reducido**: respeta `prefers-reduced-motion: reduce`.

---

## 8. Persistencia e Invariantes de Seguridad

1. **Persistencia atómica de archivos (`crates/gyrotube-core/src/models.rs`)**:
   - Todos los datos de usuario (`favorites.json`, `playlists.json`, `recent.json`, `recent_searches.json`, `session.json` bajo `~/.local/share/gyrotube/`) se escriben en un archivo temporal, se vuelcan con `sync_all()` y se renombran atómicamente para evitar corrupción ante cierres inesperados.
2. **Servidor HTTP Loopback endurecido (`crates/gyrotube-core/src/audio_streamer.rs`)**:
   - Escucha únicamente en `127.0.0.1` con timeout de lectura de cabeceras de `5 s`, máximo `64` cabeceras y máximo `8 KiB` por línea.
   - Sustituye CORS comodín (`*`) por una lista explícita de `Origin` permitidos (`tauri://localhost`, `http://tauri.localhost`, `https://tauri.localhost`, `http://localhost:*`, `http://127.0.0.1:*`).
3. **Cero inserciones de HTML crudo e importaciones seguras (`apps/tauri/ui/`)**:
   - El frontend tiene **cero** usos de `{@html}` (los códigos QR en `SpinQrCard.svelte` se renderizan mediante `<img src="data:image/svg+xml;charset=utf-8,...">`).
   - La importación de respaldos JSON sanitiza `cover_path` (permitiendo solo rutas locales seguras o URLs `http(s)`) antes de restaurar playlists.
4. **Verificación estricta de catálogo musical (`crates/gyrotube-core/src/backend/catalog.rs`)**:
   - Cada ID de pista se valida como un ID de YouTube de 11 caracteres (`[A-Za-z0-9_-]{11}`) antes de tocar el sistema de archivos o la red.
   - Las pistas no confiables (resultados externos, sesiones antiguas o invitados SPIN) deben superar la verificación de catálogo musical antes de reproducirse o añadirse a la cola.
