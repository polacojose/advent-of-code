package main

import "testing"

func TestInstructions(t *testing.T) {

	lights := [1000][1000]int{}
	setLightsPerInstruction(&lights, "turn on 0,0 through 0,0")
	count := countLights(&lights)

	if count != 1 {
		t.Fatalf("Light count incorrectly %v instead of %v", count, 1)
	}

	lights = [1000][1000]int{}
	setLightsPerInstruction(&lights, "toggle 0,0 through 999,999")
	count = countLights(&lights)

	if count != 2000000 {
		t.Fatalf("Light count incorrectly %v instead of %v", count, 2000000)
	}

}
