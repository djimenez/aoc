package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
	"unicode/utf8"
)

var (
	nodes = make(map[string]*Node)
)

type Node struct {
	Name    string
	Inputs  []string
	Outputs []string
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		tokens := strings.Split(line, " ")

		edgeName := tokens[len(tokens)-1]
		nodeName := strings.Join(tokens[:len(tokens)-2], " ")

		if node, ok := nodes[nodeName]; ok {
			node.Outputs = append(node.Outputs, edgeName)
		} else {
			node := NewNode(nodeName)
			node.Outputs = append(node.Outputs, edgeName)

			nodes[nodeName] = node
		}
	}

	dot, err := os.Create("output.dot")
	if err != nil {
		panic(err)
	}
	defer dot.Close()

	// write prelude
	dot.WriteString("digraph advent_of_code_7 {\n")
	dot.WriteString("\tnode [shape = circle];\n")

	for key, node := range nodes {
		// write node
		fmt.Fprintln(dot, "\t", node.Name, "[ label = \"", key, "\" ]")

		for _, input := range node.Inputs {

			fmt.Fprintln(dot, "\t", input, " -> ", node.Name)
		}

		for _, output := range node.Outputs {
			fmt.Fprintln(dot, "\t", node.Name, " -> ", output)
		}
	}

	// write postlude
	dot.WriteString("}\n")
}

func NewNode(name string) *Node {
	// parse the name for inputs
	tokens := strings.Split(name, " ")
	var label string

	if len(tokens) == 1 {
		if isWire(tokens[0]) {
			label = tokens[0]
		} else {
			label = "_" + tokens[0]
		}
	} else {
		label = "_" + strings.Join(tokens, "_")
	}

	node := &Node{Name: label, Inputs: make([]string, 0, 10), Outputs: make([]string, 0, 10)}

	if tokens[0] == "NOT" && isWire(tokens[1]) {
		node.Inputs = append(node.Inputs, tokens[1])
	} else if len(tokens) == 3 {
		if isWire(tokens[0]) {
			node.Inputs = append(node.Inputs, tokens[0])
		}

		if isWire(tokens[2]) {
			node.Inputs = append(node.Inputs, tokens[2])
		}
	}

	return node
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
