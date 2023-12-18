package rangemapchain

import (
	"bufio"
	"io"
	"log"
	"math"
	"strconv"
	"strings"
)

type mapRange struct {
	destination int
	source      int
	length      int
}

type rangeMapSet struct {
	name      string
	mapRanges []mapRange
}

func (r *rangeMapSet) destFromSource(s int) int {
	for _, mr := range r.mapRanges {
		if mr.source <= s && s <= mr.source+mr.length {
			return mr.destination + s - mr.source
		}
	}
	return s
}

type RangeMapChain struct {
	chain []rangeMapSet
	seeds []int
}

func NewRangeMapChain(r io.Reader) RangeMapChain {

	scanner := bufio.NewScanner(r)

	scanner.Scan()
	seeds_line := strings.TrimSpace(scanner.Text())
	seeds := extractSeeds(seeds_line)

	chain := []rangeMapSet{}
	scanner.Scan()

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		name := strings.Fields(line)[0]

		mapRanges := []mapRange{}

		for scanner.Scan() {
			line := strings.TrimSpace(scanner.Text())
			if len(line) == 0 {
				break
			}
			mr := parseMapRange(line)
			mapRanges = append(mapRanges, mr)
		}

		chain = append(chain, rangeMapSet{name: name, mapRanges: mapRanges})
	}

	return RangeMapChain{seeds: seeds, chain: chain}
}

func (r *RangeMapChain) LowestNumberLocation() int {
	lowest := int(^uint(0) >> 1)
	for _, s := range r.seeds {
		for _, ms := range r.chain {
			s = ms.destFromSource(s)
		}
		lowest = int(math.Min(float64(lowest), float64(s)))
	}
	return lowest
}

func (r *RangeMapChain) LowestNumberLocationWithSeedRanges() int {
	lowest := int(^uint(0) >> 1)
	for si := 0; si < len(r.seeds); si += 2 {
		startSeed := r.seeds[si]
		startLength := r.seeds[si+1]
		for seed := startSeed; seed < startSeed+startLength; seed++ {
			pathVar := seed
			for _, ms := range r.chain {
				pathVar = ms.destFromSource(pathVar)
			}
			lowest = int(math.Min(float64(lowest), float64(pathVar)))
		}
	}
	return lowest
}

func parseMapRange(line string) mapRange {
	mapFields := strings.Fields(line)
	destination, err := strconv.Atoi(mapFields[0])
	if err != nil {
		log.Fatal(err)
	}
	source, err := strconv.Atoi(mapFields[1])
	if err != nil {
		log.Fatal(err)
	}
	length, err := strconv.Atoi(mapFields[2])
	if err != nil {
		log.Fatal(err)
	}
	mr := mapRange{
		destination: destination,
		source:      source,
		length:      length,
	}
	return mr
}

func extractSeeds(line string) []int {
	seed_numbers_str := strings.Fields(strings.TrimSpace(strings.Split(line, ":")[1]))

	seeds := []int{}
	for _, s := range seed_numbers_str {
		i, err := strconv.Atoi(s)
		if err != nil {
			log.Fatal(err)
		}
		seeds = append(seeds, i)
	}
	return seeds
}
