> 🌐 **Language / Idioma:** [Español](DESIGN.es.md) | **English**

# GyroTube — Architecture & Design Document

GyroTube is a cross-platform **YouTube Music** client for **Linux, Windows, macOS, and Android**, built with a pure-Rust core (**Edition 2024**, `crates/gyrotube-core`), **Tauri v2**, **Svelte 5** (runes), **Tailwind CSS v4**, and **Iroh** P2P networking. It focuses on low-latency progressive chunked audio playback, an adaptive Obsidian Vinyl + Jade Emerald aesthetic with dynamic cover-art ambient lighting, synced-first karaoke lyrics with shareable Lyrics Cards, and collaborative **SPIN Mode** listening sessions.

---

## 1. High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                   Tauri v2 + Svelte 5 + Tailwind CSS v4 UI                  │
│  ┌──────┐ ┌─────────┐ ┌────────────┐ ┌──────────┐ ┌──────────────────────┐  │
│  │ Home │ │ Explore │ │ Collection │ │ Settings │ │ Party Mode (Karaoke) │  │
│  └──┬───┘ └────┬────┘ └─────┬──────┘ └────┬─────┘ └──────────┬───────────┘  │
│     └──────────┴────────────┴─────────────┴──────────────────┤              │
│                                                              ▼              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │        Sidebar Player / Mini-Player / Bottom Sheets / SPIN Bar        │  │
│  │         (Player • Lyrics + Lyrics Cards • Queue • Related)            │  │
│  └──────────────────┬─────────────────────────────────┬──────────────────┘  │
│                     ▼                                 ▼                     │
│  ┌──────────────────────────────────────┐  ┌─────────────────────────────┐  │
│  │      DualDeckAudioEngine (TS)        │  │   Svelte 5 State Stores     │  │
│  │  HTML5 Audio A/B • Crossfade • Lazy  │  │ player • localPrefs • queue │  │
│  │  Web Audio 5-Band EQ & Normalization │  │ session • nativeEvents      │  │
│  └──────────────────┬───────────────────┘  └──────────────┬──────────────┘  │
└─────────────────────┼─────────────────────────────────────┼─────────────────┘
                      │ HTTP Loopback (127.0.0.1)           │ Tauri IPC Commands
                      ▼                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Rust Core (`crates/gyrotube-core`)                       │
│  ┌──────────────────────────────────┐  ┌─────────────────────────────────┐  │
│  │   Hardened Loopback HTTP Server  │  │  Native InnerTube & Catalog     │  │
│  │   (`audio_streamer.rs`)          │  │  • `innertube/{player,browse,   │  │
│  │   • `/stream/{id}.{ext}` (206)   │  │     parsers}` (5 profiles)      │  │
│  │   • `/cover/{id}` • `/proxy/{k}` │  │  • `backend/{catalog,streaming, │  │
│  │   • `verify_audio_tail` (all OS) │  │     lyrics,parsers}`            │  │
│  └──────────────────┬───────────────┘  │  • LRCLIB synced-first lyrics   │  │
│                     ▼                  │  • Desktop `yt-dlp` fallback    │  │
│  ┌──────────────────────────────────┐  └─────────────────────────────────┘  │
│  │   LRU Disk Audio Cache (1 GiB)   │  ┌─────────────────────────────────┐  │
│  │   (`audio_cache.rs`)             │  │      Collaborative SPIN Mode    │  │
│  └──────────────────────────────────┘  │  • LAN Jukebox HTTP + QR (`jam`)│  │
│  ┌──────────────────────────────────┐  │  • Iroh QUIC P2P (`spin_remote`)│  │
│  │  Atomic JSON Store (`models.rs`) │  │    ALPN `gyrotube/spin/1`       │  │
│  │  Android Service (`android-tmpl`)│  └─────────────────────────────────┘  │
│  └──────────────────────────────────┘                                       │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Workspace & Module Breakdown

### 2.1. Rust Core (`crates/gyrotube-core/src/`)

