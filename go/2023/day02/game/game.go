package game

import (
	"strconv"
	"strings"
)

type Draw struct {
	colors map[string]int
}

func ParseDraw(s string) Draw {
	s_split := strings.Split(s, ",")
	colors := make(map[string]int, len(s_split))
	for _, c := range s_split {
		c_split := strings.Split(strings.TrimSpace(c), " ")
		amount, _ := strconv.Atoi(c_split[0])
		colors[c_split[1]] = amount
	}
	return Draw{colors: colors}
}

func drawEqual(d1 Draw, d2 Draw) bool {
	if len(d1.colors) != len(d2.colors) {
		return false
	}
	for c, v := range d1.colors {
		if d2.colors[c] != v {
			return false
		}
	}
	return true
}

type GameResult struct {
	draws []Draw
	ID    int
}

func ParseGame(s string) GameResult {
	s_split := strings.Split(s, ":")
	id, _ := strconv.Atoi(strings.TrimSpace(strings.Split(s_split[0], " ")[1]))

	draw_strings := strings.Split(strings.TrimSpace(s_split[1]), ";")
	draws := make([]Draw, 0, len(draw_strings))
	for _, ds := range draw_strings {
		draws = append(draws, ParseDraw(ds))
	}
	return GameResult{ID: id, draws: draws}
}

func (g GameResult) Valid(components map[string]int) bool {
	for _, d := range g.draws {
		for c, v := range d.colors {
			if components[c] < v {
				return false
			}
		}
	}
	return true
}

func (g GameResult) RequiredComponentsPower() int {
	requiredComponents := map[string]int{}
	for _, d := range g.draws {
		for c, v := range d.colors {
			vv := requiredComponents[c]
			if v > vv {
				requiredComponents[c] = v
			}
		}
	}

	power := 1
	for _, rc := range requiredComponents {
		power *= rc
	}
	return power
}

func gamesEqual(g1 GameResult, g2 GameResult) bool {
	if g1.ID != g2.ID {
		return false
	}

	if len(g1.draws) != len(g2.draws) {
		println("no")
		return false
	}
	for i, d := range g1.draws {
		if !drawEqual(d, g2.draws[i]) {
			return false
		}
	}
	return true
}
