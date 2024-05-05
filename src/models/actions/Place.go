package actions

import (
	"Jazzmoon/btd6_autoplay/types"
	"errors"
)

type Place struct {
	Tower types.Tower
}

func (c *Place) Run() error {

	return errors.New("[Action | Click] Not implemented")
}
