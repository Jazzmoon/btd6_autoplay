use std::error::Error as StdError;

use crate::models::{action_parser::ActionTrait, coords::Coords};

pub struct Click {
    pub coords: Coords,
}

impl ActionTrait for Click {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        println!("Running click action");

        Ok(())
    }
}
