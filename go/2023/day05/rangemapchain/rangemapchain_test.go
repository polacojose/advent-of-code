package rangemapchain

import (
	"log"
	"os"
	"testing"
)

func TestFindLowestNumberLocation(t *testing.T) {
	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	rangeMapChain := NewRangeMapChain(f)

	l := rangeMapChain.LowestNumberLocation()
	if l != 35 {
		t.Fatalf("Lowest number location should be 35 not %d", l)
	}
}

func TestFindLowestNumberLocationWithSeedRanges(t *testing.T) {
	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	rangeMapChain := NewRangeMapChain(f)

	l := rangeMapChain.LowestNumberLocationWithSeedRanges()
	if l != 46 {
		t.Fatalf("Lowest number location should be 46 not %d", l)
	}
}
