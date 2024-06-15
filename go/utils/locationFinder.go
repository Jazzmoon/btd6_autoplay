package utils

import (
	"fmt"
	"os"
	"time"

	hook "github.com/robotn/gohook"
)

func LocationFinder() {

	fmt.Println("Location Finder")
	fmt.Println("Click anywhere on the screen to get the coordinates of the mouse pointer.")
	fmt.Println("Press 'q' to exit the program.")
	fmt.Println("Press 'p' to pause the program.")
	fmt.Println("Press 'm' to switch between single point and area mode.")

	mode := PromptForUserInput("Do you want single point or area mode", []string{"Single Point", "Area Mode"})

	paused := false

	var areaModeFirstCorner *hook.Event

	if mode == "Area Mode" {
		fmt.Println("Select the top left corner of the area first.")
	} else {
		fmt.Println("Select the point")
	}

	// Register the mouse event.
	hook.Register(hook.MouseDown, []string{}, func(e hook.Event) {
		if paused || e.Button != 1 {
			return
		}

		if mode == "Single Point" {
			fmt.Println("X:", e.X, "Y:", e.Y)
			return
		}

		if areaModeFirstCorner == nil {
			areaModeFirstCorner = &e

			fmt.Println("Please select the second point")
			return
		}

		if e.X-areaModeFirstCorner.X < 0 || e.Y-areaModeFirstCorner.Y < 0 {
			fmt.Println("Invalid area selected")
			areaModeFirstCorner = nil
			return
		}

		fmt.Println("X:", areaModeFirstCorner.X, "Y:", areaModeFirstCorner.Y, "W:", e.X-areaModeFirstCorner.X, "H:", e.Y-areaModeFirstCorner.Y)
		areaModeFirstCorner = nil

	})

	// Register the keyboard event.
	hook.Register(hook.KeyDown, []string{"q"}, func(e hook.Event) {
		fmt.Println("Exiting")
		hook.End()
		time.Sleep(1 * time.Second)

		os.Exit(0)
	})

	// Register the keyboard event.
	hook.Register(hook.KeyDown, []string{"p"}, func(e hook.Event) {
		paused = !paused
		if paused {
			fmt.Println("Paused")
		} else {
			fmt.Println("Unpaused")
		}
	})

	hook.Register(hook.KeyDown, []string{"m"}, func(e hook.Event) {
		fmt.Println("Switching mode")

		if mode == "Single Point" {
			mode = "Area Mode"
			fmt.Println("Select the top left corner of the area first.")
		} else {
			mode = "Single Point"
			fmt.Println("Select the point")
		}

	})

	// Start the hook.
	s := hook.Start()

	// Wait for the hook to stop.
	<-hook.Process(s)
}
