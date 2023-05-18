# This is just an example to get you started. You may wish to put all of your
# tests into a single file, or separate them into multiple `test1`, `test2`
# etc. files (better names are recommended, just make sure the name starts with
# the letter 't').
#
# To run these tests, simply execute `nimble test`.

import unittest
import Solutions/day1

const input = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""


test "day1 part 1":
  check day1(input) == 24000

test "day1 part 2":
  check day1_2(input) == 45000
