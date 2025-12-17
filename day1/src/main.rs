use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut current = 50i64;
    let mut result = 0i64;
    let file_path = "src/data.txt";
    let contents = read_to_string(file_path)
    .expect("Should have read data.txt");


    for line in contents.lines() {
        let mut chars = line.chars();
        let first_char = chars.next();
        if let Some(direction) = first_char {
            let rotate_amount = chars.as_str().parse::<i64>().unwrap();
            match direction {
                'R' => current += rotate_amount,
                'L' => current -= rotate_amount,
                _ => {
                    eprintln!("Invalid direction: {}", direction);
                    continue;
                }
            }
            current = current.rem_euclid(100);
            if current == 0 {
                result += 1;
            }
        } else {
            println!("error?");
        }
    }
    println!("Result is {}", result)
}

fn part2() {
    let mut current = 50i64;
    let mut result = 0i64;
    let file_path = "src/data.txt";
    println!("current is : {}", current);
    let contents = read_to_string(file_path)
    .expect("Should have read data.txt");


    for line in contents.lines() {
        let mut chars = line.chars();
        let first_char = chars.next();
        if let Some(direction) = first_char {
            let rotate_amount = chars.as_str().parse::<i64>().unwrap();
            match direction {
                'R' => {
                    let dist = (100 - current) % 100;
                    if rotate_amount >= dist && dist != 0 {
                        result += 1;
                        result += (rotate_amount - dist) / 100;
                    } else {
                        result += rotate_amount / 100;
                    }
                    current += rotate_amount
                },
                'L' => {
                    let dist = current % 100;
                    if rotate_amount >= dist && dist != 0 {
                        result += 1;
                        result += (rotate_amount - dist) / 100;
                    } else {
                        result += rotate_amount / 100;
                    }

                    current -= rotate_amount 
                },
                _ => {
                    eprintln!("Invalid direction: {}", direction);
                    continue;
                }
            }
            current = current.rem_euclid(100);
        } else {
            println!("error?");
        }
    }
    println!("Result is {}", result)
}
