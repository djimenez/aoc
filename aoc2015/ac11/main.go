package main

import (
	"fmt"
	//"os"
	//"runtime/pprof"
)

type State struct {
	previous1       byte
	previous2       byte
	hasSeq          bool
	hasDoubleDouble bool
	lastDouble      byte
}

func main() {
	//profile, _ := os.Create("profile")
	//pprof.StartCPUProfile(profile)
	//defer pprof.StopCPUProfile()

	password := []byte("hxbxwxba")
	count := 0

loop:
	for inc(password) {
		prefix := password[0:4]
		bottom := password[4:8]

		state := state(prefix)

		for inc(bottom) {
			if check(*state, bottom) {
				count++
				fmt.Println(string(password))

				if count > 100000 {
					break loop
				}
			}
		}
	}

	fmt.Printf("%d valid passwords left\n", count)
}

func inc(password []byte) bool {
	lastIndex := len(password) - 1

	if password[lastIndex] == 'z' {
		password[lastIndex] = 'a'

		if lastIndex > 0 {
			return inc(password[:lastIndex])
		} else {
			return false
		}
	} else {
		password[lastIndex] += 1

		switch password[lastIndex] {
		case 'i':
			password[lastIndex] += 1
		case 'o':
			password[lastIndex] += 1
		case 'l':
			password[lastIndex] += 1
		}
	}

	return true
}

func state(prefix []byte) *State {
	state := new(State)

	for index, current := range prefix {
		if index > 2 && !state.hasSeq && current == state.previous1+1 && current == state.previous2+2 {
			state.hasSeq = true
		}

		if index > 1 && !state.hasDoubleDouble && current == state.previous1 && current != state.lastDouble {
			if state.lastDouble > 0 {
				state.hasDoubleDouble = true
			} else {
				state.lastDouble = current
			}
		}

		state.previous2, state.previous1 = state.previous1, current
	}

	return state
}

func check(state State, bottom []byte) bool {
	for _, current := range bottom {
		if !state.hasSeq && current == state.previous1+1 && current == state.previous2+2 {
			state.hasSeq = true

			if state.hasDoubleDouble {
				return true
			}
		}

		if !state.hasDoubleDouble && current == state.previous1 && current != state.lastDouble {
			if state.lastDouble > 0 {
				state.hasDoubleDouble = true

				if state.hasSeq {
					return true
				}
			} else {
				state.lastDouble = current
			}
		}

		state.previous2, state.previous1 = state.previous1, current
	}

	return state.hasSeq && state.hasDoubleDouble
}
