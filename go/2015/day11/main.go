package main

import (
	"fmt"
)

func main() {
	passwords := getNextPasswords("hxbxwxba", 2)
	fmt.Println(passwords)
}

func getNextPasswords(start string, number int) []string {
	code := codeToSlice(start)
	passwords := []string{}
	for i := 0; ; i++ {
		incrementLetterCode(&code)

		if containsIncreasingThree(&code) && noIllegalCharacters(&code) && containsTwoPairs(&code) {
			passwords = append(passwords, codeToString(code))
			if len(passwords) >= number {
				return passwords
			}
		}
	}
}
