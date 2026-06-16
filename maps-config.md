# BTD6 Autoplay Configuration

## Configuration Files

The bot reads three types of config files from the `config/` directory. You must create all of them before running the bot.

```text
config/
├── General.yaml               # Window detection settings
├── Hotkeys.yaml               # In-game hotkey bindings
└── {width}x{height}/          # One folder per screen resolution (e.g. 1920x1080)
    ├── Settings.yaml          # UI element coordinates for that resolution
    └── maps/
        └── MyMap.yaml         # One file per map you want to autoplay
```

The resolution folder is selected automatically at startup based on the detected game window dimensions. If the folder does not exist the bot will panic and tell you.

---

### General.yaml

Controls how the bot finds the BTD6 window.

```yaml
window_title_search_terms:
    - "btd"
    - "bloons"
    - "steam_app_960090"
```

| Field | Description |
| --- | --- |
| `window_title_search_terms` | List of substrings (case-insensitive) matched against the window app name / title. The first window whose name contains any term is used. |

---

### Hotkeys.yaml

Maps action names to key combinations. Each value is a list of key name strings that are pressed simultaneously.

```yaml
start: ["space"]
pause: ["`"]
sell: ["backspace"]
upgrade_top_path: [","]
upgrade_middle_path: ["."]
upgrade_bottom_path: ["/"]
dart: ["q"]
# ... etc
target_priority_left: ["l_control", "tab"]   # multi-key combo
```

All actions that must be present:

| Action | Default |
| --- | --- |
| `start` | `["space"]` |
| `pause` | `` ["`"] `` |
| `menu` | `["escape"]` |
| `sell` | `["backspace"]` |
| `upgrade_top_path` | `[","]` |
| `upgrade_middle_path` | `["."]` |
| `upgrade_bottom_path` | `["/"]` |
| `dart` `boomerang` `bomb` `tack` `ice` `glue` | tower placement keys |
| `sniper` `sub` `boat` `ace` `heli` `mortar` `dartling` | tower placement keys |
| `wizard` `super_monkey` `ninja` `alchemist` `druid` `mermonkey` | tower placement keys |
| `farm` `spike` `village` `engineer` `beast` `hero` | tower placement keys |
| `target_priority_right` | `["tab"]` |
| `target_priority_left` | `["l_control", "tab"]` |
| `target_priority_special` | `["page_down"]` |
| `send_next_round` | `["shift", "space"]` |
| `road_spikes` `moab_mine` `glue_trap` `camo_trap` `banana_farmer` | power keys |
| `tech_bot` `energizing_totem` `pontoon` `portable_lake` | power keys |
| `super_monkey_storm` `monkey_boost` `thrive` `time_stop` `cash_drop` | power keys |
| `copy` | `["l_control", "c"]` |

**Available key name strings:**

Single characters (any single letter or digit) can be written directly: `"q"`, `"1"`, `"/"`.

Named keys:

| String | Key |
| --- | --- |
| `"space"` | Space |
| `"shift"` / `"l_shift"` / `"r_shift"` | Shift (left/right) |
| `"control"` / `"l_control"` / `"r_control"` | Ctrl (left/right) |
| `"alt"` | Alt |
| `"escape"` | Escape |
| `"tab"` | Tab |
| `"backspace"` | Backspace |
| `"delete"` | Delete |
| `"return"` | Enter |
| `"up_arrow"` `"down_arrow"` `"left_arrow"` `"right_arrow"` | Arrow keys |
| `"home"` `"end"` `"page_up"` `"page_down"` | Navigation keys |
| `"insert"` | Insert |
| `"f1"` ... `"f24"` | Function keys |
| `"caps_lock"` | Caps Lock |
| `"numlock"` | Num Lock |
| `"meta"` | Win / Cmd key |
| `"print"` | Print Screen |
| `"pause"` | Pause/Break |

---

### Settings.yaml (per resolution)

Located at `config/{width}x{height}/Settings.yaml`. All coordinates are **relative to the top-left corner of the game window**, not the screen.

```yaml
screen:
    x: 0
    y: 0
    w: 1920
    h: 1080
game:
    round_counter:
        x: 1561
        y: 78
        w: 223
        h: 42
    victory_banner:
        x: 700
        y: 666
        w: 515
        h: 110
    defeat_banner:
        x: 590
        y: 584
        w: 730
        h: 115
    insta_monkey_banner:
        x: 669
        y: 269
        w: 535
        h: 48
    next_button:
        x: 965
        y: 1464
    freeplay_button:
        x: 1220
        y: 629
    freeplay_ok_button:
        x: 965
        y: 1309
    restart_game_button:
        x: 1080
        y: 1390
    confirm_button:
        x: 1135
        y: 1948
    home_button:
        x: 845
        y: 1404
```

