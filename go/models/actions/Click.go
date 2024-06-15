package actions

import (
	"Jazzmoon/btd6_autoplay/utils"
)

type Click struct {
	X int
	Y int
}

func (c Click) Run() error {
	return utils.SleepMoveAndClick(-1, c.X, c.Y)
}
