mod utils {
    pub mod screenshot;
}
mod lib {
    pub mod global;
}
mod models {
    pub mod coords;
    pub mod hotkeys;
    pub mod map;
    pub mod settings;
    pub mod tower;
}

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use device_query::{DeviceEvents, DeviceState, Keycode};
use enigo::{Enigo, Mouse, Settings as EnigoSettings};
use fragile::Fragile;
use inquire::Select;
use rusty_tesseract::Args as RTArgs;
use xcap::{Window/*, Monitor */};

use lib::global::{CURRENT_WINDOW, CONFIG_PATH, DEBUG, ENIGO_SETTINGS, HOTKEYS, MAP_CONFIG, SETTINGS};
use models::coords::CoordsArea;
use models::hotkeys::Hotkeys;
use models::map::{MapConfig, RoundCounterMode};
use models::settings::Settings;
use utils::screenshot::{capture_area, convert_to_rusty_image, ImageProcessingType};

#[derive(Debug, Clone, Copy)]
enum LocationFinderMode {
    SinglePoint,
    AreaSelection,
}

fn location_finder(window_x: i32, window_y: i32) {
    // This is a tool that acts as an infinite loop that prints the current mouse position when you left click
    // When you press the letter 'q', the program will exit
    // Pressing 'p' will pause the program so you can interact with your system without logging mouse positions
    // Pressing 'p' again will resume the program
    // Pressing 'm' changes the mode from single click to area selection
    // Print help dialog for the functionality of the program
    println!("Welcome to the location finder utility!");
    println!("Press 'q' to exit the program");
    println!("Press 'p' to pause/resume the program");
    println!("Press 'm' to change the mode from single point to area selection mode\n");

    let paused = Arc::new(AtomicBool::new(false));
    let mode = Arc::new(Mutex::new(LocationFinderMode::SinglePoint));
    let area_mode_first_coordinate: Arc<Mutex<Option<(i32, i32)>>> = Arc::new(Mutex::new(None));
    let device_state = DeviceState::new();

    let paused_clone = Arc::clone(&paused);
    let mode_clone = Arc::clone(&mode);
    let _guard = device_state.on_key_up(move |key| {
        if key.eq(&Keycode::Q) {
            println!("Exiting program");
            std::process::exit(0);
        } else if key.eq(&Keycode::P) {
            let paused = paused_clone.load(Ordering::SeqCst);
            paused_clone.store(!paused, Ordering::SeqCst);
            if !paused {
                println!("Program paused");
            } else {
                println!("Program resumed");
            }
        } else if key.eq(&Keycode::M) {
            let paused = paused_clone.load(Ordering::SeqCst);
            if !paused {
                let mut mode_lock = mode_clone.lock().unwrap();
                *mode_lock = match *mode_lock {
                    LocationFinderMode::SinglePoint => LocationFinderMode::AreaSelection,
                    LocationFinderMode::AreaSelection => LocationFinderMode::SinglePoint,
                };
                println!("Mode changed to {:?}", *mode_lock);
            }
        }
    });

    let paused_clone = Arc::clone(&paused);
    let mode_clone = Arc::clone(&mode);
    let area_mode_first_coordinate_clone = Arc::clone(&area_mode_first_coordinate);
    let _guard = device_state.on_mouse_up(move |button| {
        if *button != 1 {
            return;
        }
        if paused_clone.load(Ordering::SeqCst) {
            return;
        }
        let enigo_settings_lock = ENIGO_SETTINGS.read().unwrap();
        let coords = match enigo_settings_lock.as_ref() {
            Some(enigo_settings) => Enigo::new(enigo_settings)
                .unwrap()
                .location()
                .unwrap_or((0, 0)),
            None => {
                let default_settings = EnigoSettings::default();
                Enigo::new(&default_settings)
                    .unwrap()
                    .location()
                    .unwrap_or((0, 0))
            }
        };
        let mode_lock = mode_clone.lock().unwrap();
        match *mode_lock {
            LocationFinderMode::SinglePoint => {
                // Print two messages: Absolute coordinates and relative coordinates to window x and y
                let coords_s = (coords.0 - window_x, coords.1 - window_y);
                println!("Relative to Game Window: {:?}", coords_s);
            },
            LocationFinderMode::AreaSelection => {
                let mut area_mode_first_coordinate =
                    area_mode_first_coordinate_clone.lock().unwrap();
                match *area_mode_first_coordinate {
                    Some(area_mode_first_coordinate_val) => {

                        let first_x = area_mode_first_coordinate_val.0 - window_x;
                        let first_y = area_mode_first_coordinate_val.1 - window_y;

                        let second_x = coords.0 - window_x;
                        let second_y = coords.1 - window_y;

                        let x: i32;
                        let y: i32;
                        let w: i32;
                        let h: i32;

                        // First point is either top-left, top-right, bottom-left, or bottom-right
                        // But the coords area ALWAYS needs x and y to be the top-left corner

                        if first_x < second_x && first_y < second_y { // Top-Left
                            x = first_x;
                            y = first_y;
                            w = second_x - first_x;
                            h = second_y - first_y;
                        } else if first_x >= second_x && first_y < second_y { // Top-Right
                            x = second_x;
                            y = first_y;
                            w = first_x - second_x;
                            h = second_y - first_y;
                        } else if first_x < second_x && first_y > second_y { // Bottom-Left
                            x = first_x;
                            y = second_y;
                            w = second_x - first_x;
                            h = first_y - second_y;
                        } else { // Bottom-Right
                            x = second_x;
                            y = second_y;
                            w = first_x - second_x;
                            h = first_y - second_y;
                        }

                        let coords_area = CoordsArea { x, y, w, h };
                        println!("{:?}", coords_area);
                        *area_mode_first_coordinate = None;
                    }
                    None => {
                        *area_mode_first_coordinate = Some((coords.0, coords.1));
                    },
                }
            }
        }
    });

    println!("Starting coordinate finder in Single Point mode...");
    loop {}
}

