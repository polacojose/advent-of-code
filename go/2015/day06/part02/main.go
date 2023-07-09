package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Coord struct {
	x int
	y int
}

const LIGHTS_SIZE = 1000

func main() {

	lights := [LIGHTS_SIZE][LIGHTS_SIZE]int{}

	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		setLightsPerInstruction(&lights, line)
	}

	fmt.Println("Total brightness:", countLights(&lights))
}

func setLightsPerInstruction(lights *[LIGHTS_SIZE][LIGHTS_SIZE]int, line string) {

	instructionParts := strings.Fields(line)
	from := getCoord(instructionParts[len(instructionParts)-3:][0])
	to := getCoord(instructionParts[len(instructionParts)-3:][2])

	if strings.Contains(line, "turn on") {
		turnOnLights(lights, from, to)
	} else if strings.Contains(line, "turn off") {
		turnOffLights(lights, from, to)
	} else if strings.Contains(line, "toggle") {
		toggleLights(lights, from, to)
	}
}

func toggleLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]int, from, to Coord) {
	for y := from.y; y <= to.y; y++ {
		for x := from.x; x <= to.x; x++ {
			lights[x][y] += 2
		}
	}
}

func turnOffLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]int, from, to Coord) {
	for y := from.y; y <= to.y; y++ {
		for x := from.x; x <= to.x; x++ {
			if lights[x][y] > 0 {
				lights[x][y] -= 1
			}
		}
	}
}

func turnOnLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]int, from, to Coord) {
	for y := from.y; y <= to.y; y++ {
		for x := from.x; x <= to.x; x++ {
			lights[x][y] += 1
		}
	}
}

func countLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]int) int {
	var total int = 0
	for y := 0; y < LIGHTS_SIZE; y++ {
		for x := 0; x < LIGHTS_SIZE; x++ {
			total += lights[x][y]
		}
	}
	return total
}

func getCoord(line string) Coord {
	coordParts := strings.Split(line, ",")
	x, _ := strconv.Atoi(coordParts[0])
	y, _ := strconv.Atoi(coordParts[1])
	return Coord{
		x,
		y,
	}
}
