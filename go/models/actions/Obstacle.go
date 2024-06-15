package actions

import (
	"Jazzmoon/btd6_autoplay/types"
	"Jazzmoon/btd6_autoplay/utils"
)

type Obstacle struct {
	X int
	Y int
}

func (action Obstacle) Run() error {
	return utils.ClearObstacle(action.X, action.Y, types.Settings.Game.ConfirmButton.X, types.Settings.Game.ConfirmButton.Y)
}