**Field reference:**

| Field | Type | Description |
| --- | --- | --- |
| `screen` | area | Full game window region (`x`/`y` are usually `0`, `w`/`h` match resolution) |
| `game.round_counter` | area | The `X/Y` round counter text in the top-right of the HUD |
| `game.victory_banner` | area | The "VICTORY" text that appears at game end |
| `game.defeat_banner` | area | The "DEFEAT" text that appears at game end |
| `game.insta_monkey_banner` | area | The "INSTA-MONKeY" unlock splash (Impoppable/CHIMPS) |
| `game.next_button` | point | "Next" button on the victory screen |
| `game.freeplay_button` | point | "Freeplay" button on the victory screen |
| `game.freeplay_ok_button` | point | "OK" button inside the freeplay confirmation dialog |
| `game.restart_game_button` | point | "Restart" button on victory or defeat screen |
| `game.confirm_button` | point | Confirmation button for the restart dialog |
| `game.home_button` | point | "Home" button on the defeat screen |

`area` fields have `x`, `y`, `w`, `h`. `point` fields have only `x`, `y`.

#### Finding coordinates with the location finder

Run the bot with the `--location` flag to launch an interactive coordinate tool instead of starting a game:

```sh
btd6_autoplay.exe --location
```

Controls while the location finder is running:

| Key | Action |
| --- | --- |
| Left-click | Print the click position relative to the game window |
| `m` | Toggle between **Single Point** mode and **Area Selection** mode |
| `p` | Pause / resume (so you can interact with the game without logging positions) |
| `q` | Quit |

In **Area Selection** mode, click the top-left corner of a region and then the bottom-right corner; the tool prints the resulting `x`, `y`, `w`, `h` values ready to paste into `Settings.yaml`.

#### Adding a new screen resolution

1. Create `config/{width}x{height}/` and `config/{width}x{height}/maps/`.
2. Run the bot with `--location` and navigate to each UI element in-game to record its coordinates.
3. Fill in `Settings.yaml` using those coordinates.
4. Copy or create map YAML files in the `maps/` subdirectory.

---

### Map config files (`config/{width}x{height}/maps/`)

One YAML file per map, placed in the resolution's `maps/` folder. The filename determines the map name shown in the selection menu — CamelCase words are split on capitals and hyphens become spaces (e.g. `DarkCastle.yaml` → "Dark Castle", `Monkey-Meadow.yaml` → "Monkey Meadow").

#### Top-level structure

```yaml
round_counter_mode: "light"   # or "dark" — see table below
easy:                         # difficulty: easy / intermediate / advanced / expert
  standard:                   # gamemode (see list below)
    money_per_game: 60
    on_win_action: "restart"  # restart / end_game / continue
    restart_on_round: 40      # optional — treat this round as a win and restart
    hover_location:           # where the bot moves the mouse while idle
      x: 960
      y: 540
    instructions:
      1:                      # round number
        - "start"
        - "place myDart dart 500 400"
      5:
        - "upgrade myDart 0-0-2"
```

**Difficulties:** `easy`, `intermediate`, `advanced`, `expert`

**Gamemodes:** `standard`, `sandbox`, `primary_only`, `deflation`, `military_only`, `apopalypse`, `reverse`, `magic_monkeys_only`, `double_hpmoa_bs`, `half_cash`, `alternate_bloons_rounds`, `impoppable`, `chimps`

Multiple difficulties and gamemodes can be defined in the same file:

```yaml
round_counter_mode: "light"
easy:
  standard:
    # ...
  chimps:
    # ...
intermediate:
  standard:
    # ...
```

#### Available actions

Actions are strings listed under each round number in `instructions`.

