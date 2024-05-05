package actions

import (
	"errors"
)

type Sleep struct {
	Duration int
}

func (c *Sleep) Run() error {

	return errors.New("[Action | Click] Not implemented")
}
