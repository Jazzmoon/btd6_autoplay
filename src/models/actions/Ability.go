package actions

import "errors"

type Ability struct {
	AbilityNumber int
}

func (a Ability) Run() error {
	// Do something

	return errors.New("[Action | Ability] Not implemented")
}
