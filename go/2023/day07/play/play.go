package play

import (
	"bufio"
	"io"
	"log"
	"sort"
	"strconv"
	"strings"
)

type HandRank int

const (
	HighCard HandRank = iota
	OnePair
	TwoPair
	ThreeKind
	FullHouse
	FourKind
	FiveKind
)

type CardType int

type Play struct {
	Hand []CardType
	Bet  int
}

func ParsePlays(r io.Reader) []Play {

	plays := []Play{}

	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		line := scanner.Text()
		play := parsePlay(line)
		plays = append(plays, play)
	}

	return plays
}

func parsePlay(s string) Play {
	lineParts := strings.Split(strings.TrimSpace(s), " ")
	hand := parseHand(lineParts[0])
	bet, err := strconv.Atoi(lineParts[1])
	if err != nil {
		log.Fatal(err)
	}
	play := Play{
		Hand: hand,
		Bet:  bet,
	}
	return play
}

func parseHand(s string) []CardType {
	cards := make([]CardType, 0, 5)
	for _, r := range s {
		n_val, err := strconv.Atoi(string(r))
		if err != nil {
			switch r {
			case 'T':
				cards = append(cards, 10)
			case 'J':
				cards = append(cards, 11)
			case 'Q':
				cards = append(cards, 12)
			case 'K':
				cards = append(cards, 13)
			case 'A':
				cards = append(cards, 14)
			}
		} else {
			cards = append(cards, CardType(n_val))
		}
	}
	return cards
}

func AdjustWilds(cards []CardType) []CardType {

	typeMap := map[CardType]int{}
	for _, c := range cards {
		typeMap[c]++
	}

	maxType := CardType(0)
	maxCount := 0
	for k, v := range typeMap {
		if k == 11 {
			continue
		}

		if v > maxCount {
			maxCount = v
			maxType = k
		}

		if v == maxCount && k > maxType {
			maxType = k
		}
	}

	newCards := make([]CardType, len(cards))
	for i, c := range cards {
		if c == 11 {
			newCards[i] = maxType
		} else {
			newCards[i] = cards[i]
		}
	}

	return newCards
}

func rankHand(cards []CardType) HandRank {

	typeMap := map[CardType]int{}
	for _, c := range cards {
		typeMap[c]++
	}

	if len(typeMap) == 1 {
		return FiveKind
	}

	if len(typeMap) == 4 {
		return OnePair
	}

	if len(typeMap) == 5 {
		return HighCard
	}

	if len(typeMap) == 2 {
		for _, v := range typeMap {
			if v == 1 || v == 4 {
				return FourKind
			}
			return FullHouse
		}
	}

	if len(typeMap) == 3 {
		for _, v := range typeMap {
			if v == 3 {
				return ThreeKind
			}
		}
	}

	return TwoPair
}

func SortPlays(plays *[]Play) {
	sort.Slice(*plays, func(i, j int) bool {

		iRank := rankHand((*plays)[i].Hand)
		jRank := rankHand((*plays)[j].Hand)

		if iRank != jRank {
			return iRank < jRank
		}

		for k := 0; k < 5; k++ {
			if (*plays)[i].Hand[k] != (*plays)[j].Hand[k] {
				return (*plays)[i].Hand[k] < (*plays)[j].Hand[k]
			}
		}

		return false
	})
}

func SortPlaysWilds(plays *[]Play) {
	sort.Slice(*plays, func(i, j int) bool {

		iRank := rankHand(AdjustWilds((*plays)[i].Hand))
		jRank := rankHand(AdjustWilds((*plays)[j].Hand))

		if iRank != jRank {
			return iRank < jRank
		}

		for k := 0; k < 5; k++ {
			iCard := (*plays)[i].Hand[k]
			jCard := (*plays)[j].Hand[k]

			if iCard == 11 {
				iCard = 0
			}

			if jCard == 11 {
				jCard = 0
			}

			if iCard != jCard {
				//log.Println((*plays)[i].Hand)
				//log.Println((*plays)[j].Hand)
				//log.Println("Less than:", iCard < jCard)
				//log.Println("-----------")
				return iCard < jCard
			}
		}

		return false
	})
}
