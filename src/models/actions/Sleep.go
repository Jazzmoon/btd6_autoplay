package actions

import "time"

type Sleep struct {
	Duration time.Duration
}

func (action Sleep) Run() error {
	time.Sleep(action.Duration)
	return nil
}
