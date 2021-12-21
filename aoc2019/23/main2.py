#!/usr/bin/env python3

from itertools import *
from sys import stdin

import intcode
import select
import subprocess

class Process:
    def __init__(self, nicId, program, router):
        self.nicId = nicId
        self.state = intcode.State(program)
        self.rbuff = [nicId]
        self.wbuff = []
        self.router = router
        self.idle = False

    def read(self):
        if len(self.rbuff) > 0:
            value = self.rbuff.pop(0) 
            print("  ", self.nicId, "popped", value)
            self.idle = False
            return value
        else:
            #print("  ", self.nicId, "popped nothing")
            self.idle = True
            return -1

    def write(self, value):
        self.idle = False
        self.wbuff.append(value)

        if len(self.wbuff) == 3:
            [dstId, x, y] = self.wbuff
            self.router(self.nicId, dstId, x, y)
            self.wbuff.clear()

    def step(self):
        return self.state.step(self.read, self.write)

program = intcode.load_file(open("input1"))
processes = []

natx = None
naty = None
natxsent = None
natysent = None


def router(srcId, dstId, x, y):
    global natx
    global naty

    print(srcId, "->", dstId, ":", x, y)

    if dstId != 255:
        processes[dstId].idle = False
        processes[dstId].rbuff.append(x)
        processes[dstId].rbuff.append(y)
    else:
        natx = x
        naty = y

# spawn 50 nics each with a different ID
for nicId in range(50):
    processes.append(Process(nicId, program, router))


while True:
    for process in processes:
        if not process.step():
            raise Error("process halted")

    if natx is not None and naty is not None and all([process.idle for process in processes]):
        if naty == natysent:
            print("nat sent y twice", naty)
            exit()

        router(255, 0, natx, naty)

        natxsent = natx
        natysent = naty

