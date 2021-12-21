package main

import (
	"fmt"
	"math"
	_ "strings"
)

var minimumManaSpent int = math.MaxInt64

type State struct {
	Previous *State

	Turn      int
	ManaSpent int

	PlayerHP    int
	PlayerMana  int
	PlayerArmor int

	DragonHP     int
	DragonDamage int

	Shield   int
	Poison   int
	Recharge int
}

func main() {
	initial := new(State)
	initial.PlayerHP = 50
	initial.PlayerMana = 500
	initial.DragonHP = 71
	initial.DragonDamage = 10

	simulate(initial)

	fmt.Println(minimumManaSpent)
}

func simulate(current *State) {
	if current.Turn%2 == 0 {
		// hard mode
		current.PlayerHP -= 1

		if current.PlayerHP <= 0 {
			return // simulation ended
		}

		// player turn
		tickEffects(current)

		if current.DragonHP <= 0 {
			// record mana spent
			if current.ManaSpent < minimumManaSpent {
				minimumManaSpent = current.ManaSpent
				//fmt.Println("minimum:", current)
			}

			return // no more simulation required
		}

		if current.ManaSpent > minimumManaSpent {
			return // more simulation is wasteful
		}

		// tree through spells
		castMagicMissile(current)
		castDrain(current)
		castShield(current)
		castPoison(current)
		castRecharge(current)
	} else {
		// dragon turn
		tickEffects(current)

		if current.DragonHP <= 0 {
			// record mana spent
			if current.ManaSpent < minimumManaSpent {
				minimumManaSpent = current.ManaSpent
				//fmt.Println("minimum:", current)
			}

			return // no more simulation required
		}

		// apply dragon damage
		damage := current.DragonDamage - current.PlayerArmor

		if damage < 1 {
			damage = 1
		}

		current.PlayerHP -= damage

		//fmt.Println(strings.Repeat(" ", current.Turn), current.Turn, ": dragon attacks -", current)

		if current.PlayerHP <= 0 {
			return // no more simulation required
		}

		current.Turn += 1
		simulate(current)
	}
}

func copyState(current *State) *State {
	next := new(State)
	next.Previous = current

	next.Turn = current.Turn
	next.ManaSpent = current.ManaSpent

	next.PlayerHP = current.PlayerHP
	next.PlayerMana = current.PlayerMana
	next.PlayerArmor = current.PlayerArmor

	next.DragonHP = current.DragonHP
	next.DragonDamage = current.DragonDamage

	next.Shield = current.Shield
	next.Poison = current.Poison
	next.Recharge = current.Recharge

	return next
}

func tickEffects(current *State) {
	if current.Poison > 0 {
		current.DragonHP -= 3
		current.Poison -= 1
		//fmt.Println("applied poison")
	}

	if current.Recharge > 0 {
		current.PlayerMana += 101
		current.Recharge -= 1
		//fmt.Println("applied recharge")
	}

	if current.Shield > 0 {
		current.PlayerArmor = 7
		current.Shield -= 1
		//fmt.Println("applied shield")
	} else {
		current.PlayerArmor = 0
	}
}

func castMagicMissile(current *State) *State {
	if current.PlayerMana < 53 {
		//panic("player does not have enough mana for magic missile")
		return nil
	}

	next := copyState(current)
	next.ManaSpent += 53
	next.PlayerMana -= 53
	next.DragonHP -= 4

	//fmt.Println(strings.Repeat(" ", next.Turn), next.Turn, ": magic missile -", next)

	next.Turn += 1
	simulate(next)

	return next
}

func castDrain(current *State) *State {
	if current.PlayerMana < 73 {
		//panic("player does not have enough mana for drain")
		return nil
	}

	next := copyState(current)
	next.ManaSpent += 73
	next.PlayerMana -= 73
	next.PlayerHP += 2
	next.DragonHP -= 2

	//fmt.Println(strings.Repeat(" ", next.Turn), next.Turn, ": drain -", next)

	next.Turn += 1
	simulate(next)

	return next
}

func castShield(current *State) *State {
	// panic if current state already has shield
	if current.Shield > 0 {
		//panic("can't cast shield while shield in effect")
		return nil
	}

	if current.PlayerMana < 113 {
		//panic("player does not have enough mana for shield")
		return nil
	}

	next := copyState(current)
	next.ManaSpent += 113
	next.PlayerMana -= 113
	next.Shield = 6

	//fmt.Println(strings.Repeat(" ", next.Turn), next.Turn, ": shield -", next)

	next.Turn += 1
	simulate(next)

	return next
}

func castPoison(current *State) *State {
	if current.Poison > 0 {
		//panic("can't cast poison while poison in effect")
		return nil
	}

	if current.PlayerMana < 173 {
		//panic("player does not have enough mana for poison")
		return nil
	}

	next := copyState(current)
	next.ManaSpent += 173
	next.PlayerMana -= 173
	next.Poison = 6

	//fmt.Println(strings.Repeat(" ", next.Turn), next.Turn, ": poison -", next)

	next.Turn += 1
	simulate(next)

	return next
}

func castRecharge(current *State) *State {
	if current.Recharge > 0 {
		//panic("can't cast recharge while recharge in effect")
		return nil
	}

	if current.PlayerMana < 229 {
		//panic("player does not have enough mana for recharge")
		return nil
	}

	next := copyState(current)
	next.ManaSpent += 229
	next.PlayerMana -= 229
	next.Recharge = 5

	//fmt.Println(strings.Repeat(" ", next.Turn), next.Turn, ": recharge - ", next)

	next.Turn += 1
	simulate(next)

	return next
}

func (s *State) String() string {
	return fmt.Sprintf("spent: %d player: %d mana: %d dragon: %d", s.ManaSpent, s.PlayerHP, s.PlayerMana, s.DragonHP)
}
