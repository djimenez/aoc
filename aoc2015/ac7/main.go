package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
	"unicode/utf8"
)

var (
	wires = make(map[string][]string)
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		statement := scanner.Text()

		parseStatement(statement)
	}

	var passes uint64 = 1
	fmt.Println("processing wires...")
	replaced := processWires()

	for replaced {
		replaced = processWires()
		passes++
	}

	fmt.Printf("%d passes taken\n\n", passes)

	for key, tokens := range wires {
		fmt.Printf("%s = %v\n", key, tokens)
	}

	fmt.Printf("\na = %s\n", wires["a"][0])
}

func parseStatement(statement string) {
	tokens := strings.Split(statement, " ")

	wire := tokens[len(tokens)-1]
	wires[wire] = tokens[:len(tokens)-2]
}

func isWire(token string) bool {
	firstRune, _ := utf8.DecodeRuneInString(token)
	return unicode.IsLetter(firstRune) && unicode.IsLower(firstRune)
}

func isOp(token string) bool {
	firstRune, _ := utf8.DecodeRuneInString(token)
	return unicode.IsLetter(firstRune) && unicode.IsUpper(firstRune)
}

func isValue(token string) bool {
	firstRune, _ := utf8.DecodeRuneInString(token)
	return unicode.IsDigit(firstRune)
}

func getValue(token string) uint16 {
	value, err := strconv.ParseUint(token, 10, 16)
	if err != nil {
		panic(err)
	}
	return uint16(value)
}

func notValue(token string) string {
	value := getValue(token)
	value = ^value

	return fmt.Sprintf("%d", value)
}

func lshiftValue(value string, shift string) string {
	x := getValue(value)
	s := getValue(shift)

	x = x << s

	return fmt.Sprintf("%d", x)
}

func rshiftValue(value string, shift string) string {
	x := getValue(value)
	s := getValue(shift)

	x = x >> s

	return fmt.Sprintf("%d", x)
}

func andValue(left string, right string) string {
	l := getValue(left)
	r := getValue(right)

	x := l & r

	return fmt.Sprintf("%d", x)
}

func orValue(left string, right string) string {
	l := getValue(left)
	r := getValue(right)

	x := l | r

	return fmt.Sprintf("%d", x)
}

func processWires() (replaced bool) {
	for key, tokens := range wires {
		tokenReplaced := false
		for index, token := range tokens {
			if isWire(token) {
				replacement := wires[token]

				if len(replacement) == 1 {
					fmt.Printf("\treplacing %s with %s in %s\n", token, replacement[0], key)
					tokens[index] = replacement[0]
					tokenReplaced = true
				}
			}
		}

		if tokenReplaced {
			wires[key] = tokens
			replaced = true
		} else {
			if tokens[0] == "NOT" && isValue(tokens[1]) {
				value := notValue(tokens[1])
				fmt.Printf("\treplacing %s: NOT %s with %s\n", key, tokens[1], value)

				tokens[0] = value
				tokens = tokens[:1]
				wires[key] = tokens

				replaced = true
			} else if len(tokens) == 3 && isValue(tokens[0]) && isValue(tokens[2]) {
				var value string

				switch tokens[1] {
				case "LSHIFT":
					value = lshiftValue(tokens[0], tokens[2])
				case "RSHIFT":
					value = rshiftValue(tokens[0], tokens[2])
				case "AND":
					value = andValue(tokens[0], tokens[2])
				case "OR":
					value = orValue(tokens[0], tokens[2])
				}

				fmt.Printf("\treplacing %s: %s %s %s with %s\n", key, tokens[0], tokens[1], tokens[2], value)

				tokens[0] = value
				tokens = tokens[:1]
				wires[key] = tokens
				replaced = true
			}
		}
	}

	return
}

func replaceReferences(tokens []string) ([]string, bool) {
	result := make([]string, 0, len(tokens))
	replace := false

	for _, token := range tokens {
		if isWire(token) {
			replacements := wires[token]
			fmt.Printf("\treplacing %s -> %s\n", token, replacements)

			result = append(result, "(")

			for _, replacement := range replacements {
				result = append(result, replacement)
			}

			result = append(result, ")")

			replace = true
		} else {
			result = append(result, token)
		}
	}

	return result, replace
}
