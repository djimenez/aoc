#!/usr/bin/env python3

from collections import Counter
import math
import re
from sys import stdin

parser = re.compile(r'([0-9]+), ([0-9]+)')
def parse(line):
    return tuple(map(int, parser.match(line).groups()))

points = [parse(line) for line in stdin]

def is_contained(point):
    # a point constrains another point in an axis if its closer in the opposite axis
    # than it is far away in the same axis
    neg_x, neg_y, pos_x, pos_y = False, False, False, False

    for other in points:
        dist_x = abs(point[0] - other[0])
        dist_y = abs(point[1] - other[1])

        neg_x = neg_x or (other[0] < point[0] and dist_y <= dist_x)
        pos_x = pos_x or (other[0] > point[0] and dist_y <= dist_x)
        neg_y = neg_y or (other[1] < point[1] and dist_x <= dist_y)
        pos_y = pos_y or (other[1] > point[1] and dist_x <= dist_y)

        if neg_x and neg_y and pos_x and pos_y:
            break

    return neg_x and neg_y and pos_x and pos_y

contained_points = [point for point in points if is_contained(point)]

xs, ys = [point[0] for point in points], [point[1] for point in points]
min_x, min_y = min(xs), min(ys)
max_x, max_y = max(xs), max(ys)
m, n = max_x - min_x + 1, max_y - min_y + 1

grid = [None] * m * n

def distance(x, y, point):
    return abs(point[0] - x) + abs(point[1] - y)

for y in range(n):
    for x in range(m):
        real_x, real_y = x + min_x, y + min_y
        sorted_points = sorted(points, key=lambda point: distance(real_x, real_y, point))
        first, second = distance(real_x, real_y, sorted_points[0]), distance(real_x, real_y, sorted_points[1])

        if first < second:
            grid[y * m + x] = sorted_points[0]

        #print(real_x, real_y, grid[y * m + x], sorted_points[:2], [first, second])

counts = Counter(grid)

print(max(map(lambda point: counts[point], contained_points)))

search_distance = 10000
search_mod = int(math.ceil(search_distance / len(points))) + 10

avg_x = sum(xs) // len(xs)
avg_y = sum(ys) // len(ys)

search_min_x, search_max_x = avg_x - search_mod, avg_x + search_mod 
search_min_y, search_max_y = avg_y - search_mod, avg_y + search_mod
m, n = search_max_x - search_min_x + 1, search_max_y - search_min_y + 1

count = 0
avg_count = 0

for y in range(n):
    for x in range(m):
        real_x, real_y = x + search_min_x, y + search_min_y
        avg_distance = distance(real_x, real_y, (avg_x, avg_y))
        if avg_distance * len(points) < search_distance:
            avg_count += 1
        distances = [distance(real_x, real_y, point) for point in points]
        if sum(distances) < search_distance:
            count += 1

print(count, avg_count)


