use std::error::Error as StdError;

use crate::{
    models::{action_parser::ActionTrait, coords::Coords, hotkeys::Hotkey, tower::Tower},
    utils::{global::CURRENT_MAP, logger::Logger},
};

pub struct Place {
    pub tower_name: String,
    pub tower_type_name: String,
    pub coords: Coords,
    pub tower_hotkey: Hotkey,
}

impl ActionTrait for Place {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        {
            let current_map_read_lock = CURRENT_MAP.read().unwrap();
            let current_map = current_map_read_lock.as_ref().unwrap();

            // Check that there is no tower with that name already in the map
            if current_map.towers.contains_key(self.tower_name.as_str()) {
                return Err(format!("Tower with name {} already exists", self.tower_name).into());
            }
        }

        let tower = Tower::new(
            self.tower_name.clone(),
            self.tower_hotkey.clone(),
            self.coords.clone(),
            None,
        );

        match tower.place() {
            Ok(_) => {
                let mut current_map_write_lock = CURRENT_MAP.write().unwrap();
                let current_map = current_map_write_lock.as_mut().unwrap();
                current_map.towers.insert(self.tower_name.clone(), tower);
                Logger::debug(format!("Successfully placed tower {}", self.tower_name));
                Ok(())
            }
            Err(e) => {
                Logger::error(format!(
                    "Failed to place tower {}: {:?}",
                    self.tower_name, e
                ));
                Err(e.into())
            }
        }
    }
}
