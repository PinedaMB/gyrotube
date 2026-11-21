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

### 6.1. Brand Identity (`gyrotube.svg`)
GyroTube's visual identity is derived directly from `gyrotube.svg` — a jewel-toned **Obsidian Vinyl + Jade Emerald + Spin Gold + Ruby Crimson** system designed for high contrast, low eye fatigue, and cohesive branding across Desktop (Tauri + GTK4), Android, and the embedded **Modo SPIN** guest web UI:

- **Obsidian Vinyl Surfaces**: Deep plum-obsidian dark surfaces (`#131218` canvas, `#17151d` sidebar, `#1e1c26` elevated cards/modals/mini-player) paired with warm pearl-jade surfaces in Light Mode (`#f6f7f6` canvas, `#eef0ef` sidebar, `#ffffff` elevated).
- **Jade Emerald Primary Accent**: `#10b981` / `#34d399` in Dark Mode and `#059669` / `#047857` in Light Mode for primary actions, active navigation states, progress fills, and default ambient glow (`16, 185, 129`).
- **Spin Gold Secondary Accent**: `#eab308` / `#fde047` (Dark) and `#ca8a04` / `#b45309` (Light) echoing the golden spiral (`"The Spin"`) in `gyrotube.svg`, dedicated to **Modo SPIN** badges, live indicators, and the bespoke orbital `SpinIcon` SVG.
- **Ruby Crimson Favorites & Destructive Actions**: `#ff5c6c` for active favorites/hearts and `#e11d48` for destructive actions, echoing the central play ruby in `gyrotube.svg`.

### 6.2. Adaptive Layout & Interaction Patterns
- **Collapsible Navigation Rail & Right Player Sidebar**:
  - Desktop left navigation rail (`240px` expanded / `74px` collapsed) and right player sidebar (`360–388px`) with 4 stacked icon+label tabs: **Player**, **Lyrics**, **Queue**, and **Related**.
  - Centered content clamp (`max-width: 1400px`, `20px 28px 124px` desktop padding) for comfortable reading on 1080p and ultrawide displays.
- **Mobile & Touch Adaptation**:
  - Reactive device detection (`isMobile`, `isCompact`, `isTouch`, `isAndroid`) switches modals (`Settings`, `SPIN`, `Playlist Editor`) and the custom right-click **Context Menu** into native-feeling **Bottom Sheets** on mobile viewports.
  - Floating rounded **Mini-Player** (`rounded-2xl` on desktop, full-width docked above bottom navigation on mobile) with a GPU-driven `scaleX` live progress bar and cover-art ambient tint.
- **Borderless Content & Clean Controls**:
  - Airy borderless track rows (`rounded-xl`) with hover-revealed inline actions and clickable cover play overlays.
  - Borderless album and artist cards in horizontal carousels (`minCardWidth: 188px`, `gap: 14px`) with header-aligned `< >` navigation controls (no duplicate floating overlay arrows).
  - Interactive pill-styled **Artist** and **Album** (`Disc3` chip) metadata buttons in the player controls, replacing raw underlined text links.
  - Global `user-select: none` and `-webkit-user-drag: none` on UI chrome/images (preserving `user-select: text` on search inputs and text fields).

### 6.3. Dynamic Ambient Lighting, Synchronized Lyrics & Party Mode
- **Cover-Art Color Extraction**: When a track's cover loads, GyroTube samples its dominant RGB color (falling back to Jade Emerald `16, 185, 129`) to tint the non-scrolling `.ambient-surface` backdrop, player sidebar, and mini-player without triggering scroll-time repaints.
- **Unified GPU Lyrics (`SidebarLyricsPane` & `PartyMode`)**: Active lines animate exclusively via `transform: scale(1.045)` and `opacity` (avoiding `filter: blur` and `font-size` reflows) for smooth 60 FPS karaoke scrolling across both the sidebar and fullscreen Party Mode.

---

## 7. Visual Tokens & Styling System (`apps/tauri/ui/src/styles/app.css` & `spin.css`)

GyroTube uses semantic CSS custom properties shared across the Tauri/Svelte UI and the embedded **Modo SPIN** web client (`crates/gyrotube-core/assets/spin.html`), adapting automatically between Dark, Light, and System themes.

### 7.1. Core Color Tokens

