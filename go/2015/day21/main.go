package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type ItemType string

const (
	Weapon ItemType = "Weapon"
	Armor  ItemType = "Armor"
	Ring   ItemType = "Ring"
)

type Item struct {
	id       int
	name     string
	itemType ItemType
	cost     int
	damage   int
	armor    int
}

type PlayerInventory struct {
	weapon Item
	armor  Item
	ringA  Item
	ringB  Item
}

func main() {

	items := getItems()

	weapons := getItemsByType(items, Weapon)
	armor := getItemsByType(items, Armor)
	rings := getItemsByType(items, Ring)

	cheapestPrice := ^uint(0)
	cheapestSet := PlayerInventory{}

	expensivePrice := uint(0)
	expensiveSet := PlayerInventory{}

	for _, weapon := range weapons {
		if weapon.cost == 0 {
			continue
		}
		for _, armor := range armor {
			for _, ringA := range rings {
				for rBi, ringB := range rings {

					if rBi > 0 && ringB.id == ringA.id {
						continue
					}

					playerInventory := PlayerInventory{
						weapon: weapon,
						armor:  armor,
						ringA:  ringA,
						ringB:  ringB,
					}

					playerWin := battle(playerInventory)
					cost := getInventoryCost(playerInventory)
					if playerWin {
						if cost < cheapestPrice {
							cheapestPrice = cost
							cheapestSet = playerInventory
						}
					} else {
						if cost > uint(expensivePrice) {
							expensivePrice = cost
							expensiveSet = playerInventory
						}

					}

				}
			}
		}
	}

	fmt.Println("Cheapest Win", cheapestPrice, cheapestSet)
	fmt.Println("Most Expensive Loss", expensivePrice, expensiveSet)
}

func getItems() []Item {
	file, _ := os.Open("shop.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	id := 0

	items := []Item{}
	items = append(items, Item{
		id:     id,
		name:   "nil",
		cost:   0,
		damage: 0,
		armor:  0,
	})
	id++

	var readState ItemType = ""
	for fileScanner.Scan() {
		line := strings.TrimSpace(fileScanner.Text())

		if len(line) == 0 {
			continue
		}

		if strings.Contains(line, "Weapons:") {
			readState = Weapon
			continue
		} else if strings.Contains(line, "Armor:") {
			readState = Armor
			continue
		} else if strings.Contains(line, "Rings:") {
			readState = Ring
			continue
		}

		if readState == Weapon {
			lineParts := strings.Fields(line)
			cost, _ := strconv.Atoi(lineParts[1])
			damage, _ := strconv.Atoi(lineParts[2])
			armor, _ := strconv.Atoi(lineParts[3])

			items = append(items, Item{
				id:       id,
				name:     lineParts[0],
				itemType: Weapon,
				cost:     cost,
				damage:   damage,
				armor:    armor,
			})
			id++
		} else if readState == Armor {
			lineParts := strings.Fields(line)
			cost, _ := strconv.Atoi(lineParts[1])
			damage, _ := strconv.Atoi(lineParts[2])
			armor, _ := strconv.Atoi(lineParts[3])

			items = append(items, Item{
				id:       id,
				name:     lineParts[0],
				itemType: Armor,
				cost:     cost,
				damage:   damage,
				armor:    armor,
			})
			id++
		} else {
			lineParts := strings.Fields(line)
			cost, _ := strconv.Atoi(lineParts[2])
			damage, _ := strconv.Atoi(lineParts[3])
			armor, _ := strconv.Atoi(lineParts[4])

			items = append(items, Item{
				id:       id,
				name:     lineParts[0] + " " + lineParts[1],
				itemType: Ring,
				cost:     cost,
				damage:   damage,
				armor:    armor,
			})
			id++
		}
	}
	return items
}

func getItemsByType(items []Item, itemType ItemType) []Item {
	foundItems := []Item{}
	foundItems = append(foundItems, items[0])

	for _, item := range items {
		if item.itemType == itemType {
			foundItems = append(foundItems, item)
		}
	}

	return foundItems
}

func battle(playerInventory PlayerInventory) bool {

	playerHP := 100
	playerDamage := playerInventory.weapon.damage
	playerDamage += playerInventory.ringA.damage
	playerDamage += playerInventory.ringB.damage
	playerArmor := playerInventory.armor.armor
	playerArmor += playerInventory.ringA.armor
	playerArmor += playerInventory.ringB.armor

	bossHP := 100
	bossDamage := 8
	bossArmor := 2

	for {

		playerAttack := playerDamage - bossArmor
		if playerAttack <= 0 {
			playerAttack = 1
		}

		//fmt.Printf("The player deals %d-%d = %d damage; the boss goes down to %d hit points.\n", playerDamage, bossArmor, playerAttack, bossHP-playerAttack)
		bossHP -= playerAttack
		if bossHP <= 0 {
			return true
		}

		bossAttack := bossDamage - playerArmor
		if bossAttack <= 0 {
			bossAttack = 1
		}

		//fmt.Printf("The boss deals %d-%d = %d damage; the player goes down to %d hit points.\n", bossDamage, playerArmor, bossAttack, playerHP-bossAttack)
		playerHP -= bossAttack
		if playerHP <= 0 {
			return false
		}
	}
}

func getInventoryCost(inventory PlayerInventory) uint {
	var cost uint = 0

	cost += uint(inventory.weapon.cost)
	cost += uint(inventory.armor.cost)
	cost += uint(inventory.ringA.cost)
	cost += uint(inventory.ringB.cost)

	return cost
}
