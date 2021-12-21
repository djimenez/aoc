package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var row, column int

	for scanner.Scan() {
		line := scanner.Text()

		fmt.Sscanf(line, "To continue, please consult the code grid in the manual.  Enter the code at row %d, column %d.", &row, &column)
	}

	fmt.Println(cantor(row, column))
}

func cantor(row, column int) (result uint64) {
	result = 20151125

	for c := 2; c < 1000000; c++ {
		for i, j := c, 1; i > 0; i, j = i-1, j+1 {
			result = result * 252533 % 33554393

			if row == i && column == j {
				return result
			}
		}
	}

	return
}
