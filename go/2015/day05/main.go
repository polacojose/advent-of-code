package main

import (
	"bufio"
	"day05/part1"
	"day05/part2"
	"fmt"
	"os"
)

func main() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	totalNicePart1 := 0
	totalNicePart2 := 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		validPart1 := part1.Verify(line)
		validPart2 := part2.Verify(line)

		if validPart1 {
			totalNicePart1 += 1
		}

		if validPart2 {
			totalNicePart2 += 1
		}

		fmt.Printf("Line: %v is %v\n", line, validPart1)
		fmt.Printf("Line: %v is %v\n", line, validPart2)
	}
	fmt.Println("Total nice lines:", totalNicePart1)
	fmt.Println("Total nice lines:", totalNicePart2)
}
