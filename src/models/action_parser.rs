use std::error::Error as StdError;

use super::{actions, coords::Coords, tower::Tower};
use crate::utils::global::{CURRENT_MAP, HOTKEYS};

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
            let ability_hotkey = action_array[1..].to_vec();
            if ability_hotkey.len() < 1 || ability_hotkey.len() > 2 {
                return Err(
                    "Ability action requires 1 hotkey or a pair of (X,Y) coordinates.".into(),
                );
            }

            // Convert the ability hotkey from Vec<&str> to Vec<String>
            let ability_hotkey: Vec<String> =
                ability_hotkey.iter().map(|x| x.to_string()).collect();

            return Ok(Box::new(actions::ability::Ability {
                ability_keys: ability_hotkey,
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
            return Ok(Box::new(actions::click::Click {
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                },
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
            return Ok(Box::new(actions::hover::Hover {
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                },
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
            return Ok(Box::new(actions::obstacle::Obstacle {
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                },
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

            return Ok(Box::new(actions::place::Place {
                tower_name: tower_name.to_string(),
                tower_type_name: tower_type_name.to_string(),
                coords: Coords {
                    x: coords[0],
                    y: coords[1],
                },
                tower_hotkey: tower_hotkey,
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
            return Ok(Box::new(actions::start::Start {}));
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
