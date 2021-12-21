#!/usr/bin/env python3

from itertools import *
from sys import stdin

def index(stride, x, y):
    return y * stride + x

def find_seat(layout, stride, x, y, dx, dy, part2):
    max_x = stride - 1
    max_y = (len(layout) // stride) - 1

    while True:
        if x == 0 and dx < 0 or y == 0 and dy < 0 or x == max_x and dx > 0 or y == max_y and dy > 0:
            return None

        x += dx
        y += dy
        seat_index = index(stride, x, y)

        if layout[seat_index] != ".":
            return seat_index

        if not part2:
            return None

def add_seat(lookup, layout, stride, x, y, dx, dy, part2):
    seat_index = find_seat(layout, stride, x, y, dx, dy, part2)

    if seat_index is not None:
        lookup.append(seat_index)


def create_seat_lookups(layout, stride, part2 = False):
    lookups = []

    max_x = stride - 1
    max_y = (len(layout) // stride) - 1

    for y in range(max_y + 1):
        for x in range(max_x + 1):
            lookup = []
            lookups.append(lookup)

            add_seat(lookup, layout, stride, x, y, -1, -1, part2)
            add_seat(lookup, layout, stride, x, y, 0, -1, part2)
            add_seat(lookup, layout, stride, x, y, 1, -1, part2)

            add_seat(lookup, layout, stride, x, y, -1, 0, part2)
            add_seat(lookup, layout, stride, x, y, 1, 0, part2)

            add_seat(lookup, layout, stride, x, y, -1, 1, part2)
            add_seat(lookup, layout, stride, x, y, 0, 1, part2)
            add_seat(lookup, layout, stride, x, y, 1, 1, part2)

    return lookups


def step(layout, stride, lookups, part2 = False):
    next_layout = ""

    for (index, lookup) in enumerate(lookups):
        seat = layout[index]

        if seat == ".":
            next_layout += "."
            continue

        count = 0

        for lookup_index in lookup:
            count += 1 if layout[lookup_index] == "#" else 0

        if seat == "L" and count == 0:
            next_layout += "#"
        elif seat == "#" and (not part2 and count >= 4 or part2 and count >= 5):
            next_layout += "L"
        else:
            next_layout += seat

    return next_layout

def print_layout(layout, stride):
    for index in range(0, len(layout), stride):
        print(layout[index:index + stride])

    print()


lines = [line.rstrip() for line in stdin]
layout = "".join(lines)
stride = len(lines[0])
lookups = create_seat_lookups(layout, stride)

#print_layout(layout, stride)

while True:
    next_layout = step(layout, stride, lookups)
    #print_layout(next_layout, stride)

    if next_layout == layout:
        print(len([seat for seat in next_layout if seat == "#"]))
        break

    layout = next_layout


layout = "".join(lines)
lookups = create_seat_lookups(layout, stride, True)

#print_layout(layout, stride)

while True:
    next_layout = step(layout, stride, lookups, True)
    #print_layout(next_layout, stride)

    if next_layout == layout:
        print(len([seat for seat in next_layout if seat == "#"]))
        break

    layout = next_layout


