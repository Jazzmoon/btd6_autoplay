use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::models::Hotkeys;
use crate::models::Map;
use crate::models::Settings;

lazy_static! {
    pub static ref SETTIGNS: Mutex<Maybe<Settings>> = Mutex::new(None);
    pub static ref HOTKEYS: Mutex<Maybe<Hotkeys>> = Mutex::new(None);
    pub static ref CURRENT_MAP: Mutex<Maybe<Map>> = Mutex::new(None);
    pub static ref MAP_CONFIG: Mutex<Maybe<MapConfig>> = Mutex::new(None);
}