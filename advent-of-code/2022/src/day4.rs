/// The solution filters elements in each line only keeping the numbers. 
/// Then it splits the numbers into chunks of 4 (start and end for 2 entities). 
/// Then it compares the start and end entities searching if one is inside the other. 
/// If it is, it returns true, otherwise false.
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
    input.lines().filter(|line| {
        match line.as_bytes().iter().filter(|&n| b"0123456789".contains(n)).collect::<Vec<_>>().chunks_exact(4).next().unwrap()
        {
            &[a1, a2, b1, b2] => if a1 >= b1 && a2 <= b2 || a1 <= b1 && a2 >= b2 {
                true
            } else {
                false
            }
            _ => false,
            
        }
    }).count()
}
