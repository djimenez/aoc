#!/usr/bin/env python3

import re
from sys import stdin

parser = re.compile(r'^pos=<([-0-9]+),([-0-9]+),([-0-9]+)>, r=([0-9]+)')

def parse_line(line):
    return tuple(map(int, parser.match(line).groups()))

def dist(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1]) + abs(a[2] - b[2])

def in_range(nanobot, pos):
    return dist(nanobot, pos) <= nanobot[3]

nanobots = [parse_line(line) for line in stdin]
max_nanobot = max(nanobots, key=lambda nanobot: nanobot[3])
in_range_nanobots = [nanobot for nanobot in nanobots if in_range(max_nanobot, nanobot)]

print(max_nanobot, len(in_range_nanobots))


def gen_positions(nanobot):
    x, y, z, rx = nanobot

    for dx in range(-rx, rx + 1):
        ry = rx - abs(dx)

        for dy in range(-ry, ry + 1):
            rz = ry - abs(dy)

            for dz in range(-rz, rz + 1):
                yield (x + dx, y + dy, z + dz)
                
positions = {}

for nanobot in nanobots:
    for nanobot_pos in gen_positions(nanobot):
        if nanobot_pos not in positions:
            positions[nanobot_pos] = 0

        positions[nanobot_pos] += 1

# sort keys by most coverage and least distance to 0,0,0
keys = sorted(positions, key=lambda pos: (-positions[pos], dist((0,0,0), pos)))

print(keys)






