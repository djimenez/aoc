#!/usr/bin/env python3

from itertools import *
from sys import stdin
from time import sleep

import intcode

class State:
    def __init__(self):
        self.index = 0
        self.buffer = [0, 0, 0]
        self.score = 0
        self.screen = dict()
        self.min_x = 0
        self.max_x = 0
        self.min_y = 0
        self.max_y = 0

    def draw(self, x, y, tid):
        if tid == 4:
            self.target_x = x

        if tid == 3:
            self.current_x = x

        if x >= 0 and y >= 0:
            self.min_x = min(self.min_x, x)
            self.max_x = max(self.max_x, x)
            self.min_y = min(self.min_y, y)
            self.max_y = max(self.max_y, y)

            key = str(x) + "." + str(y)
            self.screen[key] = tid

            char = "*" if tid > 0 else " "
            print("\033[" + str(y + 1) + ";" + str(x + 1) + "H" + char)
        elif x == -1 and y == 0:
            self.score = tid

    def read(self):
        sleep(0.01)
        if self.current_x > self.target_x:
            return -1
        elif self.current_x < self.target_x:
            return 1
        else:
            return 0

    def write(self, value):
        self.buffer[self.index] = value
        self.index = (self.index + 1) % 3

        if self.index == 0:
            [x, y, tid] = self.buffer
            self.draw(x, y, tid)

    def print(self):
        print('\x1b[2J')

        for y in range(self.min_y, self.max_y + 1):
            line = ""
            for x in range(self.min_x, self.max_x + 1):
                key = str(x) + "." + str(y)

                if key in self.screen and self.screen[key] > 0:
                    line += "*"
                else:
                    line += " "
            print(line)


memory = intcode.load_file(open("input1"))
state = State()

intcode.execute(memory, state.read, state.write)
print(len([value for value in state.screen.values() if value == 2]))

state = State()

# insert coins
memory[0] = 2
print('\x1b[2J')
intcode.execute(memory, state.read, state.write)
print('\x1b[2J')

print(state.score)


