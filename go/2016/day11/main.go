package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type DeviceType string

const (
	microchip DeviceType = "M"
	generator DeviceType = "G"
)

type Device struct {
	name  string
	_type DeviceType
	floor int
	id    int
}

type BuildingState struct {
	devices  []Device
	elevator int
	maxFloor int
	minFloor int
}

type Path []BuildingState

var seenStates = map[string]bool{}

func init() {
	seenStates = map[string]bool{}
}

func main() {

	buildingState, err := getInitialBuildingState("input.txt")
	if err != nil {
		log.Fatal(err)
		return
	}

	paths := []Path{}
	paths = append(paths, Path{buildingState})

	seenStates[fmt.Sprintf("%v", buildingState)] = true

	depth := 0
	for len(paths) > 0 {
		path := paths[0]
		paths = paths[1:]
		leadState := path[len(path)-1]

		if len(path) > depth {
			depth = len(path)
			fmt.Println("Depth:", depth)
			fmt.Println("Search Space:", len(paths))
		}

		if getMinFloor(leadState) == leadState.maxFloor {
			fmt.Println(leadState)
			fmt.Println("------------")
			for _, p := range path {
				printBuildingState(p)
				fmt.Println()
			}
			fmt.Println("------------")
			fmt.Println("States:", len(path))
			fmt.Println("Steps:", len(path)-1)
			return
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

func populatePaths(nextState BuildingState, leadState BuildingState, path Path, paths []Path) []Path {
	for i := -1; i < len(nextState.devices); i++ {
		if i >= 0 && nextState.devices[i].floor != leadState.elevator {
			continue
		}

		for j := i + 1; j < len(nextState.devices); j++ {
			if j >= 0 && nextState.devices[j].floor != leadState.elevator {
				continue
			}

			subState := cloneBuildingState(nextState)
			subState.devices[j].floor = subState.elevator

			if i >= 0 {
				subState.devices[i].floor = subState.elevator
			}

			if validateBuildingState(subState) && !stateExists(subState) {
				subState.minFloor = getMinFloor(subState)
				nextPath := append(Path{}, append(path, subState)...)
				paths = append(paths, nextPath)
			}
		}
	}
	return paths
}

func stateExists(state BuildingState) bool {
	key := fmt.Sprintf("%v", state)
	if _, ok := seenStates[key]; ok {
		return true
	} else {
		seenStates[key] = true
		return false
	}
}

func cloneBuildingState(b BuildingState) BuildingState {
	newState := b
	newState.devices = append([]Device{}, b.devices...)
	return newState
}

func getMinFloor(b BuildingState) int {
	minFloor := b.maxFloor

	for _, d := range b.devices {
		if d.floor < minFloor {
			minFloor = d.floor
		}
	}

	return minFloor
}

func validateBuildingState(b BuildingState) bool {
	for _, d := range b.devices {
		if d._type != microchip {
			continue
		}

		protected := true
		for _, sD := range b.devices {
			if sD._type != generator {
				continue
			}
			if sD.floor != d.floor {
				continue
			}
			if d.name != sD.name {
				protected = false
			}
		}

		for _, sD := range b.devices {
			if sD._type != generator {
				continue
			}
			if sD.floor != d.floor {
				continue
			}
			if d.name == sD.name {
				protected = true
				break
			}
		}

		if !protected {
			return false
		}
	}

	return true
}

func printBuildingState(b BuildingState) {

	//	horzIndex := 0
	for i := b.maxFloor; i >= 0; i-- {
		fmt.Printf("F%d ", i+1)

		if b.elevator == i {
			fmt.Print("E  ")
		} else {
			fmt.Print("|  ")
		}

		floorIndex := 0
		//	for floorIndex < horzIndex {
		//		fmt.Print("  .  ")
		//		floorIndex++
		//	}

		for _, d := range b.devices {
			if d.floor == i {
				for floorIndex < d.id {
					fmt.Print("  .  ")
					floorIndex++
				}
				fmt.Printf("%s-%s ", d.name, d._type)
				floorIndex++
				//				horzIndex++
			}
		}

		for floorIndex < len(b.devices) {
			fmt.Print("  .  ")
			floorIndex++
		}

		fmt.Println()
	}
}

func getInitialBuildingState(s string) (BuildingState, error) {

	f, err := os.Open(s)
	if err != nil {
		return BuildingState{}, err
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	devices := []Device{}
	floorIndex := -1
	id := 0
	for fileScanner.Scan() {
		floorIndex++
		line := fileScanner.Text()
		line = strings.ReplaceAll(line, ".", "")

		if strings.Index(line, "nothing relevant") >= 0 {
			continue
		}
		line = line[strings.Index(line, "contains a ")+len("contains a "):]
		line = strings.ReplaceAll(line, ",", "")
		line = strings.ReplaceAll(line, " and ", "")
		parts := strings.Split(line, "a ")

		for _, part := range parts {
			deviceParts := strings.Fields(part)
			deviceType := microchip
			if deviceParts[1] == "generator" {
				deviceType = generator
			}

			devices = append(devices, Device{
				name:  strings.ToUpper(string(deviceParts[0][0:2])),
				_type: deviceType,
				floor: floorIndex,
				id:    id,
			})
			id++
		}
	}

	fmt.Println(floorIndex)

	return BuildingState{
		maxFloor: floorIndex,
		devices:  devices,
	}, nil
}
