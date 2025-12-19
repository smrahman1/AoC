use std::fs::read_to_string;

fn main() {
    let file_path = "src/data.txt";
    let contents = read_to_string(file_path)
    .expect("Should have read data.txt");

    let mut result: u64 = 0;

    for line in contents.lines() {
        let ranges = line.split(",");
        for range in ranges {
            let mut range_iter = range.split("-");
            let start = range_iter.next().unwrap();
            let end = range_iter.next().unwrap();
            let repeated_twice_sum = repeated_twice(start, end);
            result += repeated_twice_sum;
                
        }
    }
    println!("Final Result: {result}");
}

fn repeated_twice(start: &str, end: &str) -> u64 {
    let mut result: u64 = 0;
    let start_int: u64 = start.parse().unwrap();
    let end_int: u64 = end.parse().unwrap();

    for number in start_int..=end_int {
        let number_str = number.to_string();
        let length: usize = number_str.len();
        if length % 2 == 0 {
            let mut first_half = String::from("");
            for i in 0..=length/2-1 {
                first_half.push(number_str.chars().nth(i).unwrap());
            } 
            let mut second_half = String::from("");
            for i in length/2..=length-1 {
                second_half.push(number_str.chars().nth(i).unwrap());
            } 


            if first_half.eq(&second_half) {
                result += number;
            }
        }
    }
    return result;
}