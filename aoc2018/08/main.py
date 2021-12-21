#!/usr/bin/env python3

from sys import stdin

def process_node(values):
    children_len = next(values)
    metadata_len = next(values)
    metadata = []

    for _ in range(children_len):
        metadata += process_node(values)

    for _ in range(metadata_len):
        metadata.append(next(values))

    return metadata

metadata = process_node(map(int, next(stdin).split()))
print(sum(metadata))

