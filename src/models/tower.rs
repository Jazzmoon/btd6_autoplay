use crate::models::coords::Coords;
use serde::{Deserialize, Serialize};

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TowerType {
    S(String),
    V(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tower {
    pub name: String,
    pub hotkey: Vec<String>,
    pub coords: Coords,
    pub upgrade_path: [i32; 3],
}

impl Tower {
    fn new(name: String, hotkey: TowerType, coords: Coords, path: Option<[i32; 3]>) -> Tower {
        let default_path = [0, 0, 0];
        let hotkey = match hotkey {
            TowerType::S(s) => {
                // Todo: Implement utalizing configs to get this information
                let mut v = Vec::new();
                v.push(s);
                v
            }
            TowerType::V(v) => v,
        };
        Tower {
            name,
            hotkey,
            coords,
            upgrade_path: path.unwrap_or(default_path),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "Tower: {}, Hotkey: {:?}, Coords: {:?}, Path: {:?}",
            self.name, self.hotkey, self.coords, self.upgrade_path
        )
    }
}
