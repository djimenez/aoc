#!/usr/bin/env python3

RECIPES=704321
TARGET=[int(score) for score in str(RECIPES)]

scores = [3, 7]
positions = [0, 1]

matching = True 
checking = 0

while matching or len(scores) < RECIPES + 10:
    combined = sum([scores[position] for position in positions])
    scores.extend([int(score) for score in str(combined)])

    for i, position in enumerate(positions):
        positions[i] = (position + scores[position] + 1) % len(scores)

    if matching:
        while checking + len(TARGET) < len(scores):
            if TARGET == scores[checking:checking + len(TARGET)]:
                matching = False
                break
            checking += 1

print(*scores[RECIPES:RECIPES + 10], sep='')
print(checking)
