# CSPROTUI

A Terminal User Interface (TUI) application for browsing professional CS2 player settings from [prosettings.net](https://prosettings.net).

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)

## Features

- **Player Search** — Find any pro player by name (e.g., `xantares`, `s1mple`, `ZyW0o`, `donk`, `m0NESY`)
- **8 Tabbed Settings Categories:**
  - **Mouse** — DPI, Sensitivity, eDPI, Zoom Sensitivity, Hz, Windows Sensitivity
  - **Crosshair** — Import code, style, size, thickness, gap, color (RGB), dot, outlines, sniper width
  - **Viewmodel** — FOV, offset X/Y/Z, preset, bob
  - **Video** — Resolution, aspect ratio, scaling mode, brightness, display mode, G-Sync, NVIDIA Reflex, FPS cap, anti-aliasing, texture/shader/shadow detail
  - **Radar** — Always centered, rotate, square with scoreboard, HUD scale, zoom
  - **HUD** — HUD scaling, HUD color
  - **Gear** — Mouse, keyboard, monitor, headset, mousepad, earphones
  - **Launch Options** — CS2 launch options string
- **Crosshair Import Code** — `Ctrl+Y` copies the `CSGO-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx` share code to clipboard
- **CLI Mode** — Query any player and output specific data directly to stdout (16 output flags)
- **Theme Picker** — Press `Ctrl+T` to switch between 39 built-in themes; custom `.toml` themes supported
- **Theme Persistence** — Selected theme remembered across sessions (`~/.config/csprotui/theme`)
- **Landing Page** — Animated ASCII banner with inline search on first launch
- **Mouse Support** — Scroll with mouse wheel or use keyboard (arrow keys, PageUp/Down, Home/End)
- **Auto-refresh** — Scraper reloads pages to bypass Cloudflare cache for up-to-date data

## Dependencies

