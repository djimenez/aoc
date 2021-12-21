package main

import (
	"bufio"
	"fmt"
	"os"
)

var (
	deers = make(map[string]*Deer)
)

type Deer struct {
	speed uint64
	limit uint64
	rest  uint64
}

func main() {
	time := uint64(2503)
	//time := uint64(1000)

	file, err := os.Open("input.txt")
	//file, err := os.Open("test.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		var deer string
		var speed, limit, rest uint64

		fmt.Sscanf(line, "%s can fly %d km/s for %d seconds, but then must rest for %d seconds.", &deer, &speed, &limit, &rest)
		//fmt.Println(deer, speed, limit, rest)

		deers[deer] = &Deer{speed: speed, limit: limit, rest: rest}
	}

	points := make(map[string]uint64)
	var i uint64

	for i = 1; i <= time; i++ {
		bestDeer := findBest(i)

		points[bestDeer] += 1
	}

	best := uint64(0)
	bestDeer := ""
	for deer, point := range points {
		if point > best {
			best = point
			bestDeer = deer
		}
	}

	fmt.Println(bestDeer, best)
}

func findBest(time uint64) string {
	best := uint64(0)
	bestDeer := ""
	for deer, data := range deers {
		distance := simulate(data, time)

		if distance > best {
			best = distance
			bestDeer = deer
		}
	}

	return bestDeer
}

func simulate(data *Deer, time uint64) uint64 {
	total := data.limit + data.rest

	totals := time / total
	partials := time % total

	if partials > data.limit {
		partials = data.limit
	}

	return data.speed*data.limit*totals + data.speed*partials
}
