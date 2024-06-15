use serde::{Serialize, Deserialize};
use crate::models::Coords;
use crate::models::CoordsArea;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
	pub victory_banner: CoordsArea,
	pub defeat_banner:  CoordsArea,
	pub hover_location: Coords,
	pub next_button: Coords,
	pub freeplay_button: Coords,
	pub freeplay_ok_button: Coords,
	pub restart_button: Coords,
	pub confirm_button: Coords,
	pub home_button: Coords,
	pub round_counter: CoordsArea
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
	pub game: Game,
	pub screen: CoordsArea
}