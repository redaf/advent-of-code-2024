#!/usr/bin/env python3
# import math
import sys

def read_lists(file):
    lines = [line.split() for line in file]
    l = [int(line[0]) for line in lines]
    r = [int(line[1]) for line in lines]
    return (l, r)

def sum_of_distances(l, r):
    z = zip(l, r)
    diffs = [abs(x[0] - x[1]) for x in z]
    return sum(diffs)

def main():
    argc = len(sys.argv)
    if argc != 2:
        print(f'Usage: {sys.argv[0]} <input file>')
        exit(1)
    with open(sys.argv[1]) as file:
        l, r = read_lists(file)
        l = sorted(l)
        r = sorted(r)
        sum = sum_of_distances(l,r)
        print(sum)

# Using the special variable
# __name__
if __name__=="__main__":
    main()
