package utils

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"

	"github.com/go-vgo/robotgo"
)

/*
SleepMove is a function that allows the user to move the mouse to a specific location with a sleep delay between each action taken
  - @param sleep: The number of milliseconds to sleep between each action
  - @param x: The x-coordinate of the location to move the mouse to
  - @param y: The y-coordinate of the location to move the mouse to
  - @param displayId: The display ID of the monitor to move the mouse to (optional)
*/
func SleepMove(sleep, x, y int, displayId ...int) {
	mouseSleep := robotgo.MouseSleep
	robotgo.MouseSleep = sleep
	robotgo.Move(x, y, displayId...)
	robotgo.MouseSleep = mouseSleep
}

func SleepClick(sleep int, args ...interface{}) {
	mouseSleep := robotgo.MouseSleep
	robotgo.MouseSleep = sleep
	robotgo.Click(args...)
	robotgo.MouseSleep = mouseSleep
}

func SleepMoveAndClick(sleep, x, y int, args ...interface{}) {
	mouseSleep := robotgo.MouseSleep
	robotgo.MouseSleep = sleep
	robotgo.Move(x, y)
	robotgo.Click(args...)
	robotgo.MouseSleep = mouseSleep
}

func SleepKeyTap(sleep int, key string, args ...interface{}) error {
	keySleep := robotgo.KeySleep
	robotgo.KeySleep = sleep
	err := robotgo.KeyTap(key, args...)
	robotgo.KeySleep = keySleep
	return err
}

/*
UseAbility is a function that allows the user to use an ability in the game
  - @param args: A list of arguments that the user can pass in to use an ability
  - @return success: A boolean that indicates whether the ability was used successfully
  - @return err: An error that indicates why the ability was not used successfully
*/
func UseAbility(args ...string) (success bool, err error) {
	/*
		If the length of abilities is 1:
		- If the ability is a key within the regex '[\d-=]', press the key and return true
		- Otherwise, return false and an error informing them that the use of the function was incorrect
		If the length of abilities is 2:
		- Assume that the user has requested we click a specific ability and that the two arguments are mouse coords
		- Return true if the ability was clicked successfully
	*/
	if len(args) == 1 {
		if match, err := regexp.MatchString(`[\d-=]`, args[0]); err != nil {
			return false, err
		} else if match {
			keySleep := robotgo.KeySleep
			robotgo.KeySleep = 2
			robotgo.KeyTap(args[0])
			robotgo.KeySleep = keySleep
			return true, nil
		} else {
			return false, errors.New("[UseAbility] Invalid key for ability. Please use a key from 0-9, -, or =")
		}
	} else if len(args) == 2 {
		// Validate that the two arguments are integers
		x, err := strconv.Atoi(args[0])
		if err != nil {
			return false, err
		}
		y, err := strconv.Atoi(args[1])
		if err != nil {
			return false, err
		}
		robotgo.Move(x, y)
		robotgo.Click("left")
	}
	return false, fmt.Errorf("[UseAbility] Invalid number of arguments. Expected 1 or 2, got %d", len(args))
}

/*
ClearObstacle is a function that allows the user to clear an obstacle in the game.
  - @param obstacleX: The x-coordinate of the obstacle
  - @param obstacleY: The y-coordinate of the obstacle
  - @param confirmX: The x-coordinate of the confirmation button
  - @param confirmY: The y-coordinate of the confirmation button
*/
func ClearObstacle(obstacleX, obstacleY, confirmX, confirmY int) {
	mouseSleep := robotgo.MouseSleep
	robotgo.MouseSleep = 2 // Sleep for 2 milliseconds between each mouse action
	robotgo.Move(obstacleX, obstacleY)
	robotgo.Click("left")
	robotgo.Move(confirmX, confirmY)
	robotgo.Click("left")
	robotgo.MouseSleep = mouseSleep
}
