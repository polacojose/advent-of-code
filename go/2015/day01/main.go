package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, _ := os.Open("input.txt")

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {

		line := fileScanner.Text()
		var floor = 0
		var crossed_basement = len(line) + 1

		for i, a := range line {
			if a == '(' {
				floor += 1
			} else {
				floor -= 1
			}

			if floor < -1 && crossed_basement == len(line)+1 {
				crossed_basement = i
			}
		}

		fmt.Printf("%v = %v\n", line, floor)
		fmt.Printf("Crossed Basement: %v\n", crossed_basement)

	}

	file.Close()
}
