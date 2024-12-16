use std::error::Error as StdError;

use crate::models::{action_parser::ActionTrait, coords::Coords};

pub struct Hover {
    pub coords: Coords,
}

impl ActionTrait for Hover {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        println!("Running hover action");

        Ok(())
    }
}
