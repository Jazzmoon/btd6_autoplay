use std::error::Error as StdError;

use crate::{models::action_parser::ActionTrait, utils::{global::HOTKEYS, interaction}};

pub struct Start {}

impl ActionTrait for Start {
    fn run(&self) -> Result<(), Box<dyn StdError>> {
        let hotkeys_read_lock = HOTKEYS.read().unwrap();
        let hotkeys = hotkeys_read_lock.as_ref().unwrap();
        let start_hotkey = hotkeys.start.clone();
        match interaction::press_key(start_hotkey, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
