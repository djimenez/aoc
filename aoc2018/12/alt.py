#!/usr/bin/python3

from collections import deque
import re
from sys import stdin

state_parser = re.compile(r'initial state: ([.#]+)')
rule_parser = re.compile(r'([.#]+) => ([.#])')

class Plants(object):
    def __init__(self, start, initial, rules):
        self.start = start
        self.plants= deque(initial)
        self.rules = rules

    def windows(self):
        start = self.start - 2

        for plant in self.plants:
            window.append(plant)
            yield window

        for position in self.positions:
            # initial window
            for left in range(position - 4, position + 1):
                self.window.append('#' if left in self.positions else '.')

            if position - 2 not in seen:
                seen.add(position - 2)
                yield (position - 2, self.window)

            # following windows
            for right in range(position + 1, position + 5):
                self.window.append('#' if right in self.positions else '.')
                if right - 2 not in seen:
                    seen.add(right - 2)
                    yield (right - 2, self.window)

    def apply_rules(self):
        plants = deque([], len(self.plants) + 4)
        window = deque('.....', 5)
        start = self.start - 3

        for plant in self.plants:
            window.append(plant)
            plants.append(self.rules.apply(window))

        for _ in range(4):
            window.append(plant)
            plants.append(self.rules.apply(window))

        # trim left

        # trim right

    def sum(self):
        total = 0
        for i, plant in enumerate(self.plants):
            if plant == '#':
                total += self.start + i
        return total

class Trie(object):
    def __init__(self, value = None):
        self.children = dict()
        self.value = None

    def add(self, path, value):
        node = self

        for item in path:
            if item not in node.children:
                node.children[item] = Trie()
            node = node.children[item]

        node.value = value

    def apply(self, window):
        node = self
        value = window[2]

        for item in window:
            if item not in node.children:
                return value 
            node = node.children[item]

        return node.value

def parse_input():
    initial = state_parser.match(next(stdin)).group(1)

    #skip blank line
    next(stdin)

    plants = Plants(initial)
    rules = Trie()
    
    for line in stdin:
        rule, output = rule_parser.match(line).groups()

        if output == '#':
            rules.add(rule, output)

    return (plants, rules)

plants, rules = parse_input()

for gen in range(20):
    plants.apply_rules(rules)

print(plants.sum())

TARGET=50000000000
for gen in range(20, TARGET):
    plants.apply_rules(rules)
