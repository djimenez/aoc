#!/usr/bin/env python3

import re
from sys import stdin

claims = [None] * 1000 * 1000

parser = re.compile(r'#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)')
def parse(line):
    (claim, left, top, width, height) = parser.match(line).groups()
    left = int(left)
    top = int(top)
    width = int(width)
    height = int(height)
    return (claim, left, top, width, height)

lines = [parse(line.rstrip()) for line in stdin]
for (claim, left, top, width, height) in lines:
    for y in range(top, top + height):
        offset = y * 1000
        for index in range(offset + left, offset + left + width):
            if not claims[index]:
                claims[index] = set()
            claims[index].add(claim)

print(len([claim for claim in claims if claim and len(claim) > 1]))

def check_claim(claim, left, top, width, height):
    for y in range(top, top + height):
        offset = y * 1000
        for index in range(offset + left, offset + left + width):
            if len(claims[index]) > 1:
                return False
    return True

for (claim, left, top, width, height) in lines:
    if check_claim(claim, left, top, width, height):
        print(claim)


