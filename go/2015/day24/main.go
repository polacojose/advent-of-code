package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	optimumGroup(3)
	optimumGroup(4)
}

func optimumGroup(groupings int) {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	packageSizes := []int{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		lineNum, _ := strconv.Atoi(line)
		packageSizes = append(packageSizes, lineNum)
	}
	packagesSizeGroup := sumNums(packageSizes) / groupings

	paths := [][]int{}
	for _, p := range packageSizes {
		paths = append(paths, []int{p})
	}

	minPackages := int(^uint(0) >> 1)
	minQE := ^uint64(0)
	for len(paths) > 0 {
		path := paths[len(paths)-1]
		paths = paths[:len(paths)-1]

		pathSum := sumNums(path)
		if pathSum < packagesSizeGroup {

			if len(path) >= minPackages {
				continue
			}

			for _, p := range packageSizes {

				if p >= path[len(path)-1] {
					continue
				}

				if pathSum+p <= packagesSizeGroup {
					newPath := append([]int{}, path...)
					newPath = append(newPath, p)
					paths = append(paths, newPath)
				}
			}
		} else if pathSum == packagesSizeGroup {
			if len(path) < minPackages {
				minPackages = len(path)
				minQE = getQE(path)
				fmt.Println(path)
				fmt.Println(minQE)
			} else if len(path) == minPackages {
				qe := getQE(path)
				if qe < minQE {
					minPackages = len(path)
					minQE = qe
					fmt.Println(path)
					fmt.Println(minQE)
				}
			}
		}
	}
}

func getQE(nums []int) uint64 {
	var qe uint64 = 1
	for _, n := range nums {
		//fmt.Println(qe, n)
		qe *= uint64(n)
	}
	return qe
}

func sumNums(nums []int) int {
	sum := 0
	for _, num := range nums {
		sum += num
	}
	return sum
}
