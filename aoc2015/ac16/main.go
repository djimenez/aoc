package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var (
	aunts = make(map[uint64]*Aunt)
)

type Aunt struct {
	number      int64
	children    int64
	cats        int64
	samoyeds    int64
	pomeranians int64
	akitas      int64
	vizslas     int64
	goldfish    int64
	trees       int64
	cars        int64
	perfumes    int64
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

		aunt := parseAunt(line)

		if matches(aunt) {
			fmt.Println(aunt.number, "matches")
			break
		}
	}
}

func parseAunt(line string) *Aunt {
	aunt := &Aunt{children: -1, cats: -1, samoyeds: -1, pomeranians: -1, akitas: -1, vizslas: -1, goldfish: -1, trees: -1, cars: -1, perfumes: -1}

	splits := strings.Split(line, " ")

	fmt.Sscanf(splits[1], "%d:", &aunt.number)

	for i := 2; i < len(splits); i += 2 {
		name := splits[i][:len(splits[i])-1]

		switch name {
		case "children":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.children)
		case "cats":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.cats)
		case "samoyeds":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.samoyeds)
		case "pomeranians":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.pomeranians)
		case "akitas":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.akitas)
		case "vizslas":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.vizslas)
		case "goldfish":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.goldfish)
		case "trees":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.trees)
		case "cars":
			fmt.Sscanf(splits[i+1], "%d,", &aunt.cars)
		case "perfumes":
			fmt.Sscanf(splits[i+1], "%d", &aunt.perfumes)
		default:
			panic(name)
		}
	}

	return aunt
}

func matches(aunt *Aunt) bool {
	if aunt.children >= 0 && aunt.children != 3 {
		return false
	}

	if aunt.cats >= 0 && aunt.cats <= 7 {
		return false
	}

	if aunt.samoyeds >= 0 && aunt.samoyeds != 2 {
		return false
	}

	if aunt.pomeranians >= 0 && aunt.pomeranians >= 3 {
		return false
	}

	if aunt.akitas >= 0 && aunt.akitas != 0 {
		return false
	}

	if aunt.vizslas >= 0 && aunt.vizslas != 0 {
		return false
	}

	if aunt.goldfish >= 0 && aunt.goldfish >= 5 {
		return false
	}

	if aunt.trees >= 0 && aunt.trees <= 3 {
		return false
	}

	if aunt.cars >= 0 && aunt.cars != 2 {
		return false
	}

	if aunt.perfumes >= 0 && aunt.perfumes != 1 {
		return false
	}

	return true
}
