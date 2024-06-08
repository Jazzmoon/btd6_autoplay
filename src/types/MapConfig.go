package types

type OnWinAction string

const (
	Continue OnWinAction = "continue"
	Restart  OnWinAction = "restart"
)

type MapConfigMap struct {
	MoneyPerGame   int              `yaml:"moneyPerGame,omitempty"`
	OnWinActions   OnWinAction      `yaml:"onWinAction,omitempty"`
	RestartOnRound *int             `yaml:"restartOnRound,omitempty"`
	Instructions   map[int][]string `yaml:"instructions"`
	HoverLocation  Coords           `yaml:"hoverLocation"`

	// Used as a map for the placed towers
	Towers map[interface{}]*Tower
}

type MapConfigDifficulty struct {
	Standard              *MapConfigMap `yaml:"standard,omitempty"`
	Sandbox               *MapConfigMap `yaml:"sandbox,omitempty"`
	PrimaryOnly           *MapConfigMap `yaml:"primaryOnly,omitempty"`
	Deflation             *MapConfigMap `yaml:"deflation,omitempty"`
	MilitaryOnly          *MapConfigMap `yaml:"militaryOnly,omitempty"`
	Apopalypse            *MapConfigMap `yaml:"apopalypse,omitempty"`
	Reverse               *MapConfigMap `yaml:"reverse,omitempty"`
	MagicMonkeysOnly      *MapConfigMap `yaml:"magicMonkeysOnly,omitempty"`
	DoubleHPMOABs         *MapConfigMap `yaml:"doubleHPMOABs,omitempty"`
	HalfCash              *MapConfigMap `yaml:"halfCash,omitempty"`
	AlternateBloonsRounds *MapConfigMap `yaml:"alternateBloonsRounds,omitempty"`
	Impoppable            *MapConfigMap `yaml:"impoppable,omitempty"`
	CHIMPS                *MapConfigMap `yaml:"chimps,omitempty"`
}

type RoundCounterMode string

const (
	Dark  RoundCounterMode = "dark"
	Light RoundCounterMode = "light"
)

type MapConfig_Type struct {
	RoundCounterMode *RoundCounterMode `yaml:"roundCounterMode,omitempty" jazzmoon:"notInSelector"`

	Easy   *MapConfigDifficulty `yaml:"easy,omitempty"`
	Medium *MapConfigDifficulty `yaml:"medium,omitempty"`
	Hard   *MapConfigDifficulty `yaml:"hard,omitempty"`
}
