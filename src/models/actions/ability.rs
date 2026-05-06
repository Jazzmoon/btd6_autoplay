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
            AbilityType::Coords(coords) => interaction::click(coords.clone(), None).map_err(Into::into),
            AbilityType::Hotkey(hotkey) => interaction::press_key(hotkey.clone(), None).map_err(Into::into),
        }
    }
}
