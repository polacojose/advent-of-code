package main

import (
	"bufio"
	"fmt"
	"os"
)

type Coord struct {
	x int
	y int
}

func main() {
	part1()
	part2()
}

func part1() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		coordinate := Coord{x: 0, y: 0}
		visitedHomes := make(map[Coord]uint)
		visitedHomes[coordinate] = 0

		fileContents := fileScanner.Text()
		for _, byte := range fileContents {
			coordinate = translateCoord(coordinate, byte)
			visitedHomes[coordinate] = 0
		}

		fmt.Println("Part01")
		fmt.Println("Homes visited:", len(visitedHomes))
	}
}
func part2() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		santa := Coord{x: 0, y: 0}
		roboSanta := Coord{x: 0, y: 0}
		visitedHomes := make(map[Coord]uint)
		visitedHomes[santa] = 0
		visitedHomes[roboSanta] = 0

		var santaMove = true

		fileContents := fileScanner.Text()
		for _, byte := range fileContents {
			if santaMove {
				santa = translateCoord(santa, byte)
				visitedHomes[santa] = 0
			} else {
				roboSanta = translateCoord(roboSanta, byte)
				visitedHomes[roboSanta] = 0
			}
			santaMove = !santaMove
		}

		fmt.Println("Part02")
		fmt.Println("Homes Visited:", len(visitedHomes))
	}
}

func translateCoord(coord Coord, direction rune) Coord {
	switch direction {
	case '^':
		coord.y += 1
		break
	case 'v':
		coord.y -= 1
		break
	case '<':
		coord.x -= 1
		break
	case '>':
		coord.x += 1
		break
	}
	return coord
}
