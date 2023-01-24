// The only thing I don't like is we can't see what we import explicitly
use advent_of_code_2022_rust::{
    day1::Day1Solution, day2::day2_part2_solution, day2::day2_solution, day3::day_3,
};

use std::fs::read_to_string;

fn main() {
    println!(
        "Day 1 solution: {:?}",
        read_to_string("./input/1").unwrap().day1_solution()
    );
    println!(
        "Day 2 solution: {:?}, part 2: {:?}",
        day2_solution(read_to_string("./input/2").unwrap().as_str()),
        day2_part2_solution(read_to_string("./input/2").unwrap().as_str())
    );

    println!("Day 3 solution: {:?}", day_3(input));
}