fn map_config_name_to_map_name(map_config_name: &str) -> String {
    // Translate file name from "DarkCastle.yaml" to "Dark Castle"
    let mut map_name = String::new();
    for (index, c) in map_config_name.chars().enumerate() {
        if c == '-' {
            map_name.push(' ');
            continue;
        } else if c.is_uppercase() && index != 0 {
            map_name.push(' ');
        } else if c == '.' {
            break;
        }
        map_name.push(c);
    }
    map_name
}

fn load_settings() -> Settings {
    let settings_path: PathBuf;
    {
        let settings_path_read_lock = CONFIG_PATH.read().unwrap();
        settings_path = settings_path_read_lock.as_ref().unwrap().join("Settings.yaml");
    }
    let file = File::open(settings_path).unwrap();
    let reader = BufReader::new(file);
    let settings: Settings = serde_yaml::from_reader(reader).unwrap();
    let mut settings_write_lock = SETTINGS.write().unwrap();
    *settings_write_lock = Some(settings.clone());
    settings
}

fn load_hotkeys() -> Hotkeys {
    let hotkeys_path: PathBuf;
    {
        let hotkeys_path_read_lock = CONFIG_PATH.read().unwrap();
        hotkeys_path = hotkeys_path_read_lock.as_ref().unwrap().parent().unwrap().join("Hotkeys.yaml");
    }

    if hotkeys_path.exists() == false {
        panic!("The config/Hotkeys.yaml file does not exist. Please create it and add the necessary hotkeys.");
    }
    let file = File::open(hotkeys_path).unwrap();
    let reader = BufReader::new(file);
    let hotkeys: Hotkeys = serde_yaml::from_reader(reader).unwrap();
    let mut hotkeys_write_lock = HOTKEYS.write().unwrap();
    *hotkeys_write_lock = Some(hotkeys.clone());
    hotkeys
}

fn load_map_config(map_file_name: Option<String>) {
    let map_path: PathBuf;
    {
        let config_path_read_lock = CONFIG_PATH.read().unwrap();
        map_path = config_path_read_lock.as_ref().unwrap().join("maps").join(map_file_name.as_ref().unwrap());
    }

    let file = File::open(map_path).unwrap();
    let reader = BufReader::new(file);
    let map: models::map::MapConfig = serde_yaml::from_reader(reader).unwrap();
    let mut map_write_lock = MAP_CONFIG.write().unwrap();
    *map_write_lock = Some(map);
}

