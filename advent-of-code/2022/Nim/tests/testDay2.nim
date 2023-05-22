import unittest
import Solutions/day2

const input = """A Y
B X
C Z"""

test "day2 part 1":
  check input.day2a == 15

test "day2 part 2":
  check input.day2b == 12
