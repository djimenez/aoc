#!/usr/bin/env python3

from itertools import *
from sys import stdin

lines = [line.rstrip() for line in stdin]

mem = dict()
mask = "X" * 36

def run_line(line):
    global mask, mem
    [location, value] = line.split(" = ", 1)

    if location == "mask":
        mask = value
    else:
        binary_string = format(int(value), "#038b")[2:]
        masked_string = ""

        for (binary_char, mask_char) in zip(binary_string, mask):
            if mask_char == "X":
                masked_string += binary_char
            else:
                masked_string += mask_char

        print("value: ", binary_string)
        print("mask:  ", mask)
        print("result:", masked_string)
        print()

        mem[location] = int(masked_string, 2)

def run_line2(line):
    global mask, mem
    [location, value] = line.split(" = ", 1)

    if location == "mask":
        mask = value
    else:
        binary_string = format(int(location[4:-1]), "#038b")[2:]
        value = int(value)

        print(binary_string, value)
        masked_string = ""
        indexes = []

        for (index, (binary_char, mask_char)) in enumerate(zip(binary_string, mask)):
            if mask_char == "X":
                masked_string += "X"
                indexes.append(index)
            elif mask_char == "1":
                masked_string += mask_char
            else:
                masked_string += binary_char

        binary_string = masked_string.split("X")
        format_string = "#0" + str(len(indexes) + 2) + "b"

        for fill in range(pow(2, len(indexes))):
            fill_string =  list(format(fill, format_string)[2:])
            masked_string = binary_string[0]

            for (fill_piece, binary_piece) in zip(fill_string, binary_string[1:]):
                masked_string += fill_piece
                masked_string += binary_piece

            print(binary_string, fill_string, masked_string)
            mem[masked_string] = value


for line in lines:
    run_line(line)

print(sum(mem.values()))

mem.clear()
mask = "X" * 36

for line in lines:
    run_line2(line)

print(sum(mem.values()))

