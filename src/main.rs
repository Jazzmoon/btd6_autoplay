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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use lib::global::{CONFIG_PATH, DEBUG, HOTKEYS, MAPS_PATH, MAP_CONFIG, SETTINGS};
use models::hotkeys::Hotkeys;
use models::map::MapConfig;
use models::settings::Settings;

use clap::Parser;
use device_query::{DeviceEvents, DeviceState, Keycode, MousePosition};
use inquire::Select;

#[derive(Debug, Clone, Copy)]
enum LocationFinderMode {
    SinglePoint,
    AreaSelection,
}

fn location_finder() {
    // This is a tool that acts as an infinite loop that prints the current mouse position when you left click
    // When you press the letter 'q', the program will exit
    // Pressing 'p' will pause the program so you can interact with your system without logging mouse positions
    // Pressing 'p' again will resume the program
    // Pressing 'm' changes the mode from single click to area selection
    // Print help dialog for the functionality of the program
    println!("Welcome to the location finder utility!");
    println!("Press 'q' to exit the program");
    println!("Press 'p' to pause/resume the program");
    println!("Press 'm' to change the mode from single point to area selection mode");

    let paused = Arc::new(AtomicBool::new(false));
    let mode = Arc::new(Mutex::new(LocationFinderMode::SinglePoint));

    let area_mode_first_coordinate: Arc<Mutex<Option<MousePosition>>> = Arc::new(Mutex::new(None));

    let selection = Select::new(
        "Please select a mode:",
        vec!["Single Point", "Area Selection"],
    )
    .prompt();

    match selection {
        Ok(selection) => {
            let mut mode_lock = mode.lock().unwrap();
            *mode_lock = match selection {
                "Single Point" => LocationFinderMode::SinglePoint,
                "Area Selection" => LocationFinderMode::AreaSelection,
                _ => panic!("You did not select a valid mode."),
            };
            println!("{} mode selected", selection);
        }
        Err(_) => panic!("You did not select a valid mode."),
    }

    let device_state = DeviceState::new();
    let mouse_coords = Arc::new(Mutex::new(device_state.query_pointer().coords));

    let paused_clone = Arc::clone(&paused);
    let mode_clone = Arc::clone(&mode);
    let _guard = device_state.on_key_up(move |key| {
        if key.eq(&Keycode::Q) {
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
    let mouse_coords_clone = Arc::clone(&mouse_coords);
    let _guard = device_state.on_mouse_move(move |coords| {
        let paused = paused_clone.load(Ordering::SeqCst);
        if paused {
            return;
        }
        let mut mouse_coords = mouse_coords_clone.lock().unwrap();
        *mouse_coords = coords.clone();
    });

    let paused_clone = Arc::clone(&paused);
    let mode_clone = Arc::clone(&mode);
    let mouse_coords_clone = Arc::clone(&mouse_coords);
    let area_mode_first_coordinate_clone = Arc::clone(&area_mode_first_coordinate);
    let _guard = device_state.on_mouse_up(move |button| {
        if *button != 1 {
            return;
        }
        let paused = paused_clone.load(Ordering::SeqCst);
        if paused {
            return;
        }
        let coords = mouse_coords_clone.lock().unwrap().clone();
        let mode_lock = mode_clone.lock().unwrap();
        match *mode_lock {
            LocationFinderMode::SinglePoint => println!("Mouse position: {:?}", coords),
            LocationFinderMode::AreaSelection => {
                let mut area_mode_first_coordinate =
                    area_mode_first_coordinate_clone.lock().unwrap();
                match *area_mode_first_coordinate {
                    Some(area_mode_first_coordinate_val) => {
                        // Build a CoordsArea object and print it
                        let coords_area: models::coords::CoordsArea = models::coords::CoordsArea {
                            x: area_mode_first_coordinate_val.0,
                            y: area_mode_first_coordinate_val.1,
                            w: coords.0 - area_mode_first_coordinate_val.0,
                            h: coords.1 - area_mode_first_coordinate_val.1,
                        };
                        *area_mode_first_coordinate = None;
                        println!("{:?}", coords_area);
                    }
                    None => *area_mode_first_coordinate = Some(coords),
                }
            }
        }
    });

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

fn load_settings() {
    let settings_path = CONFIG_PATH.join("Settings.yaml");
    if settings_path.exists() == false {
        panic!("The config/Settings.yaml file does not exist. Please create it and add the necessary settings.");
    }
    let file = File::open(settings_path).unwrap();
    let reader = BufReader::new(file);
    let settings: Settings = serde_yaml::from_reader(reader).unwrap();
    let mut settings_write_lock = SETTINGS.write().unwrap();
    *settings_write_lock = Some(settings);
}

fn load_hotkeys() {
    let hotkeys_path = CONFIG_PATH.join("Hotkeys.yaml");
    if hotkeys_path.exists() == false {
        panic!("The config/Hotkeys.yaml file does not exist. Please create it and add the necessary hotkeys.");
    }
    let file = File::open(hotkeys_path).unwrap();
    let reader = BufReader::new(file);
    let hotkeys: Hotkeys = serde_yaml::from_reader(reader).unwrap();
    let mut hotkeys_write_lock = HOTKEYS.write().unwrap();
    *hotkeys_write_lock = Some(hotkeys);
}

fn load_map_config(map_file_name: Option<String>) {
    let map_path = MAPS_PATH.join(map_file_name.as_ref().unwrap());
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
        let mut debug_write_lock = DEBUG.write().unwrap();
        *debug_write_lock = true;
    }

    if args.location_finder {
        location_finder();
        return;
    }

    if !CONFIG_PATH.exists() {
        panic!(
            "The config directory does not exist. Please create it and add the necessary files."
        );
    }

    if !MAPS_PATH.exists() {
        panic!("The maps directory in config path does not exist. Please create it and add the necessary files.");
    }

    // Load the config/Settings.1080p.yaml file into the SETTINGS global variable
    load_settings();

    // Load the config/Hotkeys.yaml file into the HOTKEYS global variable
    load_hotkeys();

    // Start validation of arguments
    // Load the maps directory into a hashmap
    let maps: HashMap<String, String> = MAPS_PATH
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

    // Read the current map and print it
    let current_map_read_lock = lib::global::CURRENT_MAP.read().unwrap();
    let current_map = current_map_read_lock.as_ref().unwrap();
    println!("{:?}", current_map);
}
