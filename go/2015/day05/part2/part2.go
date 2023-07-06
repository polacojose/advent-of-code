package part2

import (
	"strings"
)

func Verify(line string) bool {
	return hasMagicThree(line) && hasMultipleDoubles(line)
}

func hasMultipleDoubles(line string) bool {
	for i := 2; i < len(line); i++ {
		subString := string(line[i-2]) + string(line[i-1])
		if strings.Contains(line[i:], string(subString)) {
			return true
		}
	}
	return false
}

func hasMagicThree(line string) bool {
	for i := 2; i < len(line); i++ {
		if string(line[i-2]) == string(line[i]) && string(line[i-1]) != string(line[i]) {
			return true
		}
	}
	return false
}
