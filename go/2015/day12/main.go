package main

import (
	"bufio"
	"day12/part2"
	"fmt"
	"os"
)

func main() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		numbers := part2.GetNonRedNumbers(line)
		fmt.Printf("%v\n", line)
		fmt.Printf("%v = %d\n", numbers, sumNumbers(numbers))
	}
}

func sumNumbers(numbers []int) int {
	sum := 0
	for _, n := range numbers {
		sum += n
	}
	return sum
}
