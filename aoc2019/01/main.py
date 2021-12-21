#!/usr/bin/env python3

from itertools import *
from sys import stdin

fuels = [max(0, int(line) // 3 - 2) for line in stdin]
# part 1
print(sum(fuels))

# part 2
total = 0
iteration = sum(fuels)
while iteration > 0:
    total = total + iteration
    fuels = [max(0, fuel // 3 - 2) for fuel in fuels]
    iteration = sum(fuels)

print(total)

