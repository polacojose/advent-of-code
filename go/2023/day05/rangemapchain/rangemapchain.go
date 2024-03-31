package rangemapchain

import (
	"bufio"
	"errors"
	"io"
	"log"
	"math"
	"sort"
	"strconv"
	"strings"
)

type mapRange struct {
	destination int
	source      int
	length      int
}

type seedRange struct {
	start  int
	length int
}

type rangeMapSet struct {
	name      string
	mapRanges []mapRange
}

type RangeMapChain struct {
	chain      []rangeMapSet
	seedRanges []seedRange
}

func NewRangeMapChain(r io.Reader, singleRange bool) RangeMapChain {

	scanner := bufio.NewScanner(r)

	scanner.Scan()
	seeds_line := strings.TrimSpace(scanner.Text())

	seedsNumbers := extractSeeds(seeds_line)

	seedRanges := []seedRange{}
	if singleRange {
		for _, s := range seedsNumbers {
			seedRanges = append(seedRanges, seedRange{start: s, length: 1})
		}
	} else {
		for i := 0; i < len(seedsNumbers)-1; i += 2 {
			seedRanges = append(seedRanges, seedRange{start: seedsNumbers[i], length: seedsNumbers[i+1]})
		}
	}

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
			return mapRanges[i].destination < mapRanges[j].destination
		})

		if mapRanges[0].destination != 0 {
			mapRanges = append([]mapRange{{
				destination: 0,
				source:      0,
				length:      mapRanges[0].destination,
			}}, mapRanges...)
		}

		chain = append(chain, rangeMapSet{name: name, mapRanges: mapRanges})
	}

	rangeMapChain := RangeMapChain{seedRanges: seedRanges, chain: chain}

	return rangeMapChain.compressChain()
}

func (r *rangeMapSet) initialMapRange() *mapRange {
	if r.mapRanges[0].destination == 0 {
		return &r.mapRanges[0]
	} else {
		return &mapRange{
			destination: 0,
			source:      0,
			length:      r.mapRanges[0].destination,
		}
	}
}

func (r *rangeMapSet) DestForSource(source int) (*mapRange, error) {

	for _, mr := range r.mapRanges {
		if mr.source <= source && mr.source+mr.length > source {
			diff := source - mr.source
			return &mapRange{
				destination: mr.destination + diff,
				source:      mr.source + diff,
				length:      mr.length - diff,
			}, nil
		}
	}

	return nil, errors.New("Source not found")
}

func (r *rangeMapSet) rangeForDest(dest int, mustExist bool) (*mapRange, error) {

	lastMapRange := r.initialMapRange()

	if dest < r.mapRanges[0].destination {
		return lastMapRange, nil
	}

	for _, mr := range r.mapRanges {
		if mr.destination > dest {
			return lastMapRange, nil
		} else if mr.destination <= dest && mr.destination+mr.length > dest {
			diff := dest - mr.destination
			return &mapRange{
				destination: dest,
				source:      mr.source + diff,
				length:      mr.length - diff,
			}, nil
		}
	}

	if mustExist {
		return nil, errors.New("Destination not found")
	}

	return &mapRange{
		destination: dest,
		source:      dest,
		length:      1000000000,
	}, nil
}

func (r *RangeMapChain) compressChain() RangeMapChain {
	reducedChain := RangeMapChain{
		chain:      r.chain[:len(r.chain)-1],
		seedRanges: r.seedRanges,
	}
	mapRangeArr := r.chain[len(r.chain)-1].mapRanges
	compressedMapRanges := reducedChain.compressMapRanges(mapRangeArr)

	nameFrom := strings.Split(r.chain[0].name, "-to-")[0]
	nameTo := strings.Split(r.chain[len(r.chain)-1].name, "-to-")[1]

	return RangeMapChain{
		chain: []rangeMapSet{{
			name:      nameFrom + "-to-" + nameTo,
			mapRanges: compressedMapRanges,
		}},
		seedRanges: r.seedRanges,
	}
}

func (r *RangeMapChain) compressMapRanges(mapRanges []mapRange) []mapRange {

	if len(r.chain) == 0 {
		return mapRanges
	}

	nextRangeInChain := r.chain[len(r.chain)-1]
	compressedMapRanges := []mapRange{}

	for _, mr := range mapRanges {
		workingMapRange := mr
		for workingMapRange.length > 0 {

			parentMapRange, err := nextRangeInChain.rangeForDest(workingMapRange.source, false)
			if err != nil {
				log.Fatal(err)
			}

			if parentMapRange.length < workingMapRange.length {
				compressedMapRanges = append(compressedMapRanges, mapRange{
					destination: workingMapRange.destination,
					source:      parentMapRange.source,
					length:      parentMapRange.length,
				})
				workingMapRange.length -= parentMapRange.length
				workingMapRange.destination = workingMapRange.destination + parentMapRange.length
				workingMapRange.source = workingMapRange.source + parentMapRange.length
			} else {
				compressedMapRanges = append(compressedMapRanges, mapRange{
					destination: workingMapRange.destination,
					source:      parentMapRange.source,
					length:      workingMapRange.length,
				})
				workingMapRange.length = 0
			}
		}
	}

	reducedChain := RangeMapChain{
		chain:      r.chain[:len(r.chain)-1],
		seedRanges: r.seedRanges,
	}
	return reducedChain.compressMapRanges(compressedMapRanges)
}

func (r *RangeMapChain) LowestNumberLocation() int {

	lowestLocationMatch := ^uint(0)

	mapSet := r.chain[len(r.chain)-1]
outer:
	for _, s := range r.seedRanges {
		for s.length > 0 {
			dest, err := mapSet.DestForSource(s.start)
			if err != nil {
				continue outer
			}

			//log.Println("-----------")
			//log.Println(s)
			//log.Println("to")
			//log.Println(dest)
			//log.Println("-----------")

			lowestLocationMatch = uint(math.Min(float64(lowestLocationMatch), float64(dest.destination)))

			s.length = int(math.Max(float64(s.length)-float64(dest.length), 0))
			s.start = s.start + dest.length
		}
	}

	return int(lowestLocationMatch)
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
	trimed_string := strings.TrimSpace(strings.Split(line, ":")[1])
	seed_numbers_str := strings.Fields(trimed_string)

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
