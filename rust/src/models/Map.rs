use std::collections::HashMap;
use crate::models::Coords;
use crate::models::Tower;

enum OnWinAction {
    Continue,
    Restart,
    EndGame,
};

enum RoundCounterMode {
    Dark,
    Light,
};


#[derive(Serialize, Deserialize)]
struct Map {
    MoneyPerGame: i32,
    OnWinAction: OnWinAction,
    RestartOnRound: i32,
    Instructions: HashMap<i32, Vec<String>>,
    HoverLocation: Coords,
    Towers: HashMap<String, Tower>,
};


#[derive(Serialize, Deserialize)]
struct MapConfigDifficulty {
    Standard: Maybe<Map>,
    Sandbox: Maybe<Map>,
    PrimaryOnly: Maybe<Map>,
    Deflation: Maybe<Map>,
    MilitaryOnly: Maybe<Map>,
    Apopalypse: Maybe<Map>,
    Reverse: Maybe<Map>,
    MagicMonkeysOnly: Maybe<Map>,
    DoubleHPMOABs: Maybe<Map>,
    HalfCash: Maybe<Map>,
    AlternateBloonsRounds: Maybe<Map>,
    Impoppable: Maybe<Map>,
    CHIMPS: Maybe<Map>,
};

#[derive(Serialize, Deserialize)]
struct MapConfig {
    RoundCounterMode: RoundCounterMode,
    Easy: Maybe<MapConfigDifficulty>,
    Medium: Maybe<MapConfigDifficulty>,
    Hard: Maybe<MapConfigDifficulty>,
};