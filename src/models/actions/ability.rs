use std::error::Error as StdError;

use crate::{models::{action_parser::ActionTrait, hotkeys::Hotkey, coords::Coords}, utils::interaction};

pub enum AbilityType {
    Coords(Coords),
    Hotkey(Hotkey),
}

pub struct Ability {
    pub ability: AbilityType,
}

impl ActionTrait for Ability {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        match &self.ability {
            AbilityType::Coords(coords) => {
                let res = interaction::click(coords.clone(), None);
                if res.is_err() {
                    return Err(res.err().unwrap().into());
                }
                return Ok(());
            }
            AbilityType::Hotkey(hotkey) => {
                let res = interaction::press_key(hotkey.clone(), None);
                if res.is_err() {
                    return Err(res.err().unwrap().into());
                }
                return Ok(());
            }
        }
    }
}
