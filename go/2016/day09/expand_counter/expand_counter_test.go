package expandcounter

import "testing"

func TestExpandCounter(t *testing.T) {
	expandCounter := *NewExpandCounter("ADVENT", false)
	result := expandCounter.Count()
	if result != 6 {
		t.Errorf("Expected 6, got %d", result)
	}

	expandCounter = *NewExpandCounter("A(1x5)BC", false)
	result = expandCounter.Count()
	if result != 7 {
		t.Errorf("Expected 7, got %d", result)
	}

	expandCounter = *NewExpandCounter("A(2x2)BCD(2x2)EFG", false)
	result = expandCounter.Count()
	if result != 11 {
		t.Errorf("Expected 11, got %d", result)
	}

	expandCounter = *NewExpandCounter("(6x1)(1x3)A", false)
	result = expandCounter.Count()
	if result != 6 {
		t.Errorf("Expected 6, got %d", result)
	}

	expandCounter = *NewExpandCounter("X(8x2)(3x3)ABCY", false)
	result = expandCounter.Count()
	if result != 18 {
		t.Errorf("Expected 18, got %d", result)
	}
}

func TestRecursiveExpandCounter(t *testing.T) {
	expandCounter := *NewExpandCounter("(3x3)XYZ", true)
	result := expandCounter.Count()
	if result != 9 {
		t.Errorf("Expected 9, got %d", result)
	}

	expandCounter = *NewExpandCounter("X(8x2)(3x3)ABCY", true)
	result = expandCounter.Count()
	if result != 20 {
		t.Errorf("Expected 20, got %d", result)
	}

	expandCounter = *NewExpandCounter("(27x12)(20x12)(13x14)(7x10)(1x12)A", true)
	result = expandCounter.Count()
	if result != 241920 {
		t.Errorf("Expected 241920, got %d", result)
	}

	expandCounter = *NewExpandCounter("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", true)
	result = expandCounter.Count()
	if result != 445 {
		t.Errorf("Expected 445, got %d", result)
	}
}
