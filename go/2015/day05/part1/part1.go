package part1

import "strings"

func Verify(line string) bool {
	return hasThreeVowels(line) && hasShortRun(line) && noIllegalStrings(line)
}

func noIllegalStrings(line string) bool {
	return !strings.Contains(line, "ab") &&
		!strings.Contains(line, "cd") &&
		!strings.Contains(line, "pq") &&
		!strings.Contains(line, "xy")
}

func hasShortRun(line string) bool {
	for i := 1; i < len(line); i++ {
		if line[i-1] == line[i] {
			return true
		}
	}
	return false
}

func hasThreeVowels(line string) bool {
	const vowels = "aieou"

	numVowels := 0
	for _, rune := range line {
		if strings.Contains(vowels, string(rune)) {
			numVowels += 1
			if numVowels >= 3 {
				return true
			}
		}
	}
	return false
}
