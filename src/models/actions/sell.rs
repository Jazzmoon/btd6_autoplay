use std::error::Error as StdError;

use crate::{models::action_parser::ActionTrait, utils::global::CURRENT_MAP};

pub struct Sell {
    pub tower: String,
}

impl ActionTrait for Sell {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let mut current_map_write_lock = CURRENT_MAP.write().unwrap();
        let current_map = current_map_write_lock.as_mut().unwrap();

        if !current_map.towers.contains_key(&self.tower) {
            return Err("Tower does not exist.".into());
        }

        let tower = current_map.towers.get_mut(&self.tower).unwrap();
        match tower.sell() {
            Ok(_) => {
                current_map.towers.remove(&self.tower);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
