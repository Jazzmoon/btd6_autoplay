package main

import (
	"Jazzmoon/btd6_autoplay/models"
	"Jazzmoon/btd6_autoplay/types"
	"Jazzmoon/btd6_autoplay/utils"
	"flag"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/otiai10/gosseract/v2"
	"gopkg.in/yaml.v3"
)

var (
	ConfigPath     = filepath.Join("..", "config")
	MapConfigsPath = filepath.Join(ConfigPath, "maps")
)

/*
*
The point of the main function is to:
1. Load in all relevant data from configs and settings for fetching.
- config/ScreenConfig.yaml
- config/Settings.yaml
- Argument `-m` or `--map` to specify the map to play on.
- Argument `-d` or `--difficulty` to specify the difficulty to play on.
- Argument `-g` or `--gamemode` to specify the game mode to play on.
- Argument `-n` or `--number` to specify the number of games to play before exiting. This is optional, defaulting to Infinite.
2. Launch the game loop and start playing games according to the settings.
*/
func main() {
	// Load in the user arguments.
	mapFlag := flag.String("m", "", "The map to play on.")
	difficultyFlag := flag.String("d", "", "The difficulty to play on.")
	gameModeFlag := flag.String("g", "", "The game mode to play on.")
	numGamesFlag := flag.Int("n", -1, "The number of games to play before exiting. Default is infinite.")
	locationFinder := flag.Bool("l", false, "Enable location finder.")
	debug := flag.Bool("debug", false, "Enable debug mode.")
	flag.Parse()

	// Set the debug flag.
	utils.DEBUG = *debug

	if utils.IsDebug() {
		fmt.Println("Debug mode enabled.")
	}

	// Check if the area locator flag is valid.
	if *locationFinder {
		utils.LocationFinder()
		return
	}

	// Load our global configs into the global types.
	if settingsData, err := os.ReadFile(filepath.Join(ConfigPath, "Settings.yaml")); err != nil {
		panic(err)
	} else if err = yaml.Unmarshal(settingsData, &types.Settings); err != nil {
		panic(err)
	}

	// Read the config directory into a list of file names so we can search for a map that matches the prompt.
	mapConfigsDir, err := os.ReadDir(MapConfigsPath)
	if err != nil {
		panic(err)
	}
	mapNames := utils.Map(mapConfigsDir, func(dirEntry fs.DirEntry) string {
		// Strip the file extension from the file name by splitting the string at the period and taking all but the last element.
		stringParts := strings.Split(dirEntry.Name(), ".")
		return strings.Join(stringParts[:len(stringParts)-1], ".")
	})

	// Ask the user to select a map if the map flag is not set.
	if *mapFlag == "" {
		*mapFlag = utils.PromptForUserInput("Select a map", mapNames)
	}
	// Load in the map config for the selected map.
	if mapConfigData, err := os.ReadFile(filepath.Join(MapConfigsPath, *mapFlag+".yaml")); err != nil {
		panic(err)
	} else if err = yaml.Unmarshal(mapConfigData, &types.MapConfig); err != nil {
		panic(err)
	}

	// Ask the user to select a difficulty if the difficulty flag is not set.
	if *difficultyFlag == "" {
		*difficultyFlag = utils.PromptForUserInput("Select a difficulty", utils.NonNilFieldsWithStructTagFilter(types.MapConfig, "jazzmoon", "notInSelector"))
	}

	difficultyConfig, ok := utils.GetFieldValue(types.MapConfig, *difficultyFlag).(*types.MapConfigDifficulty)
	if !ok {
		panic("Error fetching difficulty config.")
	}

	// Ask the user to select a game mode if the game mode flag is not set.
	if *gameModeFlag == "" {
		*gameModeFlag = utils.PromptForUserInput("Select a game mode", utils.NonNilFields(*difficultyConfig))
	}

	// Check if the user selected a valid difficulty for the selected map.
	if notNilDifficulties := utils.NonNilFields(types.MapConfig); !utils.Contains(notNilDifficulties, *difficultyFlag) {
		panic("Invalid difficulty selected.")
	}
	if notNilGameModes := utils.NonNilFields(*difficultyConfig); !utils.Contains(notNilGameModes, *gameModeFlag) {
		panic("Invalid game mode selected.")
	}

	types.CurrentMap = *utils.GetFieldValue(*difficultyConfig, *gameModeFlag).(*types.MapConfigMap)

	// Print the user arguments to verify that they were loaded correctly.
	utils.DebugLogf("Map: %s\n", *mapFlag)
	utils.DebugLogf("Difficulty: %s\n", *difficultyFlag)
	utils.DebugLogf("Game Mode: %s\n", *gameModeFlag)
	utils.DebugLogf("Number of Games: %d\n", *numGamesFlag)

	// Initialize the gosseract client.
	types.GosseractClient = gosseract.NewClient()
	defer types.GosseractClient.Close()
	types.GosseractClient.SetPageSegMode(gosseract.PSM_SINGLE_LINE)

	game := models.InitGame()
	game.StartGame()

}
