package game

import "testing"

func TestGameParse(t *testing.T) {
	game := ParseGame("Game 13: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
	draws := []Draw{
		{colors: map[string]int{"blue": 3, "red": 4}},
		{colors: map[string]int{"red": 1, "green": 2, "blue": 6}},
		{colors: map[string]int{"green": 2}},
	}
	expectedGame := GameResult{ID: 13, draws: draws}

	if !gamesEqual(game, expectedGame) {
		t.Fatal("Parsed game incorrectly")
	}
}

func TestGameValidation(t *testing.T) {
	gameStrings := []string{
		"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
		"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
		"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
		"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
		"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
	}

	game := ParseGame(gameStrings[0])
	components := map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}
	if !game.Valid(components) {
		t.Fatal(gameStrings[0], "should be valid")
	}

	game = ParseGame(gameStrings[1])
	if !game.Valid(components) {
		t.Fatal(gameStrings[1], "should be valid")
	}

	game = ParseGame(gameStrings[2])
	if game.Valid(components) {
		t.Fatal(gameStrings[2], "should be invalid")
	}

	game = ParseGame(gameStrings[3])
	if game.Valid(components) {
		t.Fatal(gameStrings[3], "should be invalid")
	}

	game = ParseGame(gameStrings[4])
	if !game.Valid(components) {
		t.Fatal(gameStrings[4], "should be valid")
	}
}

func TestCalcFewestRequiredComponentsPower(t *testing.T) {
	gameString := []string{
		"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
		"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
		"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
		"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
		"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
	}

	game := ParseGame(gameString[0])
	if game.RequiredComponentsPower() != 48 {
		t.Fatal("RequiredComponentsPower for", gameString[0], "should be", 48)
	}

	game = ParseGame(gameString[1])
	if game.RequiredComponentsPower() != 12 {
		t.Fatal("RequiredComponentsPower for", gameString[1], "should be", 12)
	}

	game = ParseGame(gameString[2])
	if game.RequiredComponentsPower() != 1560 {
		t.Fatal("RequiredComponentsPower for", gameString[2], "should be", 1560)
	}

	game = ParseGame(gameString[3])
	if game.RequiredComponentsPower() != 630 {
		t.Fatal("RequiredComponentsPower for", gameString[3], "should be", 630)
	}

	game = ParseGame(gameString[4])
	if game.RequiredComponentsPower() != 36 {
		t.Fatal("RequiredComponentsPower for", gameString[4], "should be", 36)
	}

}
