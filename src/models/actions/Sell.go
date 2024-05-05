package actions

import (
	"Jazzmoon/btd6_autoplay/types"
	"errors"
)

type Sell struct {
	Tower types.Tower
}

func (c *Sell) Run() error {

	return errors.New("[Action | Click] Not implemented")
}
