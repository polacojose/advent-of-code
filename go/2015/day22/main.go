package main

import "fmt"

type Spell string

const (
	MagicMissile Spell = "MagicMissile"
	Drain        Spell = "Drain"
	Shield       Spell = "Shield"
	Poison       Spell = "Poison"
	Recharge     Spell = "Recharge"
)

type Effect struct {
	spell          Spell
	turnsRemaining int
}

type SimResult string

const (
	Victory     SimResult = "Victory"
	Defeat      SimResult = "Defeat"
	OutOfMana   SimResult = "OutOfMana"
	OutOfSpells SimResult = "OutOfSpells"
)

const PRINT_BATTLE = false

type Difficulty int

const (
	Normal Difficulty = 0
	Hard   Difficulty = 1
)

var totalSpells []Spell

func init() {
	totalSpells = append(totalSpells, MagicMissile, Drain, Shield, Poison, Recharge)
}

func main() {

	fmt.Println("Normal")
	findMinMana(Normal)

	fmt.Println("Hard")
	findMinMana(Hard)
}

func findMinMana(difficulty Difficulty) {
	space := [][]Spell{}
	for _, spell := range totalSpells {
		space = append(space, []Spell{spell})
	}
	bestMana := int(^uint(0) >> 1)

	for len(space) > 0 {

		newSpace := [][]Spell{}
		for _, path := range space {

			result := simulate(path, difficulty)
			cost := totalManaCost(path)
			if cost >= bestMana {
				continue
			}

			if result == Victory {
				if cost < bestMana {
					fmt.Println(path)
					bestMana = cost
				}
			} else if result == OutOfSpells {
				for _, s := range totalSpells {
					newPath := make([]Spell, len(path)+1)
					copy(newPath, append(path, s))

					newCost := totalManaCost(newPath)
					if newCost < bestMana {
						newSpace = append(newSpace, newPath)
					}
				}
			}
		}

		space = newSpace
	}

	fmt.Println(bestMana)
}

func simulate(spells []Spell, difficulty Difficulty) SimResult {

	playerHP := 50
	playerMana := 500
	playerArmor := 0

	bossHP := 55
	bossDamage := 8

	effects := []Effect{}

	for _, spell := range spells {

		//Player Turn
		if PRINT_BATTLE {
			fmt.Println("-- Player Turn --")
			fmt.Printf("- Player has %d hit points, %d armor, %d mana\n", playerHP, playerArmor, playerMana)
			fmt.Printf("- Boss has %d hit points\n", bossHP)
		}

		if difficulty == Hard {
			playerHP--
			if playerHP <= 0 {
				return Defeat
			}
		}

		//Process effects
		effects = processEffects(effects, &playerArmor, &playerMana, &bossHP)
		if !spellInEffect(effects, spell) {
			if PRINT_BATTLE {
				fmt.Printf("Player casts %s.\n", spell)
			}
			if spell == MagicMissile {
				playerMana -= 53
				bossHP -= 4
			} else if spell == Drain {
				playerMana -= 73
				bossHP -= 2
				playerHP += 2
			} else if spell == Shield {
				playerMana -= 113
				effects = append(effects, Effect{
					spell:          Shield,
					turnsRemaining: 6,
				})
				playerArmor += 7
			} else if spell == Poison {
				playerMana -= 173
				effects = append(effects, Effect{
					spell:          Poison,
					turnsRemaining: 6,
				})
			} else if spell == Recharge {
				playerMana -= 229
				effects = append(effects, Effect{
					spell:          Recharge,
					turnsRemaining: 5,
				})
			}
		}

		if playerMana < 0 {
			return OutOfMana
		}

		//Boss Turn
		if PRINT_BATTLE {
			fmt.Println("-- Boss Turn --")
			fmt.Printf("- Player has %d hit points, %d armor, %d mana\n", playerHP, playerArmor, playerMana)
			fmt.Printf("- Boss has %d hit points\n", bossHP)
		}
		//Process Effects
		effects = processEffects(effects, &playerArmor, &playerMana, &bossHP)

		//Check Boss Health
		if bossHP <= 0 {
			return Victory
		}

		bossAttack := bossDamage - playerArmor
		if bossAttack < 1 {
			bossAttack = 1
		}

		playerHP -= bossAttack

		if playerHP <= 0 {
			return Defeat
		}
	}

	return OutOfSpells
}

func processEffects(effects []Effect, playerArmor *int, playerMana *int, bossHP *int) []Effect {
	for i := 0; i < len(effects); i++ {
		e := effects[i]
		if e.spell == Shield {
			effects[i].turnsRemaining -= 1

			if e.turnsRemaining <= 0 {
				*playerArmor -= 7
			}
		} else if e.spell == Recharge {
			*playerMana += 101
			effects[i].turnsRemaining -= 1
		} else if e.spell == Poison {
			*bossHP -= 3
			effects[i].turnsRemaining -= 1
			if PRINT_BATTLE {
				fmt.Printf("The %s effect timer is now %d.\n", e.spell, effects[i].turnsRemaining)
			}
		}

		if effects[i].turnsRemaining <= 0 {
			if PRINT_BATTLE {
				fmt.Printf("The %s effect faded.\n", e.spell)
			}
			effects = append(effects[:i], effects[i+1:]...)
			i--
		}
	}

	return effects
}

func spellInEffect(effects []Effect, spell Spell) bool {

	for _, e := range effects {
		if e.spell == spell {
			return true
		}
	}

	return false
}

func totalManaCost(spells []Spell) int {
	totalCost := 0

	for _, spell := range spells {
		if spell == MagicMissile {
			totalCost += 53
		} else if spell == Drain {
			totalCost += 73
		} else if spell == Shield {
			totalCost += 113
		} else if spell == Poison {
			totalCost += 173
		} else if spell == Recharge {
			totalCost += 229
		}
	}

	return totalCost
}
