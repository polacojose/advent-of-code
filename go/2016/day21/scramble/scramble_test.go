package scramble

import (
	"testing"
)

func TestSwapPos(t *testing.T) {
	start := "FooBar"
	end := "aooBFr"
	inst := "swap position 4 with position 0"

	scrambler := New([]byte(start))
	scrambler.Execute(inst)
	result := string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}
}

func TestSwapLetter(t *testing.T) {
	start := "FooBar"
	end := "BooFar"
	inst := "swap letter B with letter F"

	scrambler := New([]byte(start))
	scrambler.Execute(inst)
	result := string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}
}

func TestReverse(t *testing.T) {
	start := "FooBar"
	end := "FoaBor"
	inst := "reverse positions 2 through 4"

	scrambler := New([]byte(start))
	scrambler.Execute(inst)
	result := string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}
}

func TestMove(t *testing.T) {
	start := "FooBar"
	end := "ooBaFr"
	inst := "move position 0 to position 4"

	scrambler := New([]byte(start))
	scrambler.Execute(inst)
	result := string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}

	start = "FooBar"
	end = "aFooBr"
	inst = "move position 4 to position 0"

	scrambler = New([]byte(start))
	scrambler.Execute(inst)
	result = string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}
}

func TestRotateLeft(t *testing.T) {
	start := "FooBar"
	end := "oBarFo"
	inst := "rotate left 2 step"

	scrambler := New([]byte(start))
	scrambler.Execute(inst)
	result := string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}
}

func TestRotateRight(t *testing.T) {
	start := "FooBar"
	end := "arFooB"
	inst := "rotate right 2 step"

	scrambler := New([]byte(start))
	scrambler.Execute(inst)
	result := string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}
}

func TestRotateLetter(t *testing.T) {
	start := "FooBar"
	end := "rFooBa"
	inst := "rotate based on position of letter F"

	scrambler := New([]byte(start))
	scrambler.Execute(inst)
	result := string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}

	start = "FooBar"
	end = "rFooBa"
	inst = "rotate based on position of letter r"

	scrambler = New([]byte(start))
	scrambler.Execute(inst)
	result = string(scrambler.Out())

	if result != end {
		t.Fatalf("expected %s but received %s", end, result)
	}
}
