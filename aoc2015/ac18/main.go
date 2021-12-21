package main

import (
	"bufio"
	"fmt"
	"os"
)

var (
	lights = [102][102]bool{}
	buffer = [102][102]bool{}
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	x := 1

	for scanner.Scan() {
		line := scanner.Text()

		for y, char := range line {
			if char == '#' {
				lights[x][y+1] = true
			}
		}

		x++
	}

	for i := 0; i < 100; i++ {
		computeAndSwap()
	}

	count := 0

	for x := 1; x <= 100; x++ {
		for y := 1; y <= 100; y++ {
			if lights[x][y] {
				count += 1
			}
		}
	}

	fmt.Println(count)
}

func computeAndSwap() {
	// turn the corner lights
	lights[1][1] = true
	lights[1][100] = true
	lights[100][1] = true
	lights[100][100] = true

	for x := 1; x <= 100; x++ {
		for y := 1; y <= 100; y++ {
			sum := computeSum(x, y)
			buffer[x][y] = false

			if lights[x][y] {
				if sum == 2 || sum == 3 {
					buffer[x][y] = true
				}
			} else if sum == 3 {
				buffer[x][y] = true
			}
		}
	}

	buffer[1][1] = true
	buffer[1][100] = true
	buffer[100][1] = true
	buffer[100][100] = true

	lights, buffer = buffer, lights
}

func computeSum(x, y int) (sum int) {
	if lights[x-1][y-1] {
		sum += 1
	}
	if lights[x-1][y] {
		sum += 1
	}
	if lights[x-1][y+1] {
		sum += 1
	}
	if lights[x][y-1] {
		sum += 1
	}
	if lights[x][y+1] {
		sum += 1
	}
	if lights[x+1][y-1] {
		sum += 1
	}
	if lights[x+1][y] {
		sum += 1
	}
	if lights[x+1][y+1] {
		sum += 1
	}

	return
}
