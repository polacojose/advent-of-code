package main

import (
	"errors"
	"fmt"
	"log"
	"math"
	"math/bits"
)

type Vector struct {
	x int
	y int
}

type Path []Vector

const favoriteNumber = 1350

func isWall(v Vector) bool {
	x, y := v.x, v.y
	a := x*x + 3*x + 2*x*y + y + y*y
	b := a + favoriteNumber

	return bits.OnesCount(uint(b))%2 != 0
}

func reconstructPath(cameFrom map[Vector]Vector, current Vector) []Vector {
	path := []Vector{current}
	for {
		if n, ok := cameFrom[current]; ok {
			path = append([]Vector{n}, path...)
			current = n
		} else {
			break
		}
	}
	return path
}

func main() {

	path, err := findPath(Vector{1, 1}, Vector{31, 39})
	if err != nil {
		log.Fatal(err)
	}
	printPath(path)
	fmt.Println("Steps:", len(path)-1)
	fmt.Println("Nodes:", path)
}

func findPath(startNode Vector, endNode Vector) ([]Vector, error) {
	nodes := []Vector{startNode}
	gScore := map[Vector]int{
		nodes[0]: 0,
	}
	getGScore := func(v Vector) int {
		if g, ok := gScore[v]; ok {
			return g
		}

		return int(^uint(0) >> 1)
	}

	cameFrom := map[Vector]Vector{}

	for len(nodes) > 0 {
		node := nodes[0]
		nodes = nodes[1:]

		if node == endNode {
			path := reconstructPath(cameFrom, node)
			return path, nil
		}

		for _, n := range neighbors(node) {
			tempGScore := getGScore(node) + int(distance(node, n))
			if tempGScore < getGScore(n) {
				cameFrom[n] = node
				gScore[n] = tempGScore

				if !containsNode(nodes, n) {
					nodes = append(nodes, n)
				}
			}
		}

	}

	return []Vector{}, errors.New("failed to find path")
}

func containsNode(nodes []Vector, node Vector) bool {

	for _, n := range nodes {
		if n == node {
			return true
		}
	}

	return false
}

func distance(aV Vector, bV Vector) uint {
	return uint(math.Abs(float64(aV.x-bV.x)) + math.Abs(float64(aV.y-bV.y)))
}

func neighbors(v Vector) []Vector {

	n := []Vector{}

	if v.x > 0 {
		a := Vector{v.x - 1, v.y}
		if !isWall(a) {
			n = append(n, a)
		}
	}

	if v.y > 0 {
		a := Vector{v.x, v.y - 1}
		if !isWall(a) {
			n = append(n, a)
		}
	}

	a := Vector{v.x + 1, v.y}
	if !isWall(a) {
		n = append(n, a)
	}

	a = Vector{v.x, v.y + 1}
	if !isWall(a) {
		n = append(n, a)
	}

	return n
}

func printPath(p []Vector) {

	xMax := 0
	yMax := 0

	for _, p := range p {
		if p.x > xMax {
			xMax = p.x
		}
		if p.y > yMax {
			yMax = p.y
		}
	}

	xMax++
	yMax++

	fmt.Printf("%3s", "")
	for x := 0; x <= int(xMax); x++ {
		fmt.Printf("%3d", x)
	}
	fmt.Println()

	for y := 0; y <= int(yMax); y++ {
		fmt.Printf("%3d", y)

		for x := 0; x <= int(xMax); x++ {
			if isWall(Vector{x: x, y: y}) {
				fmt.Printf("%3s", "⏹")
			} else {
				current := Vector{x, y}
				if containsNode(p, current) {
					if p[0] == current {
						fmt.Printf("%3s", "🏁")
						continue
					}

					if p[len(p)-1] == current {
						fmt.Printf("%3s", "◎")
						continue
					}

					fmt.Printf("%3s", ".")
				} else {
					fmt.Printf("%3s", "")
				}
			}
		}

		fmt.Println()
	}
}
