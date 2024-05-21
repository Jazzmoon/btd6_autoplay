package models

import (
	"Jazzmoon/btd6_autoplay/types"
	"Jazzmoon/btd6_autoplay/utils"
	"fmt"
	"time"
)

type Game struct {
	CurrentRound chan int
}

func (g *Game) CreateGame() {
	// Init
	g.CurrentRound = make(chan int)

	go g.GameRoundChecker()

	for round := range g.CurrentRound {
		fmt.Println(round)
	}
}

func (g *Game) GameRoundChecker() {
	for {

		threshold := utils.DarkBackground

		// Check if round counter threshold is set
		if types.MapConfig.RoundCounterBackground != nil && *types.MapConfig.RoundCounterBackground == types.Light {
			threshold = utils.LightBackground
		}

		img, err := utils.CaptureScreenAsJpeg(threshold, types.Settings.Game.RoundCounter.X, types.Settings.Game.RoundCounter.Y, types.Settings.Game.RoundCounter.W, types.Settings.Game.RoundCounter.H)
		if err != nil {
			g.CurrentRound <- -1
		}

		oldWhitelist := types.GosseractClient.Variables["tessedit_char_whitelist"]
		// Set whitelist to only allow numbers and /
		types.GosseractClient.SetWhitelist("/0123456789")

		err = types.GosseractClient.SetImageFromBytes(img)
		if err != nil {
			g.CurrentRound <- -1
		}

		text, err := types.GosseractClient.Text()
		if err != nil {
			g.CurrentRound <- -1
		}

		fmt.Println("Text(): " + text)

		// Reset whitelist
		types.GosseractClient.SetWhitelist(oldWhitelist)

		// // Validate regex match of \d+/\d+
		// regexp := regexp.MustCompile(`(\d+)/\d+`)
		// if regexp.MatchString(text) {

		// }

		time.Sleep(1 * time.Second)
	}

}

func (g *Game) StartGame() {

}

func (g *Game) RestartGame() {

}

func (g *Game) GetRound() {

}

func (g *Game) GetBannerText() {

}
