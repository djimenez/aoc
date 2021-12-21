#!/usr/bin/env python3

from itertools import *
from sys import stdin

changes = [int(line) for line in stdin]
# part 1
print(sum(changes))

# part 2
frequencies = accumulate(cycle(changes))
seen = set([0])

for frequency in frequencies:
    if frequency in seen:
        print(frequency)
        break
    else:
        seen.add(frequency)

