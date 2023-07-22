package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var props = map[string]int{
	"children":    3,
	"cats":        7,
	"samoyeds":    2,
	"pomeranians": 3,
	"akitas":      0,
	"vizslas":     0,
	"goldfish":    5,
	"trees":       3,
	"cars":        2,
	"perfumes":    1,
}

func main() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		aunt := fileScanner.Text()

		if auntHasPropertiesPart2(props, aunt) {
			fmt.Println(aunt)
			break
		}
	}
}

func auntHasPropertiesPart1(properties map[string]int, auntString string) bool {
	auntParts := strings.SplitN(auntString, ":", 2)
	auntPropertiesString := strings.TrimSpace(auntParts[1])
	auntProperties := strings.Split(auntPropertiesString, ",")

	for _, auntProps := range auntProperties {
		propertyParts := strings.Split(auntProps, ":")
		propertyName := strings.TrimSpace(propertyParts[0])
		propertyValue, _ := strconv.Atoi(strings.TrimSpace(propertyParts[1]))

		if val, ok := properties[propertyName]; ok {
			if val != propertyValue {
				return false
			}
		}
	}

	return true
}

func auntHasPropertiesPart2(properties map[string]int, auntString string) bool {
	auntParts := strings.SplitN(auntString, ":", 2)
	auntPropertiesString := strings.TrimSpace(auntParts[1])
	auntProperties := strings.Split(auntPropertiesString, ",")

	for _, auntProps := range auntProperties {
		propertyParts := strings.Split(auntProps, ":")
		propertyName := strings.TrimSpace(propertyParts[0])
		propertyValue, _ := strconv.Atoi(strings.TrimSpace(propertyParts[1]))

		if val, ok := properties[propertyName]; ok {

			switch propertyName {
			case "cats":
				fallthrough
			case "trees":
				if propertyValue <= val {
					return false
				}
			case "pomeranians":
				fallthrough
			case "goldfish":
				if propertyValue >= val {
					return false
				}
			default:
				if val != propertyValue {
					return false
				}
			}

		}
	}

	return true
}
