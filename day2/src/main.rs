use std::fs::read_to_string;

fn main() {
    part_one();
    part_two();
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

fn part_two() {
    let file_path = "src/data.txt";
    let contents = read_to_string(file_path)
    .expect("Should have read data.txt");

    let result: u64 = contents
    .lines()
    .flat_map(|line| line.split(","))
    .filter_map(|range| {
        let (start, end) = range.split_once('-')?;
        Some(repeated_n(start, end))
    })
    .sum();

    println!("{result}");
}

fn repeated_n(start: &str, end: &str) -> u64 {
    let start: u64 = start.parse().expect("Invalid number");
    let end: u64 = end.parse().expect("Invalid number");

    (start..=end)
    .filter(|&number| {
        let s = number.to_string();
        let n = s.len();
        for i in 1..=n/2 {
            if n % i != 0 { continue; }
            let mut repeating = true;

            let pattern = &s[0..i];
            let chunks = n / i;

            for k in 1..chunks {
                 if &s[k*i..(k+1)*i] != pattern {
                    repeating = false;
                    break;
                 }
            }

            if repeating {
                return true
            }
        }
        false
    })
    .sum()
    
    /*
    Approach:
    - check all substrings and see that their intervals are all equal
    i.e. subset of size 2, check 1, 3, 5...len are all the same
    Same for 0,2,4,.... Only need to do this for each subset
    So
    for i in range(0 to len(string)) try every substring size
        for j in range(0 to i) (each interval) (0,1,2..substring size) (can just do this by having the pattern stored)
            for k in range(0, len(string), i) (0, 0+i, 0+2*i)
     */
}
