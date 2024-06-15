use lazy_static::lazy_static;
use std::path::Path;
use std::sync::RwLock;

use crate::models::hotkeys::Hotkeys;
use crate::models::map::{Map, MapConfig};
use crate::models::settings::Settings;

lazy_static! {
    pub static ref CONFIG_PATH: &'static Path = Path::new("./config");
    pub static ref MAPS_PATH: &'static Path = Path::new("./config/maps");
    pub static ref SETTINGS: RwLock<Option<Settings>> = RwLock::new(Option::None);
    pub static ref HOTKEYS: RwLock<Option<Hotkeys>> = RwLock::new(Option::None);
    pub static ref MAP_CONFIG: RwLock<Option<MapConfig>> = RwLock::new(Option::None);
    pub static ref CURRENT_MAP: RwLock<Option<Map>> = RwLock::new(Option::None);
    pub static ref DEBUG: RwLock<bool> = RwLock::new(false);
}
