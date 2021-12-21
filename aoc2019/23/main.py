#!/usr/bin/env python3

from itertools import *
from sys import stdin

import select
import subprocess

processes = []

# spawn 50 nics each with a different ID
for nicId in range(50):
    process = subprocess.Popen(["./nic.py", str(nicId)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, universal_newlines=True)
    processes.append(process)

lookup = {process.stdout: index for (index, process) in enumerate(processes)}

try:
    while True:
        (readables, _, _) = select.select([process.stdout for process in processes], [], [], 1)

        for readable in readables:
            srcId = lookup[readable]
            [dstId, x, y] = [int(x) for x in readable.readline().rstrip().split(" ")]

            print(srcId, "->", dstId, ":", x, y)
            print(x, y, file=processes[dstId].stdin, flush=True)

        if len(readables) == 0:
            # send out a pulse
            for process in processes:
                print(file=process.stdin, flush=True)

        for (nicId, process) in enumerate(processes):
            returnCode = process.poll()
            if returnCode is not None:
                print(nicId, "has terminated with", returnCode)


except:
    for process in processes:
        process.terminate()

    raise
