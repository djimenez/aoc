package main

import (
	"bufio"
	"fmt"
	"os"
)

var (
	ingredients = make(map[string]*Ingredient)
	best        int64
)

type Ingredient struct {
	capacity   int64
	durability int64
	flavor     int64
	texture    int64
	calories   int64
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		var name string
		var ingredient = new(Ingredient)

		fmt.Sscanf(line, "%s capacity %d, durability %d, flavor %d, texture %d, calories %d", &name, &ingredient.capacity, &ingredient.durability, &ingredient.flavor, &ingredient.texture, &ingredient.calories)
		name = name[:len(name)-1]

		ingredients[name] = ingredient
	}

	portions := make(map[string]int64)
	var lastName string

	for name := range ingredients {
		portions[name] = -1
		lastName = name
	}

	for i := int64(0); i <= 100; i++ {
		portions[lastName] = i
		permutate(portions, 100-i)
	}

	fmt.Println(best)
}

func permutate(portions map[string]int64, remaining int64) {
	if remaining == 0 {
		value := score(portions)
		//fmt.Println(portions, value)

		if value > best {
			best = value
		}
	} else if remaining > 0 {
		for name, amount := range portions {
			if amount < 0 {
				for i := int64(0); i <= remaining; i++ {
					portions[name] = i
					permutate(portions, remaining-i)
					portions[name] = -1
				}
			}
		}
	}
}

func score(portions map[string]int64) int64 {
	var capacity, durability, flavor, texture, calories int64

	for name, amount := range portions {
		if amount > 0 {
			capacity += ingredients[name].capacity * amount
			durability += ingredients[name].durability * amount
			flavor += ingredients[name].flavor * amount
			texture += ingredients[name].texture * amount

			calories += ingredients[name].calories * amount
		}
	}

	if capacity < 0 {
		capacity = 0
	}

	if durability < 0 {
		durability = 0
	}

	if flavor < 0 {
		flavor = 0
	}

	if texture < 0 {
		texture = 0
	}

	if calories != 500 {
		capacity = 0
	}

	//fmt.Printf("%d * %d * %d * %d\n", capacity, durability, flavor, texture)

	return capacity * durability * flavor * texture
}
