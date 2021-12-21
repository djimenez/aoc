#!/usr/bin/env python3

from itertools import *
from sys import stdin

import re

def gather_lines(lines):
    buff = []

    for line in lines:
        if line == "":
            yield " ".join(buff)
            buff = []
        else:
            buff.append(line)

    if len(buff) > 0:
        yield " ".join(buff)

def parse_line(line):
    pairs = [pair.split(":") for pair in line.split(" ")]
    return {key: value for [key, value] in pairs}

def validate_year(value, minimum, maximum):
    try:
        value = int(value)

        return value >= minimum and value <= maximum
    except:
        return False

def validate_hgt(value):
    match = re.fullmatch('([0-9]+)(cm|in)', value)

    if match is not None:
        value = int(match.group(1))
        unit = match.group(2)

        if unit == "cm":
            return value >= 150 and value <= 193
        else:
            return value >= 59 and value <= 76

    return False

def validate_hcl(value):
    return re.fullmatch('#[0-9a-f]{6}', value) is not None

def validate_ecl(value):
    return re.fullmatch('(?:amb|blu|brn|gry|grn|hzl|oth)', value) is not None

def validate_pid(value):
    return re.fullmatch('[0-9]{9}', value) is not None

def validate_field(key, value):
    required_fields = {
            "byr": lambda v: validate_year(v, 1920, 2002),
            "iyr": lambda v: validate_year(v, 2010, 2020),
            "eyr": lambda v: validate_year(v, 2020, 2030),
            "hgt": validate_hgt,
            "hcl": validate_hcl,
            "ecl": validate_ecl,
            "pid": validate_pid
    }

    if key in required_fields:
        return required_fields[key](value)

    return True

def validate(passport):
    required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    return all(key in passport for key in required_fields)

def validate_extended(passport):
    return validate(passport) and all(validate_field(key, value) for key, value in passport.items())

lines = [line.rstrip() for line in stdin]

gathered_lines = list(gather_lines(lines))
gathered_dicts = [parse_line(line) for line in gathered_lines]

print(len(list(filter(validate, gathered_dicts))))
print(len(list(filter(validate_extended, gathered_dicts))))

