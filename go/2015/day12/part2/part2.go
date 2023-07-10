package part2

import (
	"encoding/json"
)

func GetNonRedNumbers(line string) []int {

	var deepMap any
	json.Unmarshal([]byte(line), &deepMap)

	//fmt.Println("Line:", line)
	//fmt.Println("Bytes:", deepMap)

	return GetDeepNumbers(deepMap)
}

func GetDeepNumbers(input any) []int {

	if inputMap, ok := input.(map[string]any); ok {
		for _, val := range inputMap {
			if val == "red" {
				return []int{}
			}
		}
	}

	numbers := []int{}

	if inputMap, ok := input.(map[string]any); ok {
		for _, val := range inputMap {

			if subVal, ok := val.(map[string]any); ok {
				for _, v := range GetDeepNumbers(subVal) {
					numbers = append(numbers, v)
				}
				continue
			}

			if subVal, ok := val.([]any); ok {
				for _, v := range GetDeepNumbers(subVal) {
					numbers = append(numbers, v)
				}
				continue
			}

			if num, ok := val.(float64); ok {
				numbers = append(numbers, int(num))
				continue
			}
		}
	}

	if array, ok := input.([]any); ok {
		for _, val := range array {

			if subVal, ok := val.(map[string]any); ok {
				for _, v := range GetDeepNumbers(subVal) {
					numbers = append(numbers, v)
				}
				continue
			}

			if subVal, ok := val.([]any); ok {
				for _, v := range GetDeepNumbers(subVal) {
					numbers = append(numbers, v)
				}
				continue
			}

			if subVal, ok := val.(float64); ok {
				numbers = append(numbers, int(subVal))
				continue
			}
		}
	}

	return numbers
}
