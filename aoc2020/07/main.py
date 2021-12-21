#!/usr/bin/env python3

from itertools import *
from sys import stdin

rules = dict()
parentsMap = dict()

lines = [line.rstrip() for line in stdin]

for line in lines:
    [parent, children] = line.rstrip(".").split(" contain ")

    rules[parent] = []

    if children == "no other bags":
        continue

    childRules = children.split(", ")

    for childRule in childRules:
        [number, child] = childRule.split(" ", 1)
        number = int(number)

        if child[-1] != "s":
            child += "s"

        rules[parent].append((number, child))

        if child not in parentsMap:
            parentsMap[child] = set()

        parentsMap[child].add(parent)


def find_ancestors(child, found = set()):
    parents = parentsMap[child] if child in parentsMap else {}

    for parent in parents:
        if parent not in found:
            found.add(parent)
            find_ancestors(parent)

    return found

def count_bags(parent, cache = dict()):
    if parent in cache:
        print(parent, "cached with", cache[parent])
        return cache[parent]

    # count our self
    total = 1

    for (count, child) in rules[parent]:
        childTotal = count_bags(child, cache)
        total += childTotal * count

    print(parent, "resolved with", total)
    cache[parent] = total

    return total

ancestors = find_ancestors("shiny gold bags")

print(len(ancestors), ancestors)

print(count_bags("shiny gold bags") - 1)




