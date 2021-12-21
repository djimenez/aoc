package main

import (
	"fmt"
)

var (
	weapons = make(map[string]*Item)
	Armors  = make(map[string]*Item)
	rings   = make(map[string]*Item)

	dragon = &Entity{HitPoints: 103, Damage: 9, Armor: 2}
)

type Item struct {
	Cost   uint64
	Damage uint64
	Armor  uint64
}

type Entity struct {
	HitPoints uint64
	Damage    uint64
	Armor     uint64
	Cost      uint64

	Weapon string
	Suit   string
	Ring1  string
	Ring2  string
}

func init() {
	weapons["Dagger"] = &Item{Cost: 8, Damage: 4, Armor: 0}
	weapons["Shortsword"] = &Item{Cost: 10, Damage: 5, Armor: 0}
	weapons["Warhammer"] = &Item{Cost: 25, Damage: 6, Armor: 0}
	weapons["Longsword"] = &Item{Cost: 40, Damage: 7, Armor: 0}
	weapons["Greataxe"] = &Item{Cost: 74, Damage: 8, Armor: 0}

	Armors["None"] = &Item{Cost: 0, Damage: 0, Armor: 0}
	Armors["Leather"] = &Item{Cost: 13, Damage: 0, Armor: 1}
	Armors["Chainmail"] = &Item{Cost: 31, Damage: 0, Armor: 2}
	Armors["Splintmail"] = &Item{Cost: 53, Damage: 0, Armor: 3}
	Armors["Bandedmail"] = &Item{Cost: 75, Damage: 0, Armor: 4}
	Armors["Platemail"] = &Item{Cost: 102, Damage: 0, Armor: 5}

	rings["None 1"] = &Item{Cost: 0, Damage: 0, Armor: 0}
	rings["None 2"] = &Item{Cost: 0, Damage: 0, Armor: 0}
	rings["Damage +1"] = &Item{Cost: 25, Damage: 1, Armor: 0}
	rings["Damage +2"] = &Item{Cost: 50, Damage: 2, Armor: 0}
	rings["Damage +3"] = &Item{Cost: 100, Damage: 3, Armor: 0}
	rings["Defense +1"] = &Item{Cost: 20, Damage: 0, Armor: 1}
	rings["Defense +2"] = &Item{Cost: 40, Damage: 0, Armor: 2}
	rings["Defense +3"] = &Item{Cost: 80, Damage: 0, Armor: 3}
}

func main() {
	maximumCost := uint64(0)
	players := generatePlayers()

	for player := range players {
		// make sure dragon hitpoints are reset
		dragon.HitPoints = 103

		if player.Cost > maximumCost && !simulateWith(player) {
			maximumCost = player.Cost

			fmt.Println(player, maximumCost)
		}
	}

	fmt.Println(maximumCost)
}

func generatePlayers() chan *Entity {
	players := make(chan *Entity, 100)

	go func() {
		for weaponName, weapon := range weapons {
			for armorName, armor := range Armors {
				for ring1name, ring1 := range rings {
					for ring2name, ring2 := range rings {
						if ring1name != ring2name {
							player := &Entity{HitPoints: 100, Weapon: weaponName, Suit: armorName, Ring1: ring1name, Ring2: ring2name}

							player.Damage += weapon.Damage
							player.Damage += armor.Damage
							player.Damage += ring1.Damage
							player.Damage += ring2.Damage

							player.Armor += weapon.Armor
							player.Armor += armor.Armor
							player.Armor += ring1.Armor
							player.Armor += ring2.Armor

							player.Cost += weapon.Cost
							player.Cost += armor.Cost
							player.Cost += ring1.Cost
							player.Cost += ring2.Cost

							players <- player
						}
					}
				}
			}
		}

		close(players)
	}()

	return players
}

func simulateWith(player *Entity) bool {
	var playerDamage, dragonDamage uint64

	if player.Damage > dragon.Armor {
		playerDamage = player.Damage - dragon.Armor
	} else {
		playerDamage = 1
	}

	if dragon.Damage > player.Armor {
		dragonDamage = dragon.Damage - player.Armor
	} else {
		dragonDamage = 1
	}

	for {
		if dragon.HitPoints <= playerDamage {
			fmt.Println("...player kills dragon")
			return true
		} else {
			dragon.HitPoints -= playerDamage
			fmt.Println("...player deals", playerDamage, "damage")
		}

		if player.HitPoints <= dragonDamage {
			fmt.Println("...dragon kills player")
			return false
		} else {
			fmt.Println("...dragon deals", dragonDamage, "damage")
			player.HitPoints -= dragonDamage
		}
	}
}
