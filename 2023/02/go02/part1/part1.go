package part1

import (
	"fmt"
	"strconv"
	"strings"
)

func Process(input string, max_red int, max_green int, max_blue int) int {
	valid := 0
	lines := strings.Split(input, "\n")

	for _, line := range lines {
		split := strings.Split(line, ": ")
		game := split[0]
		rest := split[1]

		red := 0
		green := 0
		blue := 0

		for _, rounds := range strings.Split(rest, "; ") {
			for _, round := range strings.Split(rounds, ", ") {
				roundSplit := strings.Split(round, " ")
				num, err := strconv.Atoi(roundSplit[0])
				if err != nil {
					fmt.Println("Failed int conversion")
					continue
				}
				switch roundSplit[1] {
				case "red":
					red = max(red, num)
				case "green":
					green = max(green, num)
				case "blue":
					blue = max(blue, num)
				}
			}
		}

		if red <= max_red && green <= max_green && blue <= max_blue {
			gameIndex, err := strconv.Atoi(strings.Split(game, " ")[1])
			if err != nil {
				fmt.Println("Game index didn't parse")
				continue
			}

			valid += gameIndex
		}
	}

	return valid
}
