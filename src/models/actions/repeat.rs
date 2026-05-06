use std::{
    error::Error as StdError,
    sync::{atomic::Ordering, Arc},
    thread,
    time::Duration,
};

use crate::{
    models::action_parser::ActionTrait,
    utils::{
        global::{GAME_ACTIVE, REPEAT_POOL},
        logger::Logger,
    },
};

pub struct Repeat {
    pub interval: Duration,
    pub action: Arc<dyn ActionTrait>,
    pub count: Option<u64>,
}

impl ActionTrait for Repeat {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        match self.count {
            Some(count) => {
                // Finite repeat: run the inner action `count` times, blocking the caller.
                let mut i = 0u64;
                while i < count {
                    self.action.run()?;
                    i += 1;
                    if i < count {
                        thread::sleep(self.interval);
                    }
                }
            }
            None => {
                // Infinite repeat: spawn an independent background thread so the caller is not
                // blocked. Multiple `repeat` actions may each call this path; each gets its own
                // thread with its own interval timer, all sharing the same `GAME_ACTIVE` stop
                // token. When the game ends, `stop_repeat_pool` sets the token to `false` and
                // joins every handle in `REPEAT_POOL`.
                let action = Arc::clone(&self.action);
                let interval = self.interval;
                let stop_flag = Arc::clone(&GAME_ACTIVE);

                let handle = thread::spawn(move || {
                    while stop_flag.load(Ordering::SeqCst) {
                        if let Err(e) = action.run() {
                            Logger::error(format!("Repeat thread action error: {:?}", e));
                        }
                        // Skip the sleep if the stop token was cleared while the action was
                        // running, so the thread exits without waiting a full interval.
                        if stop_flag.load(Ordering::SeqCst) {
                            thread::sleep(interval);
                        }
                    }
                });

                REPEAT_POOL
                    .lock()
                    .expect("REPEAT_POOL mutex poisoned")
                    .push(handle);
            }
        }
        Ok(())
    }
}
