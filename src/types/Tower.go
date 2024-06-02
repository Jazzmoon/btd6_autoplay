package types

import (
	"fmt"
	"regexp"
	"strconv"
	"time"

	"Jazzmoon/btd6_autoplay/utils"

	"github.com/go-vgo/robotgo"
	"gopkg.in/yaml.v3"
)

type Tower struct {
	Name   string   `yaml:"name,omitempty"`
	Hotkey []string `yaml:"hotkey"`
	Coords Coords   `yaml:"coords"`
	Path   [3]int
}

/*
String is a function that returns a string representation of the Tower object
  - @return string: A string that contains the name, hotkey, coordinates, and path of the tower
*/
func (t *Tower) String() string {
	return fmt.Sprintf("Name: %s, Hotkey: %+v, Coords: %v, Path: %v", t.Name, t.Hotkey, t.Coords, t.Path)
}

/*
InitTower is a function that initializes a Tower object with the given name, hotkey, and coordinates
  - @param name: A string that indicates the name of the tower
  - @param hotkey: A string that indicates the hotkey of the tower
  - @param x: An integer that indicates the x-coordinate of the tower
  - @param y: An integer that indicates the y-coordinate of the tower
  - @return Tower: A Tower object that contains the name, hotkey, and coordinates of the tower
*/
func InitTower(name string, hotkey []string, x, y int) Tower {
	return Tower{
		Name:   name,
		Hotkey: hotkey,
		Coords: Coords{
			X: x,
			Y: y,
		},
		Path: [3]int{0, 0, 0},
	}
}

/*
UnmarshalYAML is a function that allows the user to unmarshal a YAML object into a Tower object
  - @param unmarshal: A function that unmarshals the YAML object into a Tower object
  - @return interface{}: An interface that contains the Tower object
  - @return error: An error that indicates why the Tower object was not unmarshaled successfully
*/
func (t *Tower) UnmarshalYAML(value *yaml.Node) error {
	var tower struct {
		Name   string   `yaml:"name"`
		Hotkey []string `yaml:"hotkey"`
		Coords Coords   `yaml:"coords"`
	}
	if err := value.Decode(&tower); err != nil {
		return err
	}
	t.Name = tower.Name
	t.Hotkey = tower.Hotkey
	t.Coords = tower.Coords
	t.Path = [3]int{0, 0, 0}
	return nil
}

/*
Deselect is a function that allows the user to deselect a tower on the screen
  - @return err: An error that indicates why the tower was not deselected successfully
*/
func (t *Tower) Deselect() error {
	// Move to center of screen and click to deselect the tower
	utils.SleepMoveAndClick(2, CurrentMap.HoverLocation.X, CurrentMap.HoverLocation.Y, "left")
	return nil
}

/*
Place is a function that allows the user to place a tower on the screen
  - @return err: An error that indicates why the tower was not placed successfully
*/
func (t *Tower) Place() error {
	mouseSleep, keySleep := robotgo.MouseSleep, robotgo.KeySleep
	robotgo.MouseSleep, robotgo.KeySleep = 2, 2
	robotgo.Move(t.Coords.X, t.Coords.Y)
	if err := robotgo.KeyTap(t.Hotkey[0], t.Hotkey[1:]); err != nil {
		return err
	}
	robotgo.Click("left")
	if err := t.Deselect(); err != nil {
		return err
	}
	robotgo.MouseSleep, robotgo.KeySleep = mouseSleep, keySleep
	return nil
}

/*
Highlight is a function that allows the user to highlight a tower on the screen
  - @param args: An optional boolean that indicates whether to select the tower or not
  - @return err: An error that indicates why the tower was not highlighted successfully
*/
func (t *Tower) Highlight(args ...bool) error {
	if len(args) > 1 {
		return fmt.Errorf("[Tower | %s] Invalid number of arguments. Expected 0 or 1, got %d", t.Name, len(args))
	}
	if len(args) == 1 && args[0] {
		utils.SleepMoveAndClick(2, t.Coords.X, t.Coords.Y, "left")
	} else {
		utils.SleepMove(2, t.Coords.X, t.Coords.Y)
	}
	return nil
}

