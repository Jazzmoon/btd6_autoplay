use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::models::Coords;
use crate::models::Tower;

#[derive(Serialize, Deserialize, Debug)]
pub enum OnWinAction {
    Continue,
    Restart,
    EndGame,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum RoundCounterMode {
    Dark,
    Light,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub money_per_game: i32,
    pub on_win_action: OnWinAction,
    pub restart_on_round: i32,
    pub inpub structions: HashMap<i32, Vec<String>>,
    pub hover_location: Coords,
    pub towers: HashMap<String, Tower>,
};



#[derive(Serialize, Deserialize, Debug)]
pub struct MapConfigDifficulty {
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
};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapConfig {
    pub round_counter_mode: RoundCounterMode,
    pub easy: Option<MapConfigDifficulty>,
    pub medium: Option<MapConfigDifficulty>,
    pub hard: Option<MapConfigDifficulty>,
};