use std::error::Error as StdError;

use crate::{models::action_parser::ActionTrait, utils::global::CURRENT_MAP};

pub struct Upgrade {
    pub tower: String,
    pub upgrade_path: String,
}

impl ActionTrait for Upgrade {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let current_map_read_lock = CURRENT_MAP.read().unwrap();
        let current_map = current_map_read_lock.as_ref().unwrap();
        if !current_map.towers.contains_key(&self.tower) {
            return Err("Tower does not exist.".into());
        }
        // Make a copy of the tower so we can mutate it without holding the read lock on the map, then update it after upgrading
        let mut tower = current_map.towers.get(&self.tower).unwrap().clone();
        match tower.upgrade(self.upgrade_path.clone()) {
            Ok(upgrade_path) => {
                // Switch from read to write lock to mutate the tower's upgrade path in the map
                drop(current_map_read_lock);
                let mut current_map_write_lock = CURRENT_MAP.write().unwrap();
                let current_map = current_map_write_lock.as_mut().unwrap();
                if let Some(tower_in_map) = current_map.towers.get_mut(&self.tower) {
                    tower_in_map.upgrade_path = upgrade_path;
                    Ok(())
                } else {
                    Err("Tower does not exist.".into())
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}
