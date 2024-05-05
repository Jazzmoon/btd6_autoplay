package actions

import (
	"errors"
)

type Obstacle struct {
	X int
	Y int
}

func (c *Obstacle) Run() error {

	return errors.New("[Action | Click] Not implemented")
}
