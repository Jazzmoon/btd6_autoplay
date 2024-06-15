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

use inquire::Select;
use lib::global::{CONFIG_PATH, DEBUG, HOTKEYS, MAPS_PATH, MAP_CONFIG, SETTINGS};
use models::hotkeys::Hotkeys;
use models::settings::Settings;

use clap::Parser;

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

#[derive(Parser, Debug)]
#[clap(version = "3.0.0")]
#[clap(about = "An autoplay bot for Bloons Tower Defense 6")]
struct Args {
    /// Name of the map
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

    if !CONFIG_PATH.exists() {
        panic!(
            "The config directory does not exist. Please create it and add the necessary files."
        );
    }

    if !MAPS_PATH.exists() {
        panic!("The maps directory in config path does not exist. Please create it and add the necessary files.");
    }

    // Load the config/Settings.1080p.yaml file into the SETTINGS global variable
    {
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

    // Load the config/Hotkeys.yaml file into the HOTKEYS global variable
    {
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
            let map_keys = maps.keys().collect::<Vec<&String>>();
            let map_selection = Select::new("Please select a map:", map_keys).prompt();
            match map_selection {
                Ok(map_selection) => match maps.get(map_selection) {
                    Some(file_name) => {
                        map_file_name = Some(file_name.clone());
                    }
                    None => {
                        panic!("You did not select a valid map.");
                    }
                },
                Err(_) => {
                    panic!("You did not select a valid map.");
                }
            }
        }
    }

    // Read the map file into the MAP_CONFIG global variable
    {
        let map_path = MAPS_PATH.join(map_file_name.as_ref().unwrap());
        let file = File::open(map_path).unwrap();
        let reader = BufReader::new(file);
        let map: models::map::MapConfig = serde_yaml::from_reader(reader).unwrap();
        let mut map_write_lock = MAP_CONFIG.write().unwrap();
        *map_write_lock = Some(map);
    }

    // Next, given we have a map, we need to check if the user provided a difficulty
    // Load the difficulties from the map config into a hashmap
    let map_config_read_lock = MAP_CONFIG.read().unwrap();
    let map_config = map_config_read_lock.as_ref().unwrap();

    let legal_difficulties = map_config.implemented_difficulties();
    let mut difficulty = args.difficulty.clone();

    match difficulty {
        Some(ref difficulty) => {
            if !legal_difficulties.contains(difficulty) {
                panic!("The difficulty provided is not a legal difficulty for the map provided.");
            }
        }
        None => {
            if legal_difficulties.len() == 0 {
                panic!("There are no difficulties available for the map provided.");
            } else if legal_difficulties.len() == 1 {
                difficulty = Some(legal_difficulties[0].clone());
                println!(
                    "Only one difficulty available, selecting \"{}\"",
                    difficulty.clone().unwrap()
                );
            } else {
                let difficulty_selection =
                    Select::new("Please select a difficulty:", legal_difficulties).prompt();
                match difficulty_selection {
                    Ok(difficulty_selection) => {
                        difficulty = Some(difficulty_selection);
                    }
                    Err(_) => {
                        panic!("You did not select a valid difficulty.");
                    }
                }
            }
        }
    }

    let legal_gamemodes = map_config
        .get(difficulty.clone().unwrap().as_ref())
        .unwrap()
        .implemented_gamemodes();
    let mut gamemode = args.gamemode.clone();

    match gamemode {
        Some(ref gamemode) => {
            if !legal_gamemodes.contains(gamemode) {
                panic!("The gamemode provided is not a legal gamemode for the map and difficulty provided.");
            }
        }
        None => {
            if legal_gamemodes.len() == 0 {
                panic!("There are no gamemodes available for the map and difficulty provided.");
            } else if legal_gamemodes.len() == 1 {
                gamemode = Some(legal_gamemodes[0].clone());
                println!(
                    "Only one gamemode available, selecting \"{}\"",
                    gamemode.clone().unwrap()
                );
            } else {
                let gamemode_selection =
                    Select::new("Please select a gamemode:", legal_gamemodes).prompt();
                match gamemode_selection {
                    Ok(gamemode_selection) => {
                        gamemode = Some(gamemode_selection);
                    }
                    Err(_) => {
                        panic!("You did not select a valid gamemode.");
                    }
                }
            }
        }
    }

    // Set the value of CURRENT_MAP to the map, difficulty, and gamemode selected
    {
        let map = map_config
            .get(difficulty.clone().unwrap().as_ref())
            .unwrap()
            .get(gamemode.clone().unwrap().as_ref())
            .unwrap()
            .clone();
        let mut current_map_write_lock = lib::global::CURRENT_MAP.write().unwrap();
        *current_map_write_lock = Some(map);
    }

    // Read the current map and print it
    let current_map_read_lock = lib::global::CURRENT_MAP.read().unwrap();
    let current_map = current_map_read_lock.as_ref().unwrap();
    println!("{:?}", current_map);
}
