import strutils, sequtils, sugar, algorithm

func prepareInput(input: string): seq[int] =
    input.split("\n\n").map(
        elf => elf.split("\n").map(line => (if line != "": line.parseInt() else: 0)
    ).foldl(a + b))

func day1a*(input: string): int =
    input.prepareInput.max

func day1b*(input: string): int =
    input.prepareInput.sorted(SortOrder.Descending)[0..2].foldl(a + b)
