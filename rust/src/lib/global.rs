use crate::models::Settings::Settings;
use crate::models::Hotkeys::Hotkeys;
use crate::models::Map::{Map, MapConfig};

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTINGS: Mutex<Option<Settings>> = Mutex::new(None);
    pub static ref HOTKEYS: Mutex<Option<Hotkeys>> = Mutex::new(None);
    pub static ref CURRENT_MAP: Mutex<Option<Map>> = Mutex::new(None);
    pub static ref MAP_CONFIG: Mutex<Option<MapConfig>> = Mutex::new(None);
}