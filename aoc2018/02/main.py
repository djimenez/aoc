#!/usr/bin/env python3

from collections import Counter
from itertools import *
from sys import stdin

doubles = 0
triples = 0

lines = [line.rstrip() for line in stdin]
for line in lines:
    counter = Counter(line)
    doubles += any(count == 2 for letter, count in counter.items())
    triples += any(count == 3 for letter, count in counter.items())

print(doubles * triples)

for (left, right) in combinations(lines, 2):
    matches = [lchar for (lchar, rchar) in zip(left, right) if lchar == rchar]
    if len(matches) >= len(left) - 1:
        print(''.join(matches))
        break
