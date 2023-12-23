package part2

import (
	"testing"
)

func TestD01Part2IsCorrect(t *testing.T) {
	input := `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`

	answer := Process(input)

	if answer != 281 {
		t.Errorf("Wrong answer, got %d", answer)
	}
}
