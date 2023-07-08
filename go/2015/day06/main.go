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

	lights := [LIGHTS_SIZE][LIGHTS_SIZE]bool{}

	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		setLightsPerInstruction(&lights, line)
	}

	fmt.Println("Total lights on:", countLights(&lights))
}

func setLightsPerInstruction(lights *[LIGHTS_SIZE][LIGHTS_SIZE]bool, line string) {

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

func toggleLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]bool, from, to Coord) {
	for y := from.y; y <= to.y; y++ {
		for x := from.x; x <= to.x; x++ {
			lights[x][y] = !lights[x][y]
		}
	}
}

func turnOffLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]bool, from, to Coord) {
	for y := from.y; y <= to.y; y++ {
		for x := from.x; x <= to.x; x++ {
			lights[x][y] = false
		}
	}
}

func turnOnLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]bool, from, to Coord) {
	for y := from.y; y <= to.y; y++ {
		for x := from.x; x <= to.x; x++ {
			lights[x][y] = true
		}
	}
}

func countLights(lights *[LIGHTS_SIZE][LIGHTS_SIZE]bool) uint {
	var total uint = 0
	for y := 0; y < LIGHTS_SIZE; y++ {
		for x := 0; x < LIGHTS_SIZE; x++ {
			if lights[x][y] {
				total += 1
			}
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
