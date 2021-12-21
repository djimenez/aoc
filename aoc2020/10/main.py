#!/usr/bin/env python3

from itertools import *
from sys import stdin

jolts = [int(line.rstrip()) for line in stdin]
jolts.sort()

jolts.insert(0, 0)
jolts.append(jolts[-1] + 3)

ones = 0
threes = 0

for index in range(1, len(jolts)):
    diff = jolts[index] - jolts[index - 1]

    if diff == 1:
        ones += 1
    elif diff == 3:
        threes += 1

print(ones, threes, ones * threes)


jolts_cache = {len(jolts) - 1: 1}

def count_jolts(index = 0):
    if index in jolts_cache:
        return jolts_cache[index]

    jolt = jolts[index]
    count = 0

    for jindex in range(index + 1, len(jolts)):
        jump = jolts[jindex]

        if jump > jolt + 3:
            break

        count += count_jolts(jindex)

    jolts_cache[index] = count
    return count

print(count_jolts())


