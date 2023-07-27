package main

import (
	"bufio"
	"fmt"
	"os"
)

const LIGHT_GRID_SIZE int = 100

var currentLightGrid [LIGHT_GRID_SIZE][LIGHT_GRID_SIZE]bool = [LIGHT_GRID_SIZE][LIGHT_GRID_SIZE]bool{}
var workingLightGrid [LIGHT_GRID_SIZE][LIGHT_GRID_SIZE]bool = [LIGHT_GRID_SIZE][LIGHT_GRID_SIZE]bool{}

func main() {

	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for y := 0; y < LIGHT_GRID_SIZE; y++ {
		fileScanner.Scan()
		line := fileScanner.Text()
		for x := 0; x < LIGHT_GRID_SIZE; x++ {
			lightOn := false
			if line[x] == '#' {
				lightOn = true
			}
			currentLightGrid[y][x] = lightOn
		}
	}

	for i := 0; i < 100; i++ {
		//updateLights()
		updateLightsStuck()
	}

	fmt.Println(lightsOn())

}

func lightsOn() int {
	lightsOn := 0
	for y := 0; y < LIGHT_GRID_SIZE; y++ {
		for x := 0; x < LIGHT_GRID_SIZE; x++ {
			if currentLightGrid[y][x] {
				lightsOn++
			}
		}
	}
	return lightsOn
}

func getNumLitNeighbors(x int, y int) int {
	lit := 0

	xMin := 0
	if x > 0 {
		xMin = x - 1
	}

	yMin := 0
	if y > 0 {
		yMin = y - 1
	}

	xMax := LIGHT_GRID_SIZE - 1
	if x < LIGHT_GRID_SIZE-1 {
		xMax = x + 1
	}

	yMax := LIGHT_GRID_SIZE - 1
	if y < LIGHT_GRID_SIZE-1 {
		yMax = y + 1
	}

	for j := yMin; j <= yMax; j++ {
		for i := xMin; i <= xMax; i++ {

			if currentLightGrid[j][i] {
				lit++
			}
		}
	}

	if currentLightGrid[y][x] {
		lit--
	}

	return lit
}

func updateLightsStuck() {
	for y := 0; y < LIGHT_GRID_SIZE; y++ {
		for x := 0; x < LIGHT_GRID_SIZE; x++ {
			neighborsOn := getNumLitNeighbors(x, y)

			if (x == 0 && y == 0) || (x == LIGHT_GRID_SIZE-1 && y == 0) || (x == 0 && y == LIGHT_GRID_SIZE-1) || (x == LIGHT_GRID_SIZE-1 && y == LIGHT_GRID_SIZE-1) {
				workingLightGrid[y][x] = true
			} else {
				if currentLightGrid[y][x] {
					if neighborsOn == 2 || neighborsOn == 3 {
						workingLightGrid[y][x] = true
					} else {
						workingLightGrid[y][x] = false
					}
				} else {
					if neighborsOn == 3 {
						workingLightGrid[y][x] = true
					} else {
						workingLightGrid[y][x] = false
					}
				}
			}
		}
	}
	for y := 0; y < LIGHT_GRID_SIZE; y++ {
		for x := 0; x < LIGHT_GRID_SIZE; x++ {
			currentLightGrid[y][x] = workingLightGrid[y][x]
		}
	}
}

func updateLights() {
	for y := 0; y < LIGHT_GRID_SIZE; y++ {
		for x := 0; x < LIGHT_GRID_SIZE; x++ {
			neighborsOn := getNumLitNeighbors(x, y)

			if currentLightGrid[y][x] {
				if neighborsOn == 2 || neighborsOn == 3 {
					workingLightGrid[y][x] = true
				} else {
					workingLightGrid[y][x] = false
				}
			} else {
				if neighborsOn == 3 {
					workingLightGrid[y][x] = true
				} else {
					workingLightGrid[y][x] = false
				}
			}

		}
	}
	for y := 0; y < LIGHT_GRID_SIZE; y++ {
		for x := 0; x < LIGHT_GRID_SIZE; x++ {
			currentLightGrid[y][x] = workingLightGrid[y][x]
		}
	}
}

func printLightGrid(lightGrid [LIGHT_GRID_SIZE][LIGHT_GRID_SIZE]bool) {
	for y := 0; y < LIGHT_GRID_SIZE; y++ {
		for x := 0; x < LIGHT_GRID_SIZE; x++ {
			if lightGrid[y][x] {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
	fmt.Println("=========")
}
