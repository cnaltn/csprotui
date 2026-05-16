# prosettings-tui

A Terminal User Interface (TUI) application for browsing professional CS2 player settings from [prosettings.net](https://prosettings.net).

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green.svg)

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

- [Rust](https://www.rust-lang.org/) (latest stable)
- [Node.js](https://nodejs.org/) (required for the external scraper)

## Building

```bash
cargo build --release
```

The compiled binary will be at `target/release/prosettings-tui`.

## Running

```bash
cargo run
```

> **Note:** This app includes a bundled Node.js scraper under the `scraper/` directory. Before first run, install its dependencies:
>
> ```bash
> cd scraper && npm install
> ```

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

## License

MIT
