package main

import (
	"Jazzmoon/btd6_autoplay/types"
	"fmt"
	"os"

	"gopkg.in/yaml.v3"
)

func main() {

	// Load a YAML file
	yamlFile := "../config/DarkCastle.yaml"

	// Load the file
	yamlData, err := os.ReadFile(yamlFile)
	if err != nil {
		panic(err)
	}

	// Parse the YAML file
	var settings types.MapConfig_Type

	err = yaml.Unmarshal(yamlData, &settings)
	if err != nil {
		panic(err)
	}

	tower := settings.Easy.Standard.Towers["dart"]

	fmt.Println(tower.String())
}
