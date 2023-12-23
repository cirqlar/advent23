package part1

import (
	"testing"
)

func TestD02Part1IsCorrect(t *testing.T) {
	input := `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`

	answer := Process(input)

	if answer != 4361 {
		t.Errorf("Wrong answer, got %d", answer)
	}
}
