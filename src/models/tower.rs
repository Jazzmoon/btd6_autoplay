use super::{coords::Coords, hotkeys::Hotkey};
use crate::utils::{
    global::{CURRENT_MAP, HOTKEYS},
    interaction,
};

use enigo::InputResult;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tower {
    pub name: String,
    pub hotkey: Hotkey,
    pub coords: Coords,
    pub upgrade_path: [i32; 3],
}

impl Tower {
    pub fn new(name: String, hotkey: Hotkey, coords: Coords, path: Option<[i32; 3]>) -> Tower {
        Tower {
            name,
            hotkey,
            coords,
            upgrade_path: path.unwrap_or([0, 0, 0]),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Tower: {}, Hotkey: {:?}, Coords: {:?}, Path: {:?}",
            self.name, self.hotkey, self.coords, self.upgrade_path
        )
    }

    pub fn highlight(&self, perform_click: Option<bool>) -> InputResult<()> {
        let click = perform_click.unwrap_or(false);
        if click {
            interaction::click(self.coords.clone(), None)
        } else {
            interaction::move_cursor(self.coords.clone(), None)
        }
    }

    pub fn deselect(&self) -> InputResult<()> {
        let current_map_read_lock = CURRENT_MAP.read().unwrap();
        let current_map = current_map_read_lock.as_ref().unwrap();
        let hover_location = current_map.hover_location.clone();
        interaction::click(hover_location, None)
    }

    pub fn place(&self) -> Result<(), String> {
        let _ = interaction::press_key(self.hotkey.clone(), None);
        let _ = interaction::click(self.coords.clone(), None);
        match self.deselect() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn upgrade(&mut self, upgrade_path: String) -> Result<[i32; 3], String> {
        // First, parse the upgrade path string (`\d-\d-\d`) into an array of 3 integers
        let upgrade_path: Vec<i32> = upgrade_path
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if upgrade_path.len() != 3 {
            return Err(format!(
                "Invalid upgrade path: {:?} for tower: {:?}",
                upgrade_path, self.name
            ));
        }

        // Next, calculate the difference vector between the current upgrade path and the new upgrade path
        let diff: [i32; 3] = upgrade_path
            .iter()
            .zip(self.upgrade_path.iter())
            .map(|(x, y)| x - y)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();

        // Check to make sure the upgrade path is valid (we cannot downgrade a tower)
        if !diff.iter().all(|&x| x >= 0) {
            // If the upgrade path is invalid, print an error message
            return Err(format!(
                "Invalid upgrade path: {:?} for tower: {:?}",
                upgrade_path, self.name
            ));
        }
        // Finally, update the tower's upgrade path by Selecting it, and pressing the upgrade hotkeys in order
        let res = self.highlight(Some(true));
        if res.is_err() {
            return Err(format!("Failed to select tower: {:?}", self.name));
        }

        {
            let hotkeys_read_lock = HOTKEYS.read().unwrap();
            let hotkeys = hotkeys_read_lock.as_ref().unwrap();

            if diff[0] > 0 {
                for _ in 0..diff[0] {
                    let _ = interaction::press_key(hotkeys.upgrade_top_path.clone(), None);
                }
            }

            if diff[1] > 0 {
                for _ in 0..diff[1] {
                    let _ = interaction::press_key(hotkeys.upgrade_middle_path.clone(), None);
                }
            }

            if diff[2] > 0 {
                for _ in 0..diff[2] {
                    let _ = interaction::press_key(hotkeys.upgrade_bottom_path.clone(), None);
                }
            }
        }

        self.upgrade_path = upgrade_path.try_into().unwrap();
        match self.deselect() {
            Ok(_) => Ok(self.upgrade_path),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn sell(&self) -> InputResult<()> {
        let res = self.highlight(Some(true));
        if res.is_err() {
            return res;
        }
        let hotkeys_read_lock = HOTKEYS.read().unwrap();
        let hotkeys = hotkeys_read_lock.as_ref().unwrap();
        let _ = interaction::press_key(hotkeys.sell.clone(), None);
        Ok(())
    }
}
