use crate::models::Coords;

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
    }
}

enum TowerType {
    S(String),
    V(Vec<String>),
}

struct Tower {
    name: String,
    hotkey: Vec<String>,
    coords: Coords,
    path: [int32; 3]
}

impl Tower {
    fn new(name: String, hotkey: TowerType, coords: Coords, path: Option<[int32; 3]>) -> Tower {
        let default_path = [0, 0, 0];
        let hotkey = match hotkey {
            TowerType::S(s) => {
                // Todo: Implement utalizing configs to get this information
                let mut v = Vec::new();
                v.push(s);
                v
            },
            TowerType::V(v) => v,
        };
        Tower { name, hotkey, coords, path: path.unwrap_or(default_path)}
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_hotkey(&self) -> &Vec<String> {
        &self.hotkey
    }

    fn get_coords(&self) -> &Coords {
        &self.coords
    }

    fn get_path(&self) -> &[int32; 3] {
        &self.path
    }

    fn set_path(&mut self, path: [int32; 3]) {
        self.path = path;
    }

    fn set_coords(&mut self, coords: Coords) {
        self.coords = coords;
    }

    fn set_hotkey(&mut self, hotkey: Vec<String>) {
        self.hotkey = hotkey;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn to_string(&self) -> String {
        format!("Tower: {}, Hotkey: {:?}, Coords: {:?}, Path: {:?}", self.name, self.hotkey, self.coords, self.path)
    }
}