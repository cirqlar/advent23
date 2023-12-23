package part2

import (
	"testing"
)

func TestD02Part2IsCorrect(t *testing.T) {
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

	if answer != 467835 {
		t.Errorf("Wrong answer, got %d", answer)
	}
}
