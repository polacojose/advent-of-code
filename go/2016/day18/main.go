package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
)

func main() {

	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	trapMap := [][]bool{}
	for fileScanner.Scan() {
		line := fileScanner.Text()

		row := []bool{}
		for _, c := range line {
			switch c {
			case '.':
				row = append(row, false)
			case '^':
				row = append(row, true)
			default:
				log.Fatal(errors.New("unknown type"))
			}
		}
		trapMap = append(trapMap, row)
	}

	safeTiles(40, append([][]bool{}, trapMap...))
	safeTiles(400000, append([][]bool{}, trapMap...))
}

func safeTiles(rows int, trapMap [][]bool) {
	mapWidth := len(trapMap[0])
	for i := 1; i < rows; i++ {
		row := make([]bool, mapWidth)
		for j := 0; j < mapWidth; j++ {
			a := false
			if j > 0 {
				a = trapMap[i-1][j-1]
			}
			b := trapMap[i-1][j]
			c := false
			if j+1 < mapWidth {
				c = trapMap[i-1][j+1]
			}

			if (a == true && b == true && c == false) || (a == false && b == true && c == true) || (a == true && b == false && c == false) || (a == false && b == false && c == true) {
				row[j] = true
			} else {
				row[j] = false
			}
		}
		trapMap = append(trapMap, row)
	}

	safeTiles := 0
	for _, r := range trapMap {
		for _, t := range r {
			if t {
			} else {
				safeTiles++
			}
		}
	}

	fmt.Println("Safe Tiles:", safeTiles)
}
