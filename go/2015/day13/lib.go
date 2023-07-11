package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

var HAPPINESS_MAP map[string]map[string]int

func init() {

	HAPPINESS_MAP = make(map[string]map[string]int)

	file, _ := os.Open("input-part2.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		line = strings.Replace(line, ".", "", -1)
		parts := strings.Fields(line)

		person_a := parts[0]
		person_b := parts[10]

		if _, ok := HAPPINESS_MAP[person_a]; !ok {
			HAPPINESS_MAP[person_a] = make(map[string]int)
		}

		happiness, _ := strconv.Atoi(parts[3])

		if parts[2] == "lose" {
			happiness = 0 - happiness
		}

		{
			relationships, _ := HAPPINESS_MAP[person_a]
			relationships[person_b] = happiness
			HAPPINESS_MAP[person_a] = relationships
		}
	}
}
