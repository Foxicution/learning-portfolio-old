
/// The solution takes in a string and splits it into lines.
/// Then it splits each line into two equal sized strings.
/// It checks each item in the first string against the second string.
/// If there is a match, it returns the score value of the match.
/// It then returns the sum of all the scores.
/// ```
/// use advent_of_code_2022_rust::day3::day_3;
/// 
/// let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
/// PmmdzqPrVvPwwTWBwg\n\
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
/// ttgJtRGJQctTZtZT\n\
/// CrZsJsPPZsGzwwsLwLmpwMDw";
/// 
/// assert_eq!(day_3(input), 157);
/// ```
pub fn day_3(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (compartment1, compartment2) = line.as_bytes().split_at(line.len() / 2);
            match compartment1
                .iter()
                .filter(|bytes| compartment2.contains(bytes))
                .next()
            {
                Some(char) => match char {
                    b'a'..=b'z' => (char - b'a' + 1) as i32,
                    b'A'..=b'Z' => (char - b'A' + 27) as i32,
                    _ => 0,
                },
                None => 0,
            }
        })
        .sum::<i32>()
}
