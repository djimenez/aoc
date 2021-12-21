#!/usr/bin/env python3

from itertools import *
from sys import stdin

def parse(line):
    x = 0
    y = 0
    w = 0
    d = 0

    for direction, distance in [(op[0], int(op[1:])) for op in line.split(',')]:
        if direction == 'U':
            u = x
            v = y + distance
            w = 1
        elif direction == 'D':
            u = x
            v = y - distance
            w = 1
        elif direction == 'L':
            u = x - distance
            v = y
            w = 0
        elif direction == 'R':
            u = x + distance
            v = y
            w = 0

        yield (x, y, u, v, w, d)
        d = d + distance
        x = u
        y = v

def intersections(path1, path2):
    for this, that in product(path1, path2):
        x0, y0, u0, v0, w0, d0 = this
        x1, y1, u1, v1, w1, d1 = that

        if w0 != w1:
            if w0 == 0:
                xr0, xr1 = (x0, u0) if x0 <= u0 else (u0, x0)
                yr0, yr1 = (y1, v1) if y1 <= v1 else (v1, y1)

                if x1 >= xr0 and x1 <= xr1 and y0 >= yr0 and y0 <= yr1:
                    yield (x1, y0, d1 + abs(x1 - x0), d0 + abs(y0 - y1))
            else:
                xr0, xr1 = (x1, u1) if x1 <= u1 else (u1, x1)
                yr0, yr1 = (y0, v0) if y0 <= v0 else (v0, y0)

                if x0 >= xr0 and x0 <= xr1 and y1 >= yr0 and y1 <= yr1:
                    yield (x0, y1, d0 + abs(x0 - x1), d1 + abs(y1 - y0))

def manhattan(coord):
    x, y, _, _ = coord
    return abs(x) + abs(y)

def steps(coord):
    _, _, d0, d1 = coord
    return d0 + d1

path1, path2 = [list(parse(line)) for line in stdin]
intersects = list(intersections(path1, path2))
distances = list(map(manhattan, intersects))
steps = list(map(steps, intersects))

#print(path1)
#print(path2)
#print(intersects)
#print(distances)
#print(steps)

print(min([distance for distance in distances if distance > 0]))
print(min([step for step in steps if step > 0]))