- [Node.js](https://nodejs.org/) (required at runtime for the scraper)
- [npm](https://www.npmjs.com/) (for installing scraper dependencies)

## Installation

Download the latest release for your platform from [GitHub Releases](https://github.com/cnaltn/csprotui/releases).

### Quick Install

**Linux / macOS / Fedora:**

```bash
curl -fsSL https://raw.githubusercontent.com/cnaltn/csprotui/main/install.sh | bash
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/cnaltn/csprotui/main/install.ps1 | iex
```

### From Release Archive

```bash
# Linux / macOS / Fedora
tar -xzf csprotui-linux-x86_64.tar.gz   # or fedora / macos variant
cd csprotui
./install.sh
```

```powershell
# Windows
Expand-Archive csprotui-windows-x86_64.zip -DestinationPath csprotui
cd csprotui
.\install.ps1
```

### From Source

```bash
cd scraper && npm install
cd ..
cargo build --release
```

The compiled binary will be at `target/release/csprotui`.

### Via npm

```bash
npm install -g csprotui
```

## Running

```bash
csprotui
```

> **Note:** When using pre-built binaries, run `npm install` inside the `scraper/` directory first. The install scripts do this automatically.

## Controls

### Landing Page

| Key | Action |
|-----|--------|
| Type characters | Enter search query |
| `Enter` | Submit search |
| `Backspace` | Delete last character |
| `T` / `Ctrl+T` | Open theme picker |
| `Esc` / `q` | Quit |

### Main View (Player Loaded)

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll settings |
| `PgUp` / `PgDn` | Scroll page by page (10 rows) |
| `Home` / `End` | Jump to top / bottom |
| `Tab` | Next tab (Mouse → Crosshair → ... → Launch Options) |
| `Shift+Tab` | Previous tab |
| `Ctrl+T` | Open theme picker |
| `Ctrl+Y` | Copy crosshair import code to clipboard |
| `Esc` | Go back to landing page |
| `Enter` | Submit search |
| `Backspace` | Delete character from search |
| `q` | Quit (when search is empty) |

### Loading State

| Key | Action |
|-----|--------|
| `Esc` | Cancel and go back |
| `Backspace` | Edit query |
| Type characters | Edit search query |

### Theme Picker

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate themes |
| `Enter` | Select theme |
| `Esc` | Cancel / close |
| Type characters | Filter themes by name |

### Mouse

| Action | Effect |
|--------|--------|
| Scroll wheel | Scroll settings table |

## CLI Usage

```bash
csprotui [<player>] [flag]
```

Without flags, launches the TUI. With a player name and a flag, outputs the specified data to stdout.

### Player Name Examples

```bash
csprotui                     # Launch TUI (landing page)
csprotui m0NESY              # Launch TUI pre-searching for m0NESY
```

### Output Flags

| Short | Long | Output |
|-------|------|--------|
| `-c` | `--crosshair` | Crosshair import code (`CSGO-xxxxx-...`) |
| `-s` | `--sensitivity` | Mouse sensitivity value |
| `-d` | `--dpi` | Mouse DPI |
| `-e` | `--edpi` | Mouse eDPI |
| `-m` | `--mouse` | All mouse settings (key: value) |
| `-v` | `--viewmodel` | Viewmodel settings |
| | `--video` | Video settings |
| `-r` | `--radar` | Radar settings |
| | `--hud` | HUD settings |
| `-l` | `--launch` | Launch options |
| | `--gear` | Gear items |
| `-a` | `--all` | All settings (human-readable) |
| `-j` | `--json` | Full JSON output (pipe to `jq`) |
| `-h` | `--help` | Show help |

### CLI Examples

```bash
csprotui xantares --crosshair     # Get crosshair import code
csprotui s1mple -m                # Get all mouse settings
csprotui zywoo --all              # Get all settings
csprotui donk --json | jq .data.mouse   # JSON output piped to jq
```

## Themes

CSPROTUI ships with **39 built-in themes** powered by [Opaline](https://github.com/hyperb1iss/opaline).

### Highlights

| Family | Variants |
|--------|----------|
| SilkCircuit | `neon`, `vibrant`, `soft`, `dawn`, `glow` |
| Tokyo Night | `tokyo-night`, `tokyo-night-moon`, `tokyo-night-storm` |
| Catppuccin | `latte`, `frappe`, `macchiato`, `mocha` |
| Rose Pine | `rose-pine`, `rose-pine-moon`, `rose-pine-dawn` |
| Kanagawa | `wave`, `dragon`, `lotus` |
| Ayu | `dark`, `mirage`, `light` |
| Flexoki | `dark`, `light` |
| Solarized | `dark`, `light` |
| Everforest | `dark`, `light` |
| GitHub | `light`, `dark-dimmed` |
| Gruvbox | `dark`, `light` |
| Others | `nord`, `dracula`, `monokai-pro`, `one-dark`, `one-light`, `night-owl`, `light-owl`, `palenight` |

### Custom Themes

Place `.toml` theme files in:

- `~/.config/opaline/themes/` (Opaline global)
- `~/.config/csprotui/themes/` (app-specific)

## Configuration

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `CSPROTUI_BASE_URL` | Override prosettings base URL | `https://prosettings.net/players` |
| `CSPROTUI_SCRAPER_DIR` | Override scraper directory path | `./scraper` |

### Persisted State

- Theme selection is saved to `~/.config/csprotui/theme` (Linux/macOS) or `%APPDATA%/csprotui/theme` (Windows)

## Build

### Supported Platforms

| Platform | Format |
|----------|--------|
| Linux x86_64 | `.tar.gz` |
| Fedora x86_64 | `.tar.gz` |
| macOS (Apple Silicon) | `.tar.gz` |
| macOS (Intel) | `.tar.gz` |
| Windows x86_64 | `.zip` |

### Build from Source

```bash
git clone https://github.com/cnaltn/csprotui.git
cd csprotui
cd scraper && npm install && cd ..
cargo build --release
```

## License

MIT
