#!/usr/bin/env python3

import re
from sys import stdin

pattern = re.compile(r'Step ([A-Z]) must be finished before step ([A-Z]) can begin.')
def parse(line):
    return pattern.match(line).groups()

dependencies = dict()
steps = set()

for step, dependent in [parse(line) for line in stdin]:
    steps.add(step)
    steps.add(dependent)

    if dependent not in dependencies:
        dependencies[dependent] = set()

    if step not in dependencies:
        dependencies[step] = set()

    dependencies[dependent].add(step)

completed = set()
completed_order = []
while len(completed) < len(steps):
    next_step = sorted([step for step, required in dependencies.items() if len(required - completed) == 0])[0]

    # apply the step
    del dependencies[next_step]
    completed.add(next_step)
    completed_order.append(next_step)

print(''.join(completed_order))



