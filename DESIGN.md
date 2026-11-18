# GyroTube — Architecture & Design Document (`DESIGN.md`)

GyroTube is a native Linux desktop client for **YouTube Music**, built in **Rust (Edition 2024)** with **GTK4**, **Libadwaita**, **GStreamer**, and **Iroh** P2P networking. It focuses on low-latency chunked audio playback, an adaptive GNOME/Adwaita aesthetic with dynamic cover-art ambient lighting, synchronized karaoke lyrics, and collaborative **Modo SPIN** listening sessions.

---

## 1. High-Level Architecture

```
┌───────────────────────────────────────────────────────────────────────────┐
│                         GTK4 / Libadwaita UI Layer                        │
│  ┌──────────┐  ┌───────────┐  ┌────────────┐  ┌────────────────────────┐  │
│  │   Home   │  │  Explore  │  │ Collection │  │  Party Mode (Karaoke)  │  │
│  └────┬─────┘  └─────┬─────┘  └─────┬──────┘  └───────────┬────────────┘  │
│       └──────────────┴──────────────┴─────────────────────┤               │
│                                                           ▼               │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │             Sidebar Player / Mini-Bars / SPIN Status Bar            │  │
│  │                (Player Tab  •  Lyrics Tab  •  Queue Tab)            │  │
│  └───────────────┬──────────────────────────────────┬──────────────────┘  │
└──────────────────┼──────────────────────────────────┼─────────────────────┘
                   │                                  │
                   ▼                                  ▼
┌─────────────────────────────────────┐  ┌──────────────────────────────────┐
│      Dual-Deck Audio Engine         │  │     Collaborative Modo SPIN      │
│  ┌───────────────────────────────┐  │  │  ┌────────────────────────────┐  │
│  │ GStreamerDeck A & B (playbin) │  │  │  │ Jukebox LAN HTTP + QR Web  │  │
│  │  (Crossfade, Accurate Seek)   │  │  │  │       (src/jam.rs)         │  │
│  └───────────────┬───────────────┘  │  │  └────────────────────────────┘  │
│                  ▼                  │  │  ┌────────────────────────────┐  │
│  ┌───────────────────────────────┐  │  │  │ Remote Sync P2P via Iroh   │  │
│  │   Progressive Chunked HTTP    │  │  │  │   QUIC + Short PIN/Ticket  │  │
│  │   Loopback Streamer (127.0.0.1│  │  │  │    (src/spin_remote.rs)    │  │
│  │     src/audio_streamer.rs)    │  │  │  └────────────────────────────┘  │
│  └───────────────┬───────────────┘  │  └──────────────────────────────────┘
│                  ▼                  │
│  ┌───────────────────────────────┐  │  ┌──────────────────────────────────┐
│  │  LRU Disk Audio Cache (512MB) │  │  │   Catalog & Extraction Backend   │
│  │      (src/audio_cache.rs)     │  │  │  ytmapi-rs • rustypipe • yt-dlp  │
│  └───────────────────────────────┘  │  │  LRCLIB    • MPRIS2 D-Bus        │
└─────────────────────────────────────┘  └──────────────────────────────────┘
```

---

## 2. Module Breakdown

