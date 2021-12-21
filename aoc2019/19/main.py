#!/usr/bin/env python3

from itertools import *
from sys import stdin
from time import sleep

import intcode

memory = intcode.load_file(open("input1"))

def probe(x, y):
    inputs = [y, x]
    output = None

    def read():
        return inputs.pop()

    def write(value):
        nonlocal output
        output = value

    intcode.execute(memory, read, write)

    return output

count = 0

for y in range(0, 50):
    for x in range(0, 50):
        count += probe(x, y)

print(count)


min_x = 0

for y in range(100, 100000):
    for x in range(min_x, 100000):
        status = probe(x, y)

        if status == 1:
            tr_status = probe(x + 99, y - 99)

            if tr_status == 1:
                tl_status = probe(x, y - 99)
                br_status = probe(x + 99, y)

                if tl_status == 1 and br_status == 1:
                    print(x, y - 99, x * 10000 + y - 99)
                    exit()

            min_x = x
            break
        
