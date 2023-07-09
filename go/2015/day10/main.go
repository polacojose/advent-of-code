package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		originalLine := fileScanner.Text()
		lsLine := lookSay(originalLine, 40)
		fmt.Printf("Part1:\nLine: %v\nOutput Length: %v\n", originalLine, len(lsLine))

		lsLine = lookSay(originalLine, 50)
		fmt.Printf("Part2:\nLine: %v\nOutput Length: %v\n", originalLine, len(lsLine))
	}
}

func lookSay(line string, iterations int) string {
	lsLine := line

	for i := 0; i < iterations; i++ {
		outputRunes := []rune{}

		lastRune := ' '
		runeCount := 0

		for _, char := range lsLine {
			if lastRune == ' ' {
				lastRune = char
				runeCount++
				continue
			}

			if char == lastRune {
				runeCount++
			} else {
				outputRunes = append(outputRunes, rune(runeCount+'0'), lastRune)

				lastRune = char
				runeCount = 1
			}
		}
		outputRunes = append(outputRunes, rune(runeCount+'0'), lastRune)

		lsLine = string(outputRunes)
	}

	return lsLine
}
