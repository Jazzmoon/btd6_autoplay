package types

type Game struct {
	StartHotkey string `yaml:"startHotkey"`
	MenuHotkey  string `yaml:"menuHotkey"`
	SellHotkey  string `yaml:"sellHotkey"`

	UpgradeTopPathHotkey    string `yaml:"upgradeTopPathHotkey"`
	UpgradeMiddlePathHotkey string `yaml:"upgradeMiddlePathHotkey"`
	UpgradeBottomPathHotkey string `yaml:"upgradeBottomPathHotkey"`

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

type ProgressBar struct {
	Prefix    string `yaml:"prefix"`
	Suffix    string `yaml:"suffix"`
	Completed string `yaml:"completed"`
	Null      string `yaml:"null"`
}

type Settings_Type struct {
	Game        Game        `yaml:"game"`
	ProgressBar ProgressBar `yaml:"progressBar"`
}
