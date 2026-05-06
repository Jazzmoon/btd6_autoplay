use std::{collections::HashMap, path::PathBuf, thread::sleep, time::Duration};

use clap::Parser;
use enigo::Settings as EnigoSettings;
use fragile::Fragile;
use inquire::Select;
use rusty_tesseract::{image_to_string, Args as RTArgs};
use xcap::Window;

use btd6_autoplay::{
    models::{
        action_parser,
        map::{OnWinAction, RoundCounterMode},
    },
    utils::{
        global::{
            CONFIG_SCREEN_PATH, CURRENT_MAP, CURRENT_WINDOW, ENIGO_SETTINGS, GAME_ACTIVE,
            GENERAL_CONFIG, HOTKEYS, MAP_CONFIG, REPEAT_THREAD,
        },
        interaction,
        location_finder::location_finder,
        logger::{LogLevel, Logger},
        parsing::{
            load_general_config, load_hotkeys, load_map, load_map_config, load_settings,
            map_config_name_to_map_name,
        },
        screenshot::{
            capture_area, capture_screenshot, convert_to_rusty_image, ImageProcessingType,
        },
    },
};

#[derive(Parser, Debug)]
#[clap(version = "3.0.0")]
#[clap(about = "An autoplay bot for Bloons Tower Defense 6")]
struct Args {
    /// The filename of the map to play, including the extension
    #[clap(short, long)]
    map: Option<String>,

    /// The difficulty of the map
    #[clap(short, long)]
    difficulty: Option<String>,

    /// The gamemode to play
    #[clap(short, long)]
    gamemode: Option<String>,

    /// The number of games that the bot will play before exiting
    #[clap(short, long, default_value_t = -1)]
    number: i32,

    /// The number of seconds to sleep before activating the bot
    #[clap(short, long, default_value_t = 5)]
    sleep: u64,

    /// Whether the app loads the bot, or the location finder utility
    #[clap(long = "location")]
    location_finder: bool,

    /// Log level (debug, info, notice, warn, error, crit, alert, emerg)
    #[clap(long = "log-level", default_value = "warn")]
    log_level: String,
}

/// Navigate the post-game UI and restart the current map from round 1.
///
/// Flow (mirrors the Python `Game.restart_game`):
///   next → decline freeplay → Escape × 2 → restart → confirm
///
/// Also clears all placed tower state from `CURRENT_MAP` so the next
/// game starts with a clean slate.
fn restart_game(settings: &btd6_autoplay::models::settings::Settings) {
    let menu_hotkey = {
        let hotkeys_read_lock = HOTKEYS.read().unwrap();
        hotkeys_read_lock.as_ref().unwrap().menu.clone()
    };

    // Click the "Next" button on the victory/defeat screen
    let _ = interaction::click(settings.game.next_button.clone(), Some(2000));
    // Decline the freeplay offer
    let _ = interaction::click(settings.game.freeplay_button.clone(), Some(2000));
    // Open the in-game menu (press Escape twice to get to the restart option)
    let _ = interaction::press_key(menu_hotkey.clone(), Some(1000));
    let _ = interaction::press_key(menu_hotkey, Some(1000));
    // Click "Restart" and then confirm
    let _ = interaction::click(settings.game.restart_game_button.clone(), Some(1000));
    let _ = interaction::click(settings.game.confirm_button.clone(), Some(1000));

    // Clear placed towers so the next game starts fresh
    {
        let mut current_map_write_lock = CURRENT_MAP.write().unwrap();
        if let Some(current_map) = current_map_write_lock.as_mut() {
            current_map.towers.clear();
        }
    }
}

/// Search all open windows for one whose app name or title contains any of the given search terms.
/// Returns the matched `Window` and a list of all other window names that were seen (for debugging).
fn find_game_window(search_terms: &[String]) -> (Option<Window>, Vec<String>) {
    let mut found = None;
    let mut seen = Vec::new();
    for window in Window::all().unwrap_or_default() {
        let name = window.app_name().or_else(|_| window.title());
        match name {
            Ok(n) if search_terms.iter().any(|t| n.to_lowercase().contains(t)) => {
                found = Some(window);
                break;
            }
            Ok(n) => seen.push(n),
            Err(_) => seen.push("Unknown Window".to_string()),
        }
    }
    (found, seen)
}

