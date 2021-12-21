#!/usr/bin/env python3

from itertools import *
from sys import stdin

def parse_lines(lines):
    earliest = int(lines[0])
    bus_indexes = [(index, int(bus)) for (index, bus) in enumerate(lines[1].split(",")) if bus != "x"]

    buses = [bus for (index, bus) in bus_indexes]
    indexes = [index for (index, bus) in bus_indexes]

    return (earliest, buses, indexes)

def gen_next(earliest, bus):
    if earliest % bus == 0:
        return earliest

    return ((earliest // bus) + 1) * bus

def find_min(array):
    min_value = min(array)
    return (array.index(min_value), min_value)

def find_max(array):
    max_value = min(array)
    return (array.index(max_value), max_value)

def check_t(t, indexes, buses):
    for (offset, bus) in zip(indexes, buses):
        modulus = (t + offset) % bus
        if modulus != 0:
            print(offset, modulus)
            return False

    return True

lines = [line.rstrip() for line in stdin]
(earliest, buses, indexes) = parse_lines(lines)

next_buses = [gen_next(earliest, bus) for bus in buses]
(index, min_leave) = find_min(next_buses)

print(min_leave, buses[index], (min_leave - earliest) * buses[index])

min_bus = buses[0]
reduced = [bus % min_bus for bus in buses]

print(reduced)

