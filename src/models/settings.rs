use super::coords::CoordsArea;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub victory_banner: CoordsArea,
    pub defeat_banner: CoordsArea,
    pub round_counter: CoordsArea,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub game: Game,
    pub screen: CoordsArea,
}
