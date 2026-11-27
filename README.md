<p align="center">
  <img src="gyrotube.svg" width="104" height="104" alt="Gyro logo" />
</p>

<h1 align="center">Gyro</h1>

<p align="center">
  <strong>Your music, without distractions, on every device.</strong><br />
  A native cross-platform YouTube Music client for <strong>Linux, Windows, macOS, and Android</strong>.<br />
  Fast, privacy-first, featuring synced karaoke lyrics, offline downloads, and real-time collaborative listening.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.6-10b981?style=for-the-badge" alt="Version 1.0.6" />
  <img src="https://img.shields.io/badge/Linux%20%7C%20Windows%20%7C%20macOS%20%7C%20Android-1e1c26?style=for-the-badge" alt="Cross-Platform" />
  <img src="https://img.shields.io/badge/Rust%20%2B%20Tauri%20v2%20%2B%20Svelte%205-eab308?style=for-the-badge" alt="Rust + Tauri v2 + Svelte 5" />
  <img src="https://img.shields.io/badge/License-MIT-ff5c6c?style=for-the-badge" alt="MIT License" />
</p>

<p align="center">
  <a href="#-key-features">Features</a> •
  <a href="#-gallery">Gallery</a> •
  <a href="#-keyboard-shortcuts">Shortcuts</a> •
  <a href="#-build--documentation">Build & Docs</a> •
  <a href="#-author--license">Author</a>
</p>

<p align="center">
  <img src="screenshots/home-dark.png" alt="GyroTube — Home Screen in Dark Mode" width="100%" />
</p>

---

## ✨ Key Features

### ⚡ Instant, Gapless, High-Fidelity Audio
- **Pure-Rust Dual-Deck Engine**: on desktop (Linux, Windows, macOS), audio decodes and mixes natively in pure Rust (`symphonia` + `cpal`) with equal-power crossfade (`0–10 s`), a 5-band biquad EQ, and stereo volume normalization—zero GStreamer/WebView audio overhead. Android uses a lazy-DSP HTML5 Dual-Deck engine.
- **Native 6-Profile Extraction + Embedded QuickJS**: progressive chunked streaming with an embedded sandboxed QuickJS (`rquickjs`) cipher/`n`-param solver, predictive next-track preloading, and a smart **1 GiB** LRU disk cache.
- **Synchronized Song ↔ Video Switch**: jump between the studio track and its official music video with unified loading/seek sync and a pipelined 1 MiB chunked video proxy.
- **SponsorBlock Integration**: opt-in privacy-preserving skipping of non-music intros, outros, and skits (`music_offtopic` via 4-char SHA-256 prefix lookup).
- **Adaptive Audio Quality**: pick *Low*, *Normal*, or *High* to match your connection.

### 🎤 Word-by-Word Karaoke Lyrics, Party Mode & Lyrics Cards
- **Multi-Source Karaoke Lyrics**: word-by-word synced lyrics (Better Lyrics TTML, LyricsPlus V2, YouTube Music, and LRCLIB) with live source switching, per-track timing offset (`±ms`), smooth scrolling, font size (`A-/A/A+`), and alignment controls (left / center / right).
- **Offline Romanization & Translation**: 100 % offline pure-Rust transliteration (Hangul, Kana, Cyrillic, Greek) plus on-demand lyrics translation (ES/EN) with multi-provider fallback.
- **Fullscreen Party Mode**: turn any monitor or TV into an immersive listening display with large artwork, live lyrics, and up-next queue.
- **Shareable Lyrics Cards**: pick 1 to 5 lines from any track and export high-DPI PNG cards (`4:5` or `1:1`) in 6 themes ready to share anywhere.

### 🌀 SPIN Mode — Collaborative Listening
- **LAN Jukebox (Local Wi-Fi)**: start a room and display a QR code—guests scan from any phone browser with zero app installs to see what's playing, search tracks, and add to the queue.
- **App-to-App Remote Sync (P2P)**: listen together across different networks over **Iroh QUIC** with a shareable `GYRO-...` invite code, synced up-next queue, and optional remote controls.

### 🎧 Your Library, Offline & Private
- **Full Offline Downloads**: save playlists and Favorites with audio, cover art, and synced lyrics for uninterrupted offline playback.
- **Spotify Playlist Importer**: import public Spotify playlists by URL (no sign-in) or Exportify CSV files with automatic YouTube Music matching.
- **Smart Local Playlists**: customize name, icon, color, or cover; pin up to 4 playlists to the sidebar; use **Enhance / Smart Shuffle (`✨`)** for instant radio recommendations, with duplicate track detection.
- **Zero-Account Privacy & Opt-In Integrations**: date-grouped play history, hide tracks, listening stats, unified search, **local JSON backup/restore**, plus strictly opt-in **Discord Rich Presence**, **ListenBrainz**, and **Last.fm** scrobbling (all disabled by default).

