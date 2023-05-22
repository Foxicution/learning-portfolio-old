import strformat, tables

import Solutions/[day1, day2, day3]

let answers = {
    1: @[day1a, day1b],
    2: @[day2a, day2b],
    3: @[day3a, day3b]
    }.toOrderedTable

for k, funcs in answers.pairs:
    echo "Day ", k, " solutions"
    for f in funcs:
        echo fmt"./input/{k}".readFile.f
