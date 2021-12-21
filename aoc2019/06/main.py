#!/usr/bin/env python3

from itertools import *
from sys import stdin

deps = [line.strip().split(')') for line in stdin]
orbits = {}
parents = {}

myOrbit = 'YOU'
sanOrbit = 'SAN'

for inner, outer in deps:
    if inner in orbits:
        orbits[inner].append(outer)
    else:
        orbits[inner] = [outer]

    parents[outer] = inner

def travel(root = 'COM', indirects = 0):
    if root not in orbits:
        return indirects

    total = indirects

    for orbiter in orbits[root]:
        total = total + travel(orbiter, indirects + 1)

    return total

print(travel())

def getPathToCom(orbiter):
    while orbiter != 'COM':
        orbiter = parents[orbiter]
        yield orbiter

youPathToCom = list(getPathToCom('YOU'))
youPathToCom.reverse()

sanPathToCom = list(getPathToCom('SAN'))
sanPathToCom.reverse()

while youPathToCom[0] == sanPathToCom[0]:
    youPathToCom.pop(0)
    sanPathToCom.pop(0)

print(len(youPathToCom) + len(sanPathToCom))



