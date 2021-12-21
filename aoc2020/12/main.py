#!/usr/bin/env python3

from itertools import *
from sys import stdin
import numpy


def parse_line(line):
    instruction = line[0]
    magnitude = int(line[1:])

    return (instruction, magnitude)

def run_nav(lines):
    pos = numpy.array([0, 0])
    direction = numpy.array([1, 0])

    north = numpy.array([0, 1])
    south = numpy.array([0, -1])
    east = numpy.array([1, 0])
    west = numpy.array([-1, 0])

    rotR = numpy.array([[0, 1], [-1, 0]])
    rotL = numpy.array([[0, -1], [1, 0]])

    for (instruction, magnitude) in lines:
        if instruction == "N":
            pos += north * magnitude
            print("went", instruction, magnitude, "->", pos)
        elif instruction == "S":
            pos += south * magnitude
            print("went", instruction, magnitude, "->", pos)
        elif instruction == "E":
            pos += east * magnitude
            print("went", instruction, magnitude, "->", pos)
        elif instruction == "W":
            pos += west * magnitude
            print("went", instruction, magnitude, "->", pos)
        elif instruction == "F":
            pos += direction * magnitude
            print("went", instruction, magnitude, "->", pos)
        elif instruction == "L":
            while magnitude >= 90:
                direction = rotL.dot(direction)
                magnitude -= 90
            print("turned", instruction, magnitude, "->", direction)
        elif instruction == "R":
            while magnitude >= 90:
                direction = rotR.dot(direction)
                magnitude -= 90
            print("turned", instruction, magnitude, "->", direction)

    return abs(pos[0]) + abs(pos[1])

def run_nav2(lines):
    pos = numpy.array([0, 0])
    direction = numpy.array([10, 1])

    north = numpy.array([0, 1])
    south = numpy.array([0, -1])
    east = numpy.array([1, 0])
    west = numpy.array([-1, 0])

    rotR = numpy.array([[0, 1], [-1, 0]])
    rotL = numpy.array([[0, -1], [1, 0]])

    for (instruction, magnitude) in lines:
        if instruction == "N":
            direction += north * magnitude
            print("waypoint", instruction, magnitude, "->", pos)
        elif instruction == "S":
            direction += south * magnitude
            print("waypoint", instruction, magnitude, "->", pos)
        elif instruction == "E":
            direction += east * magnitude
            print("waypoint", instruction, magnitude, "->", pos)
        elif instruction == "W":
            direction += west * magnitude
            print("waypoint", instruction, magnitude, "->", pos)
        elif instruction == "F":
            pos += direction * magnitude
            print("went", instruction, magnitude, "->", pos)
        elif instruction == "L":
            while magnitude >= 90:
                direction = rotL.dot(direction)
                magnitude -= 90
            print("turned", instruction, magnitude, "->", direction)
        elif instruction == "R":
            while magnitude >= 90:
                direction = rotR.dot(direction)
                magnitude -= 90
            print("turned", instruction, magnitude, "->", direction)

    return abs(pos[0]) + abs(pos[1])

lines = [parse_line(line.rstrip()) for line in stdin]

print(run_nav(lines))
print(run_nav2(lines))


