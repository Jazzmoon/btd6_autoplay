use std::{
    collections::HashMap,
    path::PathBuf,
    sync::atomic::Ordering,
    thread::sleep,
    time::Duration
};

use clap::Parser;
use enigo::Settings as EnigoSettings;
use fragile::Fragile;
use inquire::Select;
use rusty_tesseract::{image_to_string, Args as RTArgs};
use xcap::Window;

use btd6_autoplay::{
    models::{action_parser, map::{OnWinAction, RoundCounterMode}},
    utils::{
        global::{CONFIG_PATH, CURRENT_MAP, CURRENT_WINDOW, DEBUG, ENIGO_SETTINGS, HOTKEYS, MAP_CONFIG},
        interaction,
        location_finder::location_finder,
        parsing::{
            load_hotkeys, load_map, load_map_config, load_settings, map_config_name_to_map_name,
        },
        screenshot::{capture_area, capture_screenshot, convert_to_rusty_image, ImageProcessingType},
    }
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

    /// Debug mode :3
    #[clap(long)]
    debug: bool,
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

fn main() {
    let args = Args::parse();

    if args.debug {
        DEBUG.store(true, Ordering::SeqCst);
    }

    {
        let mut enigo_settings_write_lock = ENIGO_SETTINGS.write().unwrap();
        *enigo_settings_write_lock = Some(EnigoSettings {
            linux_delay: 10,
            mac_delay: 10,
            ..Default::default()
        });
    }

    // Sleep for 5 seconds to allow the user to switch to the BloonsTD6.exe window
    println!(
        "Please make sure the BloonsTD6.exe window is in focus within the next {:?} seconds.",
        args.sleep
    );
    sleep(Duration::from_secs(args.sleep));

    let mut bloons_td6_window: Option<Window> = None;
    let mut seen_windows: Vec<String> = Vec::new();
    for window in Window::all().unwrap() {
        let app_name = window.app_name().to_lowercase();
        let title = window.title().to_lowercase();
        seen_windows.push(format!("app_name='{}', title='{}'", app_name, title));

        if app_name.contains("bloons") || app_name.contains("bloonstd6") || title.contains("bloons") || title.contains("bloonstd6") {
            bloons_td6_window = Some(window.clone());
            let fragile_window = Fragile::new(window);
            let mut current_window_write_lock = CURRENT_WINDOW.write().unwrap();
            *current_window_write_lock = Some(fragile_window);
            break;
        }
    }

    if bloons_td6_window.is_none() {
        // If we didn't find an obvious match, print what we saw to help debugging.
        if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            println!("Available windows:\n{}", seen_windows.join("\n"));
        }
        panic!("The BloonsTD6 window was not found. Common causes on Linux: running under Wayland (screenshots may not be supported), or the process/window name is different when using Proton/Wine/Steam. Try running your game in an X11 session (or under XWayland) and ensure the window is focused. For a quick workaround you can edit the source to match the actual app name/title shown in the debug output.");
    }
    let window = bloons_td6_window.as_ref().unwrap();
    if window.is_minimized() {
        panic!("The BloonsTD6.exe window is minimized. Please open the game and try again.");
    }

    let (window_x, window_y, window_width, window_height) =
        (window.x(), window.y(), window.width(), window.height());

    println!("Detected window geometry: x={}, y={}, width={}, height={}", window_x, window_y, window_width, window_height);

    {
        let mut config_path_write_lock = CONFIG_PATH.write().unwrap();
        *config_path_write_lock = Some(
            PathBuf::from("./config/").join(format!("{}x{}", window_width, window_height).as_str()),
        );
        if !config_path_write_lock.as_ref().unwrap().exists() {
            panic!("The \"{}\" directory does not exist. Please create it and add the necessary settings.", config_path_write_lock.as_ref().unwrap().to_str().unwrap());
        }
        println!(
            "Using config path of {:?}",
            config_path_write_lock.as_ref().unwrap()
        );
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
        let config_path_read_lock = CONFIG_PATH.read().unwrap();
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
            if args.debug {
                // List arg and all legal map name options
                let items = maps.iter().collect::<Vec<(&String, &String)>>();
                println!("Map name: {}", map_name);
                println!("Legal map name options:");
                for (key, value) in &items {
                    println!("- Key: {} | Value: {}", key, value);
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
            if options.len() == 0 {
                panic!("There are no difficulties available for the map provided.");
            } else if options.len() == 1 {
                difficulty = Some(options[0].clone());
                println!(
                    "Only one difficulty available, selecting \"{}\"",
                    difficulty.clone().unwrap()
                );
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
            if options.len() == 0 {
                panic!("There are no gamemodes available for the map and difficulty provided.");
            } else if options.len() == 1 {
                gamemode = Some(options[0].clone());
                println!(
                    "Only one gamemode available, selecting \"{}\"",
                    gamemode.clone().unwrap()
                );
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
    println!(
        "Please make sure the BloonsTD6.exe window is in focus within the next {:?} seconds.",
        args.sleep
    );
    sleep(Duration::from_secs(args.sleep));

    // Read the current map and print it
    // let current_map_read_lock = lib::global::CURRENT_MAP.read().unwrap();
    // let current_map = current_map_read_lock.as_ref().unwrap();

    // Load tesseract
    let tessdata_dir = std::env::current_dir().unwrap().join("tessdata");
    let btd6_tessdata_path = tessdata_dir.join("btd6.traineddata");
    if args.debug {
        // Print the tessdata_dir and btd6_tessdata_path
        println!("tessdata_dir: {}", tessdata_dir.to_string_lossy());
        println!("btd6_tessdata_path: {}", btd6_tessdata_path.to_string_lossy());
    }
    let lang = if btd6_tessdata_path.exists() {
        std::env::set_var("TESSDATA_PREFIX", &tessdata_dir);
        if args.debug {
            println!("TESSDATA_PREFIX set to: {}", tessdata_dir.to_string_lossy());
            println!("Using tesseract data language: btd6+eng");
        }
        "btd6+eng"
    } else {
        std::env::remove_var("TESSDATA_PREFIX");
        if args.debug {
            println!("TESSDATA_PREFIX removed");
            println!("Using tesseract data language: eng");
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
        if map_config.round_counter_mode.eq(&RoundCounterMode::Dark) {
            threshold_value = 128;
        }
    }

    let mut last_seen_round = 0;
    let mut empty_round_counter_count = 0;
    let (mut game_wins, mut game_losses) = (0, 0);

    while args.number == -1 || game_wins + game_losses < args.number {
        // Get screenshot of the game window
        let screenshot = capture_screenshot(args.debug);

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
            if args.debug { Some("round_counter".to_string()) } else { None },
        );

        if args.debug {
            let _ = round_counter.save("debug/round_counter.png");
        }

        // Get the round counter from the screenshot
        let output = image_to_string(&convert_to_rusty_image(round_counter), &rt_round_args)
            .unwrap()
            .trim()
            .to_string();

        // Check if the round counter is empty -- Which will trigger our "Chick winner" check
        if output.is_empty() {
            empty_round_counter_count += 1;

            if empty_round_counter_count >= 3 {
                empty_round_counter_count = 0;

                let processing_actions = ImageProcessingType::Resize
                    | ImageProcessingType::Grayscale
                    | ImageProcessingType::Invert;
                let did_win: bool;

                let (victory_image, defeat_image) = (
                    capture_area(
                        screenshot.clone(),
                        settings.game.victory_banner.clone(),
                        ImageProcessingType::None,
                        None,
                        None,
                        if args.debug { Some("victory_banner".to_string()) } else { None },
                    ),
                    capture_area(
                        screenshot.clone(),
                        settings.game.defeat_banner.clone(),
                        processing_actions,
                        Some(200),
                        Some(0.9),
                        if args.debug { Some("defeat_banner".to_string()) } else { None },
                    ),
                );

                let (victory_banner, defeat_banner) = (
                    image_to_string(&convert_to_rusty_image(victory_image), &rt_victory_args),
                    image_to_string(&convert_to_rusty_image(defeat_image), &rt_defeat_args),
                );

                match (victory_banner, defeat_banner) {
                    (Ok(victory_s), Ok(defeat_s)) => {
                        let victory = victory_s.trim().to_string().to_uppercase();

                        //* NOTE: DEFEAT doesn't see the middle `e` character for some reason. It sees DEFAT...
                        let defeat = defeat_s.trim().to_string().to_uppercase();

                        if (victory.is_empty() && defeat.is_empty())
                            || (!victory.contains("VICTORY")
                                && !(defeat.starts_with("DE") && defeat.ends_with("AT")))
                        {
                            sleep(Duration::from_secs(1));
                            continue;
                        }
                        did_win = victory.contains("VICTORY");
                    }
                    (Ok(victory_s), Err(_)) => {
                        let victory = victory_s.trim().to_string().to_uppercase();
                        if victory.is_empty() || !victory.contains("VICTORY") {
                            sleep(Duration::from_secs(1));
                            continue;
                        }
                        did_win = true;
                    }
                    (Err(_), Ok(defeat_s)) => {
                        //* NOTE: DEFEAT doesn't see the middle `e` character for some reason. It sees DEFAT...
                        let defeat = defeat_s.trim().to_string().to_uppercase();

                        if defeat.is_empty()
                            || !(defeat.starts_with("DE") && defeat.ends_with("AT"))
                        {
                            sleep(Duration::from_secs(1));
                            continue;
                        }
                        did_win = false;
                    }
                    _ => {
                        sleep(Duration::from_secs(1));
                        continue;
                    }
                }

                if did_win {
                    println!("The game has ended. The player has won.");
                    game_wins += 1;
                } else {
                    println!("The game has ended. The player has lost.");
                    game_losses += 1;
                }

                let on_win_action = {
                    let current_map_read_lock = CURRENT_MAP.read().unwrap();
                    current_map_read_lock.as_ref().unwrap().on_win_action.clone()
                };

                match on_win_action {
                    OnWinAction::Restart => {
                        println!("Restarting game...");
                        restart_game(&settings);
                        last_seen_round = 0;
                        empty_round_counter_count = 0;
                    }
                    OnWinAction::EndGame => {
                        println!("EndGame action - exiting after {} win(s) and {} loss(es).", game_wins, game_losses);
                        return;
                    }
                    OnWinAction::Continue => {
                        // Stay in freeplay / do nothing – the outer loop will keep running
                        println!("Continuing in freeplay...");
                    }
                }
                continue;
            }
        }

        if args.debug {
            println!("Output: '{}'", output);
        }

        let current_round: i32;
        let total_rounds: i32;

        // Check if the round counter has a '/' in it
        if output.contains('/') {
            // Split the round counter into the current round and the total rounds by the '/'
            let round_counter: Vec<&str> = output.split('/').collect();
            match (
                round_counter[0].trim().parse::<i32>(),
                round_counter[1].trim().parse::<i32>(),
            ) {
                (Ok(current), Ok(total)) => {
                    current_round = current;
                    total_rounds = total;
                }
                (Ok(current), Err(_)) => {
                    current_round = current;
                    total_rounds = -1;
                }
                _ => {
                    sleep(Duration::from_secs(1));
                    continue;
                }
            }
        } else if let Ok(cr) = output.trim().parse::<i32>() {
            // If the round counter does not have a '/', then the current round is the outpu
            current_round = cr;
            total_rounds = -1;
        } else {
            sleep(Duration::from_secs(1));
            continue;
        }

        // If the round counter has changed, update the last seen round
        if current_round != last_seen_round {
            last_seen_round = current_round;

            if total_rounds == -1 {
                println!("The current round is: {}", current_round);
            } else {
                println!("The current round is: {}/{}", current_round, total_rounds);
            }

            // Extract everything we need from CURRENT_MAP in a single read-lock scope,
            // before executing actions (which may themselves acquire a write lock).
            let (restart_on_round, on_win_action, round_actions) = {
                let current_map_read_lock = CURRENT_MAP.read().unwrap();
                let current_map = current_map_read_lock.as_ref().unwrap();
                let actions = current_map
                    .instructions
                    .get(&current_round)
                    .cloned()
                    .unwrap_or_default();
                (current_map.restart_on_round, current_map.on_win_action.clone(), actions)
            };

            for action_str in &round_actions {
                match action_parser::parse_action(action_str) {
                    Ok(action) => {
                        match action.run() {
                            Ok(_) => {
                                println!("Successfully ran action: {:?}", action_str);
                            }
                            Err(e) => {
                                println!("Failed to run action: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Action Parser failed to parse an action: {:?}", e);
                    }
                }
            }

            // If this round matches restart_on_round, trigger the restart sequence now
            // (all towers have been placed; let the configured action decide what happens next)
            if let Some(target_round) = restart_on_round {
                if current_round == target_round {
                    println!("Reached restart_on_round ({}), triggering restart.", target_round);
                    match on_win_action {
                        OnWinAction::Restart => {
                            game_wins += 1;
                            restart_game(&settings);
                            last_seen_round = 0;
                            empty_round_counter_count = 0;
                        }
                        OnWinAction::EndGame => {
                            println!("EndGame action - exiting after {} win(s) and {} loss(es).", game_wins, game_losses);
                            return;
                        }
                        OnWinAction::Continue => {
                            // Nothing to do – let the game run in freeplay
                            println!("Continuing in freeplay after reaching restart_on_round.");
                        }
                    }
                }
            }
        }

        sleep(Duration::from_secs(1));
    }
}
