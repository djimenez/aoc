package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var total uint64

	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		fmt.Printf("checking %s...\n", line)

		var hasDoubleDouble bool
		var hasSandwich bool

		var previous, previous2 rune

		for index, char := range line {
			if !hasDoubleDouble && index > 0 {
				// we have a double - check for matching double further on in string
				double := line[index-1 : index+1]
				remains := line[index+1:]
				found := strings.Index(remains, double) >= 0

				fmt.Printf("\tchecking double %s in remaing %s - %v\n", double, remains, found)

				if found {
					hasDoubleDouble = true
				}
			}

			if !hasSandwich && index > 1 && char == previous2 {
				fmt.Printf("\tfound sandwich %s\n", line[index-2:index+1])
				hasSandwich = true
			}

			previous2 = previous
			previous = char
		}

		if hasDoubleDouble && hasSandwich {
			fmt.Printf("\tVALID!\n", line)
			total++
		}
	}

	fmt.Printf("%d nice strings found\n", total)
}
