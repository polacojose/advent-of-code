package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var containers = []int{}
var targetLiters = 150

func init() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		num, _ := strconv.Atoi(line)
		containers = append(containers, num)
	}
}

func main() {
	a := 0
	b := int(^uint32(0))
	getTargetSets([]int{}, []int{}, containers, 0, &b, &a)
}

func getTargetSets(buildSet []int, positionsTaken []int, containers []int, start int, minSize *int, numAtSize *int) {

	sum := 0
	for _, n := range buildSet {
		sum += n
	}

	if sum == targetLiters {
		//fmt.Println(buildSet)

		if len(buildSet) < int(*minSize) {
			*minSize = len(buildSet)
			*numAtSize = 0
		}

		if len(buildSet) == int(*minSize) {
			*numAtSize += 1
			fmt.Println(*numAtSize)
		}

		return
	}

	if sum > targetLiters {
		return
	}

	for i, c := range containers {

		if i < start {
			continue
		}

		exists := false
		for _, taken := range positionsTaken {
			if taken == i {
				exists = true
			}
		}
		if exists {
			continue
		}
		newBuildSet := make([]int, len(buildSet))
		copy(newBuildSet, buildSet)
		newBuildSet = append(newBuildSet, c)

		newPositionsTaken := make([]int, len(positionsTaken))
		copy(newPositionsTaken, positionsTaken)
		newPositionsTaken = append(newPositionsTaken, i)

		getTargetSets(
			newBuildSet,
			newPositionsTaken,
			containers,
			i+1,
			minSize,
			numAtSize,
		)
	}
}