| Module | File | Responsibility |
| :--- | :--- | :--- |
| **Entrypoint** | `src/main.rs` | Declares top-level modules, includes compiled `relm4-icons`, and boots `ui::run()`. |
| **Catalog & Stream Resolver** | `src/backend.rs` | Queries YouTube Music catalog via `ytmapi-rs` & `rustypipe`, verifies music-only provenance, extracts signed stream URLs via `yt-dlp`, fetches synced lyrics from `LRCLIB`, and manages in-memory caches (`stream_cache`, `music_proofs`, `home_cache`). |
| **Progressive Chunked Streamer** | `src/audio_streamer.rs` | Local HTTP loopback server (`127.0.0.1:<port>/stream/<session>.<ext>`) that downloads audio in byte-range blocks (`256 KB` initial block, then `512 KB` blocks), unblocks playback at `32 KB`, serves `206 Partial Content` for seeking, auto-refreshes expired URLs mid-stream, and commits completed files to disk cache. |
| **Disk Audio Cache** | `src/audio_cache.rs` | Reference-counted LRU disk cache (`~/.cache/gyrotube/streams/`, capped at `512 MB`) that pins active `LocalAudio` files held by the player so eviction never deletes a track currently playing. |
| **GStreamer Audio Deck** | `src/gst_deck.rs` | Wraps GStreamer `playbin` with low-latency buffering (`64 KB` / `0.5 s`), accurate seeking (`FLUSH \| ACCURATE`), soft-volume ramping for crossfades, and startup telemetry. |
| **Data Models & Persistence** | `src/models.rs` | Core domain types (`Track`, `Album`, `Artist`, `Playlist`, `Genre`, `StreamInfo`, `LyricsData`), LRC parser, title sanitizer, and atomic JSON persistence (`~/.local/share/gyrotube/`). |
| **MPRIS2 Integration** | `src/mpris.rs` | Exposes `org.mpris.MediaPlayer2.gyrotube` over D-Bus for desktop media keys, system tray controls, and lock-screen metadata. |
| **Modo SPIN — Jukebox LAN** | `src/jam.rs` | Embedded HTTP server on `0.0.0.0:<port>` with mobile web UI and SVG QR code generator (`qrcode`) so guests on the same Wi-Fi can search and enqueue songs from any phone browser. |
| **Modo SPIN — Remote P2P** | `src/spin_remote.rs` | App-to-App synchronized listening across different networks using **Iroh** QUIC streams (`iroh` + `iroh-tickets`) and a local UDP LAN registry for 6-character short codes (`SPIN-XXXXXX`). |
| **Internationalization** | `src/i18n.rs` | Lightweight ES/EN translation layer (`tr(es, en)`) with automatic system locale detection (`LC_ALL`, `LC_MESSAGES`, `LANG`) and runtime override support. |
| **UI State & Shell** | `src/ui/` | GTK4/Libadwaita views (`home.rs`, `explore.rs`, `collection.rs`, `sidebar_player.rs`, `player.rs`, `party_mode.rs`, `jam_dialog.rs`, `widgets.rs`, `state.rs`) and stylesheet (`src/style.css`). |

---

## 3. Progressive Chunked Audio Pipeline

YouTube's CDN (`googlevideo.com`) throttles open-ended HTTP `GET` connections and rejects unverified URLs with `HTTP 403`. GyroTube solves both problems without waiting for the entire song to download before playback starts:

1. **Speculative Hover Prefetch**:
   - Hovering over any track row for `180 ms` (or `220 ms` in search results) triggers `Backend::resolve_track_stream` in the background.
   - As soon as a track starts playing, the next track in the queue is prefetched after `150 ms`.
2. **Parallel Verification & Signed Extraction**:
   - Unverified tracks run catalog verification (`RustyPipe::music_details` checking `TrackType::Track`) concurrently (`tokio::join!`) with audio stream resolution.
   - `yt-dlp --js-runtimes node` extracts the signed `googlevideo.com` URL, audio bitrate, `filesize`, and required HTTP headers.
3. **Chunked Byte-Range Downloading (`AudioStreamer`)**:
   - `AudioStreamer::start_chunked_http_stream` requests **Block 0** (`Range: bytes=0-262143`, `256 KB`) into a staging file inside `~/.cache/gyrotube/streams/`.
   - As soon as **`32 KB`** (~1.5 seconds of Opus audio) are written to disk, `start_chunked_http_stream` returns `http://127.0.0.1:<port>/stream/<id>-<seq>.webm` to GStreamer.
   - A background Tokio task continues fetching subsequent **`512 KB`** blocks (`Range: bytes=start-end`) sequentially over keep-alive HTTP at full CDN speed.
   - If any mid-song block encounters a dropped connection or `HTTP 403`, `refresh_ytdlp_audio_stream` transparently re-extracts the exact same `itag` and resumes from `start = written` without interrupting GStreamer.
4. **Seekable Local Loopback Server**:
   - The local HTTP handler always advertises `Accept-Ranges: bytes` and serves `206 Partial Content` with exact `Content-Range: bytes start-end/total` (derived from `Content-Range`, URL `clen`, or `yt-dlp` `filesize`).
   - If the user seeks forward to a byte offset not yet written, the local socket awaits `progress_rx.changed().await` until the chunk writer reaches that offset, then streams the bytes seamlessly.
5. **Cache Commit & Fallback Protection**:
   - Once all bytes are written, the staging file is copied to `~/.cache/gyrotube/streams/<video_id>.ytdlp.<ext>` so future plays and replays load directly from disk (`file://`).
   - If a remote stream ever emits a premature `EOS` (< 85% of the track duration), `connect_ended_notify` intercepts it, records the current timestamp in `pending_restore_seek`, completes the file via fallback download, and resumes at the exact second.

