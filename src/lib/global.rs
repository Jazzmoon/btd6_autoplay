use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::RwLock;

use enigo::Settings as EnigoSettings;
use lazy_static::lazy_static;

use crate::models::hotkeys::Hotkeys;
use crate::models::map::{Map, MapConfig};
use crate::models::settings::Settings;

lazy_static! {
    pub static ref CONFIG_PATH: &'static Path = Path::new("./config");
    pub static ref MAPS_PATH: &'static Path = Path::new("./config/maps");
    pub static ref ENIGO_SETTINGS: RwLock<Option<EnigoSettings>> = RwLock::new(None);
    pub static ref SETTINGS: RwLock<Option<Settings>> = RwLock::new(None);
    pub static ref HOTKEYS: RwLock<Option<Hotkeys>> = RwLock::new(None);
    pub static ref MAP_CONFIG: RwLock<Option<MapConfig>> = RwLock::new(None);
    pub static ref CURRENT_MAP: RwLock<Option<Map>> = RwLock::new(None);
    pub static ref DEBUG: AtomicBool = AtomicBool::new(false);
}
