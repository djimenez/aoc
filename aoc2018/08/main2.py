#!/usr/bin/env python3

from sys import stdin

def process_node(values):
    children_len = next(values)
    metadata_len = next(values)
    child_values = [0]
    metadata = []

    for _ in range(children_len):
        child_values.append(process_node(values))

    for _ in range(metadata_len):
        metadata.append(next(values))
        
    if children_len:
        value = 0

        for ref in metadata:
            if ref < len(child_values):
                value += child_values[ref]

        return value
    else:
        return sum(metadata)

print(process_node(map(int, next(stdin).split())))

