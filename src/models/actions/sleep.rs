use std::error::Error as StdError;

use crate::models::action_parser::ActionTrait;

pub struct Sleep {
    pub sleep_time: u64,
}

impl ActionTrait for Sleep {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        // Sleep for the specified time
        std::thread::sleep(std::time::Duration::from_secs(self.sleep_time));

        Ok(())
    }
}