| Module | Path | Responsibility |
| :--- | :--- | :--- |
| **Backend Orchestrator** | `backend/mod.rs` | Shared `Backend` state, in-memory TTL caches (`stream_cache`, `music_proofs`, `home_cache`), and audio quality/session coordination. |
| **Catalog Operations** | `backend/catalog.rs` | Home feed, search, artist/album/playlist details, radio/related tracks, official Song↔Video counterparts (`RDAMVM`), and music-only provenance verification. |
| **Stream Resolution** | `backend/streaming.rs` | Adaptive stream resolver (pure-Rust InnerTube with 5 client profiles + optional desktop `yt-dlp` fallback), tail-byte verification, and mid-stream URL refresh. |
| **Lyrics Pipeline** | `backend/lyrics.rs` | Synced-first lyrics resolution: `ANDROID_MUSIC` timed lyrics → LRCLIB exact `/api/get` → validated LRCLIB `/api/search` synced (±8 s duration + title/artist match) → static fallbacks (`ytm_plain`, LRCLIB plain). |
| **Backend Parsers** | `backend/parsers.rs` | Normalization, badge filtering, duration/metadata extraction, and unit tests (`backend/tests.rs`). |
| **Native InnerTube Client** | `innertube/{mod,player,browse,parsers,tests}.rs` | Pure-Rust YouTube Music InnerTube API client: 5 playback client profiles (`ANDROID_VR`, `VISIONOS`, `ANDROID_VR_LEGACY`, `TV_EMBEDDED`, `IOS`), `/next` watch-panel cache, browse/search/radio endpoints, and localized media badge sanitization. |
| **Progressive Loopback Streamer** | `audio_streamer.rs` | Hardened local HTTP loopback server (`127.0.0.1:<port>`) serving `/stream/<id>.<ext>` (`206 Partial Content`), `/cover/<id>`, and `/proxy/<key>`. Downloads byte-range chunks, unblocks playback early, verifies tail bytes on all platforms, and commits finished tracks to disk cache. |
| **Disk Audio Cache** | `audio_cache.rs` | Reference-counted LRU disk cache (`~/.cache/gyrotube/streams/`, capped at `1 GiB`) pinning active `LocalAudio` files so eviction never removes a playing track. |
| **Data Models & Persistence** | `models.rs` | Core domain types (`Track`, `Album`, `Artist`, `Playlist`, `StreamInfo`, `LyricsData`, `AudioQuality`), LRC parser, title sanitizer, and atomic JSON persistence (`~/.local/share/gyrotube/`). |
| **SPIN Mode — LAN Jukebox** | `jam.rs` | Embedded LAN HTTP server (`0.0.0.0:<port>`) with mobile web UI (`assets/spin.html`) and SVG QR code generation via the `qrcode` crate (4-module quiet zone). |
| **SPIN Mode — Remote P2P** | `spin_remote.rs` | App-to-App synchronized listening over **Iroh** QUIC (`gyrotube/spin/1`) with `GYRO-...` tickets, bidirectional remote controls (`allow_controls`), synced up-next queue (up to 20 tracks, `MAX_FRAME = 65536`), and RAII live guest count (`remote_guest_count`). |
| **TLS & Internationalization** | `tls.rs`, `i18n.rs` | Rustls / WebPKI TLS bootstrap for desktop & Android; lightweight ES/EN translation layer (`tr(es, en)`) with locale detection. |

### 2.2. Tauri Shell & Android (`apps/tauri/`)

| Layer | Path | Responsibility |
| :--- | :--- | :--- |
| **Tauri Rust Bridge** | `src-tauri/src/{lib.rs, commands/}` | Registers Tauri v2 plugins, AppStream/desktop integration, and IPC commands (`catalog.rs`, `player.rs`, `library.rs`, `spin.rs`). |
| **Android Native Templates** | `android-templates/` | Patches `gen/android` with `PlaybackForegroundService.kt` (media notification + lock-screen controls + wake lock), `MainActivity.kt` (`gyrotube://spin/...` deep links & media key bridge), and `network_security_config.xml` (loopback cleartext). |
| **Frontend State & Engines** | `ui/src/lib/` | `audioEngine.ts` (`DualDeckAudioEngine`), `stores/player.svelte.ts`, `stores/localPrefs.ts`, `stores/queueOps.ts`, `stores/sessionPersistence.ts`, `stores/nativeEvents.ts`, `lyricsFormat.ts`, `lyricsCard.ts` (high-DPI `<canvas>` PNG Lyrics Cards), `listeningStats.ts`, `ytmUrl.ts`, `i18n.svelte.ts`. |
| **Frontend Views & Components** | `ui/src/{views/, components/}` | `HomeView`, `ExploreView`, `CollectionView`, `ArtistView`, `AlbumView`, `PlaylistView`, `SearchView`, `SettingsView` (with `SettingsEqSection` & `SettingsAboutSection`), `SidebarPlayer` + `SidebarLyricsPane`, `PartyMode`, `SpinModal`, `SpinRemotePane`, `LyricsCardModal`, `MiniPlayer`, `ModalSheet`, `ContextMenu`. |

