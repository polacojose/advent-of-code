package main

func codeToSlice(input string) []rune {
	startCode := []rune(input)
	for i := 0; i < len(startCode); i++ {
		startCode[i] -= 'a'
	}
	return startCode
}

func codeToString(code []rune) string {
	string_runes := []rune{}
	for i := 0; i < len(code); i++ {
		string_runes = append(string_runes, (code)[i]+'a')
	}
	return string(string_runes)
}

func incrementLetterCode(code *[]rune) {

	hasCarry := true
	for i := len(*code) - 1; i >= 0; i-- {

		if (*code)[i] >= 25 {
			(*code)[i] = 0
			hasCarry = true
		} else {
			(*code)[i]++
			hasCarry = false
		}

		if hasCarry {
			continue
		} else {
			break
		}
	}

	if hasCarry {
		(*code) = append([]rune{'a'}, (*code)...)
	}

}
