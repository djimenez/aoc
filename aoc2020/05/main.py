#!/usr/bin/env python3

from itertools import *
from sys import stdin


def convert(line):
    line = line.replace("F", "0")
    line = line.replace("B", "1")
    line = line.replace("L", "0")
    line = line.replace("R", "1")

    return int(line, 2)

seatIds = {convert(line.rstrip()) for line in stdin}

print(max(seatIds))

for seatId in range(1, 2 ** 11):
    if seatId not in seatIds:
        previousId = seatId - 1
        nextId = seatId +1

        if previousId in seatIds and nextId in seatIds:
             print(seatId)
             break
