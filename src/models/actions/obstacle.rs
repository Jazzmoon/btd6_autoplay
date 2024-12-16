use std::error::Error as StdError;

use crate::{models::{action_parser::ActionTrait, coords::Coords}, utils::{interaction, global::{CURRENT_MAP, SETTINGS}}};

pub struct Obstacle {
    pub coords: Coords,
}

impl ActionTrait for Obstacle {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        match interaction::click(self.coords.clone(), None) {
            Ok(_) => {
                let settings_read_lock = SETTINGS.read().unwrap();
                let settings = settings_read_lock.as_ref().unwrap();
                match interaction::click(settings.game.confirm_button.clone(), None) {
                    Ok(_) => {
                        let current_map_read_lock = CURRENT_MAP.read().unwrap();
                        let current_map = current_map_read_lock.as_ref().unwrap();
                        match interaction::click(current_map.hover_location.clone(), None) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e.into()),
                        }
                    },
                    Err(e) => Err(e.into()),
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}
