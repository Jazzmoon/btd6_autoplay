use enigo::Key;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::ops::Index;

lazy_static! {
    static ref HOTKEY_KEYS: Vec<&'static str> = vec![
        "start",
        "pause",
        "menu",
        "sell",
        "upgrade_top_path",
        "upgrade_middle_path",
        "upgrade_bottom_path",
        "dart",
        "boomerang",
        "bomb",
        "tack",
        "ice",
        "glue",
        "sniper",
        "sub",
        "boat",
        "ace",
        "heli",
        "mortar",
        "dartling",
        "wizard",
        "super_monkey",
        "ninja",
        "alchemist",
        "druid",
        "mermonkey",
        "farm",
        "spike",
        "village",
        "engineer",
        "beast",
        "hero",
        "target_priority_right",
        "target_priority_left",
        "target_priority_special",
        "send_next_round",
        "road_spikes",
        "moab_mine",
        "glue_trap",
        "camo_trap",
        "banana_farmer",
        "tech_bot",
        "energizing_totem",
        "pontoon",
        "portable_lake",
        "super_monkey_storm",
        "monkey_boost",
        "thrive",
        "time_stop",
        "cash_drop",
        "copy",
    ];
}

pub fn string_to_key(key: String) -> Key {
    // Check length of key
    match key.to_ascii_lowercase().as_str() {
        "alt" => Key::Alt,
        "backspace" => Key::Backspace,
        "cancel" => Key::Cancel,
        "caps_lock" => Key::CapsLock,
        "clear" => Key::Clear,
        "control" => Key::Control,
        "delete" => Key::Delete,
        "down_arrow" => Key::DownArrow,
        "end" => Key::End,
        "escape" => Key::Escape,
        "execute" => Key::Execute,
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        "f13" => Key::F13,
        "f14" => Key::F14,
        "f15" => Key::F15,
        "f16" => Key::F16,
        "f17" => Key::F17,
        "f18" => Key::F18,
        "f19" => Key::F19,
        "f20" => Key::F20,
        "f21" => Key::F21,
        "f22" => Key::F22,
        "f23" => Key::F23,
        "f24" => Key::F24,
        "hangul" => Key::Hangul,
        "hanja" => Key::Hanja,
        "help" => Key::Help,
        "home" => Key::Home,
        "insert" => Key::Insert,
        "kanji" => Key::Kanji,
        "l_control" => Key::LControl,
        "left_arrow" => Key::LeftArrow,
        "l_menu" => Key::LMenu,
        "l_shift" => Key::LShift,
        "media_next_track" => Key::MediaNextTrack,
        "media_play_pause" => Key::MediaPlayPause,
        "media_prev_track" => Key::MediaPrevTrack,
        "media_stop" => Key::MediaStop,
        "meta" => Key::Meta,
        "mode_change" => Key::ModeChange,
        "numlock" => Key::Numlock,
        "option" => Key::Option,
        "page_down" => Key::PageDown,
        "page_up" => Key::PageUp,
        "pause" => Key::Pause,
        "print" => Key::Print,
        "r_control" => Key::RControl,
        "return" => Key::Return,
        "right_arrow" => Key::RightArrow,
        "r_shift" => Key::RShift,
        "select" => Key::Select,
        "shift" => Key::Shift,
        "space" => Key::Space,
        "tab" => Key::Tab,
        "up_arrow" => Key::UpArrow,
        "volume_down" => Key::VolumeDown,
        "volume_mute" => Key::VolumeMute,
        "volume_up" => Key::VolumeUp,
        _ => {
            if key.len() == 1 {
                Key::Unicode(key.chars().next().unwrap())
            } else {
                panic!("Invalid keybinding detected. Please create a new issue on the GitHub repository if you believe this key should be supported.")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hotkey(Vec<String>);

impl Into<Vec<Key>> for Hotkey {
    fn into(self) -> Vec<Key> {
        self.0
            .iter()
            .map(|x| string_to_key(x.to_string()))
            .collect()
    }
}

impl From<Vec<String>> for Hotkey {
    fn from(keys: Vec<String>) -> Self {
        Hotkey(keys)
    }
}

impl From<&Vec<String>> for Hotkey {
    fn from(keys: &Vec<String>) -> Self {
        Hotkey(keys.iter().map(|x| x.to_string()).collect())
    }
}

impl From<Vec<&str>> for Hotkey {
    fn from(keys: Vec<&str>) -> Self {
        Hotkey(keys.iter().map(|x| x.to_string()).collect())
    }
}

impl From<&Vec<&str>> for Hotkey {
    fn from(keys: &Vec<&str>) -> Self {
        Hotkey(keys.iter().map(|x| x.to_string()).collect())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hotkeys {
    pub start: Hotkey,
    pub pause: Hotkey,
    pub menu: Hotkey,
    pub sell: Hotkey,
    pub upgrade_top_path: Hotkey,
    pub upgrade_middle_path: Hotkey,
    pub upgrade_bottom_path: Hotkey,
    pub dart: Hotkey,
    pub boomerang: Hotkey,
    pub bomb: Hotkey,
    pub tack: Hotkey,
    pub ice: Hotkey,
    pub glue: Hotkey,
    pub sniper: Hotkey,
    pub sub: Hotkey,
    pub boat: Hotkey,
    pub ace: Hotkey,
    pub heli: Hotkey,
    pub mortar: Hotkey,
    pub dartling: Hotkey,
    pub wizard: Hotkey,
    pub super_monkey: Hotkey,
    pub ninja: Hotkey,
    pub alchemist: Hotkey,
    pub druid: Hotkey,
    pub mermonkey: Hotkey,
    pub farm: Hotkey,
    pub spike: Hotkey,
    pub village: Hotkey,
    pub engineer: Hotkey,
    pub beast: Hotkey,
    pub hero: Hotkey,
    pub target_priority_right: Hotkey,
    pub target_priority_left: Hotkey,
    pub target_priority_special: Hotkey,
    pub send_next_round: Hotkey,
    pub road_spikes: Hotkey,
    pub moab_mine: Hotkey,
    pub glue_trap: Hotkey,
    pub camo_trap: Hotkey,
    pub banana_farmer: Hotkey,
    pub tech_bot: Hotkey,
    pub energizing_totem: Hotkey,
    pub pontoon: Hotkey,
    pub portable_lake: Hotkey,
    pub super_monkey_storm: Hotkey,
    pub monkey_boost: Hotkey,
    pub thrive: Hotkey,
    pub time_stop: Hotkey,
    pub cash_drop: Hotkey,
    pub copy: Hotkey,
}

impl Index<&str> for Hotkeys {
    type Output = Hotkey;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "start" => &self.start,
            "pause" => &self.pause,
            "menu" => &self.menu,
            "sell" => &self.sell,
            "upgrade_top_path" => &self.upgrade_top_path,
            "upgrade_middle_path" => &self.upgrade_middle_path,
            "upgrade_bottom_path" => &self.upgrade_bottom_path,
            "dart" => &self.dart,
            "boomerang" => &self.boomerang,
            "bomb" => &self.bomb,
            "tack" => &self.tack,
            "ice" => &self.ice,
            "glue" => &self.glue,
            "sniper" => &self.sniper,
            "sub" => &self.sub,
            "boat" => &self.boat,
            "ace" => &self.ace,
            "heli" => &self.heli,
            "mortar" => &self.mortar,
            "dartling" => &self.dartling,
            "wizard" => &self.wizard,
            "super_monkey" => &self.super_monkey,
            "ninja" => &self.ninja,
            "alchemist" => &self.alchemist,
            "druid" => &self.druid,
            "mermonkey" => &self.mermonkey,
            "farm" => &self.farm,
            "spike" => &self.spike,
            "village" => &self.village,
            "engineer" => &self.engineer,
            "beast" => &self.beast,
            "hero" => &self.hero,
            "target_priority_right" => &self.target_priority_right,
            "target_priority_left" => &self.target_priority_left,
            "target_priority_special" => &self.target_priority_special,
            "send_next_round" => &self.send_next_round,
            "road_spikes" => &self.road_spikes,
            "moab_mine" => &self.moab_mine,
            "glue_trap" => &self.glue_trap,
            "camo_trap" => &self.camo_trap,
            "banana_farmer" => &self.banana_farmer,
            "tech_bot" => &self.tech_bot,
            "energizing_totem" => &self.energizing_totem,
            "pontoon" => &self.pontoon,
            "portable_lake" => &self.portable_lake,
            "super_monkey_storm" => &self.super_monkey_storm,
            "monkey_boost" => &self.monkey_boost,
            "thrive" => &self.thrive,
            "time_stop" => &self.time_stop,
            "cash_drop" => &self.cash_drop,
            "copy" => &self.copy,
            _ => panic!("Invalid hotkey index"),
        }
    }
}

impl Hotkeys {
    pub fn contains_key(&self, key: &str) -> bool {
        HOTKEY_KEYS.contains(&key)
    }

    // Return the key that contains the value if it exists, otherwise None
    pub fn contains_value(&self, value: &Hotkey) -> Option<String> {
        for key in HOTKEY_KEYS.iter() {
            if self[key].0 == value.0 {
                return Some(key.to_string());
            }
        }
        None
    }

    pub fn get(&self, key: &str) -> Hotkey {
        self[key].clone()
    }
}
