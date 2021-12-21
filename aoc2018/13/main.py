#!/usr/bin/env python3

from sys import stdin

def extract_carts(tracks):
    carts = {}

    for y, line in enumerate(tracks):
        for x, char in enumerate(line):
            if char in  ['<', '>', '^', 'v']:
                carts[(y, x)] = (char, 0)
                line[x] = '-' if char in ['<', '>'] else '|'

    return carts

cart_choice = { '<': ['v', '<', '^'], '^': ['<', '^', '>'], '>': ['^', '>', 'v'], 'v': ['>', 'v', '<'] }
curve_cart = { '\\': { '<': '^', '^': '<', '>': 'v', 'v': '>' }, '/': { '<': 'v', '^': '>', '>': '^', 'v': '<' } } 

def move_carts(tracks, carts):
    ordered_keys = sorted(carts.keys())
    collisions = []

    for key in ordered_keys:
        # may have been removed by previous collision
        if key not in carts:
            continue

        y, x = key
        cart, choice = carts[key]

        if cart == '<':
            x -= 1
        elif cart == '>':
            x += 1
        elif cart == '^':
            y -= 1
        else:
            y += 1

        # check for collision
        if (y, x) in carts:
            collisions.append((x, y))

            # remove both carts involved
            del carts[key]
            del carts[(y, x)]

            # no need to do rest of cart's tick
            continue

        track = tracks[y][x]

        if track == '+':
            cart = cart_choice[cart][choice]
            choice = (choice + 1) % 3
        elif track in ['\\', '/']:
            cart = curve_cart[track][cart]
            
        # remove cart from where it was, put it where it is
        del carts[key]
        carts[(y, x)] = (cart, choice)

    return collisions


tracks = [[char for char in line.rstrip('\n')] for line in stdin]
carts = extract_carts(tracks)
collisions = []

# puzzles all eventually crash all or all but 1 cart
while len(carts) > 1:
    collisions += move_carts(tracks, carts)

if len(collisions) > 0:
    print(collisions[0])

if len(carts) == 1:
    y, x = [key for key in carts.keys()][0]
    print((x, y))
