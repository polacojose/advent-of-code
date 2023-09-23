package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Disc struct {
	posMax   int
	startPos int
}

func (d *Disc) posAt(sec int) int {
	return (sec + d.startPos) % d.posMax
}

func main() {

	discs := getDiscs()

	part1(discs)
	part2(discs)
}

func part1(discs []Disc) {
outer:
	for s := 0; ; s++ {
		sA := s
		for _, d := range discs {
			sA++
			if d.posAt(sA) != 0 {
				continue outer
			}
		}
		fmt.Println("Second:", s)
		break
	}
}

func part2(discs []Disc) {
	discs = append(discs, Disc{
		posMax:   11,
		startPos: 0,
	})
outer:
	for s := 0; ; s++ {
		sA := s
		for _, d := range discs {
			sA++
			if d.posAt(sA) != 0 {
				continue outer
			}
		}
		fmt.Println("Second:", s)
		break
	}
}

func getDiscs() []Disc {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	discs := []Disc{}
	for fileScanner.Scan() {
		fields := strings.Fields(strings.ReplaceAll(fileScanner.Text(), ".", ""))

		posMax, err := strconv.Atoi(fields[3])
		if err != nil {
			log.Fatal(err)
		}

		startPos, err := strconv.Atoi(fields[11])
		if err != nil {
			log.Fatal(err)
		}

		discs = append(discs, Disc{
			posMax:   posMax,
			startPos: startPos,
		})
	}
	return discs
}
