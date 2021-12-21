package main

import (
	"bufio"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"os"
	"strconv"
	"strings"
)

type LightCommand func(l *image.Alpha16, x1, y1, x2, y2 uint32)

func main() {
	lights := image.NewAlpha16(image.Rect(0, 0, 1000, 1000))

	// read in the input file
	file, err := os.Open("input.txt")

	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	if err != nil {
		panic(err)
	}

	scanner.Split(bufio.ScanWords)
	var outputCounter uint32 = 0

	for command := readCommand(scanner); command != nil; command = readCommand(scanner) {
		x1, y1, x2, y2 := readRanges(scanner)

		command(lights, x1, y1, x2, y2)

		output, err := os.Create(fmt.Sprintf("output%d.png", outputCounter))
		outputCounter++

		if err == nil {
			png.Encode(output, lights)
			output.Close()
		}
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	// now read out the number of lights still lit
	var brightness uint64

	for x := 0; x < 1000; x++ {
		for y := 0; y < 1000; y++ {
			brightness += uint64(lights.Alpha16At(int(x), int(y)).A)
		}
	}

	fmt.Printf("There is %d brightnesss\n", brightness)
}

func readCommand(scanner *bufio.Scanner) LightCommand {
	if scanner.Scan() {
		token := scanner.Text()

		if token == "turn" {
			if scanner.Scan() {
				token = scanner.Text()

				if token == "on" {
					return turnOn
				} else {
					return turnOff
				}
			}
		} else if token == "toggle" {
			return toggle
		}
	}

	return nil
}

func readRanges(scanner *bufio.Scanner) (x1, y1, x2, y2 uint32) {
	if scanner.Scan() {
		x1, y1 = parseRange(scanner.Text())
	} else {
		panic("did not read x1,y1 token")
	}

	// skip through token
	scanner.Scan()

	if scanner.Scan() {
		x2, y2 = parseRange(scanner.Text())
	} else {
		panic("did not read x2,y2 token")
	}

	return
}

func parseRange(r string) (uint32, uint32) {
	xy := strings.SplitN(r, ",", 2)

	x, _ := strconv.ParseUint(xy[0], 10, 32)
	y, _ := strconv.ParseUint(xy[1], 10, 32)

	return uint32(x), uint32(y)
}

func turnOn(i *image.Alpha16, x1, y1, x2, y2 uint32) {
	fmt.Println("turn on")

	for x := x1; x <= x2; x++ {
		for y := y1; y <= y2; y++ {
			value := i.Alpha16At(int(x), int(y))

			if value != color.Opaque {
				value.A += 1
				i.SetAlpha16(int(x), int(y), value)
			}
		}
	}
}

func turnOff(i *image.Alpha16, x1, y1, x2, y2 uint32) {
	fmt.Println("turn off")

	for x := x1; x <= x2; x++ {
		for y := y1; y <= y2; y++ {
			value := i.Alpha16At(int(x), int(y))

			if value != color.Transparent {
				value.A -= 1
				i.SetAlpha16(int(x), int(y), value)
			}
		}
	}
}

func toggle(i *image.Alpha16, x1, y1, x2, y2 uint32) {
	fmt.Println("toggle")

	turnOn(i, x1, y1, x2, y2)
	turnOn(i, x1, y1, x2, y2)
}
