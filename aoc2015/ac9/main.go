package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var (
	cities    = make(map[string]bool)
	distances = make(map[string]uint64)

	solutions = make([]uint64, 0, 100)
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

		var source, destination string
		var distance uint64

		fmt.Sscanf(line, "%s to %s = %d", &source, &destination, &distance)

		key := getKey(source, destination)
		distances[key] = distance
		cities[source] = false
		cities[destination] = false
	}

	for city := range cities {
		cities[city] = true
		permutate(uint64(len(cities)-1), city, 0)
		cities[city] = false
	}

	worst := solutions[0]
	best := solutions[0]

	for _, solution := range solutions {
		switch {
		case solution < best:
			best = solution
		case solution > worst:
			worst = solution
		}
	}

	fmt.Printf("best: %d\nworst: %d\n", best, worst)
}

func getKey(source, destination string) string {
	var key string

	switch strings.Compare(source, destination) {
	case -1:
		key = source + ":" + destination
	case 1:
		key = destination + ":" + source
	case 0:
		panic("can't go to the same place")
	}

	return key
}

func permutate(citiesToVisit uint64, lastCity string, totalDistance uint64) {
	if citiesToVisit > 0 {
		for key, traveled := range cities {
			if !traveled {
				cities[key] = true
				distance := distances[getKey(lastCity, key)]
				permutate(citiesToVisit-1, key, totalDistance+distance)
				cities[key] = false
			}
		}
	} else {
		solutions = append(solutions, totalDistance)
	}
}
