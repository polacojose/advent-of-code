package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	runeMap := map[int]map[rune]int{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		for i, r := range line {
			occMap := runeMap[i]
			if occMap == nil {
				runeMap[i] = map[rune]int{}
			}
			occ := occMap[r]
			runeMap[i][r] = occ + 1
		}
	}

	mostCommonRunes := mostCommonRep(runeMap)
	leastCommonRunes := leastCommonRep(runeMap)
	fmt.Println(string(mostCommonRunes))
	fmt.Println(string(leastCommonRunes))
}

func leastCommonRep(runeMap map[int]map[rune]int) []rune {
	passRunes := []rune{}
	for i := 0; i < len(runeMap); i++ {
		pos := runeMap[i]
		var passRune rune
		leadOcc := int(^uint(0) >> 1)
		for k, v := range pos {
			if v < leadOcc {
				leadOcc = v
				passRune = k
			}
		}
		passRunes = append(passRunes, passRune)
	}
	return passRunes
}

func mostCommonRep(runeMap map[int]map[rune]int) []rune {
	passRunes := []rune{}
	for i := 0; i < len(runeMap); i++ {
		pos := runeMap[i]
		var passRune rune
		leadOcc := 0
		for k, v := range pos {
			if v > leadOcc {
				leadOcc = v
				passRune = k
			}
		}
		passRunes = append(passRunes, passRune)
	}
	return passRunes
}
