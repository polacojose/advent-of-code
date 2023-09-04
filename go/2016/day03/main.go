package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	validateRowTriangles()
	validateColTriangles()
}

func validateColTriangles() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	totalValid := 0

	fieldBuff := [][]string{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		fieldBuff = append(fieldBuff, strings.Fields(line))

		if len(fieldBuff) < 3 {
			continue
		}

		triangles := make([][]int, 3)
		for y := 0; y < 3; y++ {
			for x := 0; x < 3; x++ {
				num, err := strconv.Atoi(fieldBuff[y][x])
				if err != nil {
					fmt.Println(err)
					return
				}
				triangles[x] = append(triangles[x], num)
			}
		}

		for _, t := range triangles {
			if validTriangle(t[0], t[1], t[2]) {
				totalValid++
			}
		}

		fieldBuff = [][]string{}
	}

	fmt.Println("Total Valid Triangles:", totalValid)
}

func validateRowTriangles() {
	f, _ := os.Open("input.txt")
	defer f.Close()
	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	totalValid := 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		fields := strings.Fields(line)

		a, _ := strconv.Atoi(fields[0])
		b, _ := strconv.Atoi(fields[1])
		c, _ := strconv.Atoi(fields[2])

		if validTriangle(a, b, c) {
			totalValid++
		}
	}

	fmt.Println("Total Valid Triangles:", totalValid)
}

func validTriangle(a int, b int, c int) bool {
	return (a+b) > c && (b+c) > a && (c+a) > b
}
