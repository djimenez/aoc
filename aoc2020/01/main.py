#!/usr/bin/env python3

from itertools import *
from sys import stdin

expenses = [int(line) for line in stdin]

#part 1
print(next(l * r for (l, r) in combinations(expenses, 2) if l + r == 2020))

#part 2
print(next(e1 * e2 * e3 for (e1, e2, e3) in combinations(expenses, 3) if e1 + e2 + e3 == 2020))


