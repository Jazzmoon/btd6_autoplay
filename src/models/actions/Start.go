package actions

import (
	"Jazzmoon/btd6_autoplay/types"
	"errors"

	"github.com/go-vgo/robotgo"
)

type Start struct {
}

func (s Start) Run() error {
	// Get the start hotkey from the settings in defs
	if types.Settings.Game.StartHotkey == "" {
		return errors.New("[Action | Start] No start hotkey defined in settings")
	}
	keyDelay := robotgo.KeySleep
	robotgo.KeySleep = 2                                   // Await 2 milliseconds between key presses to prevent the game from not registering the key presses
	err := robotgo.KeyTap(types.Settings.Game.StartHotkey) // Press the start hotkey
	if err != nil {
		return err
	}
	err = robotgo.KeyTap(types.Settings.Game.StartHotkey) // Press it again to fast forward the game
	if err != nil {
		return err
	}
	robotgo.KeySleep = keyDelay
	return nil
}
