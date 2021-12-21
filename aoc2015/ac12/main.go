package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	re := regexp.MustCompile("-?\\d+")

	var sum int64

	for scanner.Scan() {
		line := scanner.Text()
		matches := re.FindAllString(line, -1)

		for _, match := range matches {
			value, _ := strconv.Atoi(match)
			sum += int64(value)
		}

	}

	fmt.Printf("%d total\n", sum)
}