---

## 3. Progressive Chunked Audio Pipeline

YouTube's CDN (`googlevideo.com`) throttles open-ended HTTP `GET` connections and may serve truncated (~768 KB / 42–48 s) or `HTTP 403` streams on restricted client profiles. GyroTube resolves and streams audio cleanly without waiting for full downloads:

1. **Speculative Prefetch**:
   - Hovering over a track row or starting playback triggers background resolution for the hovered or next queued track so deck transitions start immediately.
2. **Adaptive 5-Profile Pure-Rust Extraction**:
   - `innertube/player.rs` tries adaptive client profiles (`ANDROID_VR`, `VISIONOS`, `ANDROID_VR_LEGACY`, `TV_EMBEDDED`, `IOS`) and selects the best audio stream matching the user's `AudioQuality` preference (`Low`, `Medium`, `High`).
   - Before accepting a candidate URL, `verify_audio_tail` issues a single-byte tail probe (`Range: bytes={total-1}-{total-1}`) on **all platforms** (desktop and Android) to reject truncated 42–48 s profiles before playback begins.
   - On desktop, if all native InnerTube profiles fail, `backend/streaming.rs` falls back to `yt-dlp` when available.
3. **Chunked Byte-Range Downloading (`AudioStreamer`)**:
   - `AudioStreamer` fetches audio in byte-range chunks into a staging file inside the cache directory and unblocks playback as soon as initial bytes are buffered.
   - A background Tokio task downloads subsequent chunks sequentially over keep-alive HTTP at full CDN speed. If a mid-song chunk fails with `HTTP 403` or a dropped socket, the stream URL is transparently refreshed and resumed from the exact byte offset.
4. **Seekable Local Loopback Server**:
   - Serves `http://127.0.0.1:<port>/stream/<session>.<ext>` with `Accept-Ranges: bytes` and `206 Partial Content`.
   - Forward seeks past the current download watermark await `progress_rx.changed().await` until the background writer reaches the requested byte range.
5. **1 GiB LRU Disk Cache Commit**:
   - Completed streams are committed to the reference-counted LRU disk cache so replays load instantaneously without network requests.

---

## 4. Dual-Deck Playback, Crossfade & Web Audio DSP (`audioEngine.ts`)

- **Two Independent HTML5 `<audio>` Decks (`Deck A` & `Deck B`)**:
  - `activeDeck` plays the current track while `standbyDeck` preloads and starts the next track during crossfade transitions (`0–10 s`).
- **Lazy Web Audio API Graph**:
  - Created lazily only when the user enables the **5-Band Equalizer** (`60 Hz`, `230 Hz`, `910 Hz`, `3.6 kHz`, `14 kHz` with presets) or **Volume Normalization** (`DynamicsCompressorNode`), keeping default playback zero-overhead on WebKitGTK and Android WebView.
- **Official Song ↔ Video Counterparts**:
  - Uses InnerTube's `RDAMVM` watch-panel lookup to switch seamlessly between the official audio track and its music video counterpart (and vice versa).

---

## 5. Collaborative Listening ("SPIN Mode")

GyroTube provides two collaborative modes under **SPIN Mode**, surfaced through `SpinModal`, `SpinRemotePane`, a golden live status pill in the player, and deep links (`gyrotube://spin/...`):

### 5.1. Remote Control / LAN Jukebox (`jam.rs`)
- **Use Case**: Gatherings where one host device drives the speakers and guests on the same Wi-Fi add songs from any phone browser without installing an app.
- **Implementation**:
  - Binds an embedded HTTP server on `0.0.0.0:<port>` with token authentication (`/?t=<token>`) and an SVG QR code (`qrcode` crate, rendered safely via `data:image/svg+xml`).
  - Serves `assets/spin.html` so guests can see the current track, inspect the queue, search YouTube Music, and enqueue verified music tracks.

