from collections import UserList
from collections.abc import Iterable, Iterator


with open("input.txt") as f:
    codes = f.read().split("\n")

robots = [(0, 2), (0, 2), (3, 2)]


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, other):
        if isinstance(other, Point):
            return Point(self.x + other.x, self.y + other.y)

        if isinstance(other, Iterable):
            if len(other) == 2:
                return Point(self.x + other[0], self.y + other[1])

        raise TypeError

    def __sub__(self, other):
        if isinstance(other, Point):
            return Point(self.x - other.x, self.y - other.y)

        if isinstance(other, Iterable):
            if len(other) == 2:
                return Point(self.x - other[0], self.y - other[1])

        raise TypeError


def cost(start: Point, end: Point, n: int) -> int:
    diff = end - start
    dirs = []

    while diff:  # Find all possible directions
        if diff.x > 0:
            dirs.append((1, 0))
            diff = diff - (1, 0)
        elif diff[0] < 0:
            dirs.append((-1, 0))
            diff = diff - (-1, 0)
        elif diff[1] > 0:
            dirs.append((0, 1))
            diff = diff - (0, 1)
        elif diff[1] < 0:
            dirs.append((0, -1))
            diff = diff - (0, -1)

    print(dirs)


cost((0, 0), (3, 3), 0)
