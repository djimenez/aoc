package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Op struct {
	Code string
	Arg1 string
	Arg2 string
}

func (op *Op) String() string {
	return fmt.Sprintf("%s %s %s", op.Code, op.Arg1, op.Arg2)
}

type Program []*Op

type VM struct {
	A, B uint64
	PC   int
}

func (vm *VM) execute(program Program) {
	vm.PC = 0

	for vm.PC >= 0 && vm.PC < len(program) {
		op := program[vm.PC]

		switch op.Code {
		case "inc":
			vm.inc(op.Arg1)
		case "hlf":
			vm.hlf(op.Arg1)
		case "tpl":
			vm.tpl(op.Arg1)
		case "jmp":
			vm.jmp(op.Arg1)
		case "jie":
			vm.jie(op.Arg1, op.Arg2)
		case "jio":
			vm.jio(op.Arg1, op.Arg2)
		default:
			panic("unknown op code")
		}

		fmt.Println(op, vm)
	}
}

func (vm *VM) inc(register string) {
	switch register {
	case "a":
		vm.A += 1
	case "b":
		vm.B += 1
	default:
		panic("unknown register" + register)
	}

	vm.PC++
}

func (vm *VM) hlf(register string) {
	switch register {
	case "a":
		vm.A /= 2
	case "b":
		vm.B /= 2
	default:
		panic("unknown register" + register)
	}

	vm.PC++
}

func (vm *VM) tpl(register string) {
	switch register {
	case "a":
		vm.A *= 3
	case "b":
		vm.B *= 3
	default:
		panic("unknown register" + register)
	}

	vm.PC++
}

func (vm *VM) jmp(jump string) {
	offset, err := strconv.Atoi(jump)

	if err != nil {
		panic(err)
	}

	vm.PC += offset
}

func (vm *VM) jie(reg, jump string) {
	offset, err := strconv.Atoi(jump)

	if err != nil {
		panic(err)
	}

	switch reg {
	case "a":
		if vm.A%2 == 0 {
			vm.PC += offset
		} else {
			vm.PC += 1
		}
	case "b":
		if vm.B%2 == 0 {
			vm.PC += offset
		} else {
			vm.PC += 1
		}
	default:
		panic("unknown register " + reg)
	}
}

func (vm *VM) jio(reg, jump string) {
	offset, err := strconv.Atoi(jump)

	if err != nil {
		panic(err)
	}

	switch reg {
	case "a":
		if vm.A == 1 {
			vm.PC += offset
		} else {
			vm.PC += 1
		}
	case "b":
		if vm.B == 1 {
			vm.PC += offset
		} else {
			vm.PC += 1
		}
	default:
		panic("unknown register " + reg)
	}
}

func (vm *VM) String() string {
	return fmt.Sprintf("a: %d b: %d pc: %d", vm.A, vm.B, vm.PC)
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	program := make(Program, 0, 250)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")

		op := new(Op)
		op.Code = parts[0]
		op.Arg1 = parts[1]

		if len(parts) > 2 {
			op.Arg2 = parts[2]

			// remove comma from arg1
			op.Arg1 = strings.TrimRight(parts[1], ",")
		}

		program = append(program, op)
	}

	vm := new(VM)
	vm.A = 1
	vm.execute(program)

	fmt.Println(vm)
}
