package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Heading int

const (
	North Heading = 0
	East  Heading = 1
	South Heading = 2
	West  Heading = 3
)

func main() {

	instructions := getInstructions()
	execute(instructions)

}

type Vector struct {
	x int
	y int
}

func execute(instructions []string) {
	var (
		heading = North
		x       = 0
		y       = 0
	)

	locations := []Vector{Vector{0, 0}}

	for _, inst := range instructions {
		face := inst[:1]
		dist, _ := strconv.Atoi(inst[1:])

		switch face {
		case "R":
			heading++
		case "L":
			heading--
		}

		if heading < 0 {
			heading = 3
		} else {
			heading %= 4
		}

		for dist > 0 {
			switch heading {
			case North:
				y--
			case South:
				y++
			case East:
				x++
			case West:
				x--
			}
			locations = append(locations, Vector{
				x: x,
				y: y,
			})
			dist--
		}

	}

	fmt.Println(x, y, math.Abs(float64(x))+math.Abs(float64(y)))
	loc := firstDupVect(locations)
	fmt.Println(loc.x, loc.y, math.Abs(float64(loc.x))+math.Abs(float64(loc.y)))
}

func firstDupVect(vectors []Vector) *Vector {

	for i := 0; i < len(vectors); i++ {
		for j := i + 1; j < len(vectors); j++ {
			if vectors[i] == vectors[j] {
				return &vectors[i]
			}
		}
	}

	return nil
}

func getInstructions() []string {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	instructions := []string{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		parts := strings.Split(line, ",")

		for _, part := range parts {
			instructions = append(instructions, strings.TrimSpace(part))
		}
	}
	return instructions
}
