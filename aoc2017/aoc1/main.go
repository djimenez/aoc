package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	//file, err := os.Open("test.txt")
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var sum uint64
	values, err := ioutil.ReadAll(file)
	values = values[:len(values)-1]
	skip := len(values) / 2

	for i := 0; i < len(values); i++ {
		value := values[i]
		pair := values[(i+skip)%len(values)]

		if pair == value {
			sum += uint64(value) - '0'
		}
	}

	fmt.Println(sum)
}
