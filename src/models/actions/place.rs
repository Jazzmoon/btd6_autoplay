use std::error::Error as StdError;

use crate::{
    models::{action_parser::ActionTrait, coords::Coords, hotkeys::Hotkey, tower::Tower},
    utils::global::CURRENT_MAP,
};

pub struct Place {
    pub tower_name: String,
    pub tower_type_name: String,
    pub coords: Coords,
    pub tower_hotkey: Hotkey,
}

impl ActionTrait for Place {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let mut current_map_write_lock = CURRENT_MAP.write().unwrap();
        let current_map = current_map_write_lock.as_mut().unwrap();

        // Check that there is no tower with that name already in the map
        if current_map.towers.contains_key(self.tower_name.as_str()) {
            return Err(format!("Tower with name {} already exists", self.tower_name).into());
        }

        let tower = Tower::new(
            self.tower_name.clone(),
            self.tower_hotkey.clone(),
            self.coords.clone(),
            None,
        );

        match tower.place() {
            Ok(_) => {
                current_map.towers.insert(self.tower_name.clone(), tower);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
