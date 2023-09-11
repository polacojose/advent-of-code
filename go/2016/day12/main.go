package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Register map[string]int

func execute(r Register, s string) Register {

	parts := strings.Fields(s)

	switch parts[0] {
	case "cpy":
		aVal := getRegisterValue(r, parts[1])
		reg := parts[2]

		r[reg] = aVal

		r["ip"] += 1
	case "inc":
		aVal := getRegisterValue(r, parts[1])
		r[parts[1]] = aVal + 1
		r["ip"] += 1
	case "dec":
		aVal := getRegisterValue(r, parts[1])
		r[parts[1]] = aVal - 1
		r["ip"] += 1
	case "jnz":
		aVal := getRegisterValue(r, parts[1])
		bVal := getRegisterValue(r, parts[2])

		if aVal != 0 {
			r["ip"] += bVal
		} else {
			r["ip"] += 1
		}
	}

	return r
}

func getRegisterValue(r Register, ref string) int {
	v, err := strconv.Atoi(ref)
	if err == nil {
		return v
	}

	return r[ref]
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	lines := []string{}
	for fileScanner.Scan() {
		lines = append(lines, fileScanner.Text())
	}

	r := Register{}
	for r["ip"] < len(lines) {
		line := lines[r["ip"]]
		r = execute(r, line)
	}
	fmt.Println("Part01:", r["a"])

	r = Register{"c": 1}
	for r["ip"] < len(lines) {
		line := lines[r["ip"]]
		r = execute(r, line)
	}
	fmt.Println("Part02:", r["a"])

}
