package actions

import (
	"Jazzmoon/btd6_autoplay/types"
	"errors"

	"github.com/go-vgo/robotgo"
)

type Start struct {
}

func (action Start) Run() error {
	// Get the start hotkey from the settings in defs
	if len(types.Settings.Game.Hotkeys.Start) == 0 {
		return errors.New("[Action | Start] no start hotkey defined in settings")
	}
	keyDelay := robotgo.KeySleep
	robotgo.KeySleep = 2                                                                               // Await 2 milliseconds between key presses to prevent the game from not registering the key presses
	err := robotgo.KeyTap(types.Settings.Game.Hotkeys.Start[0], types.Settings.Game.Hotkeys.Start[1:]) // Press the start hotkey
	if err != nil {
		return err
	}
	err = robotgo.KeyTap(types.Settings.Game.Hotkeys.Start[0], types.Settings.Game.Hotkeys.Start[1:]) // Press it again to fast forward the game
	if err != nil {
		return err
	}
	robotgo.KeySleep = keyDelay
	return nil
}
