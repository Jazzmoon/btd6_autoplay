package models

import (
	"Jazzmoon/btd6_autoplay/types"
	"Jazzmoon/btd6_autoplay/utils"
	"fmt"
	"regexp"
	"strconv"
	"time"
)

type Game struct {
	RoundChecker chan int
	RoundErr     chan error

	LastSeenRound        int
	LowerRoundErrorCount int

	IsFreeplay bool
}

func InitGame() *Game {

	types.CurrentMap.Towers = make(map[interface{}]*types.Tower)

	return &Game{
		RoundChecker:  make(chan int, 1),
		RoundErr:      make(chan error, 1),
		LastSeenRound: 0,
		IsFreeplay:    false,
	}
}

func (g *Game) StartGame() {
	// Init

	types.GosseractClient.SetWhitelist("0123456789/VICTORYDEFA")

	go g.GameRoundChecker()

	for round := range g.RoundChecker {
		if round == -1 {
			// Error
			err := <-g.RoundErr
			fmt.Printf("RoundChecker error: %v\n", err)
			continue
		}

		// Check if round is not different from last seen round
		if round == g.LastSeenRound {
			continue
		}
		// Check if round is less than last seen round
		if round < g.LastSeenRound {
			g.LowerRoundErrorCount++

			if g.LowerRoundErrorCount > 5 {
				// Assume tessaract read the wrong number before and set the last seen round to the current round
				fmt.Printf("Asumming tessaract read the wrong number before. Setting last seen round to %d\n", round)

				g.LastSeenRound = round
				g.LowerRoundErrorCount = 0
			} else {
				continue
			}
		}

		g.LastSeenRound = round
		utils.DebugLogf("Round: %d\n", round)

		// Do round actions
		g.DoRoundActions(round)
	}
}

func (g *Game) DoRoundActions(round int) {
	// Do round actions
	actionsForRound, ok := types.CurrentMap.Instructions[round]
	if !ok {
		fmt.Printf("No actions for round %d\n", round)
		return
	}

	for _, action := range actionsForRound {
		// Do action
		utils.DebugLogf("Action: %s\n", action)

		actionInterface, err := ActionFromString(action)
		if err != nil {
			fmt.Printf("Error creating action: %v\n", err)
			continue
		}

		err = actionInterface.Run()
		if err != nil {
			fmt.Printf("Error running action: %v\n", err)
		}
	}
}

func (g *Game) RestartGame() {

}

func (g *Game) GetBannerText() {

}

func (g *Game) GameRoundChecker() {
	for {

		threshold := utils.LightBackground

		// Check if round counter threshold is set
		if types.MapConfig.RoundCounterMode != nil && *types.MapConfig.RoundCounterMode == types.Dark {
			threshold = utils.DarkBackground
		}

		img, err := utils.CaptureScreenAsJpeg(threshold, "ROUND", types.Settings.Game.RoundCounter.X, types.Settings.Game.RoundCounter.Y, types.Settings.Game.RoundCounter.W, types.Settings.Game.RoundCounter.H)
		if err != nil {
			g.RoundErr <- err
			g.RoundChecker <- -1
		}

		err = types.GosseractClient.SetImageFromBytes(img)
		if err != nil {
			g.RoundErr <- err
			g.RoundChecker <- -1
		}

		text, err := types.GosseractClient.Text()
		if err != nil {
			g.RoundErr <- err
			g.RoundChecker <- -1
		}

		regexp := utils.Ternary[*regexp.Regexp](g.IsFreeplay, regexp.MustCompile(`\d+`), regexp.MustCompile(`(\d+)/\d+`))
		if regexp.MatchString(text) {
			// Get the first match group
			match := regexp.FindStringSubmatch(text)
			if len(match) > 1 {
				round, err := strconv.Atoi(match[1])
				if err != nil {
					g.RoundErr <- err
					g.RoundChecker <- -1
				}

				g.RoundChecker <- round
			}
		}

		time.Sleep(1 * time.Second)
	}

}
