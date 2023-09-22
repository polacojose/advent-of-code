package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Deer struct {
	points      int
	speed       int
	dashSeconds int
	restSeconds int
}

var DEER []Deer

func init() {
	DEER = []Deer{}

	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		parts := strings.Fields(line)

		speed, _ := strconv.Atoi(parts[3])
		dashSeconds, _ := strconv.Atoi(parts[6])
		restSeconds, _ := strconv.Atoi(parts[13])
		DEER = append(DEER, Deer{
			points:      0,
			speed:       speed,
			dashSeconds: dashSeconds,
			restSeconds: restSeconds,
		})

	}

}

func main() {
	part1()
	part2()
}

func part1() {
	max := 0
	for _, deer := range DEER {
		value := getDeerPosition(2503, deer)
		if value > max {
			max = value
		}
	}
	fmt.Println("Part1:", max)
}

func part2() {

	max := 0
	for i := 1; i <= 2503; i++ {
		for _, deer := range DEER {
			pos := getDeerPosition(i, deer)
			if pos > max {
				max = pos
			}
		}

		for j := range DEER {
			if getDeerPosition(i, DEER[j]) == max {
				DEER[j].points++
			}
		}
	}

	maxPoints := 0
	var maxDeer Deer
	for _, deer := range DEER {
		if deer.points > maxPoints {
			maxPoints = deer.points
			maxDeer = deer
		}
	}

	fmt.Println("Part2:", maxDeer.points)
}

func getDeerPosition(seconds int, deer Deer) int {
	position := 0
	if seconds >= deer.dashSeconds+deer.restSeconds {
		cyclePeriod := (deer.dashSeconds + deer.restSeconds)
		cycles := seconds / cyclePeriod
		seconds -= cyclePeriod * cycles
		position += deer.speed * deer.dashSeconds * cycles
	}

	if seconds >= deer.dashSeconds {
		position += deer.speed * deer.dashSeconds
	} else {
		position += deer.speed * seconds
	}

	return position
}
