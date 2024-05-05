package actions

import (
	"errors"
)

type Click struct {
	X int
	Y int
}

func (c *Click) Run() error {

	return errors.New("[Action | Click] Not implemented")
}
