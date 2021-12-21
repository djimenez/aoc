package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var floor int64
	var position uint64

	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanRunes)

	for scanner.Scan() {
		position++
		parenthesis := scanner.Text()

		switch parenthesis {
		case "(":
			floor++
		case ")":
			floor--
		}

		if floor < 0 {
			break
		}
	}

	fmt.Printf("floor %d at position %d\n", floor, position)
}
