package main

import (
	"fmt"
	"testing"
)

func Deer_test(t *testing.T) {

	comet := Deer{
		speed:       14,
		dashSeconds: 10,
		restSeconds: 127,
	}

	dancer := Deer{
		speed:       16,
		dashSeconds: 11,
		restSeconds: 162,
	}

	seconds := 1000
	cometPosition := getDeerPosition(seconds, comet)
	dancerPosition := getDeerPosition(seconds, dancer)

	fmt.Printf("At %v Seconds\n", seconds)
	fmt.Printf("Comet Position:  %v\n", cometPosition)
	fmt.Printf("Dancer Position: %v\n", dancerPosition)

	if cometPosition != 2660 {
		t.Fatalf("Comet in the wrong position: %v != %v", cometPosition, 2660)
	}

	if dancerPosition != 2660 {
		t.Fatalf("Dancer in the wrong position: %v != %v", dancerPosition, 2660)
	}
}
