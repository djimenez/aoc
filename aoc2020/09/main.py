#!/usr/bin/env python3

from itertools import *
from sys import argv, stdin

window_size = int(argv[1])
numbers = [int(line.rstrip()) for line in stdin]


def find_invalid():
    for vindex in range(window_size, len(numbers)):
        window = numbers[vindex - window_size:vindex]
        value = numbers[vindex]
        found = False

        for [a, b] in combinations(window, 2):
            if value == a + b:
                found = True
                break

        if not found:
            return value

def find_contiguous(target):
    for size in range(2, len(numbers)):
        for index in range(len(numbers) - size):
            window = numbers[index:index + size]
            value = sum(window)

            if value == target:
                return window


invalid = find_invalid()
print(invalid)


contiguous = find_contiguous(invalid)
smallest = min(contiguous)
largest = max(contiguous)

print(smallest + largest, contiguous)


