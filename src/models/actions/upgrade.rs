use std::error::Error as StdError;

use crate::{models::action_parser::ActionTrait, utils::global::CURRENT_MAP};

pub struct Upgrade {
    pub tower: String,
    pub upgrade_path: String,
}

impl ActionTrait for Upgrade {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let mut current_map_write_lock = CURRENT_MAP.write().unwrap();
        let current_map = current_map_write_lock.as_mut().unwrap();
        let tower = current_map.towers.get_mut(&self.tower).unwrap();
        match tower.upgrade(self.upgrade_path.clone()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
