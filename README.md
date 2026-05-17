# CSPROTUI

A Terminal User Interface (TUI) application for browsing professional CS2 player settings.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

## Features

- **Search** – Find any pro player by name
- **Tabbed Settings View** – Browse settings across multiple categories:
  - Mouse (DPI, Sensitivity, eDPI, etc.)
  - Crosshair (import code, size, gap, color, etc.)
  - Viewmodel
  - Video (resolution, scaling, NVIDIA settings, etc.)
  - Radar
  - HUD
  - Launch Options
- **Copy Crosshair Import Code** – Press `Ctrl+Y` to copy the player's crosshair share code to clipboard
- **Theme Picker** – Press `T` or `Ctrl+T` to switch between built-in themes
- **Mouse & Keyboard Navigation** – Scroll with mouse or use arrow keys, PageUp/PageDown, Home/End

## Dependencies

- [Node.js](https://nodejs.org/) (required at runtime for the scraper)

## Installation

### via npm (recommended)

```bash
npm install -g csprotui
```

The install script automatically downloads the correct binary and sets up the scraper.

### from source

```bash
cd scraper && npm install
cd ..
cargo build --release
```

The compiled binary will be at `target/release/csprotui`.

## Running

```bash
csprotui
```

> **Note:** The app bundles a Node.js scraper. When installing via npm, dependencies are handled automatically. When building from source, run `cd scraper && npm install` first.

## Controls

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll settings |
| `PgUp` / `PgDn` | Scroll page by page |
| `Home` / `End` | Jump to top / bottom |
| `Tab` / `Shift+Tab` | Next / previous tab |
| `Ctrl+T` / `T` | Open theme picker |
| `Ctrl+Y` | Copy crosshair import code |
| `Esc` | Cancel search / go back |
| `Enter` | Submit search |
| `q` | Quit (when search is empty) |