---

## 4. Dual-Deck Playback & Crossfade Engine

- **Two Independent `GStreamerDeck` Instances (`decks: [GStreamerDeck; 2]`)**:
  - `active_deck` plays the current track while `standby_deck` (`1 - active_deck`) prepares the next track during crossfade transitions.
- **Crossfade Ramping**:
  - When `crossfade_enabled` is active (`1–10 s`), the 250 ms tick loop detects when `remaining_us <= crossfade_secs + 3 s`, resolves the next track's stream on the standby deck, prerolls it in `Paused` state at `volume = 0.0`, and swaps `active_deck` while linearly ramping volumes over `50 ms` steps.

---

## 5. Collaborative Listening ("Modo SPIN")

GyroTube provides two distinct collaborative modes under **Modo SPIN**, unified by a Spotify-inspired bottom status bar and a live **Connected in SPIN** participants card inside the Queue tab:

### 5.1. Control Remoto / Jukebox (1 Speaker, N Phones on Local Wi-Fi)
- **Use Case**: House parties or gatherings where the Linux PC is connected to speakers and guests want to add songs from their phones without installing an app.
- **Implementation (`src/jam.rs`)**:
  - Binds an HTTP server on the local LAN IP (`http://<lan_ip>:<port>/?t=<token>`).
  - Renders a scannable SVG QR code in the SPIN dialog.
  - Serves a responsive mobile web interface where guests can view the current track, inspect the queue, search YouTube Music, and enqueue songs (rate-limited and verified against the YouTube Music catalog before insertion).

### 5.2. Escucha Sincronizada Remota (App-to-App over Internet or LAN)
- **Use Case**: Friends in different locations (or separate devices) running GyroTube who want to listen to the exact same song at the exact same second.
- **Implementation (`src/spin_remote.rs`)**:
  - Uses **Iroh** (`iroh::Endpoint`) over QUIC with ALPN `gyrotube/spin-remote/1` and Ed25519 NodeIDs.
  - Supports both full Iroh tickets (`SPIN-...`, works across any internet NAT/relay) and 6-character short codes (`SPIN-XXXXXX`, resolved via local cache and UDP LAN broadcast on port `41927`).
  - **Drift Correction**: The Host broadcasts `RemoteStatePayload` snapshots (`track`, `queue`, `is_playing`, `position_secs`, `sent_at_ms`, `connected_peers`, `host_name`). Guests compute `position_at(now_ms())` compensating for network transit time and automatically re-seek if clock drift exceeds `1.65 s`.

---

## 6. UI/UX & Visual Design System

### 6.1. Adaptive Layout (`adw::OverlaySplitView`)
- **Left Sidebar (`360–440 px`)**: Houses the persistent **Player**, **Lyrics**, and **Queue** tabs.
- **Main Content Area (`adw::ViewStack`)**:
  - **Inicio (`home.rs`)**: Time-of-day contextual greeting, 4 adaptive quick-navigation cards, *Volver a escuchar* (Listen Again) history grid, *Destacado* (Featured Release) hero card, *Nuevos lanzamientos* carousel, *Tendencias / Top Música* chart list, and *Estaciones por género*.
  - **Explorar (`explore.rs`)**: Instant search with `Tab` autocomplete, individually removable recent search history pills, category filter chips (*Todo*, *Canciones*, *Álbumes*, *Artistas*, *Playlists*), mood/genre grid with raw InnerTube JSON fallback, and full Artist & Album sub-views.
  - **Colección (`collection.rs`)**: Favorites, user-created Playlists, and Recently Played tracks.
- **Floating Mini-Player Bars & Bottom SPIN Bar**:
  - When the sidebar is collapsed or the user switches to the Lyrics/Queue tab, a floating mini-player bar appears with cover art, track/status stack, progress bar, and transport controls.
  - When a SPIN session is active, a dedicated bottom bar displays the active mode, host name, connected guest count, and quick actions (*Copiar código*, *Ver sala*, *Salir*).

### 6.2. Dynamic Ambient Lighting & Party Mode
- **Cover-Art Color Extraction**: When a track's cover texture loads, GyroTube samples the image pixels to compute a dominant accent RGB color and injects dynamic CSS gradients into `.ambient-player` and `.player-cover-glow`.
- **Party Mode (`party_mode.rs`, `F11`)**: Fullscreen two-column view with a 300×300 high-elevation cover, transport controls, *A continuación* (Up Next) preview card, and large-typography karaoke lyrics (`36 px` active line) with smooth cubic ease-out auto-centering and edge opacity fading.

