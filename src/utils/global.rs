use std::path::PathBuf;
use std::sync::{atomic::AtomicBool, RwLock};

use enigo::Settings as EnigoSettings;
use fragile::Fragile;
use lazy_static::lazy_static;
use xcap::Window;

use crate::models::{
    hotkeys::Hotkeys,
    map::{Map, MapConfig},
    settings::Settings,
};

lazy_static! {
    pub static ref CURRENT_WINDOW: RwLock<Option<Fragile<Window>>> = RwLock::new(None);
    pub static ref CONFIG_PATH: RwLock<Option<PathBuf>> = RwLock::new(None);
    pub static ref ENIGO_SETTINGS: RwLock<Option<EnigoSettings>> = RwLock::new(None);
    pub static ref SETTINGS: RwLock<Option<Settings>> = RwLock::new(None);
    pub static ref HOTKEYS: RwLock<Option<Hotkeys>> = RwLock::new(None);
    pub static ref MAP_CONFIG: RwLock<Option<MapConfig>> = RwLock::new(None);
    pub static ref CURRENT_MAP: RwLock<Option<Map>> = RwLock::new(None);
    pub static ref DEBUG: AtomicBool = AtomicBool::new(false);
}
