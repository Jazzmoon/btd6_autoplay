use std::error::Error as StdError;

use crate::{models::action_parser::ActionTrait, utils::global::CURRENT_MAP};

pub struct Select {
    pub tower: String,
}

impl ActionTrait for Select {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let current_map_read_lock = CURRENT_MAP.read().unwrap();
        let current_map = current_map_read_lock.as_ref().unwrap();

        if !current_map.towers.contains_key(&self.tower) {
            return Err("Tower does not exist.".into());
        }

        let tower = current_map.towers.get(&self.tower).unwrap();
        tower.highlight(true).map_err(Into::into)
    }
}
