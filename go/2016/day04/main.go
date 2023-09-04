package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type room struct {
	name     string
	checksum string
	sectorId int
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	validIdSum := 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		r := parseRoom(line)
		if validateChecksum(r) {
			validIdSum += r.sectorId

			deName := decryptRoomName(r)
			if strings.Index(deName, "north") >= 0 {
				fmt.Println(r.sectorId, deName)
			}

		}
	}

	fmt.Println("Valid Rooms:", validIdSum)
}

func parseRoom(line string) room {
	sepA := strings.LastIndex(line, "-")
	sepB := strings.LastIndex(line, "[")
	name := line[:sepA]
	id, err := strconv.Atoi(line[sepA+1 : sepB])
	if err != nil {
		panic(err)
	}
	checksum := line[sepB+1 : len(line)-1]

	return room{
		name:     name,
		sectorId: id,
		checksum: checksum,
	}
}

func validateChecksum(r room) bool {
	name := strings.ReplaceAll(r.name, "-", "")

	alphaSet := make(map[string]int)
	for _, rune := range name {
		s := string(rune)
		n := alphaSet[s]
		alphaSet[s] = n + 1
	}

	alphaSlice := []struct {
		name string
		inst int
	}{}
	for k, v := range alphaSet {
		alphaSlice = append(alphaSlice, struct {
			name string
			inst int
		}{
			name: k,
			inst: v,
		})
	}

	sort.Slice(alphaSlice, func(i, j int) bool {
		if alphaSlice[i].inst == alphaSlice[j].inst {
			return alphaSlice[i].name < alphaSlice[j].name
		}
		return alphaSlice[i].inst > alphaSlice[j].inst

	})

	validChecksum := ""
	for _, a := range alphaSlice[:5] {
		validChecksum += a.name
	}

	return r.checksum == validChecksum
}

func decryptRoomName(r room) string {

	decryptedName := ""
	for _, char := range r.name {
		if char == '-' {
			decryptedName += " "
			continue
		}
		runeId := int(char) - 'a' + r.sectorId
		deChar := string(rune(runeId%26 + 'a'))
		decryptedName += deChar
	}

	return decryptedName
}
