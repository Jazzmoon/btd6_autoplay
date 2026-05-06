use std::{error::Error as StdError, thread::sleep, time::Duration};

use crate::models::action_parser::ActionTrait;

pub struct Repeat {
    pub interval: Duration,
    pub action: Box<dyn ActionTrait>,
    pub count: Option<u64>,
}

impl ActionTrait for Repeat {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let mut i = 0u64;
        while self.count.map_or(true, |count| i < count) {
            self.action.run()?;
            i += 1;
            if self.count.map_or(true, |count| i < count) {
                sleep(self.interval);
            }
        }
        Ok(())
    }
}
