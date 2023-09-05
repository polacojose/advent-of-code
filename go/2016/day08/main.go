package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type Display [displayHeight][displayWidth]bool

const (
	displayHeight = 6
	displayWidth  = 50
)

func main() {

	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	display := Display{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		line = strings.ReplaceAll(line, "x=", "")
		line = strings.ReplaceAll(line, "y=", "")
		line = strings.ReplaceAll(line, "x", " ")
		fields := strings.Fields(line)

		switch {
		case strings.Index(line, "rect") >= 0:
			a, err := strconv.Atoi(fields[1])
			if err != nil {
				fmt.Println(err)
				return
			}
			b, err := strconv.Atoi(fields[2])
			if err != nil {
				fmt.Println(err)
				return
			}
			displayRect(&display, a, b)
		case strings.Index(line, "rotate row") >= 0:
			a, err := strconv.Atoi(fields[2])
			if err != nil {
				fmt.Println(err)
				return
			}
			b, err := strconv.Atoi(fields[4])
			if err != nil {
				fmt.Println(err)
				return
			}
			displayRotateRow(&display, a, b)
		case strings.Index(line, "rotate column") >= 0:
			a, err := strconv.Atoi(fields[2])
			if err != nil {
				fmt.Println(err)
				return
			}
			b, err := strconv.Atoi(fields[4])
			if err != nil {
				fmt.Println(err)
				return
			}
			displayRotateColumn(&display, a, b)
		}

		fmt.Print("\033[H\033[2J")
		printDisplay(&display)
		fmt.Print(line)
		time.Sleep(time.Millisecond * 20)
	}

	fmt.Println("Lights on:", displayCountOn(&display))
}

func displayRect(display *Display, width int, height int) {
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			(*display)[y][x] = true
		}
	}
}

func displayRotateRow(display *Display, row int, shift int) {
	for i := 0; i < shift; i++ {
		last := display[row][displayWidth-1]
		for j := displayWidth - 1; j > 0; j-- {
			display[row][j] = display[row][j-1]
		}
		display[row][0] = last
	}
}

func displayRotateColumn(display *Display, column int, shift int) {
	for i := 0; i < shift; i++ {
		last := display[displayHeight-1][column]
		for j := displayHeight - 1; j > 0; j-- {
			display[j][column] = display[j-1][column]
		}
		display[0][column] = last
	}
}

func displayCountOn(display *Display) int {
	totalOn := 0
	for _, row := range display {
		for _, on := range row {
			if on {
				totalOn++
			}
		}
	}
	return totalOn
}

func printDisplay(display *Display) {
	for _, row := range display {
		for _, col := range row {
			if col {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}
