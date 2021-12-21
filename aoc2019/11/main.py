#!/usr/bin/env python3

from itertools import *
from sys import stdin

import intcode

class State:
    def __init__(self):
        self.painting = True
        self.x = 0
        self.y = 0
        self.dx = 0
        self.dy = 1
        self.min_x = 0
        self.max_x = 0
        self.min_y = 0
        self.max_y = 0
        self.panels = dict()

    def paint(self, color):
        key = str(self.x) + "." + str(self.y)
        self.panels[key] = color

    def turn(self, direction):
        if direction == 0:
            if self.dx == 0:
                self.dx = 1 if self.dy == -1 else -1
                self.dy = 0
            else:
                self.dy = 1 if self.dx == 1 else -1
                self.dx = 0
        else:
            if self.dx == 0:
                self.dx = -1 if self.dy == -1 else 1
                self.dy = 0
            else:
                self.dy = -1 if self.dx == 1 else 1
                self.dx = 0

        self.x += self.dx
        self.y += self.dy

        self.min_x = min(self.min_x, self.x)
        self.max_x = max(self.max_x, self.x)
        self.min_y = min(self.min_y, self.y)
        self.max_y = max(self.max_y, self.y)

    def read(self):
        key = str(self.x) + "." + str(self.y)
        if key in self.panels:
            return self.panels[key]
        else:
            return 0

    def write(self, value):
        if self.painting:
            self.paint(value)
        else:
            self.turn(value)

        self.painting = not self.painting

    def print(self):
        for y in range(self.max_y, self.min_y - 1, -1):
            line = ""
            for x in range(self.min_x, self.max_x + 1):
                key = str(x) + "." + str(y)

                if key in self.panels and self.panels[key] == 1:
                    line += "*"
                else:
                    line += " "
            print(line)


memory = intcode.load_file(stdin)
state = State()

intcode.execute(memory, state.read, state.write)

print(len(state.panels))

state = State()
state.paint(1)

intcode.execute(memory, state.read, state.write)
state.print()
