use std::error::Error as StdError;

use crate::models::action_parser::ActionTrait;

pub struct Ability {
    pub ability_keys: Vec<String>,
}

impl ActionTrait for Ability {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        println!("Running ability action");

        Ok(())
    }
}
