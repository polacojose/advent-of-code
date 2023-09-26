package main

import (
	"errors"
	"fmt"
)

type elfCircle [3014387]bool

func main() {
	getLastAliveInCircle()
}

func getLastAliveInCircle() {
	elves := elfCircle{}
	killer := -1
	for alive(elves) > 1 {
		for i := 0; i < len(elves); i++ {
			if elves[i] {
				continue
			}

			if killer == -1 {
				killer = i
			} else {
				killer = -1
				elves[i] = true
			}

		}
	}

	lastIndex, _ := aliveIndex(elves)
	fmt.Println(lastIndex + 1)
}

func aliveIndex(elves elfCircle) (int, error) {

	for i, e := range elves {
		if !e {
			return i, nil
		}
	}
	return 0, errors.New("none alive")
}

func alive(elves elfCircle) int {
	num := 0

	for _, n := range elves {
		if !n {
			num++
		}
	}

	return num
}

