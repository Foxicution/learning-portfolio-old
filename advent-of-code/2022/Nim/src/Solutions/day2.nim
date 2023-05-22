import strutils, sequtils, sugar, math

func dataPreparation(input: string): seq[(int, int)] = input.split("\n")
  .filter(x => x != "")
  .map(x => (ord(x[0]) - ord('A'), ord(x[2]) - ord('X')))

func day2a*(input: string): int = input.dataPreparation
  .map(x => 1 + x[1] + 3 * (1 + x[1] - x[0]).floorMod(3)).foldl(a + b)

func day2b*(input: string): int = input.dataPreparation
  .map(x => 1 + x[1] * 3 + (2 + x[1] + x[0]) mod 3).foldl(a + b)

