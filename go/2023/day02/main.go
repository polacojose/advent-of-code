package main

import (
	"bufio"
	"day02/game"
	"log"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)
	lines := []string{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		lines = append(lines, line)
	}

	part1(lines)
	part2(lines)
}

func part1(lines []string) {
	idSum := 0
	components := map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}
	for _, line := range lines {
		gameResult := game.ParseGame(line)
		if gameResult.Valid(components) {
			idSum += gameResult.ID
		}
	}
	println("ID Sum:", idSum)
}

func part2(lines []string) {
	powerSum := 0
	for _, line := range lines {
		gameResult := game.ParseGame(line)
		powerSum += gameResult.RequiredComponentsPower()
	}
	println("Power Sum:", powerSum)
}
