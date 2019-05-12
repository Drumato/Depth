package util

import (
	"fmt"

	"github.com/logrusorgru/aurora"
)

func ColorString(content string, color string) string {
	switch color {
	case "blue":
		return fmt.Sprintf("%s", aurora.Bold(aurora.BrightBlue(content)))
	case "red":
		return fmt.Sprintf("%s", aurora.Bold(aurora.BrightRed(content)))
	case "green":
		return fmt.Sprintf("%s", aurora.Bold(aurora.BrightGreen(content)))
	}
	return ""
}
