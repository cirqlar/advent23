package main

import (
	"fmt"
	"os"

	"go02/part1"
	"go02/part2"
)

func main() {
	input, err := os.ReadFile("../input/part1.txt")
	if err != nil {
		fmt.Print(err)
		return
	}
	stringInput := string(input)
	answer1 := part1.Process(stringInput, 12, 13, 14)
	fmt.Printf("Part 1: %d\n", answer1)
	answer2 := part2.Process(stringInput)
	fmt.Printf("Part 2: %d\n", answer2)
}