/*
Upgrade is a function that allows the user to upgrade a tower on the screen
  - @param path: A string that indicates the path of the upgrade to select in the format of `\d-\d-\d` (e.g. `0-0-0`)
  - @return err: An error that indicates why the tower was not upgraded successfully
*/
func (t *Tower) Upgrade(path string) error {
	// First, we parse the upgrade path string, in the format of `\d-\d-\d` (e.g. `0-0-0`) into an array of integers
	desiredPath := [3]int{0, 0, 0}
	upgradePathRE, err := regexp.Compile(`(\d)-(\d)-(\d)`)
	if err != nil {
		return err
	}
	upgradePath := upgradePathRE.FindStringSubmatch(path)
	if upgradePath == nil {
		return fmt.Errorf("[Tower | %s] Failed to match expected path pattern", t.Name)
	}
	// Remove the first element of upgradePath, which is the full match
	upgradePath = upgradePath[1:]

	for i, path := range upgradePath {

		desiredPath[i], err = strconv.Atoi(path)
		if err != nil {
			return err
		}
	}
	// Next, we calculate the difference between the current path and the desired path to determine which upgrade(s) to select
	diff := [3]int{desiredPath[0] - t.Path[0], desiredPath[1] - t.Path[1], desiredPath[2] - t.Path[2]}
	// Finally, we select the upgrade(s) by pressing the corresponding hotkey(s)
	if err := t.Highlight(true); err != nil {
		return err
	}
	time.Sleep(50 * time.Millisecond)

	keySleep := robotgo.KeySleep
	robotgo.KeySleep = 50
	if diff[0] > 0 {
		for i := 0; i < diff[0]; i++ {
			if err := robotgo.KeyTap(Settings.Game.Hotkeys.UpgradeTopPath[0], Settings.Game.Hotkeys.UpgradeTopPath[1:]); err != nil {
				return err
			}
		}
	}
	if diff[1] > 0 {
		for i := 0; i < diff[1]; i++ {
			if err := robotgo.KeyTap(Settings.Game.Hotkeys.UpgradeMiddlePath[0], Settings.Game.Hotkeys.UpgradeMiddlePath[1:]); err != nil {
				return err
			}
		}
	}
	if diff[2] > 0 {
		for i := 0; i < diff[2]; i++ {
			if err := robotgo.KeyTap(Settings.Game.Hotkeys.UpgradeBottomPath[0], Settings.Game.Hotkeys.UpgradeBottomPath[1:]); err != nil {
				return err
			}
		}
	}
	robotgo.KeySleep = keySleep
	// Update the tower's path to reflect the upgrade
	t.Path = desiredPath

	utils.SleepMoveAndClick(1, CurrentMap.HoverLocation.X, CurrentMap.HoverLocation.Y, "left")

	return nil
}

/*
Sell is a function that allows the user to sell a tower on the screen
  - @return err: An error that indicates why the tower was not sold successfully
*/
func (t *Tower) Sell() error {
	if err := t.Highlight(true); err != nil {
		return err
	}
	if err := utils.SleepKeyTap(2, Settings.Game.Hotkeys.Sell[0], Settings.Game.Hotkeys.Sell[1:]); err != nil {
		return err
	}
	if err := t.Deselect(); err != nil {
		return err
	}
	return nil
}

/*
GetPath is a function that returns the path of the tower
  - @return [3]int: An array of integers that contains the path of the tower
*/
func (t *Tower) GetPath() [3]int {
	return t.Path
}

/*
Reset is a function that resets the path of the tower for game restarts
*/
func (t *Tower) Reset() {
	t.Path = [3]int{0, 0, 0}
}
