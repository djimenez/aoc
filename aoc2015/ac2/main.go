package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var total uint64
	var ribbon uint64

	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		var w, h, l uint64

		_, err := fmt.Sscanf(line, "%dx%dx%d", &w, &h, &l)

		if err != nil {
			panic(err)
		}

		fmt.Printf("parsed %d x %d x %d\n", w, h, l)

		wh := w * h
		wl := w * l
		hl := h * l

		area := 2*wh + 2*wl + 2*hl
		minSide := min(wh, wl, hl)

		total += area + minSide

		volume := w * h * l
		minPerimeter := min(2*(w+h), 2*(w+l), 2*(h+l))

		ribbon += volume + minPerimeter
	}

	fmt.Printf("total paper: %d sq. ft\n", total)
	fmt.Printf("total ribbon: %d ft.\n", ribbon)
}

func min(wh, wl, hl uint64) uint64 {
	switch {
	case wh < wl && wh < hl:
		return wh
	case wl < hl:
		return wl
	default:
		return hl
	}
}
