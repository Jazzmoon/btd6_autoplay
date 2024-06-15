package actions

import (
	"Jazzmoon/btd6_autoplay/types"
)

type Upgrade struct {
	Tower   *types.Tower
	Upgrade string
}

func (action Upgrade) Run() error {
	return action.Tower.Upgrade(action.Upgrade)
}
