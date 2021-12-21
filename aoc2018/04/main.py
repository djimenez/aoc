#!/usr/bin/env python3

import re
from sys import stdin

date_parser = re.compile(r'\[([0-9]+-[0-9]+-[0-9]+ [0-9]+:)([0-9]+)\] (.*)')
guard_parser = re.compile(r'Guard #([0-9]+) begins shift')

def parse(line):
    (prefix, minute, rest) = date_parser.match(line).groups()
    key = prefix + minute
    minute = int(minute)

    if rest == 'falls asleep':
        return (key, minute, None, False)
    elif rest == 'wakes up':
        return (key, minute, None, True)
    else:
        guard = guard_parser.match(rest).group(1)
        return (key, minute, guard, True)

def sort(lines):
    return sorted(lines, key=lambda line: line[0])

def process_records(lines):
    current_guard = None
    last_sleep = None
    snoozes = {}

    for (_, minute, guard, awake) in lines:
        if guard != None:
            current_guard = guard
            if current_guard not in snoozes:
                snoozes[current_guard] = { 'total': 0, 'minutes': {} }
            continue

        if awake:
            # end a sleep, add count
            time = minute - last_sleep
            snoozes[current_guard]['total'] += time
            for m in range(last_sleep, minute):
                if m not in snoozes[current_guard]['minutes']:
                    snoozes[current_guard]['minutes'][m] = 0
                snoozes[current_guard]['minutes'][m] += 1
        else:
            last_sleep = minute

    return snoozes

def find_guard_with_max_total(snoozes):
    max_total = 0
    max_guard = None

    for guard in snoozes:
        if snoozes[guard]['total'] > max_total:
            max_total = snoozes[guard]['total']
            max_guard = guard

    return (max_guard, max_total)

def find_max_minute(snoozes, guard):
    max_times = 0
    max_minute = None
    minutes = snoozes[guard]['minutes']

    for minute in minutes:
        if minutes[minute] > max_times:
            max_times = minutes[minute]
            max_minute = minute

    return (max_minute, max_times)

def find_max_minute_overall(snoozes):
    max_minute_times = 0
    max_minute_value = None
    max_minute_guard = None

    for guard in snoozes:
        max_minute, max_times = find_max_minute(snoozes, guard)
        if max_times > max_minute_times:
            max_minute_times = max_times
            max_minute_value = max_minute
            max_minute_guard = guard

    return (max_minute_guard, max_minute_value, max_minute_times)

lines = sort([parse(line.rstrip()) for line in stdin])
snoozes = process_records(lines)

max_guard, max_total = find_guard_with_max_total(snoozes)
#print(max_guard, max_total)

max_total_minute, max_total_times = find_max_minute(snoozes, max_guard)
print(int(max_guard) * max_total_minute)

max_overall_guard, max_overall_minute, max_overall_times = find_max_minute_overall(snoozes)
print(int(max_overall_guard) * max_overall_minute)
