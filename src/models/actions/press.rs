use std::error::Error as StdError;

use crate::{models::{action_parser::ActionTrait, hotkeys::Hotkey}, utils::interaction};

pub struct Press {
    pub keys: Vec<Hotkey>,
}

impl ActionTrait for Press {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        for key in &self.keys {
            interaction::press_key(key.clone(), None).map_err(|e| -> Box<dyn StdError> { e.into() })?;
        }
        Ok(())
    }
}
