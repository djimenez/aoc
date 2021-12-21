#!/usr/bin/env python3

from itertools import *
from sys import stdin

MIN_VALUE = 128392
MAX_VALUE = 643281

def increasing(value):
    chars = str(value)

    for i in range(0, len(chars) - 1):
        if chars[i] > chars[i + 1]:
            return False

    return True

def doubled(value):
    chars = '0' + str(value * 10)

    for i in range(0, len(chars) - 3):
        a, b, c, d = chars[i:i+4]
        if a != b and b == c and c != d:
            return True 

    return False

count = 0
for value in range(MIN_VALUE, MAX_VALUE + 1):
    if increasing(value) and doubled(value):
        count = count + 1


print(count)
