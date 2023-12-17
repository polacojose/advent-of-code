package main

import (
	"day03/partnumber"
	"log"
	"os"
)

func main() {
	part1()
	part2()
}

func part1() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	partNumbers := partnumber.ParsePartNumbers(f)

	sum := 0
	for _, p := range partNumbers {
		sum += p.ID
	}
	println("Partnumber Sum:", sum)
}

func part2() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	gearRatioSum := partnumber.ParseGearRatioSum(f)
	println("Gear Ratio Sum:", gearRatioSum)
}
