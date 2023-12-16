package main

import (
	"bytes"
	expandcounter "day09/expand_counter"
	"log"
	"os"
)

func main() {
	input_bytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	input := string(bytes.TrimSpace(input_bytes))
	part1(input)
	part2(input)
}

func part1(input string) {
	expandCounter := expandcounter.NewExpandCounter(input, false)
	count := expandCounter.Count()
	println("Total expanded bytes:", count)
}

func part2(input string) {
	expandCounter := expandcounter.NewExpandCounter(input, true)
	count := expandCounter.Count()
	println("Total expanded bytes recursive:", count)
}
