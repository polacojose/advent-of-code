package astar

import (
	"errors"
	"fmt"
)

type Vector struct {
	X int
	Y int
}

type Path []Vector

type AStarConfig interface {
	ValidNode(v Vector) bool
	Distance(a Vector, b Vector) uint
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

func FindPath(start Vector, end Vector, config AStarConfig) ([]Vector, error) {
	nodes := []Vector{start}
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

		if node == end {
			path := reconstructPath(cameFrom, node)
			return path, nil
		}

		for _, n := range neighbors(node, config) {
			tempGScore := getGScore(node) + int(config.Distance(node, n))
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

func PrintPath(p []Vector, config AStarConfig) {

	xMax := 0
	yMax := 0

	for _, p := range p {
		if p.X > xMax {
			xMax = p.X
		}
		if p.Y > yMax {
			yMax = p.Y
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
			if config.ValidNode(Vector{X: x, Y: y}) {
				fmt.Printf("%3s", "⏹")
			} else {
				current := Vector{x, y}
				if containsNode(p, current) {
					if p[0] == current {
						fmt.Printf("%3s", "🏁")
						continue
					}

					if p[len(p)-1] == current {
						fmt.Printf("%3s", "❌")
						continue
					}

					fmt.Printf("%3s", "•")
				} else {
					fmt.Printf("%3s", "")
				}
			}
		}

		fmt.Println()
	}
}

func neighbors(v Vector, config AStarConfig) []Vector {

	n := []Vector{}

	if v.X > 0 {
		a := Vector{X: v.X - 1, Y: v.Y}
		if !config.ValidNode(a) {
			n = append(n, a)
		}
	}

	if v.Y > 0 {
		a := Vector{X: v.X, Y: v.Y - 1}
		if !config.ValidNode(a) {
			n = append(n, a)
		}
	}

	a := Vector{X: v.X + 1, Y: v.Y}
	if !config.ValidNode(a) {
		n = append(n, a)
	}

	a = Vector{X: v.X, Y: v.Y + 1}
	if !config.ValidNode(a) {
		n = append(n, a)
	}

	return n
}
