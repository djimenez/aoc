#!/usr/bin/env python3

from sys import stdin

def process_polymer(polymer):
    processed = polymer.rstrip()
    lowercases = set(processed.lower())
    
    while polymer != processed:
        polymer = processed

        for lowercase in lowercases:
            lu = lowercase + lowercase.upper()
            ul = lowercase.upper() + lowercase

            processed = processed.replace(lu, '')
            processed = processed.replace(ul, '')

    return processed


for polymer in stdin:
    print(len(process_polymer(polymer)))

    uniques = set(polymer.lower())
    min_len = len(polymer)

    for unique in uniques:
        altered = polymer.replace(unique, '')
        altered = altered.replace(unique.upper(), '')
        processed = process_polymer(altered)

        if len(processed) < min_len:
            min_len = len(processed)

    print(min_len)

