package main

import (
	"bytes"
	"crypto/md5"
	"fmt"
)

func main() {
	secret := "ckczppom"
	buffer := bytes.NewBufferString(secret)
	length := buffer.Len()
	var n uint64

	for ; ; n++ {
		// write our decimal intot he buffer
		fmt.Fprintf(buffer, "%d", n)

		// hash it
		hash := md5.Sum(buffer.Bytes())

		// test it for 2 1/2 bytes of zeroes
		if hash[0] == 0x00 && hash[1] == 0x00 && hash[2] == 0x00 {
			fmt.Printf("found hash %h at %d", hash, n)
			break
		}

		// reset to property length
		buffer.Truncate(length)
	}
}
