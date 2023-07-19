package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Cookie []IngredientAmount
type IngredientAmount struct {
	name   string
	amount int
}

type Ingredient struct {
	capacity   Property
	durability Property
	flavor     Property
	texture    Property
	calories   Property
}

var INGREDIENTS map[string]Ingredient

func init() {
	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	INGREDIENTS = map[string]Ingredient{}

	for fileScanner.Scan() {
		line := fileScanner.Text()
		parts := strings.Split(line, ":")
		name := parts[0]
		properties := getProperties(parts[1])
		INGREDIENTS[name] = Ingredient{
			capacity:   properties["capacity"],
			durability: properties["durability"],
			flavor:     properties["flavor"],
			texture:    properties["texture"],
			calories:   properties["calories"],
		}
	}
}

type Property struct {
	name  string
	value int
}

func getProperties(s string) map[string]Property {
	properties := strings.Split(s, ",")

	props := map[string]Property{}
	for _, property := range properties {
		propFields := strings.Fields(property)
		propValue, _ := strconv.Atoi(propFields[1])
		props[propFields[0]] = Property{
			name:  propFields[0],
			value: propValue,
		}
	}
	return props
}

func main() {
	fmt.Println(INGREDIENTS)
	bestCookie := getBestCookie(0, Cookie{})
	fmt.Println(bestCookie, getCookieScore(bestCookie))
}

var MAX_TOTAL_INGREDIENT_AMOUNT int = 100

func getCookieScore(amounts Cookie) int {

	capacityScore := 0
	for _, amount := range amounts {
		capacityScore += amount.amount * INGREDIENTS[amount.name].capacity.value
	}
	if capacityScore < 0 {
		capacityScore = 0
	}

	durabilityScore := 0
	for _, amount := range amounts {
		durabilityScore += amount.amount * INGREDIENTS[amount.name].durability.value
	}
	if durabilityScore < 0 {
		durabilityScore = 0
	}

	flavorScore := 0
	for _, amount := range amounts {
		flavorScore += amount.amount * INGREDIENTS[amount.name].flavor.value
	}
	if flavorScore < 0 {
		flavorScore = 0
	}

	textureScore := 0
	for _, amount := range amounts {
		textureScore += amount.amount * INGREDIENTS[amount.name].texture.value
	}
	if textureScore < 0 {
		textureScore = 0
	}

	return capacityScore * durabilityScore * flavorScore * textureScore
}

func getCookieCalories(cookie Cookie) int {
	calories := 0
	for _, amount := range cookie {
		calories += amount.amount * INGREDIENTS[amount.name].calories.value
	}
	return calories
}

func getBestCookie(amountsUsed int, used []IngredientAmount) []IngredientAmount {

	if amountsUsed == MAX_TOTAL_INGREDIENT_AMOUNT {
		return used
	}

	bestScore := 0
	bestCookie := []IngredientAmount{}
	amountAvailable := MAX_TOTAL_INGREDIENT_AMOUNT - amountsUsed

	for name := range INGREDIENTS {
		if alreadyUsed(used, name) {
			continue
		}

		if len(INGREDIENTS)-1 == len(used) {
			newUsed := make([]IngredientAmount, len(used))
			copy(newUsed, used)
			newUsed = append(newUsed, IngredientAmount{
				name:   name,
				amount: amountAvailable,
			})
			return newUsed
		}

		for i := 1; i < amountAvailable; i++ {

			newUsed := make([]IngredientAmount, len(used))
			copy(newUsed, used)
			newUsed = append(newUsed, IngredientAmount{
				name:   name,
				amount: i,
			})
			newAmountsUsed := amountsUsed + i

			bestCookieLocal := getBestCookie(newAmountsUsed, newUsed)
			if getCookieCalories(bestCookieLocal) != 500 {
				continue
			}

			cookieScore := getCookieScore(bestCookieLocal)

			if cookieScore > bestScore {
				bestCookie = bestCookieLocal
				bestScore = cookieScore
			}
		}
	}

	return bestCookie
}

func alreadyUsed(used []IngredientAmount, name string) bool {
	for _, n := range used {
		if n.name == name {
			return true
		}
	}
	return false
}
