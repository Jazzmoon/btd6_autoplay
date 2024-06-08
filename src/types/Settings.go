package types

type Game struct {
	VictoryBanner CoordsArea `yaml:"victoryBanner"`
	DefeatBanner  CoordsArea `yaml:"defeatBanner"`

	HoverLocation    Coords `yaml:"hoverLocation"`
	NextButton       Coords `yaml:"nextButton"`
	FreeplayButton   Coords `yaml:"freeplayButton"`
	FreeplayOkButton Coords `yaml:"freeplayOkButton"`
	RestartButton    Coords `yaml:"restartButton"`
	ConfirmButton    Coords `yaml:"confirmButton"`
	HomeButton       Coords `yaml:"homeButton"`

	RoundCounter CoordsArea `yaml:"roundCounter"`
}

type Settings_Type struct {
	Game   Game       `yaml:"game"`
	Screen CoordsArea `yaml:"screen"`
}
