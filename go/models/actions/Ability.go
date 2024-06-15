package actions

import (
	"Jazzmoon/btd6_autoplay/utils"
)

type Ability struct {
	AbilityKey []string
}

func (a Ability) Run() error {
	return utils.UseAbility(a.AbilityKey...)
}
