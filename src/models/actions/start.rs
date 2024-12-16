use std::error::Error as StdError;

use crate::models::action_parser::ActionTrait;

pub struct Start {}

impl ActionTrait for Start {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        println!("Running start action");

        Ok(())
    }
}
