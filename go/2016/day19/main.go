package main

import (
	"errors"
	"fmt"
)

const elfCircleSize = 3014387

//const elfCircleSize = 5

type Elf struct {
	seat int
}

func main() {
	//getLastAliveInCircle()
	getLastAliveAcross()
}

func getLastAliveInCircle() {
	elves := make([]bool, elfCircleSize)
	for i := 0; i < elfCircleSize; i++ {
		elves[i] = true
	}

	killer := -1
	for alive(elves) > 1 {
		for i := 0; i < len(elves); i++ {
			if !elves[i] {
				continue
			}

			if killer == -1 {
				killer = i
			} else {
				killer = -1
				elves[i] = false
			}

		}
	}

	lastIndex, _ := firstAliveIndex(elves)
	fmt.Println(lastIndex + 1)
}

func getLastAliveAcross() {
	elves := make([]Elf, elfCircleSize)
	for i := 0; i < elfCircleSize; i++ {
		elves[i] = Elf{i + 1}
	}

	for len(elves) > 1 {
		midIndex := len(elves) / 2
		elves = append(elves, elves[0])
		elves = append(elves[1:midIndex], elves[midIndex+1:]...)
		fmt.Println(len(elves))
	}

	fmt.Println(elves[0].seat)
}

func firstAliveIndex(elves []bool) (int, error) {
	for i, e := range elves {
		if e {
			return i, nil
		}
	}
	return 0, errors.New("none alive")
}

func alive(elves []bool) int {
	num := 0

	for _, n := range elves {
		if n {
			num++
		}
	}

	return num
}
