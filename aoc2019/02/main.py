#!/usr/bin/env python3

from itertools import *
from sys import stdin

def execute(program, arg1, arg2):
    memory = program.copy()

    memory[1] = arg1
    memory[2] = arg2

    op = 0
    opcode = memory[op]

    while opcode != 99:
        if opcode == 1:
            add1 = memory[op + 1]
            add2 = memory[op + 2]
            add3 = memory[op + 3]
            memory[add3] = memory[add1] + memory[add2]
            op = op + 4
        elif opcode == 2:
            add1 = memory[op + 1]
            add2 = memory[op + 2]
            add3 = memory[op + 3]
            memory[add3] = memory[add1] * memory[add2]
            op = op + 4

        opcode = memory[op]

    return memory[0]

programs = [line for line in stdin]

for program in programs:
    memory = [int(opcode) for opcode in program.split(',')]

    print(execute(memory, 12, 2))  

    for arg1, arg2 in product(range(0, 100), repeat=2):
        result = execute(memory, arg1, arg2)

        if result == 19690720:
            print(arg1, arg2, 100 * arg1 + arg2)
            break

