package actions

import (
	"errors"
)

type Hover struct {
	X int
	Y int
}

func (c *Hover) Run() error {

	return errors.New("[Action | Click] Not implemented")
}
