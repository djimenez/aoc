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

max_workers = 5
completed = set()
completed_order = []
ticks = 0
pending = {}

while len(completed) < len(steps):
    next_steps = sorted([step for step, required in dependencies.items() if step not in pending and len(required - completed) == 0])

    for next_step in next_steps:
        if len(pending) == max_workers:
            break

        pending[next_step] = 60 + ord(next_step) - ord('A') + 1
        
    ticks_to_complete = min(pending.values())
    ticks += ticks_to_complete

    complete = [step for step, ticks in pending.items() if ticks == ticks_to_complete]
    
    for step in complete:
        del dependencies[step]
        del pending[step]
        completed.add(step)
        completed_order.append(step)

    pending = { step: ticks - ticks_to_complete for step, ticks in pending.items() }

print(''.join(completed_order), 'in', ticks, 'seconds')



