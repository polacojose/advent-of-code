package main

import "testing"

func TestInstructions(t *testing.T) {
	lights := [1000][1000]bool{}

	setLightsPerInstruction(&lights, "toggle 0,0 through 0,0")
	count := countLights(&lights)

	if count != 1 {
		t.Fatalf("Light count incorrectly %v instead of %v", count, 1)
	}

	setLightsPerInstruction(&lights, "turn on 0,0 through 999,999")
	count = countLights(&lights)

	if count != 1000000 {
		t.Fatalf("Light count incorrectly %v instead of %v", count, 1000000)
	}

	setLightsPerInstruction(&lights, "turn off 0,0 through 999,0")
	count = countLights(&lights)

	if count != 1000000-1000 {
		t.Fatalf("Light count incorrectly %v instead of %v", count, 1000000-1000)
	}

}
