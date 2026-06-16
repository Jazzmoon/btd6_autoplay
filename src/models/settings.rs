use super::coords::{Coords, CoordsArea};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub victory_banner: CoordsArea,
    pub defeat_banner: CoordsArea,
    pub insta_monkey_banner: CoordsArea,
    pub next_button: Coords,
    pub freeplay_button: Coords,
    pub freeplay_ok_button: Coords,
    pub restart_game_button: Coords,
    pub confirm_button: Coords,
    pub home_button: Coords,
    pub round_counter: CoordsArea,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub game: Game,
    pub screen: CoordsArea,
}
