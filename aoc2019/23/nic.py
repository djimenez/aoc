#!/usr/bin/env python3

from itertools import *
from select import select
from sys import argv, stdin,stderr
from time import sleep

import intcode


nicId = int(argv[1])
readBuffer = [nicId]
writeBuffer = []

def check_readable(readable):
    (readables, _, _) = select([readable], [], [], 0.01)

    return len(readables) > 0

def read():
    while check_readable(stdin):
        #print(" ", nicId, "receiving...", file=stderr, flush=True)
        packet = input()

        if len(packet) > 0:
            [x, y] = [int(part) for part in packet.split(" ")]
            readBuffer.append(x)
            readBuffer.append(y)
            print("  ", nicId, "received", x, y, file=stderr, flush=True)
        #else:
        #    print("  ", nicId, "received pulse", file=stderr, flush=True)

    if len(readBuffer) > 0:
        value = readBuffer.pop(0)
        print("  ", nicId, "popped", value, file=stderr, flush=True)
        return value
    else:
        return -1

def write(value):
    writeBuffer.append(value)

    if len(writeBuffer) == 3:
        [dstId, x, y] = writeBuffer
        print("  ", nicId, "writing", dstId, x, y, file=stderr, flush=True)
        print(dstId, x, y, flush=True)
        writeBuffer.clear()

#print(nicId, "loading", file=stderr)
program = intcode.load_file(open("input1"))
#print(nicId, "executing", file=stderr)
intcode.execute(program, read, write)