---

## 7. CSS & Visual Styling System (`src/style.css`)

GyroTube layers a custom GTK4 stylesheet (`src/style.css`) on top of **Libadwaita (`v1_6`)** semantic color tokens so it adapts automatically to light/dark themes while keeping a rich music-streaming visual identity.

### 7.1. Color Palette & Adwaita Semantic Tokens

| Token / Color | Value / Source | Usage |
| :--- | :--- | :--- |
| `@window_bg_color` | Libadwaita window surface | Main background and `.party-mode-bg` canvas |
| `@window_fg_color` | Libadwaita primary text | Typography and translucent surfaces via `alpha(@window_fg_color, 0.04..0.16)` |
| `@sidebar_bg_color` | Libadwaita sidebar surface | Left player/lyrics/queue sidebar (`.sidebar-pane`) |
| `@headerbar_bg_color` | Libadwaita header surface | Floating mini-player (`alpha(@headerbar_bg_color, 0.96)`) & mobile mini-bar |
| `@accent_color` / `@accent_bg_color` | System / Dynamic cover accent | Active track titles (`.playing-title`), `.quality-pill`, active lyric lines, progress bars, SPIN bars |
| **Quick-Card 1 (Red)** | `#e01b24` (Hover `#f66151`, Active `#c01c28`) | *Nuevos lanzamientos* badge (`.quick-badge-1`) & `.album-play-overlay` FAB |
| **Quick-Card 2 (Blue)** | `#3584e4` | *Top listas* badge (`.quick-badge-2`) & gradient hover |
| **Quick-Card 3 (Purple)** | `#c061cb` | *Estados de ánimo* badge (`.quick-badge-3`) & gradient hover |
| **Quick-Card 4 (Green)** | `#2ec27e` | *Tu biblioteca* badge (`.quick-badge-4`) & gradient hover |
| **Artist Hero Gradient** | `linear-gradient(135deg, #1b2832 0%, #16181d 60%, #101216 100%)` | Backdrop for artist header banners (`.artist-hero`) with `#0d0f12` bottom scrim |

### 7.2. Surface Hierarchy, Borders & Elevation
- **Glass & Translucent Cards**:
  - **Level 1 (Subtle containers)**: `background-color: alpha(@window_fg_color, 0.035..0.04)` with `1px solid alpha(@window_fg_color, 0.06)` (`.empty-state-card`, `.lyrics-card`, `.genre-chip`).
  - **Level 2 (Interactive cards & tiles)**: `background-color: alpha(@window_fg_color, 0.05)` with `1px solid alpha(@window_fg_color, 0.08)`, elevating on `:hover` to `alpha(@window_fg_color, 0.09..0.10)` and border `0.14..0.22` (`.album-featured-card`, `.quick-card`, `.bento-card`, `.listen-again-tile`, `.party-next-card`).
  - **Level 3 (Floating overlays)**: `.floating-mini-player` uses `alpha(@headerbar_bg_color, 0.96)`, `1px solid alpha(@window_fg_color, 0.16)`, and `box-shadow: 0 10px 30px alpha(black, 0.50)`.
- **Cover Art Shadows**:
  - `.cover-art-large`: `16px` radius, `box-shadow: 0 10px 28px alpha(black, 0.34)`.
  - `.home-hero-cover`: `12px` radius, `box-shadow: 0 10px 28px alpha(#000000, 0.45)`.
  - `.party-cover`: `box-shadow: 0 18px 48px alpha(#000000, 0.55)`.

### 7.3. Border Radius & Spacing Scale

| Element Type | Border Radius | CSS Classes |
| :--- | :--- | :--- |
| **Small Badges & Tags** | `6px` – `8px` | `.quality-pill`, `.brand-badge`, `.album-year-badge`, `.cover-art`, `.lyric-line` |
| **Compact Tiles & Chips** | `10px` – `12px` | `.quick-icon-badge`, `.media-card`, `.genre-chip`, `.listen-again-tile`, `.lyrics-card`, `.spin-participants-card` |
| **Featured & Bento Cards** | `14px` – `18px` | `.quick-card`, `.album-featured-card`, `.bento-card`, `.artist-hero`, `.album-page-hero`, `.home-hero-card`, `.floating-mini-player`, `.spin-qr-card` |
| **Pills & Circular Controls** | `999px` | `.play-pill`, `.pill-switch`, `.pill-switch-group`, `.avatar-art`, `.header-search`, `.album-play-overlay`, `.party-badge`, `.spin-guest-pill` |

