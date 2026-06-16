use super::{coords::Coords, tower::Tower};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use struct_iterable::Iterable;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum OnWinAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "restart")]
    Restart,
    #[serde(rename = "end_game")]
    EndGame,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RoundCounterMode {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub money_per_game: Option<i32>,
    pub on_win_action: OnWinAction,
    pub restart_on_round: Option<i32>,
    pub instructions: HashMap<i32, Vec<String>>,
    pub hover_location: Coords,
    #[serde(default, skip)]
    pub towers: HashMap<String, Tower>,
}

#[derive(Serialize, Deserialize, Clone, Iterable, Debug)]
pub struct MapConfigGamemodes {
    pub standard: Option<Map>,
    pub sandbox: Option<Map>,
    pub primary_only: Option<Map>,
    pub deflation: Option<Map>,
    pub military_only: Option<Map>,
    pub apopalypse: Option<Map>,
    pub reverse: Option<Map>,
    pub magic_monkeys_only: Option<Map>,
    pub double_hpmoa_bs: Option<Map>,
    pub half_cash: Option<Map>,
    pub alternate_bloons_rounds: Option<Map>,
    pub impoppable: Option<Map>,
    pub chimps: Option<Map>,
}

impl MapConfigGamemodes {
    pub fn implemented_gamemodes(&self) -> Vec<String> {
        let mut gamemodes = Vec::new();
        for (field_name, field_value) in self.iter() {
            if let Some(_value) = field_value.downcast_ref::<Option<Map>>() {
                if _value.is_some() {
                    gamemodes.push(field_name.to_string());
                }
            }
        }
        gamemodes
    }

    pub fn get(&self, gamemode: &str) -> Option<&Map> {
        for (field_name, field_value) in self.iter() {
            if field_name.eq(gamemode) {
                if let Some(_value) = field_value.downcast_ref::<Option<Map>>() {
                    return _value.as_ref();
                }
            }
        }
        None
    }
}

fn default_round_counter_mode() -> RoundCounterMode {
    RoundCounterMode::Light
}

#[derive(Serialize, Deserialize, Clone, Iterable, Debug)]
pub struct MapConfig {
    #[serde(default = "default_round_counter_mode")]
    pub round_counter_mode: RoundCounterMode,
    pub easy: Option<MapConfigGamemodes>,
    pub medium: Option<MapConfigGamemodes>,
    pub hard: Option<MapConfigGamemodes>,
}

impl MapConfig {
    pub fn implemented_difficulties(&self) -> Vec<String> {
        let mut difficulties = Vec::new();
        for (field_name, field_value) in self.iter() {
            if let Some(_value) = field_value.downcast_ref::<Option<MapConfigGamemodes>>() {
                if _value.is_some() {
                    difficulties.push(field_name.to_string());
                }
            }
        }
        difficulties
    }

    pub fn get(&self, difficulty: &str) -> Option<&MapConfigGamemodes> {
        for (field_name, field_value) in self.iter() {
            if field_name.eq(difficulty) {
                if let Some(_value) = field_value.downcast_ref::<Option<MapConfigGamemodes>>() {
                    return _value.as_ref();
                }
            }
        }
        None
    }
}
