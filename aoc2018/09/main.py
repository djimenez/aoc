#!/usr/bin/env python3

from collections import deque

def play_game(players, marbles):
    circle = deque([0], marbles) 
    scores = [0] * players

    def next_turn(marble):
        player = (marble - 1) % players

        if marble % 23:
            circle.rotate(-1)
            circle.append(marble)
        else:
            circle.rotate(7)
            # add marble we didn't play ande one 7 back
            scores[player] += marble + circle.pop()
            # rotate so next clockwise is current marble
            # like we do normally, but no append needed
            circle.rotate(-1)

    for marble in range(1, marbles + 1):
        next_turn(marble)
    
    return max(scores)

#print(play_game(9, 25))
print(play_game(426, 72058))
print(play_game(426, 72058 * 100))
