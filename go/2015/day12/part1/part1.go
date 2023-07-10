package part1

import (
	"regexp"
	"strconv"
)

func GetNumbers(line string) []int {
	re := regexp.MustCompile(`(-?[0-9]+)`)
	matcheGroups := re.FindAllStringSubmatch(line, -1)

	var numbers = []int{}
	for _, matchGroup := range matcheGroups {
		num, _ := strconv.Atoi(matchGroup[1])
		numbers = append(numbers, num)
	}
	return numbers
}
