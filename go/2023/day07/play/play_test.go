package play

import (
	"log"
	"os"
	"testing"
)

func TestParse(t *testing.T) {

	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	plays := ParsePlays(f)

	if len(plays) != 5 {
		t.Fatal("Number of plays is not 5")
	}

	totalBets := 0
	for i, p := range plays {
		if len(p.Hand) != 5 {
			t.Fatalf("Play %d does not have 5 cards", i)
		}
		totalBets += p.Bet
	}

	if totalBets != 2180 {
		t.Fatalf("Total bet is %d and should be 2180", totalBets)
	}

	if plays[0].Hand[0] != 3 {
		t.Fatal("First card is not 3")
	}

	if plays[0].Hand[1] != 2 {
		t.Fatal("Second card is not 2")
	}

	if plays[0].Hand[2] != 10 {
		t.Fatal("Third card is not T")
	}

	if plays[0].Hand[3] != 3 {
		t.Fatal("Fourth card is not 3")
	}

	if plays[0].Hand[4] != 13 {
		t.Fatal("Fifth card is not K")
	}
}

func TestDetectType(t *testing.T) {
	hand := parseHand("32T3K")
	if rankHand(hand) != OnePair {
		t.Fatalf("%v should be OnePair", hand)
	}

	hand = parseHand("T55J5")
	if rankHand(hand) != ThreeKind {
		t.Fatalf("%v should be ThreeKind", hand)
	}

	hand = parseHand("KK677")
	if rankHand(hand) != TwoPair {
		t.Fatalf("%v should be TwoPair", hand)
	}

	hand = parseHand("KTJJT")
	if rankHand(hand) != TwoPair {
		t.Fatalf("%v should be TwoPair", hand)
	}

	hand = parseHand("QQQJA")
	if rankHand(hand) != ThreeKind {
		t.Fatalf("%v should be ThreeKind", hand)
	}
}

func TestSortPlays(t *testing.T) {
	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	plays := ParsePlays(f)
	SortPlays(&plays)

	if plays[4].Bet != 483 {
		t.Fatal("Fifth bet is not 483")
	}

	if plays[3].Bet != 684 {
		t.Fatal("Fourth bet is not 684")
	}

	if plays[2].Bet != 28 {
		t.Fatal("Third bet is not 28")
	}

	if plays[1].Bet != 220 {
		t.Fatal("Second bet is not 220")
	}

	if plays[0].Bet != 765 {
		t.Fatal("First bet is not 765")
	}

	totalBets := 0
	for i, p := range plays {
		totalBets += p.Bet * (i + 1)
	}

	if totalBets != 6440 {
		t.Fatal("Total bet is not 6440")
	}

}

func TestWildRank(t *testing.T) {
	cards := parseHand("T55J5")
	adjusted := AdjustWilds(cards)
	if rankHand(adjusted) != FourKind {
		t.Fatalf("Wrong rank of %d", rankHand(cards))
	}
}

func TestSortPlaysWilds(t *testing.T) {
	f, err := os.Open("test-input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	plays := ParsePlays(f)
	SortPlaysWilds(&plays)

	if plays[4].Bet != 220 {
		t.Fatal("Fifth bet is not 220")
	}

	if plays[3].Bet != 483 {
		t.Fatal("Fourth bet is not 483")
	}

	if plays[2].Bet != 684 {
		t.Fatal("Third bet is not 684")
	}

	if plays[1].Bet != 28 {
		t.Fatal("Second bet is not 28")
	}

	if plays[0].Bet != 765 {
		t.Fatal("First bet is not 765")
	}

	totalBets := 0
	for i, p := range plays {
		totalBets += p.Bet * (i + 1)
	}

	log.Println(totalBets)
	if totalBets != 5905 {
		t.Fatal("Total bet is not 5905")
	}

}
