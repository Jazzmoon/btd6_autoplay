use crate::models::Coords;

#[derive(Serialize, Deserialize)]
struct Game {
	VictoryBanner: CoordsArea,
	DefeatBanner:  CoordsArea,
	HoverLocation: Coords,
	NextButton: Coords,
	FreeplayButton: Coords,
	FreeplayOkButton: Coords,
	RestartButton: Coords,
	ConfirmButton: Coords,
	HomeButton: Coords,
	RoundCounter: CoordsArea
}

#[derive(Serialize, Deserialize)]
struct Settings {
	Game: Game,
	Screen: CoordsArea
}