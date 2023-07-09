package main

func containsIncreasingThree(code *[]rune) bool {
	for i := 2; i < len(*code); i++ {
		if (*code)[i-2]+2 == (*code)[i-1]+1 && (*code)[i-1]+1 == (*code)[i] {
			return true
		}
	}
	return false
}

func noIllegalCharacters(code *[]rune) bool {

	for _, c := range *code {
		offsetRune := c + 'a'
		if offsetRune == 'i' {
			return false
		}
		if offsetRune == 'o' {
			return false
		}
		if offsetRune == 'l' {
			return false
		}
	}

	return true
}

func containsTwoPairs(code *[]rune) bool {

	foundPair := false

	i := 1
	for i < len(*code) {
		if (*code)[i-1] == (*code)[i] {
			if foundPair == true {
				return true
			}
			foundPair = true
			i++
		}
		i++
	}
	return false
}
