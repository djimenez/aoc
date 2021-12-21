#!/usr/bin/env python3

from itertools import *
from sys import stdin


trees = [list(line.rstrip()) for line in stdin]

height = len(trees)
width = len(trees[0])

def traverse(dx, dy):
    count = 0
    x = 0
    y = 0

    while y < height:
        if trees[y][x] == "#":
            count += 1

        y += dy
        x = (x + dx) % width

    return count

print(traverse(3, 1))
print(traverse(1, 1) * traverse(3, 1) * traverse(5, 1) * traverse(7, 1) * traverse(1, 2))

