package main

import (
	"crypto/md5"
	"fmt"
	"io"
)

type Position struct {
	x int
	y int
}

type Node struct {
	hash string
	pos  Position
}

const startHash = "pvhmgsws"

func main() {

	winPos := Position{3, 3}

	shortestPath(winPos)
	longestPath(winPos)
}

func longestPath(winPos Position) {
	nodes := []Node{{
		pos:  Position{0, 0},
		hash: startHash,
	}}

	longest := 0
	for len(nodes) > 0 {
		curNode := nodes[0]
		nodes = nodes[1:]

		if curNode.pos == winPos {
			length := len(curNode.hash)
			if length > longest {
				longest = length
			}
			continue
		}

		nextNodes := getNextNodes(curNode)
		nodes = append(nodes, nextNodes...)
	}

	fmt.Println("Longest Path:", longest-len(startHash))
}

func shortestPath(winPos Position) {
	nodes := []Node{{
		pos:  Position{0, 0},
		hash: startHash,
	}}
	for len(nodes) > 0 {
		curNode := nodes[0]
		nodes = nodes[1:]

		if curNode.pos == winPos {
			fmt.Println("Path:", curNode.hash[len(startHash):])
			return
		}

		nextNodes := getNextNodes(curNode)
		nodes = append(nodes, nextNodes...)
	}
	return
}

func getNextNodes(n Node) []Node {

	h := md5.New()
	io.WriteString(h, n.hash)
	hash := fmt.Sprintf("%x", h.Sum(nil))[:4]

	nextNodes := []Node{}
	for i, c := range hash {

		switch c {
		case 'b', 'c', 'd', 'e', 'f':
		default:
			continue
		}

		char := []string{"U", "D", "L", "R"}[i]

		switch char {
		case "U":
			p := n.pos
			p.y--
			if p.y < 0 {
				continue
			}
			nextNodes = append(nextNodes, Node{
				hash: n.hash + "U",
				pos:  p,
			})
		case "D":
			p := n.pos
			p.y++
			if p.y > 3 {
				continue
			}
			nextNodes = append(nextNodes, Node{
				hash: n.hash + "D",
				pos:  p,
			})
		case "L":
			p := n.pos
			p.x--
			if p.x < 0 {
				continue
			}
			nextNodes = append(nextNodes, Node{
				hash: n.hash + "L",
				pos:  p,
			})
		case "R":
			p := n.pos
			p.x++
			if p.x > 3 {
				continue
			}
			nextNodes = append(nextNodes, Node{
				hash: n.hash + "R",
				pos:  p,
			})
		}
	}

	return nextNodes
}
