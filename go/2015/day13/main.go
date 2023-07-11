package main

import (
	"errors"
	"fmt"
	"sort"
)

func main() {
	//fmt.Println(HAPPINESS_MAP)
	result, _ := find_seating()
	fmt.Println(result, "=", getSeatingHappiness(result))
}

type Seating struct {
	segments  []string
	happiness int
}

func find_seating() ([]string, error) {

	openSet := []Seating{}
	openSet = append(openSet, Seating{
		segments:  []string{"Alice"},
		happiness: 0,
	})

	for len(openSet) > 0 {
		currentSeating := openSet[0]
		openSet = openSet[1:]

		if len(currentSeating.segments) > len(HAPPINESS_MAP) {
			return currentSeating.segments, nil
		}

		if len(currentSeating.segments) == len(HAPPINESS_MAP) {
			currentSeating.segments = append(currentSeating.segments, "Alice")
			currentSeating.happiness = getSeatingHappiness(currentSeating.segments)
			openSet = append(openSet, currentSeating)
		} else {
			lastNode := currentSeating.segments[len(currentSeating.segments)-1]
			for name := range HAPPINESS_MAP[lastNode] {

				exists := false
				for _, n := range currentSeating.segments {
					if name == n {
						exists = true
						continue
					}
				}
				if exists {
					continue
				}

				newSeating := Seating{
					segments:  make([]string, len(currentSeating.segments)),
					happiness: 0,
				}
				copy(newSeating.segments, currentSeating.segments)
				newSeating.segments = append(newSeating.segments, name)
				newSeating.happiness = getSeatingHappiness(newSeating.segments)

				openSet = append(openSet, newSeating)
			}
		}

		sort.Slice(openSet, func(i, j int) bool {
			seating_a := openSet[i]
			seating_b := openSet[j]

			if len(openSet[i].segments) == len(openSet[j].segments) {
				return seating_a.happiness > seating_b.happiness
			}

			return len(openSet[i].segments) < len(openSet[j].segments)
		})
	}

	return nil, errors.New("Max happiness seating not found")
}

func getSeatingHappiness(seating []string) int {
	seatingHappiness := 0

	for i := 1; i < len(seating); i++ {
		seatingHappiness += HAPPINESS_MAP[seating[i-1]][seating[i]]
		seatingHappiness += HAPPINESS_MAP[seating[i]][seating[i-1]]
	}

	return seatingHappiness
}
