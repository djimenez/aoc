#!/usr/bin/env python3

import heapq
from sys import stdin

def extract_combatants(battlefield):
    combatants = []

    for y, line in enumerate(battlefield):
        for x, space in enumerate(line):
            if space in ['E', 'G']:
                combatants.append((space, x, y))

    return combatants

def adjacents(battlefield, x0, y0, want):
    match = ['.', want] 
    coords = [(x0, y0 - 1), (x0 - 1, y0), (x0 + 1, y0), (x0, y0 + 1)]

    for x, y in coords:
        value = battlefield[y][x]

        if value in match:
            yield (value, x, y)

def find_target(battlefield, val, x0, y0):
    pq = heapq.heapify([(0, y0, x0)]])
    seen = set()
    want = 'E' if val == 'G' else 'G'

    while pq:
        dist, y, x = heappop(pq)
        seen.add((x, y))

        for va, xa, ya in adjacents(battlefield, x, y, want):
            # we found a space adjacent to a wanted target
            if va == want:
                return (x, y)

            if (xa, ya) not in seen:
                dist = abs(x0 - xa) + abs(y0 - ya])
                heappush(pq, (dist, ya, xa))

    return (x0, y0)

battlefield = [[char for char in line.rstrip('\n')] for line in stdin]

while True:
    combatants = extract_combatants(battlefield)
    movement = False
    combat = False

    for val, x, y in combatants:
        tx, ty = find_target(battliefield, val, x, y)
        if tx != x and ty != y:
            movement = True

        # move one step in reading order
        if ty < y:
            battlefield[y][x] = '.'
            battlefield[y - 1][x] = val
            movement = True
        elif tx < x:
            battlefield[y][x] = '.'
            battlefield[y][x - 1] = val
            movement = True
        elif tx > x:
            battlefield[y][x] = '.'
            battlefield[y][x + 1] = val
            movement = True
        elif ty > y:
            battlefield[y][x] = '.'
            battlefield[y + 1][x] = val
            movement = True












