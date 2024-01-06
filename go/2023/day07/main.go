package main

import (
	"day07/play"
	"log"
	"os"
)

func main() {
	part1()
	part2()
}

func part1() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	plays := play.ParsePlays(f)
	play.SortPlays(&plays)

	totalBets := 0
	for i, p := range plays {
		totalBets += p.Bet * (i + 1)
	}

	log.Println("Total bets: ", totalBets)
}

func part2() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	plays := play.ParsePlays(f)
	play.SortPlaysWilds(&plays)

	totalBets := 0
	for i, p := range plays {
		totalBets += p.Bet * (i + 1)
	}

	log.Println("Total bets: ", totalBets)
}
