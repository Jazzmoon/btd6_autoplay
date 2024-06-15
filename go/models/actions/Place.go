package actions

import (
	"Jazzmoon/btd6_autoplay/types"
)

type Place struct {
	Tower *types.Tower
}

func (action Place) Run() error {
	return action.Tower.Place()
}
