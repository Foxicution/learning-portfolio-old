import unittest
import Solutions/day2

const input = """A Y
B X
C Z"""

test "day2 part 1":
  check day2(input) == 15

test "day2 part 2":
  check day2_2(input) == 12
