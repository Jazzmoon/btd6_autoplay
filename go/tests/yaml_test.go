package tests

import (
	"Jazzmoon/btd6_autoplay/types"
	"os"
	"testing"

	"gopkg.in/yaml.v3"
)

func TestYaml(t *testing.T) {

	// Load a YAML file
	yamlFile := "../../config/DarkCastle.yaml"

	// Load the file
	yamlData, err := os.ReadFile(yamlFile)
	if err != nil {
		t.Errorf("Error reading YAML file: %v", err)
	}

	// Parse the YAML file
	var settings types.MapConfig_Type

	err = yaml.Unmarshal(yamlData, &settings)
	if err != nil {
		t.Errorf("Error parsing YAML file: %v", err)
	}

	b, err := yaml.Marshal(settings)
	if err != nil {
		t.Errorf("Error marshalling YAML file: %v", err)
	}

	// Print the settings
	t.Log(string(b))
}
