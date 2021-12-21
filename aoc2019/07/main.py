#!/usr/bin/env python3

from itertools import *
from sys import argv, stdin
from subprocess import PIPE, Popen

def execute(program, phase, inp):
    with Popen(['./intcode', program], stdin=PIPE, stdout=PIPE, universal_newlines=True) as proc:
        proc.stdin.write(str(phase) + '\n')
        proc.stdin.write(str(inp) + '\n')
        proc.stdin.flush()

        output = proc.stdout.readline()

        return  int(output)

def execute2(program, phase):
    proc = Popen(['./intcode', program], stdin=PIPE, stdout=PIPE, universal_newlines=True)
    proc.stdin.write(str(phase) + '\n')
    proc.stdin.flush()

    return proc

def pump(proc, inp):
    try:
        proc.stdin.write(str(inp) + '\n')
        proc.stdin.flush()

        output = proc.stdout.readline()
        proc.poll()

        if output != '':
            return int(output)
        else:
            return inp
    except:
        return inp

program = argv[1]
outputs = []

for a,b,c,d,e in permutations([0, 1, 2, 3, 4]):
    output = execute(program, a, 0)
    output = execute(program, b, output)
    output = execute(program, c, output)
    output = execute(program, d, output)
    output = execute(program, e, output)

    outputs.append(output)

    #print(a, b, c, d, e, output)

print(max(outputs))
outputs.clear()

for a,b,c,d,e in permutations([5,6,7,8,9]):
    proc1 = execute2(program, a)
    proc2 = execute2(program, b)
    proc3 = execute2(program, c)
    proc4 = execute2(program, d)
    proc5 = execute2(program, e)

    output = 0

    while proc1.returncode == None:
        output = pump(proc1, output)
        output = pump(proc2, output)
        output = pump(proc3, output)
        output = pump(proc4, output)
        output = pump(proc5, output)

        #print(a, b, c, d, e, output)

        proc1.poll()

    outputs.append(output)

print(max(outputs))




    


