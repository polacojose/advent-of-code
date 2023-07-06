package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var totalRibbon uint = 0
	var totalMaterial uint = 0
	for scanner.Scan() {
		line := scanner.Text()
		nums := get_measures(line)

		material := packagePaper(nums)
		totalMaterial += material

		ribbon := packageRibbon(nums)
		totalRibbon += ribbon
	}
	fmt.Println("Total material", totalMaterial)
	fmt.Println("Total ribbon", totalRibbon)
}

func packageRibbon(measures []int) uint {
	var ribbon int

	sort.Slice(measures, func(i, j int) bool {
		return measures[i] < measures[j]
	})

	ribbon += measures[0]*2 + measures[1]*2

	var volume = 1
	for _, measure := range measures {
		volume *= measure
	}
	ribbon += volume

	return uint(ribbon)
}

func packagePaper(measures []int) uint {
	var material int

	var sides = []int{measures[0] * measures[1], measures[1] * measures[2], measures[0] * measures[2]}

	for _, side := range sides {
		material += 2 * side
	}

	var min = 0
	for i, measure := range sides {
		if i == 0 || measure < min {
			min = measure
		}
	}

	material += min

	return uint(material)
}

func get_measures(line string) []int {
	var nums = []int{}
	chars := strings.Split(line, "x")
	for _, c := range chars {
		num, _ := strconv.Atoi(c)
		nums = append(nums, num)
	}
	return nums
}
