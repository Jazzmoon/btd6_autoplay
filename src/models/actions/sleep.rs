use std::{error::Error as StdError, thread::sleep, time::Duration};

use crate::models::action_parser::ActionTrait;

pub struct Sleep {
    pub sleep_time: Duration,
}

impl ActionTrait for Sleep {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        // Sleep for the specified time
        sleep(self.sleep_time);
        Ok(())
    }
}
