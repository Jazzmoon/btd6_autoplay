package utils

import (
	"fmt"

	"github.com/manifoldco/promptui"
)

var PromptTemplate = &promptui.SelectTemplates{
	Label:    "{{ . }}",
	Active:   "{{ \">\" | yellow }} {{ . | cyan }}",
	Inactive: "  {{ . | white }}",
	Selected: "{{ \"\U00002714\" | green }} {{ . | yellow }}",
}

func PromptForUserInput(label string, items []string) string {
	prompt := promptui.Select{
		Label:     label,
		Items:     items,
		Templates: PromptTemplate,
	}

	if _, result, err := prompt.Run(); err != nil {
		panic(fmt.Errorf("'%v' was given a bad response", label))
	} else {
		return result
	}
}
