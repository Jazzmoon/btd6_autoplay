use super::{actions, actions::ability::AbilityType, coords::Coords, hotkeys::Hotkey};
use crate::utils::global::HOTKEYS;
use std::{error::Error as StdError, time::Duration};

pub trait ActionTrait {
    fn run(&self) -> Result<(), Box<dyn StdError>>;
}

/// Parse a slice of string tokens as exactly two `i32` coordinates and return a `Coords`.
fn parse_two_coords(args: &[&str]) -> Result<Coords, Box<dyn StdError>> {
    let nums: Vec<i32> = args
        .iter()
        .map(|x| x.parse::<i32>())
        .collect::<Result<_, _>>()?;
    match nums.as_slice() {
        [x, y] => Ok(Coords { x: *x, y: *y }),
        _ => Err("Expected exactly 2 coordinates.".into()),
    }
}

pub fn parse_action(action: &str) -> Result<Box<dyn ActionTrait>, Box<dyn StdError>> {
    let action_array: Vec<&str> = action.split(' ').collect();
    if action_array.is_empty() {
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
            let args = &action_array[1..];
            if args.is_empty() {
                return Err("Ability action string is invalid.".into());
            }
            // Attempt to convert the ability hotkey to a Coords struct or Hotkey struct
            if args.len() == 2 {
                // Try parse as two integers
                if let Ok(coords) = parse_two_coords(args) {
                    return Ok(Box::new(actions::ability::Ability {
                        ability: AbilityType::Coords(coords),
                    }));
                }
            }
            // Check if it is a hotkey in our struct
            let hotkeys_read_lock = HOTKEYS.read().unwrap();
            let hotkeys = hotkeys_read_lock.as_ref().ok_or("Hotkeys not loaded.")?;
            if hotkeys.contains_key(args[0]) {
                return Ok(Box::new(actions::ability::Ability {
                    ability: AbilityType::Hotkey(hotkeys.get(args[0])),
                }));
            }
            // Array of tokens is just a series of keys to press
            Ok(Box::new(actions::ability::Ability {
                ability: AbilityType::Hotkey(Hotkey::from(args)),
            }))
        }
        "click" => {
            let coords = parse_two_coords(&action_array[1..])?;
            Ok(Box::new(actions::click::Click { coords }))
        }
        "hover" => {
            let coords = parse_two_coords(&action_array[1..])?;
            Ok(Box::new(actions::hover::Hover { coords }))
        }
        "obstacle" | "clear" => {
            let coords = parse_two_coords(&action_array[1..])?;
            Ok(Box::new(actions::obstacle::Obstacle { coords }))
        }
        "place" => {
            let args = &action_array[1..];
            let tower_name = args[0];
            let tower_type_name = args[1];
            let coords = parse_two_coords(&args[2..])?;

            let tower_hotkey = {
                let hotkeys_read_lock = HOTKEYS.read().unwrap();
                let hotkeys = hotkeys_read_lock.as_ref();
                if hotkeys.is_none() || !hotkeys.unwrap().contains_key(tower_type_name) {
                    return Err("Tower type not found.".into());
                }
                hotkeys.unwrap().get(tower_type_name)
            };

            Ok(Box::new(actions::place::Place {
                tower_name: tower_name.to_string(),
                tower_type_name: tower_type_name.to_string(),
                coords,
                tower_hotkey,
            }))
        }
        "sell" => Ok(Box::new(actions::sell::Sell {
            tower: action_array[1].to_string(),
        })),
        "sleep" => {
            // Determine if the sleep string is a number, or a number + unit (e.g. "10s", "2m"), using regex
            let time_re = regex::Regex::new(r"^(\d+)(ms|s|m|h|d)?$").unwrap();
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
                _ => return Err("Invalid sleep units".into()),
            };

            Ok(Box::new(actions::sleep::Sleep {
                sleep_time: sleep_duration,
            }))
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
            Ok(Box::new(actions::start::Start {
                fast_forward: Some(action_array.len() == 2 && action_array[1] == "fast-forward"),
            }))
        }
        "upgrade" => {
            let args = &action_array[1..];
            Ok(Box::new(actions::upgrade::Upgrade {
                tower: args[0].to_string(),
                upgrade_path: args[1].to_string(),
            }))
        }
        _ => Err(std::io::Error::other("Unknown action").into()),
    }
}
