package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"sync"
)

const (
	primaryGroup = 3
	prefixLength = 1
	compartments = 4
)

var (
	packages = make([]int, 0, 100)

	targetGroupWeight int
	bestCount         int
	bestEntanglement  int
)

type Score struct {
	Count        int
	Entanglement int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	total := 0

	for scanner.Scan() {
		line := scanner.Text()

		weight, err := strconv.Atoi(line)
		if err != nil {
			panic(err)
		}
		packages = append(packages, weight)
		total += weight
	}

	targetGroupWeight = total / compartments
	bestCount = len(packages)
	bestEntanglement = math.MaxInt64

	// sort packages biggest to smallest
	sort.Sort(sort.IntSlice(packages))

	fmt.Println(packages)

	waitGroup := new(sync.WaitGroup)
	scores := make(chan *Score)

	for i := 0; i < compartments; i++ {
		waitGroup.Add(1)

		go func(prefix ...int) {
			scores <- process(prefix...)
			waitGroup.Done()
		}(i)
	}

	go func() {
		for score := range scores {
			if score.Count < bestCount {
				bestCount = score.Count
				bestEntanglement = score.Entanglement
			} else if score.Count == bestCount && score.Entanglement < bestEntanglement {
				bestEntanglement = score.Entanglement
			}
		}
	}()

	waitGroup.Wait()
	close(scores)

	fmt.Println(packages)
	fmt.Println(bestCount, bestEntanglement)
}

func process(prefix ...int) *Score {
	groups := make([]int, len(packages))

	for i := 0; i < len(prefix); i++ {
		groups[i] = prefix[i]
	}

	localBestCount := len(packages) / 3
	localBestEntanglement := math.MaxInt64

	for increment(groups) {
		if ok, newCount, newEntanglement := evaluate(groups, localBestCount, localBestEntanglement); ok {
			localBestCount = newCount
			localBestEntanglement = newEntanglement

			fmt.Println(groups, localBestCount, localBestEntanglement)
		}
	}

	return &Score{Count: localBestCount, Entanglement: localBestEntanglement}
}

func increment(groups []int) bool {
	i := len(groups) - 1
	for ; i >= prefixLength && groups[i] >= compartments-1; i-- {
	}

	if i < prefixLength {
		return false
	}

	groups[i] += 1

	for j := i + 1; j < len(groups); j++ {
		groups[j] = 0
	}

	return true
}

func incrementFrom(groups []int, index int) {
	length := len(groups)

	for i := index + 1; i < length; i++ {
		groups[i] = compartments - 1
	}
}

func evaluate(groups []int, bestCount, bestEntanglement int) (bool, int, int) {
	groupCount := [4]int{0, 0, 0, 0}
	groupWeight := [4]int{0, 0, 0, 0}
	groupEntanglement := [4]int{1, 1, 1, 1}

	for index, group := range groups {
		weight := packages[index]

		newCount := groupCount[group] + 1

		if group == primaryGroup && (newCount > bestCount) {
			// skip the rest - they will all be invalid
			incrementFrom(groups, index)

			return false, 0, 0
		}

		newWeight := groupWeight[group] + weight

		if newWeight > targetGroupWeight {
			// skip the rest - they will all be invalid
			incrementFrom(groups, index)

			return false, 0, 0
		}

		groupCount[group] = newCount
		groupWeight[group] = newWeight
		groupEntanglement[group] *= weight
	}

	primaryCount := groupCount[primaryGroup]
	primaryEntanglement := groupEntanglement[primaryGroup]

	if primaryCount < bestCount {
		return true, primaryCount, primaryEntanglement
	} else if primaryCount == bestCount && primaryEntanglement < bestEntanglement {
		return true, primaryCount, primaryEntanglement
	}

	return false, 0, 0
}