### 7.4. Typography Scale

| Role | Class | Size & Weight | Details |
| :--- | :--- | :--- | :--- |
| **Artist Hero Title** | `.artist-hero-title` | `32px`, `800` | White with `0 2px 12px` drop shadow |
| **Album Hero Title** | `.album-hero-title` | `1.95em`, `800` | Tight tracking (`letter-spacing: -0.02em`) |
| **Home Greeting** | `.home-greeting-title` | `1.9em`, `800` | Tight tracking (`letter-spacing: -0.02em`) |
| **Party Active Lyric** | `.party-lyric-line.lyric-line-active` | `36px`, `800` | Highlighted with `alpha(@accent_bg_color, 0.18)` pill |
| **Party Inactive Lyric** | `.party-lyric-line` | `28px`, `400` | `line-height: 1.4`, `14px` radius |
| **Party Track Title** | `.party-title` | `26px`, `800` | Centered under 300×300 cover |
| **Sidebar Active Lyric** | `.lyric-line-active` | `16px`, `700` | `@accent_color` text on `alpha(@accent_bg_color, 0.16)` |
| **Sidebar Inactive Lyric** | `.lyric-line` | `15px`, `400` | `opacity: 0.55` (`0.9` on hover) |
| **Plain Lyrics Body** | `.lyrics-plain` | `14.5px`, `400` | `line-height: 1.65` |
| **Uppercase Section Eyebrows** | `.album-type-label`, `.chart-header-row` | `0.76em`–`0.78em`, `700` | `letter-spacing: 0.05em..0.08em`, uppercase |
| **Micro Badges** | `.quality-pill`, `.home-hero-badge`, `.lyrics-badge-synced` | `10px`–`11px`, `700`–`800` | Tinted background `alpha(@accent_bg_color, 0.18..0.24)` with `@accent_color` foreground |

### 7.5. Micro-Interactions, Animations & Dynamic CSS Providers
- **Card Play Button Reveal (`.album-play-overlay`)**:
  - Rests at `opacity: 0; transform: translateY(4px);` and transitions over `150ms ease` to `opacity: 1; transform: translateY(0);` when `.album-featured-card:hover` or `:focus-within` is triggered.
- **Play Pill Loader (`.play-pill spinner`)**:
  - Renders a `20×20 px` pure white (`#ffffff`, `opacity: 1`) `GtkSpinner` inside the primary `56×44 px` pill button while keeping widget sensitivity active so GTK's CSS engine animates the spinner at 60 FPS.
- **Runtime Dynamic CSS Providers**:
  - **Ambient Cover Lighting (`widgets::update_ambient_color`)**: Injects a runtime `CssProvider` coloring `.ambient-player` and `.player-cover-glow` with radial/linear gradients derived from the current album cover's dominant RGB values.
  - **Per-Genre Accent Tint (`explore.rs` / `home.rs`)**: Applies per-widget `CssProvider` rules using each YouTube Music mood/genre's hex color (`linear-gradient(90deg, alpha({color}, 0.14) 0%, ...)`).
- **Responsive Breakpoints (`.mobile-layout` & `.rail-compact`)**:
  - Below narrow widths, `.mobile-layout` compacts card padding (`12px`), font sizes (`.title-2` to `18px`, `.pill-switch` to `12.5px`), and switches from the floating mini-player to the docked `.mobile-mini-player`.

---

## 8. Persistence & Security Invariants

1. **Atomic File Persistence (`src/models.rs`)**:
   - All user data (`favorites.json`, `playlists.json`, `recent.json`, `recent_searches.json`, `session.json` under `~/.local/share/gyrotube/`) is written to a temporary staging file in the target directory, flushed with `sync_all()`, and atomically renamed over the destination so crashes or power loss never corrupt user state.
2. **Strict Music-Only Catalog Verification (`src/backend.rs`)**:
   - Every track ID is validated as an 11-character YouTube ID (`[A-Za-z0-9_-]{11}`) before touching the filesystem or spawning subprocesses.
   - Untrusted tracks (from search results, older sessions, or SPIN guests) must pass `verify_music_track` (`TrackType::Track && !unavailable`) before playback, preventing arbitrary video clips or forged metadata from entering the player.
