package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("Part1:")
	part1()
	fmt.Println("Part2:")
	part2()
}

func part1() {
	var loopIndex uint = 1
	for {
		var houseSum uint = 0

		elfVisitInfinite(loopIndex, &houseSum)

		if houseSum >= 33100000 {
			fmt.Printf("House %d: %d\n", loopIndex, houseSum)
			break
		}

		loopIndex++

	}
}

func part2() {
	var loopIndex uint = 1
	for {
		var houseSum uint = 0

		elfVisitFinite(loopIndex, &houseSum)

		if houseSum >= 33100000 {
			fmt.Printf("House %d: %d\n", loopIndex, houseSum)
			break
		}

		loopIndex++

	}
}

func elfVisitInfinite(loopIndex uint, houseSum *uint) {
	for i := uint(1); i <= uint(math.Sqrt(float64(loopIndex))); i++ {
		if loopIndex%i == 0 {
			*houseSum += i * 10
			if loopIndex/i != i {
				*houseSum += loopIndex / i * 10
			}
		}
	}
}

func elfVisitFinite(loopIndex uint, houseSum *uint) {
	for i := uint(1); i <= uint(math.Sqrt(float64(loopIndex))); i++ {
		if loopIndex%i == 0 {

			if loopIndex <= i*50 {
				*houseSum += i * 11
			}

			if loopIndex/i != i {
				if loopIndex <= loopIndex/i*50 {
					*houseSum += loopIndex / i * 11
				}
			}
		}
	}
}
