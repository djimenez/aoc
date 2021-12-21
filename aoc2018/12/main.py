#!/usr/bin/python3

from collections import deque
import re
from sys import stdin

state_parser = re.compile(r'initial state: ([.#]+)')
rule_parser = re.compile(r'([.#]+) => ([.#])')

class pottrie(object):
    def __init__(self, state):
        self.children = dict()
        self.state = state

pottrie = dict()

def rotated(lst, n):
    for i in range(len(lst)):
        yield lst[(i + n) % len(lst)]

def add_rule(rule, output):
    # start with root
    node = pottrie

    # rearrange so we look from center to right then back from left
    states = rule[2:] + rule[0:2]

    for state in rotated(rule, 2):
        if state not in node:
            node[state] = dict()
        node = node[state]

    node['value'] = output

def apply_rules(states):
    node = pottrie
    current = states[2]

    for state in rotated(states, 2):
        if state not in node:
            return current
        node = node[state]

    return node['value']

def parse_input():
    initial = state_parser.match(next(stdin)).group(1)
    
    #skip blank line
    next(stdin)

    for line in stdin:
        rule, output = rule_parser.match(line).groups()

        # only care about rules that actually change the pot
        if rule[2] != output:
            add_rule(rule, output)

    return (deque(initial), 0)

def windows(pots):
    window = deque(['.'] * 5)

    for pot in pots:
        window.popleft()
        window.append(pot)

        yield window

    for _ in range(4):
        window.popleft()
        window.append('.')

        yield window

def next_state(pots, start):
    next_pots = deque()
    next_start = start - 2

    for window in windows(pots):
        next_pots.append(apply_rules(window))

    # trim left
    left = next_pots.popleft()
    while left == '.':
        left = next_pots.popleft()
        next_start += 1
    next_pots.appendleft(left)

    # trim right
    right = next_pots.pop()
    while right == '.':
        right = next_pots.pop()
    next_pots.append(right)

    return next_pots, next_start

def sum_pots(pots, start):
    total = 0
    index = start

    for pot in pots:
        if pot == '#':
            total += index
        index += 1

    return total

pots, start = parse_input()
seen = {}

for gen in range(20):
    pots, start = next_state(pots, start)

print(sum_pots(pots, start))

TARGET = 50000000000
# while generating out to practically infinity, look for a cycle
for gen in range(20, TARGET):
    pots, start = next_state(pots, start)
    key = ''.join(pots)
    if key in seen:
        # we've found a cycle, compute start offset based on gen offet
        prev_gen, prev_start = seen[key]
        offset_start = start - prev_start
        offset_gen = gen - prev_gen
        remaining_gens = TARGET - gen - 1 # this off by one haunts me

        # compute the start into the future
        # sanity check we get there nicely (they wouldn't give us an evil puzzle would they?)
        if remaining_gens % offset_gen != 0:
            raise 'cannot reach target in whole steps'

        future_start = remaining_gens // offset_gen * offset_start + start

        print(sum_pots(pots, future_start))
        break
    seen[key] = (gen, start)

    
