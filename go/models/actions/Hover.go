package actions

import (
	"Jazzmoon/btd6_autoplay/utils"
)

type Hover struct {
	X int
	Y int
}

func (c Hover) Run() error {
	return utils.SleepMove(-1, c.X, c.Y)
}
