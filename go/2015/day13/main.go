package main

import (
	"fmt"
)

func main() {
	//fmt.Println(HAPPINESS_MAP)
	result := find_seating()
	fmt.Println(result, "=", getSeatingHappiness(result))
}

type Seating struct {
	segments  []string
	happiness int
}

func find_seating() []string {

	openSet := []Seating{}
	openSet = append(openSet, Seating{
		segments:  []string{"Alice"},
		happiness: 0,
	})

	maxHappiness := 0
	var maxHappinessPath Seating

	for len(openSet) > 0 {
		currentSeating := openSet[0]
		openSet = openSet[1:]

		if len(currentSeating.segments) > len(HAPPINESS_MAP) {
			if currentSeating.happiness > maxHappiness {
				maxHappiness = currentSeating.happiness
				maxHappinessPath = currentSeating
			}
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
	}
	return maxHappinessPath.segments
}

func getSeatingHappiness(seating []string) int {
	seatingHappiness := 0

	for i := 1; i < len(seating); i++ {
		seatingHappiness += HAPPINESS_MAP[seating[i-1]][seating[i]]
		seatingHappiness += HAPPINESS_MAP[seating[i]][seating[i-1]]
	}

	return seatingHappiness
}
