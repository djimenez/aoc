package main

import (
	"bufio"
	"fmt"
	"os"
)

var (
	guests    = make(map[string]bool)
	values    = make(map[string]int64)
	solutions = make([]int64, 0, 100)
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		var guest1, guest2, gainLose string
		var value int64

		fmt.Sscanf(line, "%s would %s %d happiness units by sitting next to %s.", &guest1, &gainLose, &value, &guest2)
		// the %s will read the period in second name...
		guest2 = guest2[:len(guest2)-1]

		if gainLose == "lose" {
			value *= -1
		}

		//fmt.Printf("%s + %s = %d\n", guest1, guest2, value)

		guests[guest1] = false
		guests[guest2] = false

		values[getKey(guest1, guest2)] += value
	}

	for guest := range guests {
		values[getKey("me", guest)] = 0
	}

	guests["me"] = false

	for guest := range guests {
		guests[guest] = true
		permutate(guests, 0, guest, guest)
		guests[guest] = false
	}

	best := solutions[0]

	for _, solution := range solutions {
		if solution > best {
			best = solution
		}
	}

	fmt.Println(best)
	fmt.Println("i don't get the autocomplete yet")
}

func getKey(guest1, guest2 string) string {
	if guest1 < guest2 {
		return guest1 + ":" + guest2
	} else {
		return guest2 + ":" + guest1
	}
}

func permutate(guests map[string]bool, happiness int64, firstGuest, lastGuest string) {
	available := false

	for guest, taken := range guests {
		if !taken {
			available = true
			value := values[getKey(guest, lastGuest)]

			guests[guest] = true
			permutate(guests, happiness+value, firstGuest, guest)
			guests[guest] = false
		}
	}

	if !available {
		value := values[getKey(firstGuest, lastGuest)]

		solutions = append(solutions, happiness+value)
	}
}
