package utils

import (
	"fmt"
	"os"
)

var DEBUG bool = false

func IsDebug() bool {
	return DEBUG
}

func DebugLogf(format string, args ...interface{}) {
	if IsDebug() {
		fmt.Printf(format, args...)
	}
}

func IsWindows() bool {
	// Check if the operating system is Windows.
	// This is done by checking if the OS environment variable is set to "windows".
	// This is a common way to check the operating system in Go.
	if os := os.Getenv("OS"); os == "Windows_NT" {
		return true
	}

	return false
}
