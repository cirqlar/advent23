package part1

import (
	"testing"
)

func TestD01Part1IsCorrect(t *testing.T) {
	input := `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`

	answer := Process(input)

	if answer != 142 {
		t.Errorf("Wrong answer, got %d", answer)
	}
}
