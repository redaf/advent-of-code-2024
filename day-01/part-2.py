#!/usr/bin/env python3
# import math
import sys

def read_lists(file):
    lines = [line.split() for line in file]
    l = [int(line[0]) for line in lines]
    r = [int(line[1]) for line in lines]
    return (l, r)

def main():
    argc = len(sys.argv)
    if argc != 2:
        print(f'Usage: {sys.argv[0]} <input file>')
        exit(1)
    with open(sys.argv[1]) as file:
        l_list, r_list = read_lists(file)
        occurences = {}
        for l in l_list:
            if l in occurences:
                continue
            for r in r_list:
                if l == r:
                    if l in occurences:
                        occurences[l] += 1
                    else:
                        occurences[l] = 1

        score = 0
        for l in l_list:
            if l in occurences:
                score += l * occurences[l]

        print(score)

# Using the special variable
# __name__
if __name__=="__main__":
    main()
