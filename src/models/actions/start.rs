use std::error::Error as StdError;

use crate::{models::action_parser::ActionTrait, utils::{global::HOTKEYS, interaction}};

pub struct Start {
    pub fast_forward: Option<bool>,
}

impl ActionTrait for Start {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let hotkeys_read_lock = HOTKEYS.read().unwrap();
        let hotkeys = hotkeys_read_lock.as_ref().unwrap();
        let start_hotkey = hotkeys.start.clone();

        //Print the hotkey for debugging purposes
        println!("Start hotkey: {:?}", start_hotkey);

        // Fast-Forward means press the start hotkey twice with a short delay in between to skip the start of the round and go straight to the next one
        if self.fast_forward.unwrap_or(false) {
            match interaction::press_key(start_hotkey.clone(), Some(1000)) {
                Ok(_) => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    match interaction::press_key(start_hotkey.clone(), None) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.into()),
                    }
                }
                Err(e) => Err(e.into()),
            }
        } else {
            match interaction::press_key(start_hotkey, None) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            }
        }
    }
}
