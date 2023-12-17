package partnumber

import (
	"log"
	"os"
	"testing"
)

func TestCountDigits(t *testing.T) {
	if digits(321) != 3 {
		t.Fatalf("321 should be 3 digits instead of %d.", digits(321))
	}
	if digits(1235) != 4 {
		t.Fatalf("1235 should be 4 digits instead of %d.", digits(1235))
	}
	if digits(21) != 2 {
		t.Fatalf("21 should be 2 digits instead of %d.", digits(21))
	}
	if digits(8) != 1 {
		t.Fatalf("8 should be 1 digit instead of %d.", digits(8))
	}
}

func TestParsePartnumbers(t *testing.T) {
	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	partNumbers := ParsePartNumbers(f)

	if len(partNumbers) != 8 {
		t.Fatalf("There should be 8 partnumbers instead of %d.", len(partNumbers))
	}

	expectedPartNumbers := []int{
		467, 35, 633, 617, 592, 755, 664, 598,
	}

outer:
	for _, e := range expectedPartNumbers {
		for _, p := range partNumbers {
			if p.ID == e {
				continue outer
			}
		}
		t.Fatalf("Expected partnumber with ID %d not found.", e)
	}

}

func TestCanSumTotal(t *testing.T) {
	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	partNumbers := ParsePartNumbers(f)

	sum := 0
	for _, p := range partNumbers {
		sum += p.ID
	}
	if sum != 4361 {
		t.Fatalf("Sum must equal %d not %d", 4361, sum)
	}
}

func TestCanSumGearsRatio(t *testing.T) {
	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	gearRatioSum := ParseGearRatioSum(f)

	if gearRatioSum != 467835 {
		t.Fatalf("Sum must equal %d not %d", 467835, gearRatioSum)
	}
}
