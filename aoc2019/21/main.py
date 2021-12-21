#!/usr/bin/env python3

from itertools import *
from sys import stdin

import intcode

class AsciiRepl:
    def __init__(self):
        self.read_buffer = []
        self.write_buffer = ""
        self.success = False

    def read(self):
        if len(self.read_buffer) == 0:
            line = input() + "\n"
            self.read_buffer = [ord(char) for char in line]

        return self.read_buffer.pop(0)

    def write(self, value):
        if value <= 128:
            if value == 10:
                print(self.write_buffer)
                self.write_buffer = ""
            else:
                self.write_buffer += chr(value)
        else:
            print(value)
            self.success = True

    def flush(self):
        if len(self.write_buffer) > 0:
            print(self.write_buffer)
            self.write_buffer = ""

program = intcode.load_file(open("input1"))

repl = AsciiRepl()
intcode.execute(program, repl.read, repl.write)
repl.flush()


if not repl.success:
    exit(1)
