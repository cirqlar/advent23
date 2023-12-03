package part1

import (
	"fmt"
	"strconv"
	"strings"
	"unicode"
)

func Process(input string) int {
	acc := 0
	lines := strings.Split(input, "\n")
	width := len(lines[0])
	height := len(lines)

	currentNum := ""
	currentStartX := 0
	currentStartY := 0

	characters := make([][]string, height)
	for i, line := range lines {
		characters[i] = strings.Split(line, "")
	}

	for y, line := range characters {
		for x, ch := range line {
			if (x == 0 || !unicode.IsDigit(rune(ch[0]))) && len(currentNum) > 0 {
				numWidth := x - currentStartX
				if numWidth <= 0 {
					numWidth = width - currentStartX
				}

				numWidth += 2

				for i := 0; i < (numWidth)*2+2; i++ {
					xToCheck := currentStartX - 1
					yToCheck := currentStartY - 1
					if i < numWidth {
						xToCheck += i
					} else if i == numWidth {
						yToCheck += 1
					} else if i == numWidth+1 {
						xToCheck += numWidth - 1
						yToCheck += 1
					} else {
						xToCheck += i - numWidth - 2
						yToCheck += 2
					}

					if xToCheck < 0 || xToCheck >= width || yToCheck < 0 || yToCheck >= height {
						continue
					}

					currentCh := characters[yToCheck][xToCheck]
					if !unicode.IsLetter(rune(currentCh[0])) && !unicode.IsNumber(rune(currentCh[0])) && currentCh != "." {
						num, err := strconv.Atoi(currentNum)
						if err != nil {
							fmt.Print(err)
							break
						}

						acc += num
						break
					}
				}
				currentNum = ""
			}

			if unicode.IsDigit(rune(ch[0])) {
				if len(currentNum) == 0 {
					currentStartX = x
					currentStartY = y
				}
				currentNum += string(ch)
			}
		}
	}

	return acc
}
