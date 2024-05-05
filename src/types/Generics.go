package types

type Coords struct {
	X int `yaml:"x,omitempty"`
	Y int `yaml:"y,omitempty"`
}

type CoordsArea struct {
	X int `yaml:"x,omitempty"`
	Y int `yaml:"y,omitempty"`
	W int `yaml:"w,omitempty"`
	H int `yaml:"h,omitempty"`
}
