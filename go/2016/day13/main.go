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
