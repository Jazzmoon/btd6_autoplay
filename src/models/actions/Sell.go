package actions

import (
	"Jazzmoon/btd6_autoplay/types"
)

type Sell struct {
	Tower *types.Tower
}

func (action Sell) Run() error {
	return action.Tower.Sell()
}
