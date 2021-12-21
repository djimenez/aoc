#!/usr/bin/env python3

def compute_power(x, y, serial):
    rackId = x + 10
    power = (rackId * (rackId * y + serial) // 100) % 10
    power -= 5

    return power 

def compute_grid(serial):
    return [[compute_power(x + 1, y + 1, serial) for x in range(300)] for y in range(300)]

# compute summed area table
def sat_grid(grid):
    # +1 for a skirt of zeroes
    sat = [[0] * (300 + 1) for _ in range(300 + 1)]

    for x, y in [(x, y) for x in range(300) for y in range(300)]:
        sat[y][x] = grid[y][x] + sat[y - 1][x] + sat[y][x - 1] - sat[y - 1][x - 1]

    return sat

# generate all fuel sells at difference sizes
def fuel_cells(sat, sizes = [2]):
    for size in sizes:
        for x, y in [(x, y) for x in range(300 - size) for y in range(300 - size)]:
            # negative indexes will read 0 skirt - thanks python
            a = sat[y - 1][x - 1]
            b = sat[y - 1][x + size]
            c = sat[y + size][x - 1]
            d = sat[y + size][x + size]

            yield(x + 1, y + 1, size + 1, d - b - c + a)

sat = sat_grid(compute_grid(7857))

print('@ 3:', max(fuel_cells(sat), key=lambda cell: cell[3]))
print('all:', max(fuel_cells(sat, range(300)), key=lambda cell: cell[3]))
