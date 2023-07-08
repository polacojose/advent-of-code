package lib

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type City struct {
	name      string
	neighbors map[string]Distance
}

type Distance struct {
	name     string
	distance uint
}

var firstCity string
var cities map[string]City

func init() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	first := true

	cities = make(map[string]City)
	for fileScanner.Scan() {
		line := fileScanner.Text()
		parts := strings.Fields(line)

		source := parts[0]
		destination := parts[2]
		distance, _ := strconv.Atoi(parts[4])

		if first {
			first = false
			firstCity = source
		}

		addCityDistance(source, destination, distance)
		addCityDistance(destination, source, distance)

	}
}

func addCityDistance(destination string, source string, distance int) {
	if _, ok := cities[destination]; !ok {
		cities[destination] = City{
			name:      destination,
			neighbors: make(map[string]Distance),
		}
	}
	if city, ok := cities[source]; ok {
		city.neighbors[destination] = Distance{
			name:     destination,
			distance: uint(distance),
		}

		cities[source] = city
	}
}

func main() {

	result, _ := findPath()

	fmt.Println(result)
	fmt.Println(getPathLength(result))

}

func findPath() ([]City, error) {
	open_set := [][]City{}
	open_set = append(open_set, []City{cities[firstCity]})

	for len(open_set) > 0 {

		path := open_set[0]
		open_set = open_set[1:]

		if len(path) == len(cities) {
			return path, nil
		}

		for _, neighbor := range path[len(path)-1].neighbors {

			exists := false
			for _, n := range path {
				if neighbor.name == n.name {
					exists = true
				}
			}
			if exists {
				continue
			}

			neighborPath := append(path, cities[neighbor.name])

			open_set = append(open_set, neighborPath)
		}

		sort.Slice(open_set, func(i, j int) bool {
			return getPathLength(open_set[i]) < getPathLength(open_set[j])
		})

	}
	return nil, errors.New("Unable to find path")
}

func getPathLength(cities []City) uint {
	var totalLength uint = 0

	for i := 1; i < len(cities); i++ {
		sourceCity := cities[i-1]
		destinationCity := cities[i]
		totalLength += sourceCity.neighbors[destinationCity.name].distance
	}

	return totalLength
}
