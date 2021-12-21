#!/usr/bin/env python3

from collections import Counter
from itertools import *
from sys import stdin

def chunk(seq, size):
    return [seq[i:i + size] for i in range(0, len(seq), size)]


def get_layers(data, width, height):
    stride = width * height

    return chunk(data, stride)

def count_zeroes(layer):
    counter = Counter(layer)
    return counter["0"]

def find_min_index(zeroes):
    index = 0
    minimum = zeroes[0]

    for (count_index, count) in enumerate(zeroes):
        if count < minimum:
            minimum = count
            index = count_index

    return index

def flatten_layers(layers):
    image = list(layers[0])

    for layer in layers:
        for (index, pixel) in enumerate(layer):
            if image[index] != "2":
                continue
            image[index] = pixel

    return "".join(image)

def display_image(image, width):
    rows = chunk(image, width)

    for row in rows:
        row = row.replace("2", ".").replace("1", "*").replace("0", " ")
        print(row)

lines = [line.rstrip() for line in stdin]

for image in lines:
    layers = get_layers(image, 25, 6)

    zeroes = list(map(count_zeroes, layers))
    min_index = find_min_index(zeroes)

    layer = layers[min_index]
    counts = Counter(layer)

    print(counts["1"], counts["2"], counts["1"] * counts["2"])

    display_image(flatten_layers(layers), 25)

