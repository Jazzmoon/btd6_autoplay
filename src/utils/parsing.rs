use std::{fs::File, io::BufReader, path::PathBuf};

use crate::models::{hotkeys::Hotkeys, map::MapConfig, settings::Settings};

use super::global::{CONFIG_SCREEN_PATH, CONFIG_BASE_PATH, CURRENT_MAP, GeneralConfig, GENERAL_CONFIG, HOTKEYS, MAP_CONFIG, SETTINGS};

pub fn map_config_name_to_map_name(map_config_name: &str) -> String {
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

pub fn load_general_config() -> GeneralConfig {
    let general_config_path: PathBuf = CONFIG_BASE_PATH.join("General.yaml");
    if general_config_path.exists() == false {
        panic!("The config/General.yaml file does not exist. Please create it and add the necessary configuration.");
    }
    let file = File::open(general_config_path).unwrap();
    let reader = BufReader::new(file);
    let general_config: GeneralConfig = serde_yaml::from_reader(reader).unwrap();
    let mut general_config_write_lock = GENERAL_CONFIG.write().unwrap();
    *general_config_write_lock = Some(general_config.clone());
    general_config
}

pub fn load_hotkeys() -> Hotkeys {
    let hotkeys_path: PathBuf = CONFIG_BASE_PATH.join("Hotkeys.yaml");
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

pub fn load_settings() -> Settings {
    let settings_path: PathBuf;
    {
        let settings_path_read_lock = CONFIG_SCREEN_PATH.read().unwrap();
        settings_path = settings_path_read_lock
            .as_ref()
            .unwrap()
            .join("Settings.yaml");
    }
    let file = File::open(settings_path).unwrap();
    let reader = BufReader::new(file);
    let settings: Settings = serde_yaml::from_reader(reader).unwrap();
    let mut settings_write_lock = SETTINGS.write().unwrap();
    *settings_write_lock = Some(settings.clone());
    settings
}

pub fn load_map_config(map_file_name: Option<String>) {
    let map_path: PathBuf;
    {
        let config_path_read_lock = CONFIG_SCREEN_PATH.read().unwrap();
        map_path = config_path_read_lock
            .as_ref()
            .unwrap()
            .join("maps")
            .join(map_file_name.as_ref().unwrap());
    }

    let file = File::open(map_path).unwrap();
    let reader = BufReader::new(file);
    let map: MapConfig = serde_yaml::from_reader(reader).unwrap();
    let mut map_write_lock = MAP_CONFIG.write().unwrap();
    *map_write_lock = Some(map);
}

pub fn load_map(map_config: &MapConfig, difficulty: Option<String>, gamemode: Option<String>) {
    let map = map_config
        .get(difficulty.unwrap().as_ref())
        .unwrap()
        .get(gamemode.unwrap().as_ref())
        .unwrap()
        .clone();
    let mut current_map_write_lock = CURRENT_MAP.write().unwrap();
    *current_map_write_lock = Some(map);
}
