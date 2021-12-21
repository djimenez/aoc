#!/usr/bin/env python3

from functools import reduce
from itertools import *
from sys import stdin


def group_lines(lines):
    group = []

    for line in lines:
        if line == "":
            yield group
            group = []
        else:
            group.append(line)

    if len(group) > 0:
        yield group

groups = list(group_lines([line.rstrip() for line in stdin]))
part1_counts = [len(set("".join(group))) for group in groups]

print(sum(part1_counts))

def count_group(group):
    sets = [set(line) for line in group]
    return len(reduce(lambda a, b: a & b, sets))

part2_counts = [count_group(group) for group in groups]

print(sum(part2_counts))