### 📱 Truly Native Across Desktop & Mobile
- **Desktop Installers**: native packages for Linux (`.AppImage`, `.rpm`, `.deb`), Windows (`.msi`, `.exe`), and macOS (`.dmg`) with system tray controls, background playback, and keyboard shortcuts.
- **Native Android Experience**: foreground playback service with lock-screen `MediaSession` controls, hardware volume integration, touch-friendly bottom sheets, and `gyrotube://spin/...` deep links.
- **Curated Themes & Bilingual UI**: **Dark**, **Light**, and **System** modes across 7 curated palettes (*Default Jade, AMOLED, Catppuccin, Nord, Rosé Pine, Tokyo Night, Gruvbox*), plus full **English** and **Spanish** support across the app, desktop tray, and SPIN guest web UI.

---

## 🖼️ Gallery

### Dark Mode & Light Mode
<table>
  <tr>
    <td width="50%" align="center">
      <img src="screenshots/home-dark.png" alt="Home — Dark Mode" /><br />
      <sub><strong>Home — Dark Mode</strong> (Obsidian Vinyl + Jade Emerald)</sub>
    </td>
    <td width="50%" align="center">
      <img src="screenshots/home-light.png" alt="Home — Light Mode" /><br />
      <sub><strong>Home — Light Mode</strong> (High-contrast Pearl Jade)</sub>
    </td>
  </tr>
  <tr>
    <td width="50%" align="center">
      <img src="screenshots/search-dark.png" alt="Search — Dark Mode" /><br />
      <sub><strong>Dedicated Search — Dark Mode</strong></sub>
    </td>
    <td width="50%" align="center">
      <img src="screenshots/search-light.png" alt="Search — Light Mode" /><br />
      <sub><strong>Dedicated Search — Light Mode</strong></sub>
    </td>
  </tr>
</table>

### Party Mode, SPIN Mode, Playlists & Settings
<table>
  <tr>
    <td width="50%" align="center">
      <img src="screenshots/party-mode.png" alt="Party Mode with synced lyrics" /><br />
      <sub><strong>Party Mode</strong> — Synced lyrics & dynamic ambient lighting</sub>
    </td>
    <td width="50%" align="center">
      <img src="screenshots/SPIN-mode.png" alt="Collaborative SPIN Mode" /><br />
      <sub><strong>SPIN Mode</strong> — Wi-Fi QR Jukebox & App-to-App P2P rooms</sub>
    </td>
  </tr>
  <tr>
    <td width="50%" align="center">
      <img src="screenshots/playlist.png" alt="Playlist and Library view" /><br />
      <sub><strong>Playlists & Library</strong> — Offline downloads, sorting & Smart Shuffle</sub>
    </td>
    <td width="50%" align="center">
      <img src="screenshots/settings.png" alt="GyroTube Settings" /><br />
      <sub><strong>Settings</strong> — 5-band EQ, audio quality, cache & JSON backups</sub>
    </td>
  </tr>
</table>

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Ctrl+K` / `Cmd+K` | Focus search |
| `Space` | Play / Pause |
| `←` / `→` | Seek backward / forward 5 seconds |
| `Shift+←` / `Shift+→` | Previous / Next track |
| `M` | Mute / Unmute |

---

## 🛠️ Build & Documentation

### macOS Installation Note
Pre-built macOS `.dmg` releases are ad-hoc signed and not notarized with a paid Apple Developer account. If macOS Gatekeeper reports that **GyroTube is damaged and can't be opened**, drag **GyroTube.app** to **Applications**, then run in Terminal:

```bash
xattr -cr /Applications/GyroTube.app
```

### Build from Source

```bash
# Desktop (Linux .AppImage/.rpm/.deb, Windows .msi/.exe, macOS .dmg)
./scripts/build-desktop.sh

# Android aarch64 APK (signed release -> output/android/gyrotube-android-release.apk)
./scripts/build-android.sh
```

### Technical Documentation
- **Build Guide**: [English](docs/BUILDING.en.md) · [Español](docs/BUILDING.es.md)
- **Architecture & Design**: [English](docs/DESIGN.en.md) · [Español](docs/DESIGN.es.md)

### Workspace Overview
- `crates/gyrotube-core`: shared pure-Rust core (native 6-profile InnerTube client + sandboxed QuickJS solver, pure-Rust `symphonia`+`cpal` desktop audio engine, hardened loopback HTTP `Range` streamer & chunked video proxy, 1 GiB LRU disk cache, multi-source karaoke lyrics + offline romanization + translation, SponsorBlock, Spotify importer, scrobblers, atomic JSON persistence, and SPIN LAN/P2P).
- `apps/tauri`: cross-platform **Tauri v2 + Svelte 5 + Tailwind CSS v4** app, native Discord RPC IPC bridge, and Android templates (`apps/tauri/android-templates`).

---

## 👤 Author & License

Created and maintained by **[PinedaMB](https://github.com/PinedaMB)**.

Released under the **[MIT License](LICENSE)**.
