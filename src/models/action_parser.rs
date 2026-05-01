use std::error::Error as StdError;
use super::{actions, actions::ability::AbilityType, coords::Coords, hotkeys::Hotkey};
use crate::utils::global::{
    CURRENT_WINDOW,
    HOTKEYS,
};

pub trait ActionTrait {
    fn run(&self) -> Result<(), Box<dyn StdError>>;
}

pub fn parse_action(action: &str) -> Result<Box<dyn ActionTrait>, Box<dyn StdError>> {
    let action_array: Vec<&str> = action.split(" ").collect();
    if action_array.len() == 0 {
        return Err("Empty actions are not supported.".into());
    }

    match action_array[0] {
        "ability" => {
            /*
             * Ability action accepts one of 3 forms in the following order of priority:
             | 1. coords (e.g. "100 240")
             | 2. ability hotkey name (e.g. "road_spikes" or "super_monkey_storm")
             | 3. ability hotkey (e.g. "shift 8" or "\")
             * This means that you cannot use a custom combination of keys in which would be a valid coordinate.
             */
            let args = action_array[1..].to_vec();
            if args.len() < 1 {
                return Err(
                    "Ability action string is invalid.".into(),
                );
            }
            // Attempt to convert the ability hotkey to a Coords struct or Hotkey struct
            if args.len() == 2 {
                // Try parse as two integers
                let coords = args
                    .iter()
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, _>>();
                match coords {
                    Ok(coords) => {
                        if coords.len() == 2 {
                            let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
                            let current_window = current_window_read_lock.as_ref().unwrap().get();
                            return Ok(Box::new(actions::ability::Ability {
                                ability: AbilityType::Coords(Coords {
                                    x: coords[0],
                                    y: coords[1],
                                }.relative_to_window(current_window)),
                            }));
                        }
                    }
                    Err(_) => {}
                }
            }
            // Check if it is a hotkey in our struct
            let hotkeys_read_lock = HOTKEYS.read().unwrap();
            let hotkeys = hotkeys_read_lock.as_ref();
            if hotkeys.is_none() {
                return Err("Hotkeys not loaded.".into());
            }
            if hotkeys.unwrap().contains_key(args[0]) {
                return Ok(Box::new(actions::ability::Ability {
                    ability: AbilityType::Hotkey(hotkeys.unwrap().get(args[0])),
                }));
            }
            // Array of vectors is just a series of keys to press
            return Ok(Box::new(actions::ability::Ability {
                ability: AbilityType::Hotkey(Hotkey::from(&args)),
            }));
        }
        "click" => {
            let coords = action_array[1..]
                .iter()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;
            if coords.len() != 2 {
                return Err("Click action requires 2 coordinates.".into());
            }
            let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
            let current_window = current_window_read_lock.as_ref().unwrap().get();
            return Ok(Box::new(actions::click::Click {
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                }.relative_to_window(current_window),
            }));
        }
        "hover" => {
            let coords = action_array[1..]
                .iter()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;
            if coords.len() != 2 {
                return Err("Hover action requires 2 coordinates.".into());
            }
            let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
            let current_window = current_window_read_lock.as_ref().unwrap().get();
            return Ok(Box::new(actions::hover::Hover {
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                }.relative_to_window(current_window),
            }));
        }
        "obstacle" | "clear" => {
            let coords = action_array[1..]
                .iter()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;
            if coords.len() != 2 {
                return Err("Obstacle action requires 2 coordinates.".into());
            }
            let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
            let current_window = current_window_read_lock.as_ref().unwrap().get();
            return Ok(Box::new(actions::obstacle::Obstacle {
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                }.relative_to_window(current_window),
            }));
        }
        "place" => {
            let args: Vec<&str> = action_array[1..].to_vec();
            let tower_name = args[0];
            let tower_type_name = args[1];
            let coords = args[2..]
                .iter()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;
            if coords.len() != 2 {
                return Err("Place action requires 2 coordinates.".into());
            }

            let tower_hotkey: Hotkey;
            {
                let hotkeys_read_lock = HOTKEYS.read().unwrap();
                let hotkeys = hotkeys_read_lock.as_ref();
                if hotkeys.is_none() || !hotkeys.unwrap().contains_key(tower_type_name) {
                    return Err("Tower type not found.".into());
                }
                tower_hotkey = hotkeys.unwrap().get(tower_type_name);
            }

            let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
            let current_window = current_window_read_lock.as_ref().unwrap().get();
            return Ok(Box::new(actions::place::Place {
                tower_name: tower_name.to_string(),
                tower_type_name: tower_type_name.to_string(),
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                }.relative_to_window(current_window),
                tower_hotkey,
            }));
        }
        "sell" => {
            let tower_name = action_array[1];
            return Ok(Box::new(actions::sell::Sell {
                tower: tower_name.to_string(),
            }));
        }
        "sleep" => {
            let sleep_time = action_array[1].parse::<u64>()?;
            return Ok(Box::new(actions::sleep::Sleep { sleep_time }));
        }
        "start" => {
            if action_array.len() > 2 {
                return Err("Start action can only take one optional argument: 'fast-forward'.".into());
            }
            if action_array.len() == 2 && action_array[1] != "fast-forward" {
                return Err("Start action's only optional argument is 'fast-forward'.".into());
            }
            return Ok(Box::new(actions::start::Start {
                fast_forward: Some(action_array.len() == 2 && action_array[1] == "fast-forward")
            }));
        }
        "upgrade" => {
            let args = action_array[1..].to_vec();
            let tower_name = args[0];
            let upgrade_path = args[1];
            return Ok(Box::new(actions::upgrade::Upgrade {
                tower: tower_name.to_string(),
                upgrade_path: upgrade_path.to_string(),
            }));
        }
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown action",
            )));
        }
    };
}
