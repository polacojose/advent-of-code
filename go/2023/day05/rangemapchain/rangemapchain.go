package rangemapchain

import (
	"bufio"
	"io"
	"log"
	"sort"
	"strconv"
	"strings"
)

const PROCESSOR_CORES = 4
const JOBS_PER_CORE = 10

type mapRange struct {
	destination int
	source      int
	length      int
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

		sort.Slice(mapRanges, func(i, j int) bool {
			return mapRanges[i].source < mapRanges[j].source
		})

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
		lowest = min(lowest, s)
	}
	return lowest
}

func (r *RangeMapChain) LowestNumberLocationWithSeedRanges() int {

	numberOfSeeds := 0
	for si := 0; si < len(r.seeds); si += 2 {
		numberOfSeeds += r.seeds[si+1]
	}
	numberOfSeedsPerCore := numberOfSeeds / (PROCESSOR_CORES * JOBS_PER_CORE)

	type searchJob struct {
		start  int
		length int
	}

	jobs := make([]searchJob, 0, PROCESSOR_CORES)
	for si := 0; si < len(r.seeds); si += 2 {
		startSeed := r.seeds[si]
		startLength := r.seeds[si+1]
		for startLength > 0 {
			jobLength := min(startLength, numberOfSeedsPerCore)
			jobs = append(jobs, searchJob{
				start:  startSeed,
				length: startLength,
			})
			startSeed += jobLength
			startLength -= jobLength
		}
	}

	ch := make(chan int)
	defer close(ch)

	for _, job := range jobs {
		startSeed := job.start
		startLength := job.length
		go func() {
			lowest := int(^uint(0) >> 1)
			for seed := startSeed; seed < startSeed+startLength; seed++ {
				pathVar := seed
				for _, ms := range r.chain {
					pathVar = ms.destFromSource(pathVar)
				}
				lowest = min(lowest, pathVar)
			}
			ch <- lowest
		}()
	}

	lowest := int(^uint(0) >> 1)
	for si := 0; si < len(r.seeds); si += 2 {
		lowest = min(lowest, <-ch)
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
