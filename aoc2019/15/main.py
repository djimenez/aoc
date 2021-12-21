#!/usr/bin/env python3

from itertools import *
from sys import stdin
from time import sleep

import intcode

class State:
    commands = {1: (0, 1), 2: (0, -1), 3: (-1, 0), 4: (1, 0)}
    backtracks = {1: 2, 2: 1, 3: 4, 4: 3}

    def __init__(self):
        self.bt_queue = []
        self.pos = (0, 0)
        self.next_cmd = 4
        self.explored = {(0, 0)}
        self.map = {(0, 0): 1}
        self.queue = {(0, 0) : [1, 2, 3, 4]}

    def rotate(self, keys):
        if self.rot > 0:
            return keys[self.rot:] + keys[:-self.rot]

        return keys

    def add(self, next_pos):
        return tuple(sum(t) for t in zip(self.pos, next_pos)) 

    def read(self):
        queue = self.queue[self.pos]
        bt_queue = self.bt_queue

        if len(queue) > 0:
            self.next_cmd = queue.pop()
        elif len(self.bt_queue) > 0:
            self.next_cmd = self.bt_queue.pop()
        else:
            return 0

        return self.next_cmd

    def write(self, value):
        next_cmd = self.next_cmd
        next_pos = self.add(self.commands[next_cmd])

        self.explored.add(next_pos)
        self.map[next_pos] = value

        if value >= 1:
            self.pos = next_pos

            if next_pos not in self.queue:
                self.print()
                backtrack = self.backtracks[next_cmd]
                self.bt_queue.append(backtrack)

                self.queue[self.pos] = [1, 2, 3, 4]
                # exclude where we came from
                self.queue[self.pos].remove(backtrack)

            if value == 2:
                self.target = self.pos

    def find_path(self):
        explored = {(0, 0)}
        queue = [(0, 0, 0)]
        target = self.target

        while len(queue) > 0:
            (x, y, d) = queue.pop()
            explored.add((x, y))

            if (x, y) == target:
                return d

            n = (x, y + 1)
            e = (x + 1, y)
            s = (x, y - 1)
            w = (x - 1, y)

            for m in [n, e, s, w]:
                if m in self.map and self.map[m] >= 1 and m not in explored:
                    queue.append((m[0], m[1], d + 1))

            queue.sort(key = lambda q:q[2], reverse = True)

        return -1

    def time_oxygen(self):
        steps = 0
        found = {self.target}

        self.map[(0, 0)] = 1
        
        while len(found) > 0:
            found.clear()

            for pos in self.map:
                if self.map[pos] == 1:
                    (x, y) = pos

                    n = (x, y + 1)
                    e = (x + 1, y)
                    s = (x, y - 1)
                    w = (x - 1, y)

                    for m in [n, e, s, w]:
                        if m in self.map and self.map[m] == 2:
                            found.add(pos)


            for pos in found:
                self.map[pos] = 2

            steps += 1
            self.print()

        return steps - 1



    def print(self):
        print('\033[1;1H')
        xs = [x for (x, y) in self.map.keys()]
        ys = [y for (x, y) in self.map.keys()]

        min_x = min(xs)
        max_x = max(xs)
        min_y = min(ys)
        max_y = max(ys)

        self.map[(0, 0)] = 3

        for y in range(min_y, max_y + 1):
            line = ""
            for x in range(min_x, max_x + 1):
                if (x, y) in self.map:
                    value = self.map[(x, y)]

                    if value == 0:
                        line += "#"
                    elif value == 1:
                        line += "."
                    elif value == 2:
                        line += "O"
                    elif value == 3:
                        line += "D"
                    else:
                        line += " "
                else:
                    line += " "
            print(line)

        print()


memory = intcode.load_file(open("input1"))
state = State()

print('\x1b[2J')
intcode.execute(memory, state.read, state.write)
state.print()
print(state.find_path())
input("press return...")
print(state.time_oxygen())

