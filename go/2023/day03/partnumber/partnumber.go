package partnumber

import (
	"bufio"
	"io"
	"log"
	"math"
	"strconv"
)

func isDigit(r byte) bool {
	if r >= '0' && r <= '9' {
		return true
	}
	return false
}

type vector2d struct {
	x, y int
}

type partNumber struct {
	pos vector2d
	ID  int
}

func ParseEngine(r io.Reader) ([]partNumber, []partSymbol) {
	partNumbers := []partNumber{}
	partSymbols := []partSymbol{}
	scanner := bufio.NewScanner(r)

	y := 0
	for scanner.Scan() {
		val_str := []byte{}
		line := scanner.Text()
		for x := 0; x < len(line); x++ {
			b := line[x]

			if isDigit(b) {
				val_str = append(val_str, b)
				continue
			}

			if len(val_str) > 0 {
				val, err := strconv.Atoi(string(val_str))
				if err != nil {
					log.Fatal(err)
				}
				val_str = []byte{}
				p := partNumber{pos: vector2d{x - digits(val), y}, ID: val}
				partNumbers = append(partNumbers, p)
			}

			if b != '.' {
				p := partSymbol{pos: vector2d{x, y}, id: b}
				partSymbols = append(partSymbols, p)
			}
		}

		if len(val_str) > 0 {
			val, err := strconv.Atoi(string(val_str))
			if err != nil {
				log.Fatal(err)
			}
			val_str = []byte{}
			p := partNumber{pos: vector2d{len(line) - digits(val), y}, ID: val}
			partNumbers = append(partNumbers, p)
		}

		y++
	}

	validPartNumbers := []partNumber{}

outer:
	for _, p := range partNumbers {
		for _, s := range partSymbols {
			if s.pos.y < p.pos.y-1 {
				continue
			}
			if s.pos.y > p.pos.y+1 {
				continue
			}

			if p.isAdjacent(s.pos) {
				validPartNumbers = append(validPartNumbers, p)
				continue outer
			}
		}
	}

	return validPartNumbers, partSymbols
}

func ParsePartNumbers(r io.Reader) []partNumber {
	validPartNumbers, _ := ParseEngine(r)
	return validPartNumbers
}

func ParseGearRatioSum(r io.Reader) int {
	partNumbers, partSymbols := ParseEngine(r)

	gearsum := 0
	for _, p := range partSymbols {
		if p.id == '*' {
			adjacentPartNumbers := getAdjacentPartNumbers(p, partNumbers)
			if len(adjacentPartNumbers) == 2 {
				gearsum += adjacentPartNumbers[0].ID * adjacentPartNumbers[1].ID
			}
		}
	}
	return gearsum
}

func getAdjacentPartNumbers(ps partSymbol, pns []partNumber) []partNumber {
	adjacentPartNumbers := []partNumber{}
	for _, p := range pns {
		if p.isAdjacent(ps.pos) {
			adjacentPartNumbers = append(adjacentPartNumbers, p)
		}
	}
	return adjacentPartNumbers
}

func (p *partNumber) isAdjacent(v vector2d) bool {
	length := digits(p.ID)
	for y := p.pos.y - 1; y <= p.pos.y+1; y++ {
		for x := p.pos.x - 1; x <= p.pos.x+length; x++ {
			if v.x == x && v.y == y {
				return true
			}
		}
	}

	return false
}

type partSymbol struct {
	id  byte
	pos vector2d
}

func digits(i int) int {
	d := 1.0
	for math.Abs(float64(i))/math.Pow(10.0, d) >= 1 {
		d++
	}
	return int(d)
}
