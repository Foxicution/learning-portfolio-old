// TODO: Clean up the implementation to only be inside of the trait
pub trait Day1Solution {
    fn day1_solution(&self) -> Option<usize>;
}

impl Day1Solution for str {
    /// Day 1 solution
    /// This solution takes the input file and splits it into groups
    /// Then it splits each group into items and parses them into usize
    /// Then it sStringums the items in each group and returns the max
    /// ```
    /// use advent_of_code_2022_rust::day1::Day1Solution;
    /// let input = "1000\n2000\n\n3000\n4000\n6000";
    /// assert_eq!(input.day1_solution(), Some(13000));
    /// ```
    fn day1_solution(&self) -> Option<usize> {
        day1_solution(self)
    }
}

/// Day 1 solution
/// This solution takes the input file and splits it into groups
/// Then it splits each group into items and parses them into usize
/// Then it sums the items in each group and returns the max
/// ```
/// use advent_of_code_2022_rust::day1::day1_solution;
/// let input = "1000\n2000\n\n3000\n4000\n6000";
/// let result = day1_solution(input);
/// assert_eq!(result, Some(13000));
/// ```
pub fn day1_solution(input: &str) -> Option<usize> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|item| item.parse::<usize>().unwrap_or(0))
                .sum::<usize>()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::{day1_solution, Day1Solution};

    #[test]
    fn test_day1_solution() {
        let input = "1000\n\
            2000\n\
            3000\n\n\
            4000\n\n\
            5000\n\
            6000\n\n\
            7000\n\
            8000\n\
            9000\n\n\
            10000";

        assert_eq!(day1_solution(input), Some(24000));
    }

    #[test]
    fn test_trait_impl() {
        let input = "1000\n\
            2000\n\
            3000\n\n\
            4000\n\n\
            5000\n\
            6000\n\n\
            7000\n\
            8000\n\
            9000\n\n\
            10000";

        assert_eq!(input.day1_solution(), Some(24000));
    }
}
