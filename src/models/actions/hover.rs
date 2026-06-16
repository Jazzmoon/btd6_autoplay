use std::error::Error as StdError;

use crate::{utils::interaction, models::{action_parser::ActionTrait, coords::Coords}};

pub struct Hover {
    pub coords: Coords,
}

impl ActionTrait for Hover {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        match interaction::move_cursor(self.coords.clone(), None) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
