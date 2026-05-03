use std::path::PathBuf;
use std::sync::{atomic::AtomicU8, Arc, RwLock};

use enigo::Settings as EnigoSettings;
use fragile::Fragile;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use xcap::Window;

use crate::models::{
    hotkeys::Hotkeys,
    map::{Map, MapConfig},
    settings::Settings,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralConfig {
    pub window_title_search_terms: Vec<String>,
}

const DEFAULT_LOG_LEVEL: u8 = 4; // warn

lazy_static! {
    pub static ref CURRENT_WINDOW: Arc<RwLock<Option<Fragile<Window>>>> =
        Arc::new(RwLock::new(None));
    pub static ref CONFIG_BASE_PATH: PathBuf = PathBuf::from("./config");
    pub static ref CONFIG_SCREEN_PATH: Arc<RwLock<Option<PathBuf>>> = Arc::new(RwLock::new(None));
    pub static ref ENIGO_SETTINGS: Arc<RwLock<Option<EnigoSettings>>> = Arc::new(RwLock::new(None));
    pub static ref SETTINGS: Arc<RwLock<Option<Settings>>> = Arc::new(RwLock::new(None));
    pub static ref GENERAL_CONFIG: Arc<RwLock<Option<GeneralConfig>>> = Arc::new(RwLock::new(None));
    pub static ref HOTKEYS: Arc<RwLock<Option<Hotkeys>>> = Arc::new(RwLock::new(None));
    pub static ref MAP_CONFIG: Arc<RwLock<Option<MapConfig>>> = Arc::new(RwLock::new(None));
    pub static ref CURRENT_MAP: Arc<RwLock<Option<Map>>> = Arc::new(RwLock::new(None));
    pub static ref LOG_LEVEL: AtomicU8 = AtomicU8::new(DEFAULT_LOG_LEVEL);
}
