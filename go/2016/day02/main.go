package main

import (
	"bufio"
	"fmt"
	"os"
)

type vector struct {
	x int
	y int
}

func main() {
	navKeyboard(validateSquareKeypadPos, squareToInt)
	navKeyboard(validateStarKeypadPos, starToInt)
}

func navKeyboard(validation func(v vector) bool, translation func(v vector) string) {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	pos := vector{}
	for fileScanner.Scan() {
		line := fileScanner.Text()

		for _, dir := range line {
			nextPos := pos
			switch dir {
			case 'U':
				nextPos.y--
			case 'D':
				nextPos.y++
			case 'L':
				nextPos.x--
			case 'R':
				nextPos.x++
			}
			if validation(nextPos) {
				pos = nextPos
			}
		}

		fmt.Print(translation(pos))
	}
	fmt.Println()
}

func validateSquareKeypadPos(v vector) bool {
	return v.x >= -1 && v.x <= 1 && v.y >= -1 && v.y <= 1
}

func squareToInt(v vector) string {
	return fmt.Sprintf("%d", (v.x+2)+(v.y+1)*3)
}

func validateStarKeypadPos(v vector) bool {
	return validateSquareKeypadPos(v) || v == vector{x: 0, y: -2} || v == vector{x: 0, y: 2} || v == vector{x: -2, y: 0} || v == vector{x: 2, y: 0}
}

func starToInt(v vector) string {
	switch v.y {
	case -2:
		return "1"
	case -1:
		return fmt.Sprintf("%d", v.x+3)
	case 0:
		return fmt.Sprintf("%d", v.x+7)
	case 1:
		return string(rune(v.x) + 'B')
	case 2:
		return "D"
	}
	panic("invalid input")
}
