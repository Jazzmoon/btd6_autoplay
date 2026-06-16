# Bloons TD 6 Auto Play

An automation bot for Bloons Tower Defense 6, written in Rust. It uses screen capture and Tesseract OCR to detect the current round number, then executes pre-configured instruction scripts to place and upgrade towers automatically.

## Disclaimer

This project is an unofficial automation tool and is not affiliated with, endorsed by, or supported by Ninja Kiwi. Automating gameplay may violate Ninja Kiwi's Terms of Service and could result in account action, including permanent bans. By using this software you accept full responsibility for any consequences and agree that we (Jazzmoon) are not liable for any damages or account actions arising from its use.

---

## Table of Contents

- [Bloons TD 6 Auto Play](#bloons-td-6-auto-play)
  - [Disclaimer](#disclaimer)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installing Rust (rustup)](#installing-rust-rustup)
    - [Linux / macOS](#linux--macos)
    - [Windows](#windows)
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

```bash
./target/release/btd6_autoplay --location
```

Prints cursor coordinates relative to the game window on each click. Use the reported `(x, y)` values when writing `place` instructions in a map config. See [maps-config.md](maps-config.md) for the full list of controls.

---

## Configuration

All configuration lives in the `config/` directory. See [maps-config.md](maps-config.md) for full documentation covering every field, all available hotkey names, how to calibrate a new screen resolution, and the complete map scripting reference.

```text
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
