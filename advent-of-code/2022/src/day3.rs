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
