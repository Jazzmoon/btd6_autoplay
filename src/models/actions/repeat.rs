use std::{
    error::Error as StdError,
    sync::{atomic::Ordering, Arc},
    thread,
    time::Duration,
};

use crate::{
    models::action_parser::ActionTrait,
    utils::{
        global::{GAME_ACTIVE, REPEAT_THREAD},
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
                // Infinite repeat: spawn a background thread so the caller is not blocked.
                //
                // If a previous infinite-repeat thread is still alive (e.g. this action was
                // triggered twice in the same game), stop and join it first.
                if let Some(old_handle) = REPEAT_THREAD.lock().unwrap().take() {
                    GAME_ACTIVE.store(false, Ordering::SeqCst);
                    let _ = old_handle.join();
                }

                // Arm the stop token for the new iteration.
                GAME_ACTIVE.store(true, Ordering::SeqCst);

                let action = Arc::clone(&self.action);
                let interval = self.interval;
                let stop_flag = Arc::clone(&GAME_ACTIVE);

                let handle = thread::spawn(move || {
                    while stop_flag.load(Ordering::SeqCst) {
                        if let Err(e) = action.run() {
                            Logger::error(format!("Repeat thread action error: {:?}", e));
                        }
                        // Re-check before sleeping so we exit as soon as the flag drops.
                        if stop_flag.load(Ordering::SeqCst) {
                            thread::sleep(interval);
                        }
                    }
                });

                *REPEAT_THREAD.lock().unwrap() = Some(handle);
            }
        }
        Ok(())
    }
}
