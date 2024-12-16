use std::error::Error as StdError;

use crate::{models::{action_parser::ActionTrait, coords::Coords}, utils::interaction};

pub struct Click {
    pub coords: Coords,
}

impl ActionTrait for Click {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        match interaction::click(self.coords.clone(), None) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
