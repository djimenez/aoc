package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var sum int = 0

	for scanner.Scan() {
		line := scanner.Text()
		vscan := bufio.NewScanner(strings.NewReader(line))
		vscan.Split(bufio.ScanWords)
		var values []int = make([]int, 0, 100)

		for vscan.Scan() {
			value := vscan.Text()
			ivalue, err := strconv.ParseInt(value, 10, 64)
			if err != nil {
				panic(err)
			}

			values = append(values, int(ivalue))
		}

		// sort them
		sort.Ints(values)

	loop:
		for i := 0; i < len(values); i++ {
			for j := i + 1; j < len(values); j++ {
				if values[j]%values[i] == 0 {
					sum += values[j] / values[i]
					break loop
				}
			}
		}
	}
	fmt.Println(sum)
}
