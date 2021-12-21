#!/usr/bin/env python3

from itertools import *
from sys import argv, stdin, stdout

def extend(memory, newlen):
    memory.extend([0] * (newlen - len(memory)))

def getValue(memory, offset, mode, addr):
    if mode == 0:
        if len(memory) <= addr:
            extend(memory, addr + 1)
        return memory[addr]
    elif mode == 1:
        return addr
    elif mode == 2:
        if len(memory) <= offset + addr:
            extend(memory, offset + addr + 1)
        return memory[offset + addr]

    raise Exception('unexpected mode ' + mode)

def setValue(memory, offset, mode, addr, value):
    if mode == 0:
        if len(memory) <= addr:
            extend(memory, addr + 1)
        memory[addr] = value
    elif mode == 2:
        if len(memory) <= offset + addr:
            extend(memory, offset + addr + 1)
        memory[offset + addr] = value
    else:
        raise Exception('unexpected mode ' + mode)

class State:
    def __init__(self, program):
        self.memory = program.copy()
        self.offset = 0
        self.op = 0

    def step(self, read, write):
        memory = self.memory
        offset = self.offset
        op = self.op
        
        opcode = memory[op] % 100
        modes = memory[op] // 100
        mode1 = modes % 10
        mode2 = (modes // 10) % 10
        mode3 = (modes // 100) % 10

        if opcode != 99:
            if opcode == 1:
                add1 = memory[op + 1]
                add2 = memory[op + 2]
                add3 = memory[op + 3]
                value = getValue(memory, offset, mode1, add1) + getValue(memory, offset, mode2, add2) 
                setValue(memory, offset, mode3, add3, value)
                op = op + 4
            elif opcode == 2:
                add1 = memory[op + 1]
                add2 = memory[op + 2]
                add3 = memory[op + 3]
                value = getValue(memory, offset, mode1, add1) * getValue(memory, offset, mode2, add2) 
                setValue(memory, offset, mode3, add3, value)
                op = op + 4
            elif opcode == 3: # input
                add1 = memory[op + 1]
                setValue(memory, offset, mode1, add1, int(read()))
                op = op + 2
            elif opcode == 4: # print
                add1 = memory[op + 1]
                write(getValue(memory, offset, mode1, add1))
                op = op + 2
            elif opcode == 5: # jump if true
                add1 = memory[op + 1]
                add2 = memory[op + 2]
                if getValue(memory, offset, mode1, add1) > 0:
                    op = getValue(memory, offset, mode2, add2)
                else:
                    op = op + 3
            elif opcode == 6: # jump if false
                add1 = memory[op + 1]
                add2 = memory[op + 2]
                if getValue(memory, offset, mode1, add1) == 0:
                    op = getValue(memory, offset, mode2, add2)
                else:
                    op = op + 3
            elif opcode == 7: # less than
                add1 = memory[op + 1]
                add2 = memory[op + 2]
                add3 = memory[op + 3]
                value = 1 if getValue(memory, offset, mode1, add1) < getValue(memory, offset, mode2, add2) else 0
                setValue(memory, offset, mode3, add3, value)
                op = op + 4
            elif opcode == 8: # equals
                add1 = memory[op + 1]
                add2 = memory[op + 2]
                add3 = memory[op + 3]
                value = 1 if getValue(memory, offset, mode1, add1) == getValue(memory, offset, mode2, add2) else 0
                setValue(memory, offset, mode3, add3, value)
                op = op + 4
            elif opcode == 9: # relative base
                add1 = memory[op + 1]
                offset = offset + getValue(memory, offset, mode1, add1)
                op = op + 2

            self.op = op
            self.offset = offset
            return True

        return False

    def execute(self, read, write):
        while self.step(read, write):
            pass

def execute(program, read, write):
    state = State(program)
    state.execute(read, write)
    return state

def load(program):
    return [int(opcode) for opcode in program.split(',')]

def load_file(program_file):
    return load(program_file.read())