| Token | Dark Mode | Light Mode | Role |
| :--- | :--- | :--- | :--- |
| `--bg` | `#131218` | `#f6f7f6` | Main application canvas (Obsidian Vinyl / Pearl Jade) |
| `--bg-sidebar` | `#17151d` | `#eef0ef` | Navigation rail & right player sidebar |
| `--bg-elevated` | `#1e1c26` | `#ffffff` | Modals, bottom sheets, mini-player, SPIN cards |
| `--surface` / `--surface-hover` | `rgba(255,255,255,0.048)` / `0.09` | `rgba(22,20,28,0.04)` / `0.08` | Interactive tiles, pills, and subtle containers |
| `--border` / `--border-strong` | `rgba(255,255,255,0.08)` / `0.15` | `rgba(22,20,28,0.09)` / `0.16` | Hairline dividers & focused/elevated outlines |
| `--text` / `--text-muted` | `#f5f4f8` / `rgba(245,244,248,0.72)` | `#16141c` / `rgba(22,20,28,0.70)` | Primary & secondary high-contrast typography |
| `--accent` / `--accent-bright` | `#10b981` / `#34d399` | `#059669` / `#047857` | Primary Jade Emerald actions, active states, progress fills |
| `--accent-soft` / `--accent-border` | `rgba(16,185,129,0.16)` / `0.40` | `rgba(5,150,105,0.12)` / `0.34` | Active pills, selected rows, quality badges |
| `--gold` / `--gold-bright` | `#eab308` / `#fde047` | `#ca8a04` / `#b45309` | **Modo SPIN** icon, live badges, and orbital accents |
| `--gold-soft` / `--gold-border` | `rgba(234,179,8,0.16)` / `0.38` | `rgba(202,138,4,0.14)` / `0.36` | SPIN modal emblem & live session pill |
| `--fav-coral` / `--danger` | `#ff5c6c` / `#e11d48` | `#ff5c6c` / `#e11d48` | Ruby Crimson favorites, hearts, and destructive actions |
| `--ambient-rgb` | `16, 185, 129` (default) | `16, 185, 129` (default) | Dynamic cover-art ambient radial glow |

### 7.2. Custom Brand & Feature Iconography
- **App Emblem (`gyrotube.svg`)**: 3D metallic jade sphere housing an obsidian vinyl platter, symmetric golden spin trails, and a crimson ruby play button.
- **Modo SPIN Icon (`SpinIcon.svelte` / `spin-icon.svg`)**: Custom `24×24` Lucide-compatible orbital vinyl icon with two symmetric arcs, two synced satellite nodes (host & guest), inner vinyl grooves, and a central spindle; smoothly rotates (`5.5s linear infinite`) whenever a SPIN session is live.

### 7.3. Border Radius & Spacing Scale

| Element Type | Border Radius | Usage |
| :--- | :--- | :--- |
| **Track Thumbnails** | `8px` – `9px` | Compact `40–44px` square album covers inside `TrackRow` and SPIN lists |
| **Small Controls & Inputs** | `--radius-sm` (`10px`) | Dropdowns, compact buttons, keycaps |
| **Cards & Rows** | `--radius-md` (`14px`) / `12px` | `TrackRow`, search cards, carousel artwork, SPIN cards |
| **Hero Cards, Modals & Mini-Player** | `--radius-lg` (`18px`) / `16px` | Featured hero, artist/album headers, `ModalSheet`, floating `MiniPlayer` |
| **Pills & Circular Controls** | `--radius-pill` (`999px`) | Play buttons, album/artist chips, search bar, SPIN/quality badges, avatars |

### 7.4. Typography & Readability Scale

| Role | Size & Weight | Details |
| :--- | :--- | :--- |
| **Font Stack** | `"Inter Variable", Inter, "SF Pro Display", "Segoe UI", system-ui` | `tabular-nums` (`.numeric`) for durations, counters, and seek timestamps |
| **Home Greeting / Artist Hero** | `32px`, `800` | Tight tracking (`-0.02em`), paired with `14px` muted subtitle |
| **Section Titles (`.section-title`)** | `22px`, `800` | Tight tracking (`-0.02em`) |
| **Sidebar Player Title** | `19px`, `800` | `line-clamp-2`, paired with `14.5px` interactive artist button and `12px` album chip |
| **Track Row Title / Subtitle** | `14px`–`15px` (`600`) / `12.5px`–`13px` | `44×44px` cover on desktop (`40×40px` on mobile) |
| **Album / Artist Card Title** | `14.5px` (`700`) / `13px` muted | Borderless cards inside `188px+` carousels |
| **Party Mode Lyrics** | `24px`–`34px`, `700`–`800` | GPU `scale(1.045)` + opacity highlight for active line |
| **Micro Badges & Eyebrows** | `11px`–`11.5px`, `700` | Uppercase tracked labels (`0.08em`), quality pills, SPIN live pill |

### 7.5. Performance & Rendering Guidelines
- **No Persistent `backdrop-blur` on Chrome/Modals**: Elevated surfaces use opaque `--bg-elevated` with subtle hairlines and soft shadows to avoid GPU compositing stalls in WebKitGTK and Android WebView.
- **Transform & Opacity Animations Only**: Progress bars (`scaleX`), lyric focus (`scale` + `opacity`), and `SpinIcon` rotation stay strictly on the compositor thread.
- **Reduced Motion**: Respects `prefers-reduced-motion: reduce` across transitions and animated indicators.

---

## 8. Persistence & Security Invariants

1. **Atomic File Persistence (`src/models.rs`)**:
   - All user data (`favorites.json`, `playlists.json`, `recent.json`, `recent_searches.json`, `session.json` under `~/.local/share/gyrotube/`) is written to a temporary staging file in the target directory, flushed with `sync_all()`, and atomically renamed over the destination so crashes or power loss never corrupt user state.
2. **Strict Music-Only Catalog Verification (`src/backend.rs`)**:
   - Every track ID is validated as an 11-character YouTube ID (`[A-Za-z0-9_-]{11}`) before touching the filesystem or spawning subprocesses.
   - Untrusted tracks (from search results, older sessions, or SPIN guests) must pass `verify_music_track` (`TrackType::Track && !unavailable`) before playback, preventing arbitrary video clips or forged metadata from entering the player.
