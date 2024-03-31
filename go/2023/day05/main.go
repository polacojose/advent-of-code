package main

import (
	"day05/rangemapchain"
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
	rangeMapChain := rangemapchain.NewRangeMapChain(f, true)
	l := rangeMapChain.LowestNumberLocation()
	println("Lowest number location", l)
}

func part2() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	rangeMapChain := rangemapchain.NewRangeMapChain(f, false)
	l := rangeMapChain.LowestNumberLocation()
	println("Lowest number location with seed ranges", l)
}
