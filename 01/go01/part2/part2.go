package part2

import (
	"strconv"
	"strings"
)

func Process(input string) int {
	intsAsNums := []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

	lines := strings.Split(input, "\n")
	answer := 0
	for _, line := range lines {
		chars := strings.Split(line, "")
		nums := []int{}
		for i, c := range chars {
			for j, ias := range intsAsNums {
				if strings.HasPrefix(line[i:], ias) {
					nums = append(nums, j+1)
				}
			}

			k, err := strconv.Atoi(c)
			if err != nil {
				continue
			}
			nums = append(nums, k)
		}

		if len(nums) > 0 {
			answer += nums[0]*10 + nums[len(nums)-1]
		}
	}

	return answer
}
