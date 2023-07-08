package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	part1()
	part2()

}

func part1() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	characterValue := 0
	for fileScanner.Scan() {
		line := fileScanner.Text()

		valueBefore := len(line)
		line, _ = strconv.Unquote(line)
		valueAfter := len(line)

		characterValue += valueBefore - valueAfter
	}

	fmt.Println("Part1 Character Value:", characterValue)
}

func part2() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	characterValue := 0
	for fileScanner.Scan() {
		line := fileScanner.Text()

		valueBefore := len(line)
		line = strconv.Quote(line)
		valueAfter := len(line)

		characterValue += valueAfter - valueBefore
	}

	fmt.Println("Part2 Character Value:", characterValue)
}
