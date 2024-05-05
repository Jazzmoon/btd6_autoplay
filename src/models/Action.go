package models

import (
	"Jazzmoon/btd6_autoplay/models/actions"
	"strings"
)

type ActionInterface interface {
	Run() (err error)
}

func ActionFromString(aString string) ActionInterface {

	stringArray := strings.Split(aString, "")
	if len(stringArray) == 0 {
		return nil
	}

	switch stringArray[0] {
	case "ability":
		return actions.Ability{AbilityNumber: stringArray[1]}
	case "click":
		return actions.Click{X: stringArray[1], Y: stringArray[2]}
	case "hover":
		return actions.Hover{X: stringArray[1], Y: stringArray[2]}
	case "obstacle":
	case "clear":
		return actions.Obstacle{X: stringArray[1], Y: stringArray[2]}
	case "place":
		return actions.Place{Tower: stringArray[1]}
	case "sell":
		return actions.Sell{Tower: stringArray[1]}
	case "sleep":
		return actions.Sleep{Duration: stringArray[1]}
	case "start":
		return actions.Start{}
	case "upgrade":
		return actions.Upgrade{Tower: stringArray[1], Upgrade: stringArray[2]}
	default:
		return nil
	}
}
