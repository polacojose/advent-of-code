package main

import (
	"bufio"
	"day07/lib"
	"fmt"
	"os"
)

func main() {
	part1()
	part2()
}

func part2() {
	file, _ := os.Open("input2.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	instructionLines := []string{}
	for fileScanner.Scan() {
		instructionLines = append(instructionLines, fileScanner.Text())
	}

	for k := range lib.Wires {
		delete(lib.Wires, k)
	}

	for len(instructionLines) > 0 {
		line := instructionLines[0]
		instructionLines = instructionLines[1:]

		if !lib.ExecuteInstruction(line) {
			instructionLines = append(instructionLines, line)
		}
	}

	fmt.Println("Part2 Wire a =", lib.Wires["a"])
}
func part1() {

	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	instructionLines := []string{}
	for fileScanner.Scan() {
		instructionLines = append(instructionLines, fileScanner.Text())
	}

	for k := range lib.Wires {
		delete(lib.Wires, k)
	}

	for len(instructionLines) > 0 {
		line := instructionLines[0]
		instructionLines = instructionLines[1:]

		if !lib.ExecuteInstruction(line) {
			instructionLines = append(instructionLines, line)
		}
	}

	fmt.Println("Part1 Wire a =", lib.Wires["a"])
}
