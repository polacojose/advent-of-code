package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

var STRING_INT_MAP map[string]rune

func init() {
	STRING_INT_MAP = map[string]rune{
		"zero":  '0',
		"one":   '1',
		"two":   '2',
		"three": '3',
		"four":  '4',
		"five":  '5',
		"six":   '6',
		"seven": '7',
		"eight": '8',
		"nine":  '9',
	}
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	fileScanner := bufio.NewScanner(f)

	lines := []string{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		lines = append(lines, line)
	}

	part1(lines)
	part2(lines)
}

func part1(lines []string) {
	total := 0
	for _, line := range lines {
		val, err := GetPart1CalibrationValue(line)
		if err != nil {
			log.Fatal(err)
		}
		total += val

	}
	println(total)
}

func part2(lines []string) {
	total := 0
	for _, line := range lines {
		val, err := GetPart2CalibrationValue(line)
		if err != nil {
			log.Fatal(err)
		}
		total += val

	}
	println(total)
}

func GetPart1CalibrationValue(s string) (int, error) {
	var nums = []rune{}
	for _, a := range s {
		_, err := strconv.Atoi(string(a))
		if err == nil {
			nums = append(nums, a)
		}
	}
	return strconv.Atoi(string(nums[0]) + string(nums[len(nums)-1]))
}

func GetPart2CalibrationValue(s string) (int, error) {
	var nums = []rune{}
	for i, a := range s {
		_, err := strconv.Atoi(string(a))
		if err == nil {
			nums = append(nums, a)
			continue
		}

		for k, v := range STRING_INT_MAP {
			if strings.Index(s[i:], string(k)) == 0 {
				nums = append(nums, v)
			}
		}
	}
	return strconv.Atoi(string(nums[0]) + string(nums[len(nums)-1]))
}
