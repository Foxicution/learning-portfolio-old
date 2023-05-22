import sugar, sequtils, strutils

func processInput(input: string): seq[string] = input.split("\n")
  .filter(line => line != "")

func letterToPoints(c: char): int = (case c
  of 'a'..'z': ord(c) - ord('a') + 1
  of 'A'..'Z': ord(c) - ord('A') + 27
  else: 0)

func day3a*(input: string): int = input.processInput
  .map(line => cast[seq[char]](line).distribute(2))
  .map(comp => comp[0].filter(c => comp[1].contains(c))[0])
  .map(letterToPoints
  ).concat().foldl(a + b)

# func day3b*(input: string): int = input.processInput.distribute()
