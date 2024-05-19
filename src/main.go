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

	"github.com/manifoldco/promptui"
	"github.com/otiai10/gosseract/v2"
	"gopkg.in/yaml.v3"
)

var (
	ConfigPath     = filepath.Join("..", "config")
	MapConfigsPath = filepath.Join(ConfigPath, "maps")

	PromptTemplate = &promptui.SelectTemplates{
		Label:    "{{ . }}",
		Active:   "{{ \">\" | yellow }} {{ . | cyan }}",
		Inactive: "  {{ . | white }}",
		Selected: "{{ \"\U00002714\" | green }} {{ . | yellow }}",
	}
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
	// Load our global configs into the global types.
	if settingsData, err := os.ReadFile(filepath.Join(ConfigPath, "Settings.yaml")); err != nil {
		panic(err)
	} else if err = yaml.Unmarshal(settingsData, &types.Settings); err != nil {
		panic(err)
	}

	// Initialize the gosseract client.
	types.GosseractClient = gosseract.NewClient()
	defer types.GosseractClient.Close()

	types.GosseractClient.SetPageSegMode(gosseract.PSM_SINGLE_BLOCK)

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

	// Load in the user arguments.
	mapFlag := flag.String("m", "", "The map to play on.")
	difficultyFlag := flag.String("d", "", "The difficulty to play on.")
	gameModeFlag := flag.String("g", "", "The game mode to play on.")
	numGamesFlag := flag.Int("n", -1, "The number of games to play before exiting. Default is infinite.")
	isDebugFlag := flag.Bool("debug", false, "Enable debug mode.")
	flag.Parse()

	// Check if the debug flag is valid.
	if *isDebugFlag {
		utils.DEBUG = true
	}

	// Check if the map flag is valid.
	if *mapFlag == "" {
		// Prompt the user to select a map manually.
		prompt := promptui.Select{
			Label:     "Select a map",
			Items:     mapNames,
			Templates: PromptTemplate,
		}

		if _, result, err := prompt.Run(); err != nil {
			panic(fmt.Errorf("invalid map selected: %v", err))
		} else {
			mapFlag = &result
		}
	}

	// Load in the map config for the selected map.
	if mapConfigData, err := os.ReadFile(filepath.Join(MapConfigsPath, *mapFlag+".yaml")); err != nil {
		panic(err)
	} else if err = yaml.Unmarshal(mapConfigData, &types.MapConfig); err != nil {
		panic(err)
	}

	// Check if the difficulty flag is valid.
	if *difficultyFlag == "" {
		// Prompt the user to select a difficulty manually.
		// Use the map flag to load in only the difficulties that are valid for the selected map.
		prompt := promptui.Select{
			Label:     "Select a difficulty",
			Items:     utils.NonNilFields(types.MapConfig),
			Templates: PromptTemplate,
		}

		if _, result, err := prompt.Run(); err != nil {
			panic(fmt.Errorf("invalid difficulty selected: %v", err))
		} else {
			difficultyFlag = &result
		}
	}

	// Check if the game mode flag is valid.
	if *gameModeFlag == "" {
		difficultyConfig, ok := utils.GetFieldValue(types.MapConfig, *difficultyFlag).(*types.MapConfigDifficulty)
		if !ok {
			panic("Error fetching difficulty config.")
		}
		// Prompt the user to select a game mode manually.
		prompt := promptui.Select{
			Label:     "Select a game mode",
			Items:     utils.NonNilFields(*difficultyConfig),
			Templates: PromptTemplate,
		}

		if _, result, err := prompt.Run(); err != nil {
			panic(fmt.Errorf("invalid game mode selected: %v", err))
		} else {
			gameModeFlag = &result
		}
	}

	// Validate that the selected Map, Difficulty, and Game Mode are valid in combination.
	if notNilDifficulties := utils.NonNilFields(types.MapConfig); !utils.Contains(notNilDifficulties, *difficultyFlag) {
		panic("Invalid difficulty selected.")
	} else if difficultyConfig, ok := utils.GetFieldValue(types.MapConfig, *difficultyFlag).(*types.MapConfigDifficulty); !ok {
		panic("Invalid difficulty selected.")
	} else if notNilGameModes := utils.NonNilFields(*difficultyConfig); !utils.Contains(notNilGameModes, *gameModeFlag) {
		panic("Invalid game mode selected.")
	} else if _, ok := utils.GetFieldValue(*difficultyConfig, *gameModeFlag).(*types.MapConfigMap); !ok {
		panic("Invalid game mode selected.")
	}

	// Print the user arguments to verify that they were loaded correctly.
	//fmt.Println("Map:", *mapFlag)
	//fmt.Println("Difficulty:", *difficultyFlag)
	//fmt.Println("Game Mode:", *gameModeFlag)
	fmt.Println("Number of Games:", *numGamesFlag)

	game := models.Game{}
	game.CreateGame()

}
