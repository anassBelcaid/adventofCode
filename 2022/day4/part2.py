"""
Script to solve the first half of the problem
"""
import re
pattern = "^([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)"

def readInput(line):
    """
    function to read and input and split into numbers
    """
    M = re.match(pattern, line)
    if M:
        invervals = list(map(int, M.groups()))
        return invervals
    return None

def overlap(intervals):
    """
    Function to check if the invervals have interesection
    """

    x1, y1, x2, y2 = intervals

    if x1 <= x2 <= y1:
        return True
    if x2 <= x1 <= y2:
        return True

    return False

if __name__ == "__main__":
    
    filename = 'input'

    lines =  []

    with open(filename, "r") as f:
        lines = list(map(readInput, f.readlines()))

    print(sum([overlap(inverval) for inverval in lines]))

        



