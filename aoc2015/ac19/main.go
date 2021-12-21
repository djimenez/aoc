package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

var (
	replacements = make(map[string][]string)
	breakdowns   = make(map[string][]string)
	start        string
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

		if strings.Contains(line, " => ") {
			var element, replacement string

			fmt.Sscanf(line, "%s => %s", &element, &replacement)

			entry, ok := replacements[element]

			if ok {
				replacements[element] = append(entry, replacement)
			} else {
				replacements[element] = []string{replacement}
			}

			entry, ok = breakdowns[replacement]

			if ok {
				breakdowns[replacement] = append(entry, element)
			} else {
				breakdowns[replacement] = []string{element}
			}
		} else if len(line) > 0 {
			start = line
		}

	}

	uniques := doReplacements(start)

	fmt.Println(len(uniques))

	/*
		uniqueBreakdowns := make(map[string]bool)
		uniqueBreakdowns[start] = true
		var steps uint64

		for {
			_, ok := uniqueBreakdowns["e"]

			if ok {
				fmt.Println(steps, "steps to e")
				break
			} else {
				fmt.Println(steps, "has", len(uniqueBreakdowns))
			}

			steps++
			uniqueBreakdowns = doBreakdowns(uniqueBreakdowns)
		}
	*/
	minSteps := doDepthBreakdowns(start, 0, math.MaxUint64)

	fmt.Println(minSteps)
}

func doReplacements(source string) map[string]bool {
	uniques := make(map[string]bool)

	for key, keyReplacements := range replacements {
		prefix := source[0:0]
		suffix := source

		for index := strings.Index(suffix, key); index >= 0; index = strings.Index(suffix, key) {
			// do each key replacement
			for _, replacment := range keyReplacements {
				replaced := prefix + strings.Replace(suffix, key, replacment, 1)
				uniques[replaced] = true
			}

			offset := len(prefix) + index + 1

			prefix = source[0:offset]
			suffix = source[offset:]
		}
	}

	return uniques
}

func doBreakdowns(molecules map[string]bool) map[string]bool {
	uniques := make(map[string]bool)

	for molecule := range molecules {
		for key, keyReplacements := range breakdowns {
			prefix := molecule[0:0]
			suffix := molecule

			for index := strings.Index(suffix, key); index >= 0; index = strings.Index(suffix, key) {
				// do each key replacement
				for _, replacment := range keyReplacements {
					replaced := prefix + strings.Replace(suffix, key, replacment, 1)
					uniques[replaced] = true
				}

				offset := len(prefix) + index + 1

				prefix = molecule[0:offset]
				suffix = molecule[offset:]
			}
		}
	}

	return uniques
}

func doDepthBreakdowns(molecule string, steps uint64, minimum uint64) uint64 {
	uniques := make(map[string]bool)

	for key, keyReplacements := range breakdowns {
		prefix := molecule[0:0]
		suffix := molecule

		for index := strings.Index(suffix, key); index >= 0; index = strings.Index(suffix, key) {
			// do each key replacement
			for _, replacment := range keyReplacements {
				replaced := prefix + strings.Replace(suffix, key, replacment, 1)

				if replaced == "e" {
					return steps + 1
				}

				if !uniques[replaced] {
					uniques[replaced] = true

					if steps < minimum {
						minSteps := doDepthBreakdowns(replaced, steps+1, minimum)

						if minSteps < minimum {
							fmt.Println("new min:", minSteps, "from", minimum)
							return minSteps
						}
					}
				}
			}

			offset := len(prefix) + index + 1

			prefix = molecule[0:offset]
			suffix = molecule[offset:]
		}
	}

	return minimum
}
