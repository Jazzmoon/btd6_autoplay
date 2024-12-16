use std::error::Error as StdError;

use crate::models::{action_parser::ActionTrait, coords::Coords};

pub struct Obstacle {
    pub coords: Coords,
}

impl ActionTrait for Obstacle {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        println!("Running obstacle action");

        Ok(())
    }
}
