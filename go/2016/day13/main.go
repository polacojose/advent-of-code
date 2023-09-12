package main

import (
	astar "day13/path"
	"fmt"
	"log"
	"math"
	"math/bits"
)

const favoriteNumber = 1350

func main() {

	config := ASConfig{}
	path, err := astar.FindPath(astar.Vector{X: 1, Y: 1}, astar.Vector{X: 31, Y: 39}, config)
	if err != nil {
		log.Fatal(err)
	}
	astar.PrintPath(path, config)
	fmt.Println("Steps:", len(path)-1)
	fmt.Println("Nodes:", path)

	const nearSteps = 50
	nearNodes := map[astar.Vector]bool{
		{X: 1, Y: 1}: true,
	}
	usedNodes := map[astar.Vector]bool{}
	for y := nearSteps; y >= 0; y-- {
		for x := nearSteps; x >= 0; x-- {
			node := astar.Vector{X: x, Y: y}

			if _, ok := usedNodes[node]; ok {
				continue
			}

			usedNodes[node] = true

			path, err := astar.FindPath(astar.Vector{X: 1, Y: 1}, node, config)
			if err != nil {
				continue
			}
			path = path[1:]

			if len(path) > nearSteps {
				path = path[:nearSteps]
			}

			for _, n := range path {
				nearNodes[n] = true
				usedNodes[n] = true
			}
		}
	}

	fmt.Printf("Nodes within %d steps: %d\n", nearSteps, len(nearNodes))
}

type ASConfig struct{}

func (c ASConfig) ValidNode(v astar.Vector) bool {
	x, y := v.X, v.Y
	a := x*x + 3*x + 2*x*y + y + y*y
	b := a + favoriteNumber

	return bits.OnesCount(uint(b))%2 != 0
}

func (c ASConfig) Distance(aV astar.Vector, bV astar.Vector) uint {
	return uint(math.Abs(float64(aV.X-bV.X)) + math.Abs(float64(aV.Y-bV.Y)))
}
