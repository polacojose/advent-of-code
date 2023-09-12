package main

import (
	astar "day13/path"
	"log"
	"testing"
)

func BenchmarkPath(b *testing.B) {
	for i := 0; i < b.N; i++ {
		config := ASConfig{}
		_, err := astar.FindPath(astar.Vector{X: 1, Y: 1}, astar.Vector{X: 31, Y: 39}, config)
		if err != nil {
			log.Fatal(err)
		}
	}
}
