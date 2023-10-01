package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

const input = "input.txt"

type BlackListRange struct {
	start int
	end   int
}

type IPValidation struct {
	invalidRange *BlackListRange
	valid        bool
}

func main() {
	blacklist, err := getBlacklistRanges()
	if err != nil {
		log.Fatal(err)
	}

	sort.Slice(blacklist, func(i, j int) bool {
		return blacklist[i].start < blacklist[j].start
	})

	validIPs := []int{}
	for i := 0; i <= 4294967295; i++ {
		v, end := validIP(i, blacklist)
		if !v {
			i = *end
			continue
		}

		validIPs = append(validIPs, i)
	}

	fmt.Println("First valid IP:", validIPs[0])
	fmt.Println("Num valid IPs:", len(validIPs))
}

func validIP(ip int, blacklist []BlackListRange) (bool, *int) {
	for _, r := range blacklist {
		if ip >= r.start && ip <= r.end {
			return false, &r.end
		}
	}
	return true, nil
}

func getBlacklistRanges() ([]BlackListRange, error) {
	f, err := os.Open(input)
	if err != nil {
		return []BlackListRange{}, err
	}

	fileScanner := bufio.NewScanner(f)

	ranges := []BlackListRange{}
	for fileScanner.Scan() {
		lineParts := strings.Split(fileScanner.Text(), "-")
		start, errA := strconv.Atoi(lineParts[0])
		end, errB := strconv.Atoi(lineParts[1])

		if errA != nil || errB != nil {
			return nil, errors.New("error parsing range")
		}

		ranges = append(ranges, BlackListRange{
			start: start,
			end:   end,
		})
	}

	return ranges, nil
}
