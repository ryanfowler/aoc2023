fn main() {
    println!("Day 01");
    println!("======");

    let input = include_str!("./input.txt");

    let ts = std::time::Instant::now();
    let out1: u32 = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum();
    println!("Part 1: {} ({:?})", out1, ts.elapsed());

    let ts = std::time::Instant::now();
    const NUMS: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let out2: u32 = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.as_bytes();
            let mut num: u32 = 0;

            let mut idx = 0;
            while idx < line.len() {
                let c = line[idx];
                if c.is_ascii_digit() {
                    num += 10 * (c - b'0') as u32;
                    break;
                }
                if let Some(n) = NUMS.iter().find_map(|(s, n)| {
                    if line[idx..].starts_with(s.as_bytes()) {
                        Some(n)
                    } else {
                        None
                    }
                }) {
                    num += 10 * n;
                    break;
                }
                idx += 1;
            }

            idx = line.len() - 1;
            loop {
                let c = line[idx];
                if c.is_ascii_digit() {
                    num += (c - b'0') as u32;
                    break;
                }
                if let Some(n) = NUMS.iter().find_map(|(s, n)| {
                    if line[..(idx + 1)].ends_with(s.as_bytes()) {
                        Some(n)
                    } else {
                        None
                    }
                }) {
                    num += n;
                    break;
                }
                idx -= 1;
            }

            num
        })
        .sum();
    println!("Part 2: {} ({:?})", out2, ts.elapsed());
}
