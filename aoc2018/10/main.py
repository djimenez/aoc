#!/usr/bin/env python3

import re
from sys import stdin

parser = re.compile(r'position=< ?(-?[0-9]+),  ?(-?[0-9]+)> velocity=< ?(-?[0-9]+),  ?(-?[0-9]+)>')
def parse(line):
    return list(map(int, parser.match(line).groups()))

dots = [parse(line) for line in stdin]

def simulate(multi = 1):
    for index in range(len(dots)):
        x, y, vx, vy = dots[index]

        dots[index][0] = x + vx * multi
        dots[index][1] = y + vy * multi

def minmax_xy():
    min_x = max_x = dots[0][0]
    min_y = max_y = dots[0][1]

    for x, y, vx, vy in dots:
        min_x = x if x < min_x else min_x
        max_x = x if x > max_x else max_x
        min_y = y if y < min_y else min_y
        max_y = y if y > max_y else max_y

    return (min_x, min_y, max_x, max_y)

def bounding_box_area():
    min_x, min_y, max_x, max_y = minmax_xy()
    return (max_x - min_x) * (max_y - min_y)

area = bounding_box_area()
seconds = 0

while True:
    simulate()
    new_area = bounding_box_area()

    # assuming the message will appear when dots are most tightly together
    if new_area > area:
        # backup 1 second
        simulate(-1)
        break

    area = new_area
    seconds += 1


def print_dots():
    min_x, min_y, max_x, max_y = minmax_xy()

    m = max_x - min_x + 1
    n = max_y - min_y + 1

    lines = [[' '] * m for _ in range(n)]

    for dot in dots:
        x = dot[0] - min_x
        y = dot[1] - min_y

        lines[y][x] = '*'

    for line in lines:
        print(''.join(line))

print_dots()
print(seconds, 'seconds')
