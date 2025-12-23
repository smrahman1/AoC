use std::fs::read_to_string;

fn main() {
    part_one();
}

fn part_one() {
    let file_path = "src/data.txt";
    let contents = read_to_string(file_path)
    .expect("Should have read data.txt");

    let result: u64 = contents
    .lines()
    .flat_map(|line| line.split(","))
    .filter_map(|range| {
        let (start, end) = range.split_once('-')?;
        Some(repeated_twice(start, end))
    })
    .sum();

    println!("{result}");
}

fn repeated_twice(start: &str, end: &str) -> u64 {
    let start: u64 = start.parse().expect("Invalid number");
    let end: u64 = end.parse().expect("Invalid number");

    (start..=end)
    .filter(|&number| {
        let s = number.to_string();
        let mid = s.len() / 2;
        s.len() % 2 == 0 && s[..mid] == s[mid..]
    })
    .sum()
}
