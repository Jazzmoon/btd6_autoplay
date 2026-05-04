# Bloons TD 6 Auto Play

An automation bot for Bloons Tower Defense 6, written in Rust. It uses screen capture and Tesseract OCR to detect the current round number, then executes pre-configured instruction scripts to place and upgrade towers automatically.

## Disclaimer

Please be aware, Ninja Kiwi doesn't support the use of internal or external modding. Under Ninja Kiwi's Terms of Service, scripts such as this one can lead to a ban on your account. Using this, or any other script like it, means that you accept this risk and agree we (Jazzmoon) are not held accountable for any actions taken against your account.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installing Rust (rustup)](#installing-rust-rustup)
- [Installing Tesseract OCR](#installing-tesseract-ocr)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows (MSYS2)](#windows-msys2)
- [Building the Bot](#building-the-bot)
- [Game Setup](#game-setup)
- [Usage](#usage)
  - [Interactive Mode](#interactive-mode)
  - [CLI Arguments](#cli-arguments)
  - [Location Finder Utility](#location-finder-utility)
- [Configuration](#configuration)
  - [General Config](#general-config)
  - [Hotkeys](#hotkeys)
  - [Screen Resolution Settings](#screen-resolution-settings)
  - [Map Config Files](#map-config-files)
- [Writing a Map Config](#writing-a-map-config)
- [Recommended `roundCounterMode` per Map](#recommended-roundcountermode-per-map)

---

## Prerequisites

- **Bloons TD 6** installed (Steam or standalone)
- **Rust toolchain** (via `rustup`) — see below
- **Tesseract OCR** — see below
- A supported screen resolution: `1920×1080` or `2560×1440`
  - Other resolutions require you to create a matching `config/<WIDTHxHEIGHT>/` directory

---

## Installing Rust (rustup)

The bot is built with Rust (edition 2021). The recommended way to install Rust is via `rustup`.

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions, then reload your shell:

```bash
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version   # e.g. rustc 1.78.0
cargo --version   # e.g. cargo 1.78.0
```

### Windows

Download and run the installer from <https://rustup.rs/>. Accept the defaults. After installation, open a new command prompt and verify:

```cmd
rustc --version
cargo --version
```

> **Note for Windows users:** You will also need either the **MSVC build tools** (Visual Studio Build Tools) or the **MinGW-w64 toolchain** from MSYS2 (recommended for this project because it is also required for Tesseract — see below).

---

## Installing Tesseract OCR

The bot uses `rusty-tesseract` which requires the Tesseract library (libtesseract) to be installed on your system.

### Linux

Install Tesseract and its development headers using your distribution's package manager.

**Debian/Ubuntu:**

```bash
sudo apt update
sudo apt install tesseract-ocr libtesseract-dev libleptonica-dev pkg-config
```

**Fedora/RHEL:**

```bash
sudo dnf install tesseract tesseract-devel leptonica-devel pkgconf
```

**Arch Linux:**

```bash
sudo pacman -S tesseract tesseract-data-eng leptonica pkg-config
```

After installing, ensure the `TESSDATA_PREFIX` is set if Tesseract cannot find its language data:

```bash
export TESSDATA_PREFIX=/usr/share/tessdata   # adjust path as needed
```

> **Note:** The bot ships its own custom-trained data file (`tessdata/btd6.traineddata`) for better accuracy on in-game text. When this file is present, the bot automatically sets `TESSDATA_PREFIX` to the local `tessdata/` directory and uses `btd6+eng` as the OCR language. No manual configuration is required.

### macOS

```bash
brew install tesseract
```

`pkg-config` is also needed during compilation:

```bash
brew install pkg-config
```

### Windows (MSYS2)

For Windows, the recommended approach is to use **MSYS2** to provide the required libraries. See [windows-setup.md](windows-setup.md) for full step-by-step instructions. A summary is below.

1. Download and install MSYS2 from <https://www.msys2.org/>. Uncheck "Run MSYS2 now" at the end of the installer.

2. Open **Command Prompt** and add the MSYS2 binaries to your `PATH` (adjust the path if you chose a non-default install location):

   ```cmd
   setx PATH "C:\msys64\mingw64\bin;C:\msys64\usr\bin;%PATH%"
   setx TESSDATA_PREFIX "C:\msys64\mingw64\share\tessdata"
   ```

   Close and reopen the command prompt to apply the changes.

3. Open **MSYS2 MinGW 64-bit** and install the required packages:

   ```bash
   pacman -S mingw-w64-x86_64-leptonica \
             mingw-w64-x86_64-tesseract-ocr \
             mingw-w64-x86_64-tesseract-data-eng \
             mingw-w64-x86_64-pkg-config
   ```

4. Verify the libraries are visible to `pkg-config`:

   ```cmd
   pkg-config --cflags --libs lept tesseract
   ```

   Expected output (paths may differ):

   ```
   -IC:/msys64/mingw64/include -IC:/msys64/mingw64/include/leptonica -LC:/msys64/mingw64/lib -lleptonica -ltesseract -larchive -lcurl
   ```

5. Install the MinGW-w64 Rust target so `cargo` uses the right linker:

   ```cmd
   rustup target add x86_64-pc-windows-gnu
   rustup default stable-x86_64-pc-windows-gnu
   ```

---

## Building the Bot

Clone the repository, then build a release binary:

```bash
git clone https://github.com/Jazzmoon/btd6_autoplay.git
cd btd6_autoplay
cargo build --release
```

The compiled binary is placed in `target/release/btd6_autoplay` (Linux/macOS) or `target\release\btd6_autoplay.exe` (Windows).

---

## Game Setup

Before launching the bot, configure your game:

1. Run **Bloons TD 6 in fullscreen** at one of the supported resolutions (`1920×1080` or `2560×1440`).
2. Enable **auto-start** in the game options so rounds begin automatically after the bot places towers.
3. Navigate to the map you want to automate and leave the game on the **loading/pre-round** screen.
4. The recommended in-game knowledge upgrades for the default map scripts are:
   - Free Dart Monkey
   - Additional $200 starting cash
   - Half of 1st Military Monkey cost
   - Military Monkey Upgrade Discounts

   These are not mandatory — adjust the timing in your map config if you don't have them.

5. Ensure the game window is **not minimized** before running the bot.

---

## Usage

Run the binary from the repository root directory so it can locate the `config/` and `tessdata/` directories:

```bash
./target/release/btd6_autoplay [OPTIONS]
```

The bot sleeps for a few seconds after startup (default: 5 s) so you can switch focus to the game window.

### Interactive Mode

Run the bot with no arguments to be guided through map, difficulty, and gamemode selection via interactive prompts:

```bash
./target/release/btd6_autoplay
```

### CLI Arguments

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--map <NAME>` | `-m` | Map name (must match a file in `config/<RES>/maps/`) | interactive prompt |
| `--difficulty <DIFF>` | `-d` | Difficulty: `easy`, `medium`, or `hard` | interactive prompt |
| `--gamemode <MODE>` | `-g` | Gamemode (e.g. `standard`, `chimps`, `half_cash`) | interactive prompt |
| `--number <N>` | `-n` | Number of games to play before exiting (`-1` = unlimited) | `-1` |
| `--sleep <SECS>` | `-s` | Seconds to wait before the bot activates | `5` |
| `--location` | | Launch the location finder utility instead of the bot | — |
| `--log-level <LEVEL>` | | Logging verbosity: `debug`, `info`, `notice`, `warn`, `error` | `warn` |

**Example — run Dark Castle, Easy, Standard, 10 games:**

```bash
./target/release/btd6_autoplay \
  --map "Dark Castle" \
  --difficulty easy \
  --gamemode standard \
  --number 10
```

### Location Finder Utility

If you need to determine pixel coordinates for placing towers on a new map, use the built-in location finder. It prints the cursor's current position relative to the game window whenever you move the mouse.

```bash
./target/release/btd6_autoplay --location
```

Use the reported `(x, y)` values when writing `place` instructions in a map config file.

---

## Configuration

All configuration lives in the `config/` directory.

```
config/
├── General.yaml                # Window search terms
├── Hotkeys.yaml                # In-game hotkey bindings
├── 1920x1080/
│   ├── Settings.yaml           # UI element coordinates for 1080p
│   └── maps/
│       ├── DarkCastle.yaml
│       └── Logs.yaml
└── 2560x1440/
    ├── Settings.yaml           # UI element coordinates for 1440p
    └── maps/
        ├── DarkCastle.yaml
        └── Logs.yaml
```

### General Config

`config/General.yaml` controls how the bot finds the BTD6 window by searching for these substrings in the window title / app name:

```yaml
window_title_search_terms:
  - "btd"
  - "bloons"
  - "steam_app_960090"
```

Add additional terms here if the bot cannot find the game window on your system.

### Hotkeys

`config/Hotkeys.yaml` maps logical action names to key combinations matching your in-game hotkey settings. Edit this file if you have customised the default BTD6 hotkeys.

```yaml
start: ["space"]
dart:  ["q"]
sub:   ["x"]
# ... etc.
```

### Screen Resolution Settings

`config/<WIDTHxHEIGHT>/Settings.yaml` stores the pixel coordinates of game UI elements (round counter, victory banner, buttons, etc.) calibrated for that resolution. If you use a different resolution you must create a new directory and `Settings.yaml` for it, using the location finder utility to determine correct coordinates.

### Map Config Files

`config/<WIDTHxHEIGHT>/maps/<MapName>.yaml` contains the full automation script for a map. The filename (without `.yaml`) is used as the display name in the interactive prompt.

---

## Writing a Map Config

A map config is a YAML file with the following top-level keys:

```yaml
round_counter_mode: "light"   # "light" or "dark" — see maps-config.md

easy:
  standard:
    money_per_game: 60          # Expected income per game (informational)
    on_win_action: "restart"    # "restart" | "continue" | "end_game"
    hover_location:
      x: 960
      y: 540
    instructions:
      1:                        # Round number (actions run when this round starts)
        - "start"
        - "place <name> <tower_type> <x> <y>"
        - "sleep 1s"
        - "upgrade <name> <top>-<mid>-<bot>"
      3:
        - "place sub sub 1084 434"
```

### Supported Instructions

| Instruction | Syntax | Description |
|-------------|--------|-------------|
| `start` | `start [fast-forward]` | Press the start/fast-forward hotkey |
| `place` | `place <name> <type> <x> <y>` | Place a tower (`<name>` is a unique label; `<type>` matches a key in `Hotkeys.yaml`) |
| `upgrade` | `upgrade <name> <T>-<M>-<B>` | Upgrade a previously placed tower to path `T-M-B` (e.g. `2-0-3`) |
| `sell` | `sell <name>` | Sell a tower |
| `ability` | `ability <name_or_key> [key2]` | Activate an ability by hotkey name or raw key |
| `click` | `click <x> <y>` | Click at absolute coordinates |
| `hover` | `hover <x> <y>` | Move the mouse to coordinates |
| `obstacle` / `clear` | `obstacle <x> <y>` | Click to remove a map obstacle |
| `sleep` | `sleep <N>[ms\|s\|m\|h]` | Pause execution (e.g. `sleep 500ms`, `sleep 2s`) |

### `on_win_action` Values

| Value | Behaviour |
|-------|-----------|
| `restart` | Replay the same map from round 1 |
| `continue` | Proceed into free-play mode |
| `end_game` | Return to the main menu |

---

## Recommended `roundCounterMode` per Map

The bot reads the round counter using OCR. Maps with a dark background need `round_counter_mode: "dark"` so the image is inverted before OCR. See [maps-config.md](maps-config.md) for a full list of recommended values per map.