### 5.2. Remote Synchronized Listening — App-to-App (`spin_remote.rs`)
- **Use Case**: Friends on separate devices or networks running GyroTube who want synchronized playback and shared queue control.
- **Implementation**:
  - Uses **Iroh** (`iroh::Endpoint`) over QUIC with ALPN `gyrotube/spin/1` and shareable `GYRO-...` tickets (plus custom display name and Share button).
  - **Bidirectional Controls**: When the host enables `allow_controls`, guests can send `spin_remote_control` (play/pause, next, prev, seek) and `spin_remote_add_track` commands directly to the host.
  - **Queue Sync & Presence**: Broadcasts `RemoteStatePayload` snapshots containing the current track, synced up-next queue (up to 20 tracks with cover art preserved, `MAX_FRAME = 65536`), playback state, timestamp for drift correction, and live `remote_guest_count` (RAII-tracked).

---

## 6. UI/UX & Visual Design System

### 6.1. Brand Identity (`gyrotube.svg`)
GyroTube's visual identity derives from `gyrotube.svg` — a jewel-toned **Obsidian Vinyl + Jade Emerald + Spin Gold + Ruby Crimson** palette across Desktop, Android, and the embedded SPIN guest web UI:

- **Obsidian Vinyl Surfaces**: Deep plum-obsidian dark surfaces (`#131218` canvas, `#17151d` sidebar, `#1e1c26` elevated cards/modals/mini-player) paired with warm pearl-jade surfaces in Light Mode (`#f6f7f6` canvas, `#eef0ef` sidebar, `#ffffff` elevated).
- **Jade Emerald Primary Accent**: `#10b981` / `#34d399` (Dark) and `#059669` / `#047857` (Light) for primary actions, active navigation states, progress fills, and default ambient glow (`16, 185, 129`).
- **Spin Gold Secondary Accent**: `#eab308` / `#fde047` (Dark) and `#ca8a04` / `#b45309` (Light) for **SPIN Mode** badges, live guest pills, and the orbital `SpinIcon`.
- **Ruby Crimson Favorites & Destructive Actions**: `#ff5c6c` for active favorites/hearts and `#e11d48` for destructive actions.

### 6.2. Adaptive Layout & Library Features
- **Collapsible Navigation Rail & Right Player Sidebar**:
  - Desktop left rail (`240px` expanded / `74px` collapsed) with pinned playlists (up to 4) and right player sidebar (`360–388px`) with 4 tabs: **Player**, **Lyrics** (with compact playback & seek controls at the bottom), **Queue**, and **Related**.
  - Dedicated `#/settings` route (`SettingsView` + `SettingsEqSection` + `SettingsAboutSection`).
- **Mobile & Touch Adaptation**:
  - Reactive device detection (`isMobile`, `isCompact`, `isTouch`, `isAndroid`) adapts modals (`SPIN`, `Playlist Editor`, `Lyrics Card`) and the custom **Context Menu** into **Bottom Sheets** on mobile viewports.
  - Responsive `ArtistHero` (full-bleed masked artwork on mobile with bio hidden; tall cinematic hero on desktop).
- **Library & Discovery Workflows**:
  - **Smart Shuffle / Enhance (`✨`)** in local playlists via radio recommendations.
  - **Pinned Playlists** (max 4, synced across Collection, NavSidebar, and HomeShortcuts).
  - **Date-Grouped History** (*Today / Yesterday / This Week / Earlier*), **Hide Tracks** (`EyeOff` with auto-skip), duplicate track warnings, and unified **Recent Searches** (`query`, `artist`, `track`, `album`).

### 6.3. Dynamic Ambient Lighting, Polished Lyrics & Lyrics Cards
- **Cover-Art Color Extraction**: Samples the dominant RGB color of the active cover (falling back to Jade Emerald `16, 185, 129`) to tint the non-scrolling `.ambient-surface` backdrop, player sidebar, and mini-player.
- **Unified Synced & Static Lyrics (`lyricsFormat.ts`, `SidebarLyricsPane`, `PartyMode`)**:
  - Shared parser renders intro waiting dots (`• • •`), asymmetric forward-reading opacity curves (past lines recede gently, upcoming lines stay legible), and stanza grouping with `[Section]` headers for static lyrics.
  - Active lines animate strictly via GPU-composited `transform: scale(1.03)` / `scale(1.035)` and `opacity` (no `filter: blur` or `font-size` reflows).
- **Shareable Lyrics Cards (`lyricsCard.ts`, `LyricsCardModal.svelte`)**:
  - High-DPI `<canvas>` PNG exporter supporting `4:5` and `1:1` aspect ratios, 6 themes (including dynamic cover-art HSL and pitch black), 1–5 selectable lines from synced or static lyrics, real `/gyrotube.svg` emblem, and Share / Download / Copy actions.