/// Signal the infinite-repeat background thread (if any) to stop and wait for it to finish.
///
/// This should be called at every game-end point (victory, defeat, or early restart) so that
/// the thread is cleanly joined before the next game begins.
fn stop_repeat_thread() {
    use std::sync::atomic::Ordering;
    GAME_ACTIVE.store(false, Ordering::SeqCst);
    if let Some(handle) = REPEAT_THREAD.lock().unwrap().take() {
        if let Err(e) = handle.join() {
            Logger::error(format!("Repeat thread panicked during join: {:?}", e));
        }
    }
}

fn main() {
    let args = Args::parse();

    let log_level = args.log_level.parse::<LogLevel>().unwrap_or_else(|_| {
        Logger::warn(format!(
            "Unknown log level '{}', defaulting to warn.",
            args.log_level
        ));
        LogLevel::Warn
    });
    Logger::set_level(log_level);

    let _ = load_general_config();

    {
        let mut enigo_settings_write_lock = ENIGO_SETTINGS.write().unwrap();
        *enigo_settings_write_lock = Some(EnigoSettings {
            linux_delay: 10,
            // mac_delay: 10,
            ..Default::default()
        });
    }

    // Sleep for 5 seconds to allow the user to switch to the BloonsTD6.exe window
    Logger::notice(format!(
        "Please make sure the BloonsTD6.exe window is in focus within the next {:?} seconds.",
        args.sleep
    ));
    sleep(Duration::from_secs(args.sleep));

    let search_terms = {
        let general_config_read_lock = GENERAL_CONFIG.read().unwrap();
        general_config_read_lock
            .as_ref()
            .unwrap()
            .window_title_search_terms
            .clone()
    };
    let (bloons_td6_window, seen_windows) = find_game_window(&search_terms);

    if bloons_td6_window.is_none() {
        Logger::debug(format!("Available windows:\n{}", seen_windows.join("\n")));
        Logger::error("The BloonsTD6 window was not found. Common causes on Linux: running under Wayland (screenshots may not be supported), or the process/window name is different when using Proton/Wine/Steam. Try running your game in an X11 session (or under XWayland) and ensure the window is focused. For a quick workaround you can edit the source to match the actual app name/title shown in the debug output.");
        panic!("The BloonsTD6 window was not found. Common causes on Linux: running under Wayland (screenshots may not be supported), or the process/window name is different when using Proton/Wine/Steam. Try running your game in an X11 session (or under XWayland) and ensure the window is focused. For a quick workaround you can edit the source to match the actual app name/title shown in the debug output.");
    }
    let window = bloons_td6_window.as_ref().unwrap();
    {
        let mut current_window_write_lock = CURRENT_WINDOW.write().unwrap();
        *current_window_write_lock = Some(Fragile::new(window.clone()));
    }
    if window.is_minimized().unwrap_or(false) {
        panic!("The BloonsTD6.exe window is minimized. Please open the game and try again.");
    }

    let (window_x, window_y, window_width, window_height) = (
        window.x().expect("Game window must exist to continue"),
        window.y().expect("Game window must exist to continue"),
        window.width().expect("Game window must exist to continue"),
        window.height().expect("Game window must exist to continue"),
    );

    Logger::info(format!(
        "Detected window geometry: x={}, y={}, width={}, height={}",
        window_x, window_y, window_width, window_height
    ));

    {
        let mut config_path_write_lock = CONFIG_SCREEN_PATH.write().unwrap();
        *config_path_write_lock = Some(
            PathBuf::from("./config/").join(format!("{}x{}", window_width, window_height).as_str()),
        );
        if !config_path_write_lock.as_ref().unwrap().exists() {
            panic!("The \"{}\" directory does not exist. Please create it and add the necessary settings.", config_path_write_lock.as_ref().unwrap().to_str().unwrap());
        }
        Logger::info(format!(
            "Using config path of {:?}",
            config_path_write_lock.as_ref().unwrap()
        ));
    }

    if args.location_finder {
        location_finder(window_x, window_y);
        return;
    }

    // Load the config/Settings.{width}x{height}.yaml file into the SETTINGS global variable using the windows width and height
    let settings = load_settings();

    // Load the config/Hotkeys.yaml file into the HOTKEYS global variable
    let _ = load_hotkeys();

    // Start validation of arguments
    // Load the maps directory into a hashmap
    let maps_path: PathBuf;
    {
        let config_path_read_lock = CONFIG_SCREEN_PATH.read().unwrap();
        maps_path = config_path_read_lock.as_ref().unwrap().join("maps");
    }
    let maps: HashMap<String, String> = maps_path
        .read_dir()
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let map_name = map_config_name_to_map_name(&file_name);
            (map_name, file_name)
        })
        .collect();

    // If the user did provide a map, we check it is a legal map within the maps directory
    let mut map_file_name = args.map.clone();
    match map_file_name {
        Some(ref map_name) => {
            if Logger::is_enabled(LogLevel::Debug) {
                // List arg and all legal map name options
                let items = maps.iter().collect::<Vec<(&String, &String)>>();
                Logger::debug(format!("Map name: {}", map_name));
                Logger::debug("Legal map name options:");
                for (key, value) in &items {
                    Logger::debug(format!("- Key: {} | Value: {}", key, value));
                }
            }
            if !maps.contains_key(map_name) {
                panic!("The map provided is not a legal map within the maps directory.");
            }
            map_file_name = Some(maps.get(map_name).unwrap().clone());
        }
        None => {
            let mut options = maps.keys().collect::<Vec<&String>>();
            options.sort();
            let map_selection = Select::new("Please select a map:", options).prompt();
            match map_selection {
                Ok(map_selection) => match maps.get(map_selection) {
                    Some(file_name) => map_file_name = Some(file_name.clone()),
                    None => panic!("You did not select a valid map."),
                },
                Err(_) => panic!("You did not select a valid map."),
            }
        }
    }

    // Read the map file into the MAP_CONFIG global variable
    load_map_config(map_file_name.clone());

    // Next, given we have a map, we need to check if the user provided a difficulty
    // Load the difficulties from the map config into a hashmap
    let map_config_read_lock = MAP_CONFIG.read().unwrap();
    let map_config = map_config_read_lock.as_ref().unwrap();

    let mut options = map_config.implemented_difficulties();
    options.sort();

    let mut difficulty = args.difficulty.clone();

    match difficulty {
        Some(ref difficulty) => {
            if !options.contains(difficulty) {
                panic!("The difficulty provided is not a legal difficulty for the map provided.");
            }
        }
        None => {
            if options.is_empty() {
                panic!("There are no difficulties available for the map provided.");
            } else if options.len() == 1 {
                difficulty = Some(options[0].clone());
                Logger::info(format!(
                    "Only one difficulty available, selecting \"{}\"",
                    difficulty.clone().unwrap()
                ));
            } else {
                let selection = Select::new("Please select a difficulty:", options).prompt();
                match selection {
                    Ok(selection) => difficulty = Some(selection),
                    Err(_) => panic!("You did not select a valid difficulty."),
                }
            }
        }
    }

    let mut options = map_config
        .get(difficulty.clone().unwrap().as_ref())
        .unwrap()
        .implemented_gamemodes();
    options.sort();

    let mut gamemode = args.gamemode.clone();

    match gamemode {
        Some(ref gamemode) => {
            if !options.contains(gamemode) {
                panic!("The gamemode provided is not a legal gamemode for the map and difficulty provided.");
            }
        }
        None => {
            if options.is_empty() {
                panic!("There are no gamemodes available for the map and difficulty provided.");
            } else if options.len() == 1 {
                gamemode = Some(options[0].clone());
                Logger::info(format!(
                    "Only one gamemode available, selecting \"{}\"",
                    gamemode.clone().unwrap()
                ));
            } else {
                let selection = Select::new("Please select a gamemode:", options).prompt();
                match selection {
                    Ok(selection) => gamemode = Some(selection),
                    Err(_) => panic!("You did not select a valid gamemode."),
                }
            }
        }
    }

    // Set the value of CURRENT_MAP to the map, difficulty, and gamemode selected
    load_map(map_config, difficulty.clone(), gamemode.clone());

    // Wait for the user to switch to the BloonsTD6.exe window
    Logger::notice(format!(
        "Please make sure the BloonsTD6.exe window is in focus within the next {:?} seconds.",
        args.sleep
    ));
    sleep(Duration::from_secs(args.sleep));

    // Read the current map and print it
    // let current_map_read_lock = lib::global::CURRENT_MAP.read().unwrap();
    // let current_map = current_map_read_lock.as_ref().unwrap();

    // Load tesseract
    let tessdata_dir = std::env::current_dir().unwrap().join("tessdata");
    let btd6_tessdata_path = tessdata_dir.join("btd6.traineddata");
    if Logger::is_enabled(LogLevel::Debug) {
        // Print the tessdata_dir and btd6_tessdata_path
        Logger::debug(format!("tessdata_dir: {}", tessdata_dir.to_string_lossy()));
        Logger::debug(format!(
            "btd6_tessdata_path: {}",
            btd6_tessdata_path.to_string_lossy()
        ));
    }
    let lang = if btd6_tessdata_path.exists() {
        std::env::set_var("TESSDATA_PREFIX", &tessdata_dir);
        if Logger::is_enabled(LogLevel::Debug) {
            Logger::debug(format!(
                "TESSDATA_PREFIX set to: {}",
                tessdata_dir.to_string_lossy()
            ));
            Logger::debug("Using tesseract data language: btd6+eng");
        }
        "btd6+eng"
    } else {
        std::env::remove_var("TESSDATA_PREFIX");
        if Logger::is_enabled(LogLevel::Debug) {
            Logger::debug("TESSDATA_PREFIX removed");
            Logger::debug("Using tesseract data language: eng");
        }
        "eng"
    };

    let (rt_round_args, rt_victory_args, rt_defeat_args) = (
        RTArgs {
            lang: lang.into(),
            config_variables: HashMap::from([(
                "tessedit_char_whitelist".into(),
                "0123456789/".into(),
            )]),
            dpi: Some(150),
            psm: Some(6),
            oem: Some(3),
        },
        RTArgs {
            lang: lang.into(),
            config_variables: HashMap::from([("tessedit_char_whitelist".into(), "VICTORY".into())]),
            dpi: Some(150),
            psm: Some(6),
            oem: Some(3),
        },
        RTArgs {
            lang: lang.into(),
            config_variables: HashMap::from([("tessedit_char_whitelist".into(), "DeFeAT".into())]),
            dpi: Some(300),
            psm: Some(6),
            oem: Some(3),
        },
    );

    let mut threshold_value = 1;
    {
        let map_config_read_lock = MAP_CONFIG.read().unwrap();
        let map_config = map_config_read_lock.as_ref().unwrap();
        Logger::debug(format!(
            "Round counter mode: {:?}",
            map_config.round_counter_mode
        ));
        if map_config.round_counter_mode.eq(&RoundCounterMode::Dark) {
            threshold_value = 128;
        }
    }
    Logger::debug(format!(
        "Round counter threshold value set to {}",
        threshold_value
    ));

    let mut last_seen_round = 0;
    let mut round_unchanged_count: u32 = 0;
    let (mut game_wins, mut game_losses) = (0, 0);

    while args.number == -1 || game_wins + game_losses < args.number {
        // Get screenshot of the game window
        let screenshot = capture_screenshot();

        // Do round counter processing
        let processing_actions = ImageProcessingType::Resize
            | ImageProcessingType::Grayscale
            | ImageProcessingType::SkewVertical
            | ImageProcessingType::Invert;

        let round_counter = capture_area(
            screenshot.clone(),
            settings.game.round_counter.clone(),
            processing_actions,
            Some(threshold_value),
            Some(0.5),
            "round_counter".to_string(),
        );

        if Logger::is_enabled(LogLevel::Debug) {
            let _ = round_counter.save("debug/round_counter.png");
        }

        // Get the round counter from the screenshot
        let output = image_to_string(&convert_to_rusty_image(round_counter), &rt_round_args)
            .unwrap()
            .trim()
            .to_string();

        if Logger::is_enabled(LogLevel::Debug) {
            Logger::debug(format!("Output: '{}'", output));
        }

        // Parse the round counter output into (current_round, total_rounds)
        let parsed_round: Option<(i32, i32)> = if output.is_empty() {
            None
        } else if output.contains('/') {
            let parts: Vec<&str> = output.split('/').collect();
            match (
                parts[0].trim().parse::<i32>(),
                parts[1].trim().parse::<i32>(),
            ) {
                (Ok(cur), Ok(tot)) => Some((cur, tot)),
                (Ok(cur), Err(_)) => Some((cur, -1)),
                _ => None,
            }
        } else if let Ok(cr) = output.trim().parse::<i32>() {
            Some((cr, -1))
        } else {
            None
        };

        if let Some((current_round, total_rounds)) = parsed_round {
            if current_round != last_seen_round {
                round_unchanged_count = 0;
                let previous_round = last_seen_round;
                last_seen_round = current_round;

                if total_rounds == -1 {
                    Logger::info(format!("The current round is: {}", current_round));
                } else {
                    Logger::info(format!(
                        "The current round is: {}/{}",
                        current_round, total_rounds
                    ));
                }

                // Extract everything we need from CURRENT_MAP in a single read-lock scope,
                // before executing actions (which may themselves acquire a write lock).
                let (restart_on_round, on_win_action, round_actions) = {
                    let current_map_read_lock = CURRENT_MAP.read().unwrap();
                    let current_map = current_map_read_lock.as_ref().unwrap();
                    // Get all missed instructions in range (last_seen_round + 1, current_round)
                    let actions = current_map
                        .instructions
                        .iter()
                        .filter(|&(round, _)| *round > previous_round && *round <= current_round)
                        .flat_map(|(_, actions)| actions.iter())
                        .cloned()
                        .collect::<Vec<_>>();
                    Logger::debug(format!(
                        "Executing {} actions for rounds {} to {}...",
                        actions.len(),
                        previous_round + 1,
                        current_round
                    ));
                    (
                        current_map.restart_on_round,
                        current_map.on_win_action.clone(),
                        actions,
                    )
                };

                if round_actions.is_empty() {
                    Logger::debug(format!(
                        "No actions found for rounds {}-{}",
                        previous_round + 1,
                        current_round
                    ));
                }

                for action_str in &round_actions {
                    Logger::info(format!("Executing action: {}", action_str));
                    match action_parser::parse_action(action_str) {
                        Ok(action) => match action.run() {
                            Ok(_) => {
                                Logger::info(format!("Successfully ran action: {:?}", action_str))
                            }
                            Err(e) => Logger::error(format!("Failed to run action: {:?}", e)),
                        },
                        Err(e) => Logger::error(format!(
                            "Action Parser failed to parse an action: {:?}",
                            e
                        )),
                    }
                }

                // If this round matches restart_on_round, trigger the restart sequence now
                // (all towers have been placed; let the configured action decide what happens next)
                if let Some(target_round) = restart_on_round {
                    if current_round == target_round {
                        Logger::notice(format!(
                            "Reached restart_on_round ({}), triggering restart.",
                            target_round
                        ));
                        match on_win_action {
                            OnWinAction::Restart => {
                                game_wins += 1;
                                stop_repeat_thread();
                                restart_game(&settings);
                                last_seen_round = 0;
                                round_unchanged_count = 0;
                            }
                            OnWinAction::EndGame => {
                                Logger::notice(format!(
                                    "EndGame action - exiting after {} win(s) and {} loss(es).",
                                    game_wins, game_losses
                                ));
                                stop_repeat_thread();
                                return;
                            }
                            OnWinAction::Continue => {
                                Logger::info(
                                    "Continuing in freeplay after reaching restart_on_round.",
                                );
                            }
                        }
                    }
                }
            } else {
                round_unchanged_count += 1;
                Logger::debug(format!(
                    "Round unchanged at {} (count={})",
                    current_round, round_unchanged_count
                ));
            }
        } else {
            // Unparseable output counts as unchanged
            round_unchanged_count += 1;
            Logger::debug(format!(
                "Unparseable round counter output '{}' (count={})",
                output, round_unchanged_count
            ));
        }

        // After 5 iterations with no round change, check for victory/defeat
        if round_unchanged_count >= 5 {
            Logger::debug("Round unchanged for 5 iterations; checking victory/defeat");
            round_unchanged_count = 0;

            let processing_actions = ImageProcessingType::Resize
                | ImageProcessingType::Grayscale
                | ImageProcessingType::Invert;

            let (victory_image, defeat_image) = (
                capture_area(
                    screenshot.clone(),
                    settings.game.victory_banner.clone(),
                    processing_actions,
                    None,
                    None,
                    "victory_banner".to_string(),
                ),
                capture_area(
                    screenshot.clone(),
                    settings.game.defeat_banner.clone(),
                    processing_actions,
                    Some(200),
                    Some(0.9),
                    "defeat_banner".to_string(),
                ),
            );

            let (victory_banner, defeat_banner) = (
                image_to_string(&convert_to_rusty_image(victory_image), &rt_victory_args),
                image_to_string(&convert_to_rusty_image(defeat_image), &rt_defeat_args),
            );

            // If debug is enabled, print the OCR outputs for victory and defeat banners
            if Logger::is_enabled(LogLevel::Debug) {
                Logger::debug(format!(
                    "Victory banner OCR output: '{}'",
                    victory_banner.as_ref().unwrap_or(&"".to_string())
                ));
                Logger::debug(format!(
                    "Defeat banner OCR output: '{}'",
                    defeat_banner.as_ref().unwrap_or(&"".to_string())
                ));
            }

            //* NOTE: DEFEAT doesn't see the middle `e` character for some reason. It sees DEFAT...
            let did_win: Option<bool> = match (victory_banner, defeat_banner) {
                (Ok(v), Ok(d)) => {
                    let victory = v.trim().to_uppercase();
                    let defeat = d.trim().to_uppercase();
                    if victory.contains("VICTORY") {
                        Some(true)
                    } else if defeat.starts_with("DE") && defeat.ends_with("AT") {
                        Some(false)
                    } else {
                        None
                    }
                }
                (Ok(v), Err(_)) => {
                    let victory = v.trim().to_uppercase();
                    if victory.contains("VICTORY") {
                        Some(true)
                    } else {
                        None
                    }
                }
                (Err(_), Ok(d)) => {
                    let defeat = d.trim().to_uppercase();
                    if defeat.starts_with("DE") && defeat.ends_with("AT") {
                        Some(false)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if did_win.is_none() {
                Logger::debug("Victory/defeat OCR inconclusive");
            }

            if let Some(won) = did_win {
                if won {
                    Logger::notice("The game has ended. The player has won.");
                    game_wins += 1;
                } else {
                    Logger::warn("The game has ended. The player has lost.");
                    game_losses += 1;
                }

                let on_win_action = {
                    let current_map_read_lock = CURRENT_MAP.read().unwrap();
                    current_map_read_lock
                        .as_ref()
                        .unwrap()
                        .on_win_action
                        .clone()
                };

                match on_win_action {
                    OnWinAction::Restart => {
                        Logger::notice("Restarting game...");
                        stop_repeat_thread();
                        restart_game(&settings);
                        last_seen_round = 0;
                    }
                    OnWinAction::EndGame => {
                        Logger::notice(format!(
                            "EndGame action - exiting after {} win(s) and {} loss(es).",
                            game_wins, game_losses
                        ));
                        stop_repeat_thread();
                        return;
                    }
                    OnWinAction::Continue => {
                        Logger::info("Continuing in freeplay...");
                    }
                }
                continue;
            }
            // No banner detected — round_unchanged_count already reset to 0, wait another 5 iterations
        }

        sleep(Duration::from_secs(1));
    }
}
