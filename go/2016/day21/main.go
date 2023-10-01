package main

import (
	"bufio"
	"day21/scramble"
	"fmt"
	"log"
	"os"
)

const input = "input.txt"
const subjectPart1 = "abcdefgh"
const subjectPart2 = "fbgdceah"

func main() {
	part1()
	part2()
}

func part1() {
	lines, err := newFunction()
	if err != nil {
		log.Fatal(err)
	}

	scrambler := scramble.New([]byte(subjectPart1))
	for _, l := range lines {
		scrambler.Execute(l)
	}
	fmt.Println(string(scrambler.Out()))
}

func part2() {
	lines, err := newFunction()
	if err != nil {
		log.Fatal(err)
	}

	scrambler := scramble.New([]byte(subjectPart2))
outer:
	for {
		lastResult := string(scrambler.Out())
		for _, l := range lines {
			scrambler.Execute(l)
		}
		result := string(scrambler.Out())
		if result == subjectPart2 {
			fmt.Println(lastResult)
			break outer
		}
		lastResult = result
	}
}

func newFunction() ([]string, error) {
	f, err := os.Open(input)
	if err != nil {
		return []string{}, nil
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)

	lines := []string{}
	for fileScanner.Scan() {
		lines = append(lines, fileScanner.Text())
	}

	return lines, nil
}
