package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

var (
	containers = make(map[string]bool)
	values     = make(map[string]uint64)
	solutions  = make(map[uint64]uint64)
	uniques    = make(map[string]bool)
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var count uint64

	for scanner.Scan() {
		line := scanner.Text()

		count++
		var value uint64
		fmt.Sscanf(line, "%d", &value)
		key := fmt.Sprintf("%d", count)

		containers[key] = false
		values[key] = value
	}

	permutate(150)

	var min uint64 = uint64(len(containers))
	var minCount uint64 = 0

	for i, c := range solutions {
		if i < min {
			min = i
			minCount = c
		}
	}

	fmt.Println(min, ": ", minCount)
}

func permutate(remaining uint64) {
	if remaining == 0 {
		isNew, length := unique()
		if isNew {
			solutions[length] += 1
		}
	} else {
		for key, taken := range containers {
			if !taken {
				value := values[key]

				if value <= remaining {
					containers[key] = true
					permutate(remaining - value)
					containers[key] = false
				}
			}
		}
	}
}

func unique() (bool, uint64) {
	keys := make([]string, 0, len(containers))

	for key, taken := range containers {
		if taken {
			keys = append(keys, key)
		}
	}

	sort.Sort(sort.StringSlice(keys))
	uniqueKey := strings.Join(keys, "-")
	_, exists := uniques[uniqueKey]
	uniques[uniqueKey] = true

	return !exists, uint64(len(keys))
}
