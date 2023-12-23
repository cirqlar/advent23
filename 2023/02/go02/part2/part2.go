package part2

import (
	"fmt"
	"strconv"
	"strings"
)

func Process(input string) int {
	total := 0
	lines := strings.Split(input, "\n")

	for _, line := range lines {
		split := strings.Split(line, ": ")
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

		total += red * green * blue
	}

	return total
}
