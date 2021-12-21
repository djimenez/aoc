#!/usr/bin/env python3

from sys import stdin

def invert(unit):
    return unit.lower() if unit.isupper() else unit.upper()

def process_polymer(polymer):
    left = [] 
    checks = []
    check = None

    for unit in polymer:
        if unit == check:
            # annihilate from left and move on
            left.pop()
            check = checks.pop()
        else:
            left.append(unit)
            checks.append(check)
            check = invert(unit)

    return left

def process_removal(polymer, unit):
        return process_polymer(polymer.replace(unit, '').replace(invert(unit), ''))

for polymer in [line.rstrip() for line in stdin]:
    processed = ''.join(process_polymer(polymer))
    print(len(processed))

    unreacted = set(processed.lower())
    print(min([len(process_removal(processed, unit)) for unit in unreacted]))

