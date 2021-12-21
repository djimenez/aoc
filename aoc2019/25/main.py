#!/usr/bin/env python3

from itertools import *
from sys import stdin, stdout

import intcode

rbuff = []

def read():
    if len(rbuff) == 0:
        rbuff.extend([ord(char) for char in input()])
        rbuff.append(10)

    return rbuff.pop(0)

def write(value):
    stdout.write(chr(value))

    if value == 10:
        stdout.flush()


program = intcode.load_file("input1")
state = intcode.State(program)

with open("preamble") as preamble:
    rbuff.extend([ord(char) for char in preamble.read()])

state.execute(read, write)

