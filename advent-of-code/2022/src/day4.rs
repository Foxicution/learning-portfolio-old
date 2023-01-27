/// Processes the input into 4 numbers. Start and end for 2 elfs.
/// Then it applies the function to the numbers and returns the count.
/// The function contains the logic for the solution.
fn processing<F>(input: &str, f: F) -> usize
where
    F: Fn(usize, usize, usize, usize) -> bool,
{
    input
        .lines()
        .filter(|line| {
            match line
                .split(",")
                .flat_map(|elf| elf.split("-").map(|num| num.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
                .chunks_exact(4)
                .next()
                .unwrap()
            {
                &[a1, a2, b1, b2] => f(a1, a2, b1, b2),
                _ => false,
            }
        })
        .count()
}

/// The solution filters elements in each line only keeping the numbers.
/// Then it splits the numbers into chunks of 4 (start and end for 2 entities).
/// Then it compares the start and end entities searching if one is inside the other.
/// If it is it takes those lines and returns the count.
/// ```
/// use advent_of_code_2022_rust::day4::day4;
/// let input = "2-4,6-8\n\
/// 2-3,4-5\n\
/// 5-7,7-9\n\
/// 2-8,3-7\n\
/// 6-6,4-6\n\
/// 2-6,4-8";
/// assert_eq!(day4(input), 2);
/// ```
pub fn day4(input: &str) -> usize {
    processing(input, |a1, a2, b1, b2| {
        if a1 >= b1 && a2 <= b2 || a1 <= b1 && a2 >= b2 {
            true
        } else {
            false
        }
    })
}

/// The solution filters elements in each line only keeping the numbers.
/// Then it splits the numbers into chunks of 4 (start and end for 2 entities).
/// Then it compares if the ranges do not overlap.
/// If they don't it filters out those entries and returns the count.
/// ```
/// use advent_of_code_2022_rust::day4::day4_part2;
/// let input = "2-4,6-8\n\
/// 2-3,4-5\n\
/// 5-7,7-9\n\
/// 2-8,3-7\n\
/// 6-6,4-6\n\
/// 2-6,4-8";
/// assert_eq!(day4_part2(input), 4);
pub fn day4_part2(input: &str) -> usize {
    processing(input, |a1, a2, b1, b2| {
        if a1 > b2 && a2 > b2 || a1 < b2 && a2 < b1 {
            false
        } else {
            true
        }
    })
}
