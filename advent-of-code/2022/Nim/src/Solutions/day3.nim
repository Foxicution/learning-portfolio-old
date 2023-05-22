import sugar, sequtils, strutils

func processInput(input: string): seq[string] = 
  input.split("\n").filter(line => line != "")

func letterToPoints(c: char): int =
  case c
    of 'a'..'z': ord(c) - ord('a') + 1
    of 'A'..'Z': ord(c) - ord('A') + 27
    else: 0

func chunk[T](input: seq[T], size: int): seq[seq[T]] =
  for i in countup(0, input.len - size, size):
    result.add(input[i .. i  + size - 1])

func day3a*(input: string): int = input.processInput
  .map(line => cast[seq[char]](line).distribute(2))
  .map(comp => comp[0].filter(c => c in comp[1])[0])
  .map(letterToPoints).concat().foldl(a + b)

func day3b*(input: string): int = input.processInput.chunk(3)
  .map(chunk => chunk[0].filter(c => c in chunk[1] and c in chunk[2])[0])
  .concat().map(letterToPoints).foldl(a + b)

