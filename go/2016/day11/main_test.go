package main

import (
	"fmt"
	"log"
	"testing"
)

func BenchmarkMain(b *testing.B) {
	buildingState, err := getInitialBuildingState("demo-input.txt")
	if err != nil {
		log.Fatal(err)
		return
	}

	for i := 0; i < b.N; i++ {
		paths := []Path{}
		paths = append(paths, Path{cloneBuildingState(buildingState)})

		for len(paths) > 0 {
			path := paths[0]
			paths = paths[1:]
			leadState := path[len(path)-1]

			if getMinFloor(leadState) == leadState.maxFloor {
				log.Println(leadState)
				fmt.Println("------------")
				for _, p := range path {
					printBuildingState(p)
					fmt.Println()
				}
				fmt.Println("------------")
				log.Println(len(path))
				break
			}

			if leadState.elevator < buildingState.maxFloor {
				nextState := cloneBuildingState(leadState)
				nextState.elevator++
				paths = populatePaths(nextState, leadState, path, paths)
			}

			if leadState.elevator > buildingState.minFloor {
				nextState := cloneBuildingState(leadState)
				nextState.elevator--
				paths = populatePaths(nextState, leadState, path, paths)
			}

		}
	}
}
