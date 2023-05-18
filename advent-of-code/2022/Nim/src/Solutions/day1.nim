import std/strutils
import std/sequtils
import std/sugar
import std/algorithm

func prepareInput(input: string): seq[int] =
    return input.split("\n\n").map(
        elf => elf.split("\n").map(line => (if line != "": line.parseInt() else: 0)
    ).foldl(a + b))

func day1*(input: string): int =
    return input.prepareInput().max()

func day1Part2*(input: string): int =
    return input.prepareInput().sorted(SortOrder.Descending)[0..2].foldl(a + b)