---

## 7. Visual Tokens & Styling System (`apps/tauri/ui/src/styles/app.css` & `spin.css`)

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
| `--gold` / `--gold-bright` | `#eab308` / `#fde047` | `#ca8a04` / `#b45309` | **SPIN Mode** icon, live badges, and orbital accents |
| `--gold-soft` / `--gold-border` | `rgba(234,179,8,0.16)` / `0.38` | `rgba(202,138,4,0.14)` / `0.36` | SPIN modal emblem & live session pill |
| `--fav-coral` / `--danger` | `#ff5c6c` / `#e11d48` | `#ff5c6c` / `#e11d48` | Ruby Crimson favorites, hearts, and destructive actions |
| `--ambient-rgb` | `16, 185, 129` (default) | `16, 185, 129` (default) | Dynamic cover-art ambient radial glow |

### 7.2. Typography & Readability Scale

| Role | Size & Weight | Details |
| :--- | :--- | :--- |
| **Font Stack (`--lyrics-font`)** | `"Plus Jakarta Sans Variable", "Plus Jakarta Sans", system-ui` | `tabular-nums` (`.numeric`) for durations, counters, and seek timestamps |
| **Home Greeting / Artist Hero** | `32px`, `800` | Tight tracking (`-0.02em`), paired with `14px` muted subtitle |
| **Section Titles (`.section-title`)** | `22px`, `800` | Tight tracking (`-0.02em`) |
| **Sidebar Player Title** | `19px`, `800` | `line-clamp-2`, paired with interactive artist pill(s) and `Disc3` album chip |
| **Track Row Title / Subtitle** | `14px`–`15px` (`600`) / `12.5px`–`13px` | `44×44px` cover on desktop (`40×40px` on mobile) |
| **Sidebar / Party Mode Lyrics** | `18px`–`34px`, `700`–`800` | Asymmetric forward-reading opacity curve + GPU `scale(1.03–1.035)` |
| **Micro Badges & Eyebrows** | `11px`–`11.5px`, `700` | Uppercase tracked labels (`0.08em`), quality pills, SPIN live pill |

### 7.3. Performance & Rendering Guidelines
- **No Persistent `backdrop-blur` on Chrome/Modals**: Elevated surfaces use opaque `--bg-elevated` with subtle hairlines and soft shadows to prevent GPU compositing stalls in WebKitGTK and Android WebView.
- **Transform & Opacity Animations Only**: Progress bars (`scaleX`), lyric focus (`scale` + `opacity`), and `SpinIcon` rotation stay strictly on the compositor thread.
- **Reduced Motion**: Respects `prefers-reduced-motion: reduce` across transitions and animated indicators.

---

## 8. Persistence & Security Invariants

1. **Atomic File Persistence (`crates/gyrotube-core/src/models.rs`)**:
   - All user data (`favorites.json`, `playlists.json`, `recent.json`, `recent_searches.json`, `session.json` under `~/.local/share/gyrotube/`) is written to a staging file in the target directory, flushed with `sync_all()`, and atomically renamed over the destination so crashes never corrupt user state.
2. **Hardened Loopback HTTP Server (`crates/gyrotube-core/src/audio_streamer.rs`)**:
   - Binds strictly to `127.0.0.1` with a `5 s` per-connection header read timeout, max `64` headers, and max `8 KiB` per header line.
   - Replaces wildcard CORS (`*`) with an explicit `Origin` allowlist (`tauri://localhost`, `http://tauri.localhost`, `https://tauri.localhost`, `http://localhost:*`, `http://127.0.0.1:*`).
3. **Zero Raw HTML Sinks & Safe Backup Imports (`apps/tauri/ui/`)**:
   - The frontend contains **zero** `{@html}` sinks (QR codes in `SpinQrCard.svelte` render via `<img src="data:image/svg+xml;charset=utf-8,...">`).
   - JSON backup imports sanitize `cover_path` (allowing only safe local paths or `http(s)` URLs) before restoring playlists.
4. **Strict Music-Only Catalog Verification (`crates/gyrotube-core/src/backend/catalog.rs`)**:
   - Every track ID is validated as an 11-character YouTube ID (`[A-Za-z0-9_-]{11}`) before touching the filesystem or network endpoints.
   - Untrusted tracks (from search results, older sessions, or SPIN guests) must pass music-only catalog verification before playback or queue insertion.
