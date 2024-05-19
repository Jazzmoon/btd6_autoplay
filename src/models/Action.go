package models

import (
	"Jazzmoon/btd6_autoplay/models/actions"
	"Jazzmoon/btd6_autoplay/types"
	"errors"
	"regexp"
	"strconv"
	"strings"
	"time"
)

type ActionInterface interface {
	Run() (err error)
}

func ActionFromString(aString string) (ActionInterface, error) {
	stringArray := strings.Split(aString, "")
	if len(stringArray) == 0 {
		return nil, errors.New("empty string cannot be converted to action")
	}

	switch stringArray[0] {
	case "ability":
		if len(stringArray) < 1 || len(stringArray) > 2 {
			return nil, errors.New("invalid ability action (expected either a hotkey or an X and Y coordinate)")
		}
		return actions.Ability{AbilityKey: stringArray}, nil
	case "click":
		if len(stringArray) != 3 {
			return nil, errors.New("invalid click action")
		}
		x, err := strconv.Atoi(stringArray[1])
		if err != nil {
			return nil, errors.New("invalid x coordinate")
		}
		y, err := strconv.Atoi(stringArray[2])
		if err != nil {
			return nil, errors.New("invalid y coordinate")
		}
		return actions.Click{X: x, Y: y}, nil
	case "hover":
		if len(stringArray) != 3 {
			return nil, errors.New("invalid click action")
		}
		x, err := strconv.Atoi(stringArray[1])
		if err != nil {
			return nil, errors.New("invalid x coordinate")
		}
		y, err := strconv.Atoi(stringArray[2])
		if err != nil {
			return nil, errors.New("invalid y coordinate")
		}
		return actions.Hover{X: x, Y: y}, nil
	case "obstacle", "clear":
		x, err := strconv.Atoi(stringArray[1])
		if err != nil {
			return nil, errors.New("invalid x coordinate")
		}
		y, err := strconv.Atoi(stringArray[2])
		if err != nil {
			return nil, errors.New("invalid y coordinate")
		}
		return actions.Obstacle{X: x, Y: y}, nil
	case "place":
		tower, ok := types.CurrentMap.Towers[stringArray[1]]
		if !ok {
			return nil, errors.New("invalid tower")
		}
		return actions.Place{Tower: tower}, nil
	case "sell":
		tower, ok := types.CurrentMap.Towers[stringArray[1]]
		if !ok {
			return nil, errors.New("invalid tower")
		}
		return actions.Sell{Tower: tower}, nil
	case "sleep":
		// Match the string against "(\d+)(m?s?)?" and extract the groups
		re := regexp.MustCompile(`(\d+)(m?s)?`)
		matches := re.FindStringSubmatch(aString)
		if len(matches) < 3 {
			return nil, errors.New("invalid sleep duration provided (should match \"(\\d+)(m?s?)?\" regular expression, eg: 100ms, 1s, 2)")
		}
		duration, err := strconv.Atoi(matches[1])
		if err != nil {
			return nil, err
		}
		if len(matches) < 3 {
			return actions.Sleep{Duration: time.Duration(duration) * time.Second}, nil
		}
		var timeDuration time.Duration
		switch matches[2] {
		case "ms":
			timeDuration = time.Duration(duration) * time.Millisecond
		default:
			timeDuration = time.Duration(duration) * time.Second
		}
		return actions.Sleep{Duration: timeDuration}, nil
	case "start":
		return actions.Start{}, nil
	case "upgrade":
		tower, ok := types.CurrentMap.Towers[stringArray[1]]
		if !ok {
			return nil, errors.New("invalid tower")
		}
		return actions.Upgrade{Tower: tower, Upgrade: stringArray[2]}, nil
	default:
		return nil, errors.New("invalid action")
	}
}
