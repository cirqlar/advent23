package part1

import (
	"strconv"
	"strings"
)

func Process(input string) int {
	lines := strings.Split(input, "\n")
	answer := 0
	for _, line := range lines {
		chars := strings.Split(line, "")
		nums := []int{}
		for _, c := range chars {
			i, err := strconv.Atoi(c)
			if err != nil {
				continue
			}

			nums = append(nums, i)
		}

		if len(nums) > 0 {
			answer += nums[0]*10 + nums[len(nums)-1]
		}
	}

	return answer
}
