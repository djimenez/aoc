package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var codeTotal uint64 = 0
	var stringTotal uint64 = 0

	for scanner.Scan() {
		line := []byte(scanner.Text())
		//converted := parseLine(line)
		//converted := fmt.Sprintf("%q", string(line))
		var converted string
		fmt.Sscanf(string(line), "%q", &converted)

		fmt.Printf("%v -> %v\n", string(line), string(converted))

		codeTotal += uint64(len(line))
		stringTotal += uint64(len(converted))
	}

	fmt.Printf("\n%d\n", codeTotal-stringTotal)
}

func parseLine(line []byte) []byte {
	result := make([]byte, 0, len(line))
	result = append(result, '"')

	for i := 0; i < len(line); i++ {
		char := line[i]

		switch char {
		case '\\':
			result = append(result, '\\')
		case '"':
			result = append(result, '\\')
		}

		result = append(result, char)
	}

	result = append(result, '"')

	return result

}
