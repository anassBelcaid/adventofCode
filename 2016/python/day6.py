from collections import Counter
from typing import List


def partI(lines: List[str]) -> str:
    """
    Function to compute the most repeated character in each position
    """

    # computing the number of available position
    m = len(lines[0])

    # found characters
    chars = []

    for pos in range(m):
        c = Counter([line[pos] for line in lines])
        chars.append(c.most_common(1)[0][0])
    return "".join(chars)


def partII(lines: List[str]) -> str:
    """
    Function to compute the second part of the answer
    """

    # computing the number of available position
    m = len(lines[0])

    # found characters
    chars = []

    for pos in range(m):
        c = Counter([line[pos] for line in lines])
        chars.append(c.most_common()[-1][0])
    return "".join(chars)


if __name__ == "__main__":
    # reading the parts
    with open("../input/day6") as f:
        lines = [line.strip() for line in f.readlines()]

    # computing the first answer
    answer1 = partI(lines)
    print(answer1)

    # compute the second answer
    answer2 = partII(lines)
    print(answer2)
