package main

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
	neighbors map[string]uint
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

func addCityDistance(source string, destination string, distance int) {
	if _, ok := cities[source]; !ok {
		cities[source] = City{
			name:      source,
			neighbors: make(map[string]uint),
		}
	}

	if city, ok := cities[source]; ok {
		city.neighbors[destination] = uint(distance)
		cities[source] = city
	}
}

func main() {

	shortest, _ := findShortestPath()
	longest, _ := findLongestPath()

	fmt.Println(shortest)
	fmt.Println(longest)
	fmt.Println("Shortest:", getPathLength(shortest))
	fmt.Println("Longest:", getPathLength(longest))

}

//[Faerun AlphaCentauri Tambi Snowdin Norrath Tristram Arbre Straylight]

func findShortestPath() ([]string, error) {
	open_set := [][]string{}
	open_set = append(open_set, []string{firstCity})

	for len(open_set) > 0 {

		path := open_set[0]
		open_set = open_set[1:]

		if len(path) == len(cities) {
			return path, nil
		}

		for _, neighbor := range getNeighbors(path[len(path)-1]) {

			exists := false
			for _, n := range path {
				if neighbor == n {
					exists = true
				}
			}

			if exists {
				continue
			}

			neighborPath := make([]string, len(path))
			neighborPath = append(neighborPath, neighbor)
			copy(neighborPath, path)

			open_set = append(open_set, neighborPath)

		}

		sort.Slice(open_set, func(i, j int) bool {
			left := getPathLength(open_set[i])
			right := getPathLength(open_set[j])

			return left < right
		})

	}
	return nil, errors.New("Unable to find path")
}

func findLongestPath() ([]string, error) {

	var maxLength uint = 0
	maxPath := []string{}

	for _, j := range cities {
		open_set := [][]string{}
		open_set = append(open_set, []string{j.name})

		for len(open_set) > 0 {

			path := open_set[0]
			open_set = open_set[1:]

			if len(path) == len(cities) {
				length := getPathLength(path)
				if length > maxLength {
					maxLength = length
					maxPath = path
				}
			}

			for _, neighbor := range getNeighbors(path[len(path)-1]) {

				exists := false
				for _, n := range path {
					if neighbor == n {
						exists = true
					}
				}

				if exists {
					continue
				}

				neighborPath := make([]string, len(path))
				neighborPath = append(neighborPath, neighbor)
				copy(neighborPath, path)

				open_set = append(open_set, neighborPath)

			}

			sort.Slice(open_set, func(i, j int) bool {
				left := getPathLength(open_set[i])
				right := getPathLength(open_set[j])

				return left > right
			})

		}
	}

	return maxPath, nil
}

func getNeighbors(cityName string) []string {

	neighbors := []string{}

	for k, _ := range cities[cityName].neighbors {
		neighbors = append(neighbors, k)
	}

	return neighbors
}

func getPathLength(cityNames []string) uint {
	var totalLength uint = 0

	for i := 1; i < len(cityNames); i++ {
		sourceCity := cities[cityNames[i-1]]
		destinationCity := cities[cityNames[i]]
		totalLength += sourceCity.neighbors[destinationCity.name]
	}

	return totalLength
}