| Action | Syntax | Description |
| --- | --- | --- |
| `start` | `start` or `start fast-forward` | Starts/unpauses the round. Add `fast-forward` to also enable fast-forward. |
| `place` | `place <name> <type> <x> <y>` | Places a tower. `name` is any label you choose (used to reference the tower later). `type` must match a hotkey name (e.g. `dart`, `sub`, `hero`). |
| `upgrade` | `upgrade <name> <t>-<m>-<b>` | Upgrades a previously placed tower to the given path levels, e.g. `0-2-3`. |
| `sell` | `sell <name>` | Sells a previously placed tower. |
| `select` | `select <name>` | Clicks a previously placed tower to select it. |
| `ability` | `ability <n>` | Activates ability number `n` (1-based) on the currently selected tower. |
| `ability` | `ability <hotkey_name>` | Activates a power by hotkey name, e.g. `ability road_spikes`. |
| `ability` | `ability <x> <y>` | Clicks a specific screen coordinate to activate an ability. |
| `click` | `click <x> <y>` | Clicks a screen coordinate (relative to the game window). |
| `hover` | `hover <x> <y>` | Moves the mouse to a coordinate without clicking. |
| `obstacle` / `clear` | `obstacle <x> <y>` | Clicks to clear an obstacle at a coordinate. |
| `press` | `press <key> [key ...]` | Presses one or more keys in sequence (not simultaneously). |
| `sleep` | `sleep <n>[ms\|s\|m\|h\|d]` | Pauses for the given duration, e.g. `sleep 1s`, `sleep 500ms`. |
| `repeat` | `repeat <interval> [count] <action>` | Runs `action` every `interval` in a background thread. Optional `count` limits repetitions. E.g. `repeat 30s ability 1` or `repeat 10s 5 ability road_spikes`. |

#### Full annotated example

```yaml
round_counter_mode: "dark"
easy:
  standard:
    money_per_game: 60
    on_win_action: "restart"
    hover_location:
      x: 960
      y: 540
    instructions:
      1:
        - "start"
        - "place myHero hero 585 489"
        - "place myDart dart 589 611"
        - "sleep 1s"
        - "upgrade myDart 0-0-2"
      3:
        - "place mySub sub 1084 434"
      7:
        - "upgrade mySub 2-0-0"
      14:
        - "upgrade mySub 2-0-2"
      18:
        - "upgrade mySub 2-0-3"
        - "repeat 30s ability 1"   # activate hero ability every 30 s for the rest of the game
      28:
        - "upgrade mySub 2-0-4"
      34:
        - "upgrade myDart 0-2-2"
      38:
        - "upgrade myDart 0-2-4"
```

---

## Recommended settings for roundCounterMode per map

If `roundCounterMode` is not set, the default value is `light`.

Majority of the maps have a light background, but some have a dark background. The following is a list of maps and their recommended settings for `roundCounterMode`:

- Easy

  ```yaml
    - Monkey Meadow:      light
    - In The Loop:        light
    - Middle of the Road: light
    - Tree Stump:         light
    - Town Center:        light
    - One Two Tree:       light
    - Scrapyard:          light
    - The Cabin:          dark
    - Resort:             light
    - Skates:             light
    - Lotus Island:       light
    - Candy Falls:        light
    - Winter Park:        light
    - Carved:             dark
    - Park Path:          light
    - Alpine Run:         light
    - Frozen Over:        light
    - Cubism:             light
    - Four Circles:       light
    - Hedge:              ! Has issues on both !
    - End of the Road:    light
    - Logs:               dark
  ```

- Intermediate

  ```yaml
  - Sulfur Springs:   light
  - Water Park:
  - Polyphemus:       light
  - Covered Garden:
  - Quarry:           light
  - Quiet Street:     light
  - Bloonarius Prime: dark
  - Balance:          light
  - Encrypted:        dark
  - Bazaar:           light
  - Adora's Temple:   light
  - Spring Spring:    light
  - KartsNDarts:      light
  - Moon Landing:     light
  - Haunted:          dark
  - Downstream:       light
  - Firing Range:     light
  - Cracked:          light
  - Streambed:        light
  - Chutes:           light
  - Rake:             light
  - Spice Islands:    light
  ```

- Advanced

  ```yaml
  - Castle Revenge:   dark
  - Dark Path:
  - Erosion:          light
  - Midnight Mansion: light
  - Sunken Columns:   dark
  - X Factor:         light
  - Mesa:             light
  - Geared:           light
  - Spillway:         light
  - Cargo:            light
  - Pat's Pond:       light
  - Peninsula:        light
  - High Finance:     light
  - Another Brick:    light
  - Off The Coast:    light
  - Cornfield:        light
  - Underground:      dark
  ```

- Expert

  ```yaml
  - Glacial Trail:
  - Dark Dungeons:
  - Sanctuary:
  - Ravine:
  - Flooded Valley: light
  - Infernal:       light
  - Bloody Puddles: dark
  - Workshop:
  - Quad:           light
  - Dark Castle:    dark
  - Muddy Puddles:  light
  - "#Ouch":
  ```
