#!/usr/bin/env python3

from itertools import *
from sys import stdin

def parse(line):
    [op, offset] = line.rstrip().split(" ")

    return (op, int(offset))

def execute(lines):
    acc = 0
    index = 0
    visited = set()

    while index < len(lines):
        if index in visited:
            return (False, acc)

        visited.add(index)

        (op, offset) = lines[index]

        if op == "acc":
            acc += offset
            index += 1
        elif op == "jmp":
            index += offset
        else:
            index += 1

    return (True, acc)

lines = [parse(line) for line in stdin]

print(execute(lines))


for index in range(len(lines)):
    (op, offset) = lines[index]

    if op == "jmp":
        lines[index] = ("nop", offset)
    elif op == "nop":
        lines[index] = ("jmp", offset)
    else:
        continue

    (finished, acc) = execute(lines)

    if finished:
        print(index, acc)
        break

    lines[index] = (op, offset)
    







