use super::{actions, actions::ability::AbilityType, coords::Coords, hotkeys::Hotkey};
use crate::utils::global::HOTKEYS;
use std::{error::Error as StdError, time::Duration};

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
                return Err("Ability action string is invalid.".into());
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
                            return Ok(Box::new(actions::ability::Ability {
                                ability: AbilityType::Coords(Coords {
                                    x: coords[0],
                                    y: coords[1],
                                }),
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
            // Determine if the sleep string is a number, or a number + unit (e.g. "10s", "2m"), using regex
            let time_re = regex::Regex::new(r"^(\d+)(ms|s|m|h|d)?$").unwrap();
            // If the regex is not matched, return an error
            if !time_re.is_match(action_array[1]) {
                return Err("Invalid sleep time format".into());
            }
            // Split the incoming string into the number and unit (default to milliseconds if no unit is provided)
            let re_match = time_re.captures(action_array[1]).unwrap();
            let sleep_time = re_match
                .get(1)
                .and_then(|s| s.as_str().parse::<u64>().ok())
                .ok_or("Invalid sleep time format")?;
            let sleep_units = re_match.get(2).map(|s| s.as_str());

            let sleep_duration = match sleep_units {
                None | Some("ms") => Duration::from_millis(sleep_time),
                Some("s") => Duration::from_secs(sleep_time),
                Some("m") => Duration::from_secs(sleep_time * 60),
                Some("h") => Duration::from_secs(sleep_time * 60 * 60),
                Some("d") => Duration::from_secs(sleep_time * 60 * 60 * 24),
                _ => {
                    return Err("Invalid sleep units".into());
                }
            };

            return Ok(Box::new(actions::sleep::Sleep {
                sleep_time: sleep_duration,
            }));
        }
        "start" => {
            if action_array.len() > 2 {
                return Err(
                    "Start action can only take one optional argument: 'fast-forward'.".into(),
                );
            }
            if action_array.len() == 2 && action_array[1] != "fast-forward" {
                return Err("Start action's only optional argument is 'fast-forward'.".into());
            }
            return Ok(Box::new(actions::start::Start {
                fast_forward: Some(action_array.len() == 2 && action_array[1] == "fast-forward"),
            }));
        }
        "repeat" => {
            if action_array.len() < 3 {
                return Err("Repeat action requires at least an interval and an action.".into());
            }
            // Parse the interval using the same format as sleep
            let time_re = regex::Regex::new(r"^(\d+)(ms|s|m|h|d)?$").unwrap();
            if !time_re.is_match(action_array[1]) {
                return Err("Invalid repeat interval format.".into());
            }
            let re_match = time_re.captures(action_array[1]).unwrap();
            let interval_value = re_match
                .get(1)
                .and_then(|s| s.as_str().parse::<u64>().ok())
                .ok_or("Invalid repeat interval format.")?;
            let interval_units = re_match.get(2).map(|s| s.as_str());
            let interval = match interval_units {
                None | Some("ms") => Duration::from_millis(interval_value),
                Some("s") => Duration::from_secs(interval_value),
                Some("m") => Duration::from_secs(interval_value * 60),
                Some("h") => Duration::from_secs(interval_value * 60 * 60),
                Some("d") => Duration::from_secs(interval_value * 60 * 60 * 24),
                _ => return Err("Invalid repeat interval units.".into()),
            };

            // Check if the next token is a plain integer (optional repeat count)
            let (count, action_start) =
                if let Ok(n) = action_array[2].parse::<u64>() {
                    (Some(n), 3)
                } else {
                    (None, 2)
                };

            if action_array.len() <= action_start {
                return Err("Repeat action requires an action to repeat.".into());
            }
            let inner_action_str = action_array[action_start..].join(" ");
            let inner_action = parse_action(&inner_action_str)?;

            return Ok(Box::new(actions::repeat::Repeat {
                interval,
                action: inner_action,
                count,
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
