import sugar, sequtils, strutils


func day3a*(input: string): int = input.split("\n").filter(line => line != "")
  .map(line => cast[seq[char]](line).distribute(2))
  .map(comp => comp[0].filter(c => comp[1].contains(c))[0])
  .map(c => (case c
  of 'a'..'z': ord(c) - ord('a') + 1
  of 'A'..'Z': ord(c) - ord('A') + 27
  else: 0)
).concat().foldl(a + b)