fn load_map(map_config: &MapConfig, difficulty: Option<String>, gamemode: Option<String>) {
    let map = map_config
        .get(difficulty.unwrap().as_ref())
        .unwrap()
        .get(gamemode.unwrap().as_ref())
        .unwrap()
        .clone();
    let mut current_map_write_lock = lib::global::CURRENT_MAP.write().unwrap();
    *current_map_write_lock = Some(map);
}

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

fn main() {
    let args = Args::parse();

    if args.debug {
        DEBUG.store(true, Ordering::SeqCst);
    }

    {
        let mut enigo_settings_write_lock = lib::global::ENIGO_SETTINGS.write().unwrap();
        *enigo_settings_write_lock = Some(EnigoSettings {
            linux_delay: 10,
            mac_delay: 10,
            ..Default::default()
        });
    }

    // Sleep for 5 seconds to allow the user to switch to the BloonsTD6.exe window
    println!("Please make sure the BloonsTD6.exe window is in focus within the next {:?} seconds.", args.sleep);
    sleep(Duration::from_secs(args.sleep));

    // Load all Windows and locate the BloonsTD6.exe executable
    let mut bloons_td6_window: Option<Window> = None;
    for window in Window::all().unwrap() {
        if window.app_name().eq("BloonsTD6.exe") {
            bloons_td6_window = Some(window.clone());
            let fragile_window = Fragile::new(window);
            let mut current_window_write_lock = CURRENT_WINDOW.write().unwrap();
            *current_window_write_lock = Some(fragile_window);
            break;
        }
    }

    if bloons_td6_window.is_none() {
        panic!("The BloonsTD6.exe window was not found. Please open the game and try again.");
    }
    let window = bloons_td6_window.as_ref().unwrap();
    if window.is_minimized() {
        panic!("The BloonsTD6.exe window is minimized. Please open the game and try again.");
    }

    let (window_x ,window_y, window_width, window_height) = (window.x(), window.y(), window.width(), window.height());

    {
        let mut config_path_write_lock = CONFIG_PATH.write().unwrap();
        *config_path_write_lock = Some(PathBuf::from("./config/").join(format!("{}x{}", window_width, window_height).as_str()));
        if !config_path_write_lock.as_ref().unwrap().exists() {
            panic!("The \"{}\" directory does not exist. Please create it and add the necessary settings.", config_path_write_lock.as_ref().unwrap().to_str().unwrap());
        }
        println!("Using config path of {:?}", config_path_write_lock.as_ref().unwrap());
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
            let values = maps.values().collect::<Vec<&String>>();
            if !values.contains(&&map_name) {
                panic!("The map provided is not a legal map within the maps directory.");
            }
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
    println!("Please make sure the BloonsTD6.exe window is in focus within the next {:?} seconds.", args.sleep);
    sleep(Duration::from_secs(args.sleep));

    // Read the current map and print it
    // let current_map_read_lock = lib::global::CURRENT_MAP.read().unwrap();
    // let current_map = current_map_read_lock.as_ref().unwrap();

    // Load tesseract
    let rt_args = RTArgs {
        lang: "eng".into(),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "1234567890/".into(),
        )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3),
    };

    let mut threshold_value = 1;
    {
        let map_config_read_lock = MAP_CONFIG.read().unwrap();
        let map_config = map_config_read_lock.as_ref().unwrap();
        if map_config.round_counter_mode.eq(&RoundCounterMode::Dark) {
            threshold_value = 128;
        }
    }

    loop {
        // Get screenshot of the game window
        let screenshot = capture_area(
            settings.game.round_counter.clone(),
            ImageProcessingType::Resize        |
            ImageProcessingType::Grayscale     |
            ImageProcessingType::FloodFill     |
            ImageProcessingType::SheerVertical |
            ImageProcessingType::Invert,
            Some(threshold_value),
            Some(0.5)
        );
        let rt_image = convert_to_rusty_image(screenshot);

        // Get the round counter from the screenshot
        let output = rusty_tesseract::image_to_string(&rt_image, &rt_args).unwrap();
        println!("The round counter is: {}", output);

        sleep(Duration::from_secs(1));
    }
}
