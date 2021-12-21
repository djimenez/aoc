#!/usr/bin/env python3

from itertools import *
from sys import stdin

import numpy

asteroids = []

def collinear(a, b, c):
    [x1, y1, _] = a
    [x2, y2, _] = b
    [x3, y3, _] = c

    return x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2) == 0

def find_asteroids(a, b):
    min_x = min(a[0], b[0])
    max_x = max(a[0], b[0])
    min_y = min(a[1], b[1])
    max_y = max(a[1], b[1])

    for asteroid in asteroids:
        if asteroid == a or asteroid == b:
            continue

        [x, y, _] = asteroid
    
        if x >= min_x and x <= max_x and y >= min_y and y <= max_y:
            if collinear(a, b, asteroid):
                return True

    return False

def find_best_asteroid():
    found = asteroids[0]
    maximum = found[2]

    for asteroid in asteroids:
        if asteroid[2] > maximum:
            found = asteroid
            maximum = asteroid[2]

    return found

lines = [line.rstrip() for line in stdin]

for (y, line) in enumerate(lines):
    for (x, value) in enumerate(line):
        if value == "#":
            asteroids.append([x, y, 0])


for (a, b) in combinations(asteroids, 2):
    if not find_asteroids(a, b):
        a[2] += 1
        b[2] += 1

best_asteroid = find_best_asteroid()
asteroids.remove(best_asteroid)

print (best_asteroid)

[best_x, best_y, _] = best_asteroid

def get_angled_asteroid(asteroid):
    [x, y, _] = asteroid
    vec = [x - best_x, y - best_y]
    dist = numpy.linalg.norm(vec)
    uvec = vec / dist
    angle = numpy.arccos(numpy.dot([0, -1], uvec))

    if best_x > x:
        angle = 2 * numpy.pi - angle

    return [x, y, numpy.round(angle, 6), dist, 0]

angled_asteroids = list(map(get_angled_asteroid, asteroids))
angled_asteroids.sort(key=lambda a: a[3]) # sort by distance
angled_asteroids.sort(key=lambda a: a[2]) # sort by angle

for (k, g) in groupby(angled_asteroids, lambda a: a[2]):
    for (i, a) in enumerate(g):
        a[4] = i

angled_asteroids.sort(key=lambda a: a[4]) # sort by round

target_asteroid = angled_asteroids[199]
[x, y, _, _, _] = target_asteroid

for (i, asteroid) in enumerate(angled_asteroids):
    print(i + 1, asteroid)

print(target_asteroid, x * 100 + y)

