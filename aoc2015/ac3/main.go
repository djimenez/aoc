package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var x1, y1, x2, y2 int64
	houses := make(map[string]bool)

	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanRunes)

	// there's always a presetn at 0,0
	addPresent(houses, x1, y1)

	var count uint64

	for scanner.Scan() {
		count++

		switch {
		case count%2 > 0:
			x1, y1 = apply(scanner.Text(), x1, y1)
			addPresent(houses, x1, y1)
		default:
			x2, y2 = apply(scanner.Text(), x2, y2)
			addPresent(houses, x2, y2)
		}
	}

	// count the nubmer of keys
	fmt.Printf("%d houses got presents\n", len(houses))
}

func apply(direction string, x, y int64) (int64, int64) {
	switch direction {
	case "<":
		x--
	case ">":
		x++
	case "v":
		y--
	case "^":
		y++
	}

	return x, y
}

func addPresent(houses map[string]bool, x, y int64) {
	key := fmt.Sprintf("%d,%d", x, y)
	houses[key] = true
}
