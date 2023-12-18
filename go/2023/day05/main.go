package main

import (
	"day05/rangemapchain"
	"log"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	rangeMapChain := rangemapchain.NewRangeMapChain(f)

	part1(rangeMapChain)
	part2(rangeMapChain)
}

func part1(rangeMapChain rangemapchain.RangeMapChain) {
	l := rangeMapChain.LowestNumberLocation()
	println("Lowest number location", l)
}

func part2(rangeMapChain rangemapchain.RangeMapChain) {
	l := rangeMapChain.LowestNumberLocationWithSeedRanges()
	println("Lowest number location with seed ranges", l)
}
