#!/usr/bin/env python3

from collections import Counter
from itertools import *
from sys import stdin

lines = [line.rstrip().split(' ') for line in stdin]


part1 = 0
part2 = 0

for [rng, letterSegment, password] in lines:
    [letterMin, letterMax] = map(int, rng.split('-'))
    letter = letterSegment.rstrip(':')

    counts = Counter(password)

    #print(letterMin, letterMax, letter, password, counts)

    if counts[letter] >= letterMin and counts[letter] <= letterMax:
        part1 += 1

    if (password[letterMin - 1] == letter) != (password[letterMax - 1] == letter):
        part2 += 1

print(part1, "passwords passed part 1")
print(part2, "passwords passed part 2")



