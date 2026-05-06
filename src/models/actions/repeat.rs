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
                // triggered twice in the same game), signal it to stop and join it before
                // spawning a new one. GAME_ACTIVE is set to false here and reset to true only
                // after the old thread has fully exited, so there is no window in which both
                // the old and new threads could observe different values of the flag.
                if let Some(old_handle) = REPEAT_THREAD.lock().unwrap().take() {
                    GAME_ACTIVE.store(false, Ordering::SeqCst);
                    if let Err(e) = old_handle.join() {
                        Logger::error(format!("Repeat thread panicked: {:?}", e));
                    }
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
                        // Skip the sleep if the stop token was cleared while the action was
                        // running, so the thread exits without waiting a full interval.
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
