package main

import (
	"bytes"
	"fmt"
	"os"
	"strconv"
)

func main() {
	line := bytes.NewBuffer(make([]byte, 0, 8*1024*1024))
	buffer := bytes.NewBuffer(make([]byte, 0, 8*1024*1024))

	line.WriteString("1113222113")
	iterations, err := strconv.Atoi(os.Args[1])

	if err != nil {
		panic(err)
	}

	for i := 0; i < iterations; i++ {
		lengthEncode(line, buffer)

		//fmt.Printf("%s\n", buffer.String())

		// now swap
		line, buffer = buffer, line
	}

	fmt.Println(line.Len())
}

func lengthEncode(input, output *bytes.Buffer) {
	output.Reset()

	previous, _ := input.ReadByte()
	count := 1

	for {
		current, eof := input.ReadByte()

		if eof != nil {
			break
		}

		if previous != current {
			fmt.Fprintf(output, "%d%c", count, previous)
			count = 1
		} else {
			count++
		}

		previous = current
	}

	fmt.Fprintf(output, "%d%c", count, previous)
}
